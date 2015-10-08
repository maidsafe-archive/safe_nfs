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

/// DirectoryListing is the representation of a deserialised Directory in the network
#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct DirectoryListing {
    metadata       : ::metadata::directory_metadata::DirectoryMetadata,
    sub_directories: Vec<::metadata::directory_metadata::DirectoryMetadata>,
    files          : Vec<::file::File>,
}

impl DirectoryListing {
    /// Create a new DirectoryListing
    pub fn new(name           : String,
               tag_type       : u64,
               user_metadata  : Vec<u8>,
               versioned      : bool,
               access_level   : ::AccessLevel,
               parent_dir_key: Option<::metadata::directory_key::DirectoryKey>) -> Result<DirectoryListing, ::errors::NfsError> {
        let meta_data = try!(::metadata::directory_metadata::DirectoryMetadata::new(name,
                                                                                    tag_type,
                                                                                    versioned,
                                                                                    access_level,
                                                                                    user_metadata,
                                                                                    parent_dir_key));
        Ok(DirectoryListing {
            metadata       : meta_data,
            sub_directories: Vec::new(),
            files          : Vec::new(),
        })
    }

    /// Returns the DirectoryKey representing the DirectoryListing
    pub fn get_key(&self) -> &::metadata::directory_key::DirectoryKey {
        &self.metadata.get_key()
    }

    /// Get Directory metadata
    pub fn get_metadata(&self) -> &::metadata::directory_metadata::DirectoryMetadata {
        &self.metadata
    }

    /// Get Directory metadata in mutable format so that it can also be updated
    pub fn get_mut_metadata(&mut self) -> &mut ::metadata::directory_metadata::DirectoryMetadata {
        &mut self.metadata
    }

    /// Get all files in this DirectoryListing
    pub fn get_files(&self) -> &Vec<::file::File> {
        &self.files
    }

    /// Get all files in this DirectoryListing with mutability to update the listing of files
    pub fn get_mut_files(&mut self) -> &mut Vec<::file::File> {
        &mut self.files
    }

    /// Get all subdirectories in this DirectoryListing
    pub fn get_sub_directories(&self) -> &Vec<::metadata::directory_metadata::DirectoryMetadata> {
        &self.sub_directories
    }

    /// Get all subdirectories in this DirectoryListing with mutability to update the listing of subdirectories
    pub fn get_mut_sub_directories(&mut self) -> &mut Vec<::metadata::directory_metadata::DirectoryMetadata> {
        &mut self.sub_directories
    }

    /// Decrypts a directory listing
    pub fn decrypt(client      : ::std::sync::Arc<::std::sync::Mutex<::safe_client::client::Client>>,
                   directory_id: &::routing::NameType,
                   data        : Vec<u8>) -> Result<DirectoryListing, ::errors::NfsError> {
        let decrypted_data_map = try!(eval_result!(client.lock()).hybrid_decrypt(&data,
                                                                                 Some(&DirectoryListing::generate_nonce(directory_id))));
        let datamap: ::self_encryption::datamap::DataMap = try!(::safe_client::utility::deserialise(&decrypted_data_map));
        let mut se = ::self_encryption::SelfEncryptor::new(::safe_client::SelfEncryptionStorage::new(client.clone()), datamap);
        let length = se.len();
        debug!("Reading encrypted storage of length {:?} ...", length);
        let serialised_directory_listing = se.read(0, length);
        Ok(try!(::safe_client::utility::deserialise(&serialised_directory_listing)))
    }

    /// Encrypts the directory listing
    pub fn encrypt(&self,
                   client: ::std::sync::Arc<::std::sync::Mutex<::safe_client::client::Client>>) -> Result<Vec<u8>, ::errors::NfsError> {
        let serialised_data = try!(::safe_client::utility::serialise(&self));
        let mut se = ::self_encryption::SelfEncryptor::new(::safe_client::SelfEncryptionStorage::new(client.clone()), ::self_encryption::datamap::DataMap::None);
        debug!("Writing to storage using self encryption ...");
        se.write(&serialised_data, 0);
        let datamap = se.close();
        let serialised_data_map = try!(::safe_client::utility::serialise(&datamap));
        Ok(try!(eval_result!(client.lock()).hybrid_encrypt(&serialised_data_map, Some(&DirectoryListing::generate_nonce(&self.get_key().get_id())))))
    }

    /// Get DirectoryInfo of sub_directory within a DirectoryListing.
    /// Returns the Option<DirectoryInfo> for the directory_name from the DirectoryListing
    pub fn find_file(&self,
                     file_name: &String) -> Option<&::file::File> {
        self.get_files().iter().find(|file| *file.get_name() == *file_name)
    }

