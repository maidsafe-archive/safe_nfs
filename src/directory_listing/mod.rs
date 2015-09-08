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

/// DirectoryInfo holds teh metadata information about the directory
pub mod directory_info;

/// DirectoryListing is the representation of a deserialised Directory in the network
#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct DirectoryListing {
    info           : directory_info::DirectoryInfo,
    sub_directories: Vec<directory_info::DirectoryInfo>,
    files          : Vec<::file::File>,
}

impl DirectoryListing {
    /// Create a new DirectoryListing
    pub fn new(name           : String,
               tag_type       : u64,
               user_metadata  : Vec<u8>,
               versioned      : bool,
               access_level   : ::AccessLevel,
               parent_dir_info: Option<&directory_info::DirectoryInfo>) -> Result<DirectoryListing, ::errors::NfsError> {
        let meta_data = ::metadata::directory_metadata::DirectoryMetadata::new(name,
                                                                               user_metadata,
                                                                               versioned,
                                                                               access_level,
                                                                               parent_dir_info.iter().next().map(|info| {
                                                                                   (info.get_id().clone(),
                                                                                    info.get_type_tag(),
                                                                                    info.get_metadata().is_versioned(),
                                                                                    info.get_metadata().get_access_level().clone())
                                                                               }));
        Ok(DirectoryListing {
            info           : try!(directory_info::DirectoryInfo::new(meta_data, tag_type)),
            sub_directories: Vec::new(),
            files          : Vec::new(),
        })
    }

    /// Get directory_info::DirectoryInfo
    pub fn get_info(&self) -> &directory_info::DirectoryInfo {
        &self.info
    }

    /// Get Directory metadata
    pub fn get_metadata(&self) -> &::metadata::directory_metadata::DirectoryMetadata {
        self.info.get_metadata()
    }

    /// Get Directory metadata in mutable format so that it can also be updated
    #[allow(dead_code)]
    pub fn get_mut_metadata(&mut self) -> &mut ::metadata::directory_metadata::DirectoryMetadata {
        self.info.get_mut_metadata()
    }

    /// If file is present in the DirectoryListing then replace it else insert it
    pub fn upsert_file(&mut self, file: ::file::File) -> Result<(), ::errors::NfsError> {
        let modified_time = file.get_metadata().get_modified_time().clone();
        match self.files.iter().position(|entry| entry.get_name() == file.get_name()) {
            Some(pos) => {
                let mut_file = try!(self.files.get_mut(pos).ok_or(::errors::NfsError::FailedToUpdateFile));
                *mut_file = file;
            },
            None => self.files.push(file),
        };
        self.get_mut_metadata().set_modified_time(modified_time);
        Ok(())
    }

    /// If DirectoryInfo is present in the sub_directories of DirectoryListing then replace it else insert it
    pub fn upsert_sub_directory(&mut self, dir_info: ::directory_listing::directory_info::DirectoryInfo) -> Result<(), ::errors::NfsError> {
        let modified_time = dir_info.get_metadata().get_modified_time().clone();
        match self.sub_directories.iter().position(|entry| entry.get_id() == dir_info.get_id()) {
            Some(pos) => {
                let mut_dir_info = try!(self.sub_directories.get_mut(pos).ok_or(::errors::NfsError::FailedToUpdateDirectory));
                *mut_dir_info = dir_info;
            },
            None => self.sub_directories.push(dir_info),
        };
        self.get_mut_metadata().set_modified_time(modified_time);
        Ok(())
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
    pub fn get_sub_directories(&self) -> &Vec<directory_info::DirectoryInfo> {
        &self.sub_directories
    }

    /// Get all subdirectories in this DirectoryListing with mutability to update the listing of subdirectories
    pub fn get_mut_sub_directories(&mut self) -> &mut Vec<directory_info::DirectoryInfo> {
        &mut self.sub_directories
    }

    /// Get the unique ID that represents this DirectoryListing in the network
    pub fn get_key(&self) ->  (&::routing::NameType, u64, bool, &::AccessLevel) {
        self.info.get_key()
    }

    /// Decrypts a directory listing
    pub fn decrypt(client      : ::std::sync::Arc<::std::sync::Mutex<::safe_client::client::Client>>,
                   directory_id: &::routing::NameType,
                   access_level: &::AccessLevel,
                   data        : Vec<u8>) -> Result<DirectoryListing, ::errors::NfsError> {
        let decrypted_data_map = match *access_level {
            ::AccessLevel::Private => try!(client.lock().unwrap().hybrid_decrypt(&data,
                                                                                 Some(&DirectoryListing::generate_nonce(directory_id)))),
            ::AccessLevel::Public => data,
        };

        let datamap: ::self_encryption::datamap::DataMap = try!(::safe_client::utility::deserialise(&decrypted_data_map));
        let mut se = ::self_encryption::SelfEncryptor::new(::safe_client::SelfEncryptionStorage::new(client.clone()), datamap);
        let length = se.len();
        let serialised_directory_listing = se.read(0, length);
        Ok(try!(::safe_client::utility::deserialise(&serialised_directory_listing)))
    }

    /// Encrypts the directory listing
    pub fn encrypt(&self,
                   client: ::std::sync::Arc<::std::sync::Mutex<::safe_client::client::Client>>) -> Result<Vec<u8>, ::errors::NfsError> {
        let serialised_data = try!(::safe_client::utility::serialise(&self));
        let mut se = ::self_encryption::SelfEncryptor::new(::safe_client::SelfEncryptionStorage::new(client.clone()), ::self_encryption::datamap::DataMap::None);
        se.write(&serialised_data, 0);
        let datamap = se.close();
        let serialised_data_map = try!(::safe_client::utility::serialise(&datamap));
        Ok(try!(client.lock().unwrap().hybrid_encrypt(&serialised_data_map, Some(&DirectoryListing::generate_nonce(&self.get_key().0)))))
    }

    /// Get DirectoryInfo of sub_directory within a DirectoryListing.
    /// Returns the Option<DirectoryInfo> for the directory_name from the DirectoryListing
    pub fn find_file(&self,
                     file_name: &String) -> Option<&::file::File> {
        self.get_files().iter().find(|file| *file.get_name() == *file_name)
    }

    /// Get DirectoryInfo of sub_directory within a DirectoryListing.
    /// Returns the Option<DirectoryInfo> for the directory_name from the DirectoryListing
    pub fn find_sub_directory(&self,
                              directory_name: &String) -> Option<&directory_info::DirectoryInfo> {
        self.get_sub_directories().iter().find(|info| *info.get_name() == *directory_name)
    }

    /// Returns the position of the sub-directory in the subdirectory list
    /// None is returned if the sub-directory is not found
    pub fn get_sub_directory_index(&self,
                                   directory_name: &String) -> Option<usize> {
        self.get_sub_directories().iter().position(|dir_info| *dir_info.get_name() == *directory_name)
    }

    /// Returns the position of the files in the file list
    /// None is returned if the file is not found
    pub fn get_file_index(&self,
                          file_name: &String) -> Option<usize> {
        self.get_files().iter().position(|file| *file.get_name() == *file_name)
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
    use super::*;

    #[test]
    fn serialise() {
        let obj_before = eval_result!(DirectoryListing::new("Home".to_string(),
                                                            10,
                                                            "{mime:\"application/json\"}".to_string().into_bytes(),
                                                            true,
                                                            ::AccessLevel::Private, None));

        let serialised_data = eval_result!(::safe_client::utility::serialise(&obj_before));
        let obj_after = eval_result!(::safe_client::utility::deserialise(&serialised_data));
        assert_eq!(obj_before, obj_after);
    }
}
