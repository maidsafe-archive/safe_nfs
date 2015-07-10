// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences".to_string()).
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

use ::maidsafe_types::TypeTag;
use ::routing::sendable::Sendable;

/// DirectoryHelper provides helper functions to perform Operations on Directory
pub struct DirectoryHelper {
    client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
}

impl DirectoryHelper {
    /// Create a new DirectoryHelper instance
    pub fn new(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>) -> DirectoryHelper {
        DirectoryHelper {
            client: client,
        }
    }

    pub fn init_root_dir(&mut self) -> Result<::routing::NameType, String> {
        self.create("root".to_string(), Vec::new()).and_then(|dir_id| {
                self.create("SAFE-Drive".to_string(), Vec::new()).and_then(|safe_drive_id| {
                        self.get(&safe_drive_id).and_then(|safe_dir| {
                            self.get(&dir_id).and_then(|mut root_dir| {
                                    root_dir.get_mut_sub_directories().push(safe_dir.get_info().clone());
                                    self.update(&root_dir).and_then(|_| {Ok(dir_id)})
                                })
                        })
                    })
            })
    }

    /// Creates a Directory in the network.
    pub fn create(&mut self,
                  directory_name: String,
                  user_metadata: Vec<u8>) -> Result<::routing::NameType, String> {
        let directory = ::directory_listing::DirectoryListing::new(directory_name, user_metadata);
        let mut sdv: ::maidsafe_types::StructuredData = ::maidsafe_types::StructuredData::new(directory.get_id().clone(),
                                                                                          self.client.lock().unwrap().get_owner(),
                                                                                          Vec::new());
        match self.save_directory(&mut sdv, &directory) {
            Ok(_) => Ok(directory.get_id().clone()),
            Err(err) => Err(err),
        }
    }

    /// Updates an existing DirectoryListing in the network.
    pub fn update(&mut self, directory: &::directory_listing::DirectoryListing) -> Result<(), String> {
        let structured_data_type_id = ::maidsafe_types::data::StructuredDataTypeTag;
        match self.network_get(structured_data_type_id.type_tag(), directory.get_id()) {
            Ok(serialised_sdv) => {
                let mut sdv: ::maidsafe_types::StructuredData = match ::maidsafe_client::utility::deserialise_parser(serialised_sdv) {
                    ::maidsafe_client::data_parser::Parser::StructuredData(obj) => obj,
                    _ => return Err("Unexpected type".to_string())
                };
                self.save_directory(&mut sdv, directory)
            },
            Err(_) => Err("Network IO Error".to_string()),
        }
    }

    /// Return the versions of the directory
    pub fn get_versions(&mut self, directory_id: &::routing::NameType) -> Result<Vec<::routing::NameType>, String> {
        let structured_data_type_id = ::maidsafe_types::data::StructuredDataTypeTag;
        match self.network_get(structured_data_type_id.type_tag(), directory_id) {
            Ok(serialised_sdv) => {
                let sdv: ::maidsafe_types::StructuredData = match ::maidsafe_client::utility::deserialise_parser(serialised_sdv) {
                    ::maidsafe_client::data_parser::Parser::StructuredData(obj) => obj,
                    _ => return Err("Unexpected type".to_string())
                };
                Ok(sdv.value())
            },
            Err(_) => Err("Network IO Error".to_string()),
        }
    }

    /// Return the DirectoryListing for the specified version
    pub fn get_by_version(&mut self,
                          directory_id: &::routing::NameType,
                          version: &::routing::NameType) -> Result<::directory_listing::DirectoryListing, String> {
        let structured_data_type_id = ::maidsafe_types::data::StructuredDataTypeTag;
        match self.network_get(structured_data_type_id.type_tag(), directory_id) {
            Ok(serialised_sdv) => {
                let sdv: ::maidsafe_types::StructuredData = match ::maidsafe_client::utility::deserialise_parser(serialised_sdv) {
                    ::maidsafe_client::data_parser::Parser::StructuredData(obj) => obj,
                    _ => return Err("Unexpected type".to_string())
                };
                match sdv.value().iter().find(|v| *v == version) {
                    Some(version) => {
                            self.get_directory_version(directory_id, version)
                        },
                    None => Err("Could not find data".to_string())
                }
            },
            Err(_) => Err("Network IO Error".to_string()),
        }
    }