    /// Get DirectoryInfo of sub_directory within a DirectoryListing.
    /// Returns the Option<DirectoryInfo> for the directory_name from the DirectoryListing
    pub fn find_file_by_id(&self,
                           id: &::routing::NameType) -> Option<&::file::File> {
        self.get_files().iter().find(|file| *file.get_id() == *id)
    }

    /// Get DirectoryInfo of sub_directory within a DirectoryListing.
    /// Returns the Option<DirectoryInfo> for the directory_name from the DirectoryListing
    pub fn find_sub_directory(&self,
                              directory_name: &String) -> Option<&::metadata::directory_metadata::DirectoryMetadata> {
        self.get_sub_directories().iter().find(|info| *info.get_name() == *directory_name)
    }

    /// Get DirectoryInfo of sub_directory within a DirectoryListing.
    /// Returns the Option<DirectoryInfo> for the directory_name from the DirectoryListing
    pub fn find_sub_directory_by_id(&self,
                                    id: &::routing::NameType) -> Option<&::metadata::directory_metadata::DirectoryMetadata> {
        self.get_sub_directories().iter().find(|info| *info.get_id() == *id)
    }

    /// If file is present in the DirectoryListing then replace it else insert it
    pub fn upsert_file(&mut self, file: ::file::File) {
        let modified_time = file.get_metadata().get_modified_time().clone();
        // TODO try using the below approach for efficiency - also try the same in upsert_sub_directory
        // if let Some(mut existing_file) = self.files.iter_mut().find(|entry| *entry.get_name() == *file.get_name()) {
        // *existing_file = file;
        if let Some(index) = self.files.iter().position(|entry| *entry.get_id() == *file.get_id()) {
            debug!("Replacing file in directory listing ...");
            let mut existing = eval_option!(self.files.get_mut(index), "Programming Error - Report this as a Bug.");
            *existing = file;
        } else {
            debug!("Adding file to directory listing ...");
            self.files.push(file);
        }
        self.get_mut_metadata().set_modified_time(modified_time)
    }

    /// If DirectoryMetadata is present in the sub_directories of DirectoryListing then replace it else insert it
    pub fn upsert_sub_directory(&mut self, directory_metadata: ::metadata::directory_metadata::DirectoryMetadata) {
        let modified_time = directory_metadata.get_modified_time().clone();
        if let Some(index) = self.sub_directories.iter().position(|entry| *entry.get_key().get_id() == *directory_metadata.get_key().get_id()) {
            debug!("Replacing directory listing metadata ...");
            let mut existing = eval_option!(self.sub_directories.get_mut(index), "Programming Error - Report this as a Bug.");
            *existing = directory_metadata;
        } else {
            debug!("Adding metadata to directory listing ...");
            self.sub_directories.push(directory_metadata);
        }
        self.get_mut_metadata().set_modified_time(modified_time);
    }

    /// Remove a sub_directory
    pub fn remove_sub_directory(&mut self, directory_name: &String) -> Result<(), ::errors::NfsError> {
        let index = try!(self.get_sub_directories().iter().position(|dir_info| *dir_info.get_name() == *directory_name).ok_or(::errors::NfsError::DirectoryNotFound));
        debug!("Removing sub directory at index {:?} ...", index);
        let _ = self.get_mut_sub_directories().remove(index);
        Ok(())
    }

    /// Remove a file
    pub fn remove_file(&mut self, file_name: &String) -> Result<(), ::errors::NfsError> {
        let index = try!(self.get_files().iter().position(|file| *file.get_name() == *file_name).ok_or(::errors::NfsError::FileNotFound));
        debug!("Removing file at index {:?} ...", index);
        let _ = self.get_mut_files().remove(index);
        Ok(())
    }

    /// Generates a nonce based on the directory_id
    pub fn generate_nonce(directory_id: &::routing::NameType) -> ::sodiumoxide::crypto::box_::Nonce {
        let mut nonce = [0u8; ::sodiumoxide::crypto::box_::NONCEBYTES];
        let min_length = ::std::cmp::min(nonce.len(), directory_id.0.len());
        for i in 0..min_length {
            nonce[i] = directory_id.0[i];
        }
        ::sodiumoxide::crypto::box_::Nonce(nonce)
    }
}

#[cfg(test)]
mod test {
    use super::DirectoryListing;

    #[test]
    fn serialise_and_deserialise_directory_listing() {
        let obj_before = eval_result!(DirectoryListing::new("Home".to_string(),
                                                            10,
                                                            "some metadata about the directory".to_string().into_bytes(),
                                                            true,
                                                            ::AccessLevel::Private,
                                                            None));

        let serialised_data = eval_result!(::safe_client::utility::serialise(&obj_before));
        let obj_after = eval_result!(::safe_client::utility::deserialise(&serialised_data));
        assert_eq!(obj_before, obj_after);
    }

