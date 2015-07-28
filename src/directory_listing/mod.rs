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

pub mod directory_info;

#[derive(RustcEncodable, RustcDecodable, PartialEq, Eq, PartialOrd, Ord, Clone)]
/// DirectoryListing is the representation of a deserialised Directory in the network
pub struct DirectoryListing {
    info: ::directory_listing::directory_info::DirectoryInfo,
    sub_directories: Vec<::directory_listing::directory_info::DirectoryInfo>,
    files: Vec<::file::File>,
}

impl DirectoryListing {
    /// Create a new DirectoryListing
    pub fn new(name: String, tag_type: u64, user_metadata: Option<Vec<u8>>,
               versioned: bool, share_level: ::AccessLevel) -> DirectoryListing {
        DirectoryListing {
            info: ::directory_listing::directory_info::DirectoryInfo::new(::directory_metadata::DirectoryMetadata::new(name,
                                                                                                                       tag_type,
                                                                                                                       user_metadata,
                                                                                                                       versioned,
                                                                                                                       share_level)),
            sub_directories: Vec::new(),
            files: Vec::new(),
        }
    }

    /// Get ::directory_listing::directory_info::DirectoryInfo
    pub fn get_info(&self) -> &::directory_listing::directory_info::DirectoryInfo {
        &self.info
    }

    #[allow(dead_code)]
    /// Get Directory metadata in mutable format so that it can also be updated
    pub fn get_mut_metadata(&mut self) -> &mut ::directory_metadata::DirectoryMetadata {
        self.info.get_mut_metadata()
    }

    /// Get Directory metadata
    pub fn get_metadata(&self) -> &::directory_metadata::DirectoryMetadata {
        self.info.get_metadata()
    }

    // pub fn get_parent_dir_id(&self) -> &::routing::NameType {
    //     self.info.get_parent_dir_id()
    // }

    /// If file is present in the DirectoryListing then replace it else insert it
    pub fn upsert_file(&mut self, file: ::file::File) {
        match self.files.iter().position(|entry| entry.get_name() == file.get_name()) {
            Some(pos) => *self.files.get_mut(pos).unwrap() = file,
            None => self.files.push(file),
        }
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
    pub fn get_sub_directories(&self) -> &Vec<::directory_listing::directory_info::DirectoryInfo> {
        &self.sub_directories
    }

    /// Get all subdirectories in this DirectoryListing with mutability to update the listing of subdirectories
    pub fn get_mut_sub_directories(&mut self) -> &mut Vec<::directory_listing::directory_info::DirectoryInfo> {
        &mut self.sub_directories
    }

    /// Get the unique ID that represents this DirectoryListing in the network
    pub fn get_key(&self) ->  (&::routing::NameType, u64) {
        self.info.get_key()
    }

    /// Decrypts a directory listing
    pub fn decrypt(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
                   directory_id: &::routing::NameType,
                   share_level: ::AccessLevel,
                   data: Vec<u8>) -> Result<::directory_listing::DirectoryListing, ::errors::NfsError> {
         let decrypted_data_map = match share_level {
             ::AccessLevel::Private => try!(client.lock().unwrap().hybrid_decrypt(&data[..],
                                                                                 Some(&::directory_listing::DirectoryListing::generate_nonce(directory_id)))),
             ::AccessLevel::Public => data,
         };
         let datamap: ::self_encryption::datamap::DataMap = try!(::maidsafe_client::utility::deserialise(&decrypted_data_map));
         let mut se = ::self_encryption::SelfEncryptor::new(::maidsafe_client::SelfEncryptionStorage::new(client.clone()), datamap);
         let length = se.len();
         let serialised_directory_listing = se.read(0, length);
         Ok(try!(::maidsafe_client::utility::deserialise(&serialised_directory_listing)))
    }

    /// Encrypts the directory listing
    pub fn encrypt(&self,
                   client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>) -> Result<Vec<u8>, ::errors::NfsError> {
        let serialised_data = try!(::maidsafe_client::utility::serialise(&self));
        let mut se = ::self_encryption::SelfEncryptor::new(::maidsafe_client::SelfEncryptionStorage::new(client.clone()), ::self_encryption::datamap::DataMap::None);
        se.write(&serialised_data, 0);
        let datamap = se.close();
        let serialised_data_map = try!(::maidsafe_client::utility::serialise(&datamap));
        Ok(try!(client.lock().unwrap().hybrid_encrypt(&serialised_data_map, Some(&::directory_listing::DirectoryListing::generate_nonce(self.get_key().0)))))
    }

    /// Get DirectoryInfo of sub_directory within a DirectoryListing.
    /// Returns the Option<DirectoryInfo> for the directory_name from the DirectoryListing
    pub fn find_file(&self,
                     file_name: String) -> Option<&::file::File> {
        self.get_files().iter().find(|file| *file.get_name() == file_name)
    }

    /// Get DirectoryInfo of sub_directory within a DirectoryListing.
    /// Returns the Option<DirectoryInfo> for the directory_name from the DirectoryListing
    pub fn find_sub_directory(&self,
                              directory_name: String) -> Option<&::directory_listing::directory_info::DirectoryInfo> {
        self.get_sub_directories().iter().find(|info| *info.get_name() == directory_name)
    }

    pub fn get_sub_directory_index(&self,
                              directory_name: String) -> Option<usize> {
        self.get_sub_directories().iter().position(|dir_info| *dir_info.get_name() == directory_name)
    }

    /// Generates a nonce based on the directory_id
    pub fn generate_nonce(directory_id: &::routing::NameType) -> ::sodiumoxide::crypto::box_::Nonce {
        let mut nonce = [0u8; ::sodiumoxide::crypto::box_::NONCEBYTES];
        for i in 0..nonce.len() {
            nonce[i] = directory_id.0[i];
        }
        ::sodiumoxide::crypto::box_::Nonce(nonce)
    }

}

impl ::std::fmt::Debug for DirectoryListing {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "info: {}", self.info)
    }
}

impl ::std::fmt::Display for DirectoryListing {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "info: {}", self.info)
    }
}
/*
#[cfg(test)]
mod test {
    use super::*;
    use cbor;

    #[test]
    fn serialise() {
        let obj_before = DirectoryListing::new("Home".to_string(), Some("{mime:\"application/json\"}".to_string().into_bytes()), true, ::AccessLevel::Private);

        let mut e = cbor::Encoder::from_memory();
        e.encode(&[&obj_before]).unwrap();

        let mut d = cbor::Decoder::from_bytes(e.as_bytes());
        let obj_after: DirectoryListing = d.decode().next().unwrap().unwrap();

        assert_eq!(obj_before, obj_after);
    }
}
*/