    /// Return the DirectoryListing for the latest version
    pub fn get(&mut self, directory_id: &::routing::NameType) -> Result<::directory_listing::DirectoryListing, String> {
        let structured_data_type_id = ::maidsafe_types::data::StructuredDataTypeTag;
        match self.network_get(structured_data_type_id.type_tag(), directory_id) {
            Ok(serialised_sdv) => {
                let sdv: ::maidsafe_types::StructuredData = match ::maidsafe_client::utility::deserialise_parser(serialised_sdv) {
                    ::maidsafe_client::data_parser::Parser::StructuredData(obj) => obj,
                    _ => return Err("Unexpected type".to_string())
                };
                match sdv.value().last() {
                    Some(version) => {
                            self.get_directory_version(directory_id, version)
                        },
                    None => Err("Could not find data".to_string())
                }
            },
            Err(_) => Err("Network IO Error".to_string()),
        }
    }

    fn save_directory(&self,
                      sdv: &mut ::maidsafe_types::StructuredData,
                      directory: &::directory_listing::DirectoryListing) -> Result<(), String> {
        let mut se = ::self_encryption::SelfEncryptor::new(::std::sync::Arc::new(::io::NetworkStorage::new(self.client.clone())), ::self_encryption::datamap::DataMap::None);
        se.write(&::maidsafe_client::utility::serialise(directory.clone())[..], 0);
        let datamap = se.close();

        let encrypt_result: _;
        {
            let client = self.client.lock().unwrap();
            encrypt_result = client.hybrid_encrypt(&::maidsafe_client::utility::serialise(datamap)[..], self.get_nonce(directory.get_id()));
        }

        match encrypt_result {
            Ok(encrypted_data) => {
                let immutable_data = ::maidsafe_types::ImmutableData::new(encrypted_data);
                let name = immutable_data.name();
                match self.network_put(immutable_data) {
                    Ok(_) => {
                        let mut versions = sdv.value();
                        versions.push(name);
                        sdv.set_value(versions);
                        match self.network_put(sdv.clone()){
                            Ok(_) => Ok(()),
                            Err(_) => Err("Failed to update directory version".to_string()),
                        }
                    },
                    Err(_) => Err("IO Error".to_string()),
                }
            },
            Err(_) => Err("Encryption failed".to_string()),
        }
    }

    fn get_directory_version(&self, directory_id: &::routing::NameType,
                             version: &::routing::NameType) -> Result<::directory_listing::DirectoryListing, String> {
        let immutable_data_type_id = ::maidsafe_types::data::ImmutableDataTypeTag;
        match self.network_get(immutable_data_type_id.type_tag(), &version) {
            Ok(serialised_data) => {
                let imm: ::maidsafe_types::ImmutableData = match ::maidsafe_client::utility::deserialise_parser(serialised_data) {
                    ::maidsafe_client::data_parser::Parser::ImmutableData(obj) => obj,
                    _ => return Err("Unexpected type".to_string())
                };
                let client_mutex = self.client.clone();
                let client = client_mutex.lock().unwrap();
                match client.hybrid_decrypt(&imm.value()[..], self.get_nonce(directory_id)) {
                    Some(decrypted_data) => {
                        Ok(self.deserialise_directory(decrypted_data))
                    },
                    None => return Err("Failed to decrypt".to_string()),
                }
            },
            Err(_) => Err("Network IO Error".to_string()),
        }
    }

    fn deserialise_directory(&self, decrypted_data: Vec<u8>) -> ::directory_listing::DirectoryListing {
        let datamap = ::maidsafe_client::utility::deserialise(decrypted_data);
        let mut se = ::self_encryption::SelfEncryptor::new(::std::sync::Arc::new(::io::NetworkStorage::new(self.client.clone())), datamap);
        let size = se.len();
        ::maidsafe_client::utility::deserialise(se.read(0, size))
    }