    #[test]
    fn encrypt_and_decrypt_directory_listing() {
        let test_client = eval_result!(::safe_client::utility::test_utils::get_client());
        let client = ::std::sync::Arc::new(::std::sync::Mutex::new(test_client));
        let directory_listing = eval_result!(DirectoryListing::new("Home".to_string(),
                                                                   10,
                                                                   Vec::new(),
                                                                   true,
                                                                   ::AccessLevel::Private,
                                                                   None));
        let encrypted_data = eval_result!(directory_listing.encrypt(client.clone()));
        let decrypted_listing = eval_result!(DirectoryListing::decrypt(client.clone(),
                                                                       directory_listing.get_key().get_id(),
                                                                       encrypted_data));
        assert_eq!(directory_listing, decrypted_listing);
    }

    #[test]
    fn find_upsert_remove_file() {
        let mut directory_listing = eval_result!(DirectoryListing::new("Home".to_string(),
                                                                       10,
                                                                       Vec::new(),
                                                                       true,
                                                                       ::AccessLevel::Private,
                                                                       None));
        let mut file = eval_result!(::file::File::new(::metadata::file_metadata::FileMetadata::new("index.html".to_string(), Vec::new()),
                                                      ::self_encryption::datamap::DataMap::None));
        assert!(directory_listing.find_file(file.get_name()).is_none());
        directory_listing.upsert_file(file.clone());
        assert!(directory_listing.find_file(file.get_name()).is_some());

        file.get_mut_metadata().set_name("home.html".to_string());
        directory_listing.upsert_file(file.clone());
        assert_eq!(directory_listing.get_files().len(), 1);
        let file2 = eval_result!(::file::File::new(::metadata::file_metadata::FileMetadata::new("demo.html".to_string(), Vec::new()),
                                                   ::self_encryption::datamap::DataMap::None));
        directory_listing.upsert_file(file2.clone());
        assert_eq!(directory_listing.get_files().len(), 2);

        let _ = eval_option!(directory_listing.find_file(file.get_name()), "File not found");
        let _ = eval_option!(directory_listing.find_file(file2.get_name()), "File not found");

        let _ = eval_result!(directory_listing.remove_file(file.get_metadata().get_name()));
        assert!(directory_listing.find_file(file.get_name()).is_none());
        assert!(directory_listing.find_file(file2.get_name()).is_some());
        assert_eq!(directory_listing.get_files().len(), 1);

        let _ = eval_result!(directory_listing.remove_file(file2.get_metadata().get_name()));
        assert_eq!(directory_listing.get_files().len(), 0);
    }

    #[test]
    fn find_upsert_remove_directory() {
        let mut directory_listing = eval_result!(DirectoryListing::new("Home".to_string(),
                                                                       10,
                                                                       Vec::new(),
                                                                       true,
                                                                       ::AccessLevel::Private,
                                                                       None));
        let mut sub_directory = eval_result!(DirectoryListing::new("Child one".to_string(),
                                                                   10,
                                                                   Vec::new(),
                                                                   true,
                                                                   ::AccessLevel::Private,
                                                                   None));
        assert!(directory_listing.find_sub_directory(sub_directory.get_metadata().get_name()).is_none());
        directory_listing.upsert_sub_directory(sub_directory.get_metadata().clone());
        assert!(directory_listing.find_sub_directory(sub_directory.get_metadata().get_name()).is_some());

        sub_directory.get_mut_metadata().set_name("Child_1".to_string());
        directory_listing.upsert_sub_directory(sub_directory.get_metadata().clone());
        assert_eq!(directory_listing.get_sub_directories().len(), 1);
        let sub_directory_two = eval_result!(DirectoryListing::new("Child Two".to_string(),
                                                                   10,
                                                                   Vec::new(),
                                                                   true,
                                                                   ::AccessLevel::Private,
                                                                   None));
        directory_listing.upsert_sub_directory(sub_directory_two.get_metadata().clone());
        assert_eq!(directory_listing.get_sub_directories().len(), 2);

        let _ = eval_option!(directory_listing.find_sub_directory(sub_directory.get_metadata().get_name()), "Directory not found");
        let _ = eval_option!(directory_listing.find_sub_directory(sub_directory_two.get_metadata().get_name()), "Directory not found");

        let _ = eval_result!(directory_listing.remove_sub_directory(sub_directory.get_metadata().get_name()));
        assert!(directory_listing.find_sub_directory(sub_directory.get_metadata().get_name()).is_none());
        assert!(directory_listing.find_sub_directory(sub_directory_two.get_metadata().get_name()).is_some());
        assert_eq!(directory_listing.get_sub_directories().len(), 1);

        let _ = eval_result!(directory_listing.remove_sub_directory(sub_directory_two.get_metadata().get_name()));
        assert_eq!(directory_listing.get_sub_directories().len(), 0);
    }

}
