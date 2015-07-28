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

    /// Creates a Directory in the network.
    pub fn create(&self,
                  directory_name: String,
                  tag_type: u64,
                  user_metadata: Option<Vec<u8>>,
                  versioned: bool,
                  share_level: ::AccessLevel) -> Result<::directory_listing::DirectoryListing, ::errors::NfsError> {
        let directory = ::directory_listing::DirectoryListing::new(directory_name,
                                                                   tag_type,
                                                                   user_metadata,
                                                                   versioned,
                                                                   share_level);

        let structured_data = try!(self.save_directory_listing(&directory));

        let id = structured_data.get_identifier();
        let _ = self.client.lock().unwrap().put(::maidsafe_client::client::StructuredData::compute_name(tag_type, id),
                                                ::maidsafe_client::client::Data::StructuredData(structured_data.clone()));
        Ok(directory)
    }

    /// Deletes a sub directory
    pub fn delete(&self, directory: &mut ::directory_listing::DirectoryListing,
                  directory_to_delete: String) -> Result<::directory_listing::DirectoryListing, ::errors::NfsError> {
        match directory.get_sub_directory_index(directory_to_delete) {
            Some(pos) => {
                directory.get_mut_sub_directories().remove(pos);
                try!(self.update(directory));
                Ok(directory.clone())
            },
            None => Err(::errors::NfsError::NotFound),
        }
    }

    /// Updates an existing DirectoryListing in the network.
    pub fn update(&self, directory: &::directory_listing::DirectoryListing) -> Result<(), ::errors::NfsError> {
        let updated_structured_data = try!(self.save_directory_listing(directory));
        let _ = self.client.lock().unwrap().post(directory.get_key().0.clone(),
                                                 ::maidsafe_client::client::Data::StructuredData(updated_structured_data));
        Ok(())
    }

    /// Return the versions of the directory
    pub fn get_versions(&self, directory_key: (&::routing::NameType, u64)) -> Result<Vec<::routing::NameType>, ::errors::NfsError> {
        let structured_data = try!(self.get_structured_data(directory_key.0, ::VERSION_DIRECTORY_LISTING_TAG));
        Ok(try!(::maidsafe_client::structured_data_operations::versioned::get_all_versions(&mut *self.client.lock().unwrap(), &structured_data)))
    }

    /// Return the DirectoryListing for the specified version
    pub fn get_by_version(&self,
                          directory_key: (&::routing::NameType, u64),
                          share_level: ::AccessLevel,
                          version: ::routing::NameType) -> Result<::directory_listing::DirectoryListing, ::errors::NfsError> {
          let immutable_data = try!(self.get_immutable_data(version, ::maidsafe_client::client::ImmutableDataType::Normal));
          ::directory_listing::DirectoryListing::decrypt(self.client.clone(), directory_key.0, share_level, immutable_data.value().clone())
    }

    /// Return the DirectoryListing for the latest version
    pub fn get(&self, directory_key: (&::routing::NameType, u64),
               versioned: bool, share_level: ::AccessLevel) -> Result<::directory_listing::DirectoryListing, ::errors::NfsError> {
        let structured_data = try!(self.get_structured_data(directory_key.0,
                                                            directory_key.1));
        match versioned {
            true => {
               let versions = try!(::maidsafe_client::structured_data_operations::versioned::get_all_versions(&mut *self.client.lock().unwrap(), &structured_data));
               let latest_version = versions.last().unwrap();
               self.get_by_version(directory_key, share_level, latest_version.clone())
            },
            false => {
               let private_key = self.client.lock().unwrap().get_public_encryption_key().clone();
               let secret_key = self.client.lock().unwrap().get_secret_encryption_key().clone();
               let nonce = ::directory_listing::DirectoryListing::generate_nonce(directory_key.0);
               let encryption_keys = match share_level {
                   ::AccessLevel::Private => Some((&private_key,
                                                   &secret_key,
                                                   &nonce)),
                   ::AccessLevel::Public => None,
               };
               let structured_data = try!(::maidsafe_client::structured_data_operations::unversioned::get_data(self.client.clone(),
                                                                                                               &structured_data,
                                                                                                               encryption_keys));
               ::directory_listing::DirectoryListing::decrypt(self.client.clone(),
                                                              directory_key.0,
                                                              share_level,
                                                              structured_data)
            },
        }
    }


    /// Returns the Root Directory
    pub fn get_user_root_directory_listing(&self) -> Result<::directory_listing::DirectoryListing, ::errors::NfsError> {
        let root_directory;
        {
            root_directory = match self.client.lock().unwrap().get_user_root_directory_id() {
                Some(id) => Some(id.clone()),
                None => None,
            }
        }
        match root_directory {
            Some(id) => {
                self.get((&id, ::UNVERSION_DIRECTORY_LISTING_TAG), false, ::AccessLevel::Private)
            },
            None => {
                let created_directory = try!(self.create(::ROOT_DIRECTORY_NAME.to_string(),
                                                         ::UNVERSION_DIRECTORY_LISTING_TAG,
                                                         None,
                                                         false,
                                                         ::AccessLevel::Private));
                let _ = try!(self.client.lock().unwrap().set_user_root_directory_id(created_directory.get_key().0.clone()));
                Ok(created_directory)
            }
        }
    }

    /// Returns the Configuration DirectoryListing from the configuration root folder
    /// Creates the directory if the directory does not exists
    #[allow(dead_code)]
    pub fn get_configuration_directory_listing(&self,
                                       directory_name: String) -> Result<::directory_listing::DirectoryListing, ::errors::NfsError> {
        let config_root_directory;
        {
            config_root_directory = match self.client.lock().unwrap().get_configuration_root_directory_id() {
                Some(id) => Some(id.clone()),
                None => None,
            }
        }
        let mut config_directory_listing = match config_root_directory {
            Some(id) => try!(self.get((&id, ::UNVERSION_DIRECTORY_LISTING_TAG), false, ::AccessLevel::Private)),
            None => {
                let created_directory = try!(self.create(::CONFIGURATION_DIRECTORY_NAME.to_string(),
                                                         ::UNVERSION_DIRECTORY_LISTING_TAG,
                                                         None,
                                                         false,
                                                         ::AccessLevel::Private));
                try!(self.client.lock().unwrap().set_configuration_root_directory_id(created_directory.get_key().0.clone()));
                created_directory
            }
        };
        match config_directory_listing.get_sub_directories().iter().position(|dir_info| *dir_info.get_name() == directory_name.clone()) {
            Some(index) => Ok(try!(self.get(config_directory_listing.get_sub_directories()[index].get_key(),
                                            false,
                                            ::AccessLevel::Private))),
            None => {
                let new_dir_listing = try!(self.create(directory_name, ::UNVERSION_DIRECTORY_LISTING_TAG, None, false, ::AccessLevel::Private));
                config_directory_listing.get_mut_sub_directories().push(new_dir_listing.get_info().clone());
                try!(self.update(&config_directory_listing));
                Ok(new_dir_listing)
            },
        }
    }

    fn save_directory_listing(&self, directory: &::directory_listing::DirectoryListing) -> Result<::maidsafe_client::client::StructuredData, ::errors::NfsError> {
        let signing_key = self.client.lock().unwrap().get_secret_signing_key().clone();
        let owner_key = self.client.lock().unwrap().get_public_signing_key().clone();
        let share_level = directory.get_metadata().get_access_level();
        let versioned = directory.get_metadata().is_versioned();
        let encrypted_data = match share_level {
            ::AccessLevel::Private => try!(directory.encrypt(self.client.clone())),
            ::AccessLevel::Public => try!(::maidsafe_client::utility::serialise(&directory)),
        };
        match versioned {
            true => {
                let version = try!(self.save_as_immutable_data(encrypted_data,
                                                               ::maidsafe_client::client::ImmutableDataType::Normal));
                Ok(try!(::maidsafe_client::structured_data_operations::versioned::create(&mut *self.client.lock().unwrap(),
                                                                                         version,
                                                                                         ::VERSION_DIRECTORY_LISTING_TAG,
                                                                                         directory.get_key().0.clone(),
                                                                                         0,
                                                                                         vec![owner_key.clone()],
                                                                                         Vec::new(),
                                                                                         &signing_key)))
            },
            false => {
                let private_key = self.client.lock().unwrap().get_public_encryption_key().clone();
                let secret_key = self.client.lock().unwrap().get_secret_encryption_key().clone();
                let nonce = ::directory_listing::DirectoryListing::generate_nonce(directory.get_key().0);
                let encryption_keys = match share_level {
                    ::AccessLevel::Private => Some((&private_key,
                                                    &secret_key,
                                                    &nonce)),
                    ::AccessLevel::Public => None,
                };
                Ok(try!(::maidsafe_client::structured_data_operations::unversioned::create(self.client.clone(),
                                                                                           ::UNVERSION_DIRECTORY_LISTING_TAG,
                                                                                           directory.get_key().0.clone(),
                                                                                           0,
                                                                                           encrypted_data,
                                                                                           vec![owner_key.clone()],
                                                                                           Vec::new(),
                                                                                           &signing_key,
                                                                                           encryption_keys)))
            },
        }
    }

    /// Saves the data as ImmutableData in the network and returns the name
    fn save_as_immutable_data(&self,
                              data: Vec<u8>,
                              data_type: ::maidsafe_client::client::ImmutableDataType) -> Result<::routing::NameType, ::errors::NfsError> {
        let immutable_data = ::maidsafe_client::client::ImmutableData::new(data_type, data);
        let name = immutable_data.name();
        let _ = self.client.lock().unwrap().put(name.clone(),
                                                ::maidsafe_client::client::Data::ImmutableData(immutable_data));
        Ok(name)
    }

    fn get_structured_data(&self,
                           id: &::routing::NameType,
                           type_tag: u64) -> Result<::maidsafe_client::client::StructuredData, ::errors::NfsError> {
        let mut response_getter = try!(self.client.lock().unwrap().get(::maidsafe_client::client::StructuredData::compute_name(type_tag, id),
                                                                  ::maidsafe_client::client::DataRequest::StructuredData(type_tag)));
        let data = try!(response_getter.get());
        match data {
            ::maidsafe_client::client::Data::StructuredData(structured_data) => Ok(structured_data),
            _ => Err(::errors::NfsError::from(::maidsafe_client::errors::ClientError::ReceivedUnexpectedData)),
        }
    }

    /// Get ImmutableData from the Network
    fn get_immutable_data(&self,
                          id: ::routing::NameType,
                          data_type: ::maidsafe_client::client::ImmutableDataType) -> Result<::maidsafe_client::client::ImmutableData, ::errors::NfsError> {
        let mut response_getter = try!(self.client.lock().unwrap().get(id, ::maidsafe_client::client::DataRequest::ImmutableData(data_type)));
        let data = try!(response_getter.get());
        match data {
            ::maidsafe_client::client::Data::ImmutableData(immutable_data) => Ok(immutable_data),
            _ => Err(::errors::NfsError::from(::maidsafe_client::errors::ClientError::ReceivedUnexpectedData)),
        }
    }
}