    fn network_get(&self, tag_id: u64, name: &::routing::NameType) -> Result<Vec<u8>, String> {
        let get_result = self.client.lock().unwrap().get(tag_id, name.clone());
        if get_result.is_err() {
            return Err("Network IO Error".to_string());
        }

        match get_result.ok().unwrap().get() {
            Ok(data) => Ok(data),
            Err(_) => Err("Failed to fetch data".to_string()),
        }
    }

    fn network_put<T>(&self, sendable: T) -> Result<Vec<u8>, String> where T: Sendable {
        let get_result = self.client.lock().unwrap().put(sendable);
        if get_result.is_err() {
            return Err("Network IO Error".to_string());
        }

        match get_result.ok().unwrap().get() {
            Ok(data) => Ok(data),
            Err(_) => Err("Failed to fetch data".to_string()),
        }
    }

    fn get_nonce(&self, id: &::routing::NameType) -> Option<::sodiumoxide::crypto::box_::Nonce> {
        let mut nonce = [0u8;24];
        for i in 0..24 {
            nonce[i] = id.0[i * 2]
        }
        Some(::sodiumoxide::crypto::box_::Nonce(nonce))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_new_client() -> ::maidsafe_client::client::Client {
        let keyword = ::maidsafe_client::utility::generate_random_string(10);
        let password = ::maidsafe_client::utility::generate_random_string(10);
        let pin = ::maidsafe_client::utility::generate_random_pin();

        ::maidsafe_client::client::Client::create_account(&keyword,
                                         pin,
                                         &password).ok().unwrap()
    }

    #[test]
    fn create_dir_listing() {
        let client = ::std::sync::Arc::new(::std::sync::Mutex::new(get_new_client()));
        let mut dir_helper = DirectoryHelper::new(client.clone());

        assert!(dir_helper.create("DirName".to_string(),
                                  vec![7u8; 100]).is_ok());
    }

    #[test]
    fn get_dir_listing() {
        let client = ::std::sync::Arc::new(::std::sync::Mutex::new(get_new_client()));
        let mut dir_helper = DirectoryHelper::new(client.clone());

        let created_dir_id: _;
        {
            let put_result = dir_helper.create("DirName".to_string(),
                                               vec![7u8; 100]);

            assert!(put_result.is_ok());
            created_dir_id = put_result.ok().unwrap();
        }

        {
            let get_result_should_pass = dir_helper.get(&created_dir_id);
            assert!(get_result_should_pass.is_ok());
        }
        let get_result_wrong_dir_id_should_fail = dir_helper.get(&::routing::NameType::new([111u8; 64]));

        assert!(get_result_wrong_dir_id_should_fail.is_err());
    }

    #[test]
    fn update_and_versioning() {
        let client = ::std::sync::Arc::new(::std::sync::Mutex::new(get_new_client()));
        let mut dir_helper = DirectoryHelper::new(client.clone());

        let created_dir_id: _;
        {
            let put_result = dir_helper.create("DirName2".to_string(),
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

        let mut versions: _;
        {
            let get_result = dir_helper.get_versions(&created_dir_id);
            assert!(get_result.is_ok());
            versions = get_result.ok().unwrap();
        }

        assert_eq!(versions.len(), 1);

        {
            dir_listing.get_mut_metadata().set_name("NewName".to_string());
            let update_result = dir_helper.update(&dir_listing);
            assert!(update_result.is_ok());
        }

        {
            let get_result = dir_helper.get_versions(&created_dir_id);
            assert!(get_result.is_ok());
            versions = get_result.ok().unwrap();
        }

        assert_eq!(versions.len(), 2);

        {
            let get_result = dir_helper.get_by_version(&created_dir_id, &versions.last().unwrap().clone());
            assert!(get_result.is_ok());

            let rxd_dir_listing = get_result.ok().unwrap();

            assert_eq!(rxd_dir_listing, dir_listing);
        }

        {
            let get_result = dir_helper.get_by_version(&created_dir_id, &versions.first().unwrap().clone());
            assert!(get_result.is_ok());

            let rxd_dir_listing = get_result.ok().unwrap();

            assert!(rxd_dir_listing != dir_listing);
            assert_eq!(*rxd_dir_listing.get_metadata().get_name(), "DirName2".to_string());
        }
    }
}
