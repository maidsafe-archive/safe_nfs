// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

/// File provides helper functions to perform Operations on Files
pub struct FileHelper {
    client: ::std::sync::Arc<::std::sync::Mutex<::safe_client::client::Client>>,
}

impl FileHelper {
    /// Create a new FileHelper instance
    pub fn new(client: ::std::sync::Arc<::std::sync::Mutex<::safe_client::client::Client>>) -> FileHelper {
        FileHelper {
            client: client,
        }
    }

    /// Helper function to create a file in a directory listing
    /// A writer object is returned, through which the data for the file can be written to the network
    /// The file is actually saved in the directory listing only after `writer.close()` is invoked
    pub fn create(&self,
                  name            : String,
                  user_metatdata  : Vec<u8>,
                  parent_directory: ::directory_listing::DirectoryListing) -> Result<::helper::writer::Writer, ::errors::NfsError> {
        match parent_directory.find_file(&name) {
            Some(_) => Err(::errors::NfsError::AlreadyExists),
            None => {
                let file = ::file::File::new(::metadata::file_metadata::FileMetadata::new(name, user_metatdata), ::self_encryption::datamap::DataMap::None);
                Ok(::helper::writer::Writer::new(self.client.clone(), ::helper::writer::Mode::Overwrite, parent_directory, file))
            },
        }
    }

    /// Delete a file from the DirectoryListing
    pub fn delete(&self,
                  file_name       : String,
                  parent_directory: &mut ::directory_listing::DirectoryListing) -> Result<(), ::errors::NfsError> {
         let index = try!(parent_directory.get_file_index(&file_name).ok_or(::errors::NfsError::FileNotFound));
         parent_directory.get_mut_files().remove(index);
         let directory_helper = ::helper::directory_helper::DirectoryHelper::new(self.client.clone());
         try!(directory_helper.update(&parent_directory));
         Ok(())
    }

    /// Helper function to Update content of a file in a directory listing
    /// A writer object is returned, through which the data for the file can be written to the network
    /// The file is actually saved in the directory listing only after `writer.close()` is invoked
    pub fn update(&self,
                  file            : ::file::File,
                  mode            : ::helper::writer::Mode,
                  parent_directory: ::directory_listing::DirectoryListing) -> Result<::helper::writer::Writer, ::errors::NfsError> {
        // TODO improve by comparing file::metadata::parent_info with parent_dir_listing::info and
        // return new error NotAValidParent
        try!(parent_directory.find_file(file.get_name()).ok_or(::errors::NfsError::FileNotFound));
        Ok(::helper::writer::Writer::new(self.client.clone(), mode, parent_directory, file))
    }

    /// Updates the file metadata. Returns the updated parent DirectoryListing
    pub fn update_metadata(&self,
                           mut file            : ::file::File,
                           user_metadata       : Vec<u8>,
                           mut parent_directory: ::directory_listing::DirectoryListing) -> Result<::directory_listing::DirectoryListing, ::errors::NfsError> {
        // TODO Should we remove the below validation?
        try!(parent_directory.find_file(file.get_name()).ok_or(::errors::NfsError::FileNotFound));
        file.get_mut_metadata().set_user_metadata(user_metadata);
        try!(parent_directory.upsert_file(file));
        let directory_helper = ::helper::directory_helper::DirectoryHelper::new(self.client.clone());
        directory_helper.update(&parent_directory)
    }

    /// Return the versions of a directory containing modified versions of a file
    pub fn get_versions(&self,
                        file            : &::file::File,
                        parent_directory: &::directory_listing::DirectoryListing) -> Result<Vec<::file::File>, ::errors::NfsError> {
        let mut versions = Vec::<::file::File>::new();
        let directory_helper = ::helper::directory_helper::DirectoryHelper::new(self.client.clone());

        let sdv_versions = try!(directory_helper.get_versions(parent_directory.get_key()));
        let mut modified_time = ::time::empty_tm();
        for version_id in sdv_versions {
            let directory_listing = try!(directory_helper.get_by_version(parent_directory.get_key(),
                                                                         parent_directory.get_metadata().get_access_level(),
                                                                         version_id.clone()));
            if let Some(file) = directory_listing.get_files().iter().find(|&entry| entry.get_name() == file.get_name()) {
                if *file.get_metadata().get_modified_time() != modified_time {
                     modified_time = file.get_metadata().get_modified_time().clone();
                     versions.push(file.clone());
                 }
            }
        }
        Ok(versions)
    }

