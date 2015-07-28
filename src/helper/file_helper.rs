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
    client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
    directory_listing: ::directory_listing::DirectoryListing,
}

impl FileHelper {
    /// Create a new FileHelper instance
    pub fn new(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
               directory_listing: ::directory_listing::DirectoryListing) -> FileHelper {
        FileHelper {
            client: client,
            directory_listing: directory_listing,
        }
    }

    /// Helper function to create a file in a directory listing
    /// A writer object is returned, through which the data for the file can be written to the network
    /// The file is actually saved in the directory listing only after `writer.close()` is invoked
    pub fn create(&mut self,
                  name: String,
                  user_metatdata: Option<Vec<u8>>) -> Result<::helper::writer::Writer, ::errors::NfsError> {
        match self.directory_listing.find_file(name.clone()) {
            Some(_) => Err(::errors::NfsError::AlreadyExists),
            None => {
                let file = ::file::File::new(::metadata::Metadata::new(name, user_metatdata), ::self_encryption::datamap::DataMap::None);
                Ok(::helper::writer::Writer::new(self.client.clone(), ::helper::writer::Mode::Overwrite, self.directory_listing.clone(), file))
            },
        }
    }

    /// Helper function to Update content of a file in a directory listing
    /// A writer object is returned, through which the data for the file can be written to the network
    /// The file is actually saved in the directory listing only after `writer.close()` is invoked
    pub fn update(&mut self,
                  file: ::file::File,
                  mode: ::helper::writer::Mode) -> Result<::helper::writer::Writer, ::errors::NfsError> {
        match self.directory_listing.find_file(file.get_name().clone()) {
            Some(_) => Ok(::helper::writer::Writer::new(self.client.clone(), mode, self.directory_listing.clone(), file)),
            None => Err(::errors::NfsError::NotFound),
        }
    }

    /// Updates the file metadata. Returns the updated DirectoryListing
    pub fn update_metadata(&mut self,
                           mut file: ::file::File,
                           user_metadata: Option<Vec<u8>>) -> Result<(), ::errors::NfsError> {
        match self.directory_listing.find_file(file.get_name().clone()) {
            Some(_) => {
                file.get_mut_metadata().set_user_metadata(user_metadata);
                self.directory_listing.upsert_file(file);
                let directory_helper = ::helper::directory_helper::DirectoryHelper::new(self.client.clone());
                directory_helper.update(&self.directory_listing)
            },
            None => Err(::errors::NfsError::NotFound),
        }
    }

    /// Return the versions of a directory containing modified versions of a file
    pub fn get_versions(&mut self,
                        file: &::file::File) -> Result<Vec<::routing::NameType>, String> {
        let mut versions = Vec::<::routing::NameType>::new();
        let directory_helper = ::helper::directory_helper::DirectoryHelper::new(self.client.clone());

        match directory_helper.get_versions(self.directory_listing.get_key()) {
            Ok(sdv_versions) => {
                let mut modified_time = ::time::empty_tm();
                for version_id in sdv_versions {
                    match directory_helper.get_by_version(self.directory_listing.get_key(), self.directory_listing.get_metadata().get_access_level(), version_id.clone()) {
                        Ok(directory_listing) => {
                            match directory_listing.get_files().iter().find(|&entry| entry.get_name() == file.get_name()) {
                                Some(file) => {
                                   if file.get_metadata().get_modified_time() != modified_time {
                                        modified_time = file.get_metadata().get_modified_time();
                                        versions.push(version_id);
                                    }
                                },
                                None => ()
                            }
                        },
                        Err(_) => { () }
                    }
                }
            },
            Err(_) => { () },
        }

        Ok(versions)
    }

}

/*
#[cfg(test)]
mod test {
    use super::*;
    use ::std::ops::Index;

    #[test]
    fn create_read_update() {
        let test_client = ::maidsafe_client::utility::test_utils::get_client().unwrap_or_else(|error| { println!("Error: {}", error); unimplemented!() });
        let client = ::std::sync::Arc::new(::std::sync::Mutex::new(test_client));
        let mut dir_helper = ::helper::DirectoryHelper::new(client.clone());

        let created_dir_id: _;
        {
            let put_result = dir_helper.create("DirName".to_string(),
                                               vec![7u8; 100]);

            assert!(put_result.is_ok());
            created_dir_id = put_result.ok().unwrap();
        }

        let mut dir_listing: _;
        {
            let get_result = dir_helper.get(&created_dir_id);
            assert!(get_result.is_ok());
            dir_listing = get_result.ok().unwrap();
        }

        let mut file_helper = FileHelper::new(client.clone());
        let mut writer: _;
        {
            let result = file_helper.create("Name".to_string(), vec![98u8; 100], &dir_listing);
            assert!(result.is_ok());

            writer = result.ok().unwrap();
        }

        let data = vec![12u8; 20];
        writer.write(&data[..], 0);
        let _ = writer.close();

        {
            let get_result = dir_helper.get(&created_dir_id);
            assert!(get_result.is_ok());
            dir_listing = get_result.ok().unwrap();
        }

        {
            let result = dir_listing.get_files();
            assert_eq!(result.len(), 1);

            let file = result[0].clone();

            let mut reader = ::io::Reader::new(file.clone(), client.clone());
            let rxd_data = reader.read(0, data.len() as u64).ok().unwrap();

            assert_eq!(rxd_data, data);

            {
                let mut writer: _;
                {
                    let result = file_helper.update(result.index(0), &dir_listing, ::io::writer::Mode::Overwrite);
                    assert!(result.is_ok());

                    writer = result.ok().unwrap();
                }

                let data = vec![11u8; 90];
                writer.write(&[11u8; 90], 0);
                let _ = writer.close();

                let get_result = dir_helper.get(&created_dir_id);
                assert!(get_result.is_ok());
                let dir_listing = get_result.ok().unwrap();

                let result = dir_listing.get_files();
                assert_eq!(result.len(), 1);

                let file = result[0].clone();

                let mut reader =  ::io::Reader::new(file.clone(), client.clone());
                let rxd_data = reader.read(0, data.len() as u64).ok().unwrap();

                assert_eq!(rxd_data, data);

                {
                    let versions = file_helper.get_versions(&created_dir_id, &file);
                    assert_eq!(versions.unwrap().len(), 2);
                }
            }
        }
    }
}
*/