/*
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_dir_listing() {
        let test_client = ::maidsafe_client::utility::test_utils::get_client().unwrap_or_else(|error| { println!("Error: {}", error); unimplemented!() });
        let client = ::std::sync::Arc::new(::std::sync::Mutex::new(test_client));
        let mut dir_helper = DirectoryHelper::new(client.clone());

        assert!(dir_helper.create("DirName".to_string(),
                                  vec![7u8; 100]).is_ok());
    }

    #[test]
    fn get_dir_listing() {
        let test_client = ::maidsafe_client::utility::test_utils::get_client().unwrap_or_else(|error| { println!("Error: {}", error); unimplemented!() });
        let client = ::std::sync::Arc::new(::std::sync::Mutex::new(test_client));
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
        // TO FIX Krishna - get hangs if the data is not present in the network
        // {
        //     let get_result_wrong_dir_id_should_fail = dir_helper.get(&::routing::NameType::new([111u8; 64]));
        //     assert!(get_result_wrong_dir_id_should_fail.is_err());
        // }
    }

    #[test]
    fn update_and_versioning() {
        let test_client = ::maidsafe_client::utility::test_utils::get_client().unwrap_or_else(|error| { println!("Error: {}", error); unimplemented!() });
        let client = ::std::sync::Arc::new(::std::sync::Mutex::new(test_client));
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
}*/
