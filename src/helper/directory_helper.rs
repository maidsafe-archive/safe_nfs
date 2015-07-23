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
    pub fn create(&mut self,
                  directory_name: String,
                  user_metadata: Vec<u8>) -> Result<::routing::NameType, ::errors::NFSError> {
        let directory = ::directory_listing::DirectoryListing::new(directory_name, user_metadata);

        let version = try!(::utility::save_directory_listing(self.client.clone(), &directory));
        let signing_key = ::utility::get_secret_signing_key(self.client.clone());
        let owner_key = ::utility::get_public_signing_key(self.client.clone());
        let mut mutex_client = self.client.lock().unwrap();
        let structured_data = try!(::maidsafe_client::structured_data_operations::versioned::create(&mut *mutex_client,
                                                                                                    version,
                                                                                                    ::VERSION_DIRECTORY_LISTING_TAG,
                                                                                                    directory.get_id().clone(),
                                                                                                    0,
                                                                                                    vec![owner_key],
                                                                                                    Vec::new(),
                                                                                                    &signing_key));
        let _ = mutex_client.put(directory.get_id().clone(), ::maidsafe_client::client::Data::StructuredData(structured_data));
        Ok(directory.get_id().clone())
    }


    /// Updates an existing DirectoryListing in the network.
    pub fn update(&mut self, directory: &::directory_listing::DirectoryListing) -> Result<(), ::errors::NFSError> {

        let version = try!(::utility::save_directory_listing(self.client.clone(), &directory));
        let structured_data = try!(::utility::get_structured_data(self.client.clone(),
                                                                  directory.get_id().clone(),
                                                                  ::VERSION_DIRECTORY_LISTING_TAG));
        let signing_key = ::utility::get_secret_signing_key(self.client.clone());

        let mut mutex_client = self.client.lock().unwrap();
        let updated_structured_data = try!(::maidsafe_client::structured_data_operations::versioned::append_version(&mut *mutex_client,
                                                                                                                    structured_data,
                                                                                                                    version,
                                                                                                                    &signing_key));
        let _ = mutex_client.post(directory.get_id().clone(), ::maidsafe_client::client::Data::StructuredData(updated_structured_data));
        Ok(())
    }

    /// Return the versions of the directory
    // TODO version parameter change it to value instead of &
    pub fn get_versions(&mut self, directory_id: &::routing::NameType) -> Result<Vec<::routing::NameType>, ::errors::NFSError> {
        let structured_data = try!(::utility::get_structured_data(self.client.clone(), directory_id.clone(), ::VERSION_DIRECTORY_LISTING_TAG));
        Ok(try!(::maidsafe_client::structured_data_operations::versioned::get_all_versions(&mut *self.client.lock().unwrap(), &structured_data)))
    }

    /// Return the DirectoryListing for the specified version
    // TODO version parameter change it to value instead of &
    pub fn get_by_version(&mut self,
                          directory_id: &::routing::NameType,
                          version: &::routing::NameType) -> Result<::directory_listing::DirectoryListing, ::errors::NFSError> {
        Ok(try!(::utility::get_directory_listing(self.client.clone(), directory_id, version.clone())))
    }

    /// Return the DirectoryListing for the latest version
    // TODO version parameter change it to value instead of &
    pub fn get(&mut self, directory_id: &::routing::NameType) -> Result<::directory_listing::DirectoryListing, ::errors::NFSError> {
        let structured_data = try!(::utility::get_structured_data(self.client.clone(), directory_id.clone(), ::VERSION_DIRECTORY_LISTING_TAG));
        let versions = try!(::maidsafe_client::structured_data_operations::versioned::get_all_versions(&mut *self.client.lock().unwrap(), &structured_data));
        let latest_version = versions.last().unwrap();
        self.get_by_version(directory_id, &latest_version)
    }


}


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
}