    /// Returns a reader for reading the file contents
    pub fn read<'a>(&self, file: &'a ::file::File) -> ::helper::reader::Reader<'a> {
        ::helper::reader::Reader::new(self.client.clone(), file)
    }
}

#[cfg(test)]
mod test {
    fn get_client() -> ::std::sync::Arc<::std::sync::Mutex<::safe_client::client::Client>> {
        let test_client = eval_result!(::safe_client::utility::test_utils::get_client());
        ::std::sync::Arc::new(::std::sync::Mutex::new(test_client))
    }

    #[test]
    fn file_crud() {
        let client = get_client();
        let dir_helper = ::helper::directory_helper::DirectoryHelper::new(client.clone());
        let mut directory = eval_result!(dir_helper.create("DirName".to_string(),
                                                           ::VERSIONED_DIRECTORY_LISTING_TAG,
                                                           Vec::new(),
                                                           true,
                                                           ::AccessLevel::Private,
                                                           None));
        let file_helper = ::helper::file_helper::FileHelper::new(client.clone());
        let file_name = "hello.txt".to_string();
        { // create
            let mut writer = eval_result!(file_helper.create(file_name.clone(), Vec::new(), directory));
            writer.write(&vec![0u8; 100], 0);
            directory = eval_result!(writer.close());
            assert!(directory.find_file(&file_name).is_some());
        }
        {// read
            let file = eval_option!(directory.find_file(&file_name), "File not found");
            let mut reader = file_helper.read(file);
            let size = reader.size();
            assert_eq!(eval_result!(reader.read(0, size)), vec![0u8; 100]);
        }
        {// update - full rewrite
            let file = eval_option!(directory.find_file(&file_name).map(|file| file.clone()), "File not found");
            let mut writer = eval_result!(file_helper.update(file, ::helper::writer::Mode::Overwrite, directory));
            writer.write(&vec![1u8; 50], 0);
            directory = eval_result!(writer.close());
            let file = eval_option!(directory.find_file(&file_name), "File not found");
            let mut reader = file_helper.read(file);
            let size = reader.size();
            assert_eq!(eval_result!(reader.read(0, size)), vec![1u8; 50]);
        }
        {// update - partial rewrite
            let file = eval_option!(directory.find_file(&file_name).map(|file| file.clone()), "File not found");
            let mut writer = eval_result!(file_helper.update(file, ::helper::writer::Mode::Modify, directory));
            writer.write(&vec![2u8; 10], 0);
            directory = eval_result!(writer.close());
            let file = eval_option!(directory.find_file(&file_name), "File not found");
            let mut reader = file_helper.read(file);
            let size = reader.size();
            let data = eval_result!(reader.read(0, size));
            assert_eq!(&data[0..10], [2u8; 10]);
            assert_eq!(&data[10..20], [1u8; 10]);
        }
        {// versions
            let file = eval_option!(directory.find_file(&file_name).map(|file| file.clone()), "File not found");
            let versions = eval_result!(file_helper.get_versions(&file, &directory));
            assert_eq!(versions.len(), 3);
        }
        {// Update Metadata
            let file = eval_option!(directory.find_file(&file_name).map(|file| file.clone()), "File not found");
            directory = eval_result!(file_helper.update_metadata(file, vec![12u8; 10], directory));
            let file = eval_option!(directory.find_file(&file_name).map(|file| file.clone()), "File not found");
            assert_eq!(*file.get_metadata().get_user_metadata(), vec![12u8; 10]);
        }
        {// Delete
            eval_result!(file_helper.delete(file_name.clone(), &mut directory));
            assert!(directory.find_file(&file_name).is_none());
        }
    }
}
