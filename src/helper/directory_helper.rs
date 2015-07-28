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
                  user_metadata: Option<Vec<u8>>,
                  versioned: bool,
                  share_level: ::ShareLevel) -> Result<::directory_listing::DirectoryListing, ::errors::NfsError> {
        let directory = ::directory_listing::DirectoryListing::new(directory_name, user_metadata,
                                                                   versioned, share_level);

        let structured_data = try!(::utility::directory_listing_util::save_directory_listing(self.client.clone(), &directory));

        let tag = structured_data.get_tag_type();
        let id = structured_data.get_identifier();
        let _ = self.client.lock().unwrap().put(::maidsafe_client::client::StructuredData::compute_name(tag, id),
                                                ::maidsafe_client::client::Data::StructuredData(structured_data.clone()));
        Ok(directory)
    }

    /// Deletes a sub directory
    pub fn delete(&self, directory: &mut ::directory_listing::DirectoryListing,
                  directory_to_delete: String) -> Result<::directory_listing::DirectoryListing, ::errors::NfsError> {
        match ::utility::directory_listing_util::get_sub_directory_index(directory, directory_to_delete) {
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
        let updated_structured_data = try!(::utility::directory_listing_util::save_directory_listing(self.client.clone(), &directory));
        let _ = self.client.lock().unwrap().post(directory.get_id().clone(),
                                  ::maidsafe_client::client::Data::StructuredData(updated_structured_data));
        Ok(())
    }

    /// Return the versions of the directory
    pub fn get_versions(&self, directory_id: &::routing::NameType) -> Result<Vec<::routing::NameType>, ::errors::NfsError> {
        let structured_data = try!(::utility::get_structured_data(self.client.clone(), directory_id.clone(), ::VERSION_DIRECTORY_LISTING_TAG));
        Ok(try!(::maidsafe_client::structured_data_operations::versioned::get_all_versions(&mut *self.client.lock().unwrap(), &structured_data)))
    }

    /// Return the DirectoryListing for the specified version
    pub fn get_by_version(&self,
                          directory_id: &::routing::NameType,
                          share_level: ::ShareLevel,
                          version: ::routing::NameType) -> Result<::directory_listing::DirectoryListing, ::errors::NfsError> {
        ::utility::directory_listing_util::get_directory_listing_for_version(self.client.clone(), directory_id, share_level, version)
    }

    /// Return the DirectoryListing for the latest version
    pub fn get(&self, directory_id: ::routing::NameType, versioned: bool, share_level: ::ShareLevel) -> Result<::directory_listing::DirectoryListing, ::errors::NfsError> {
        ::utility::directory_listing_util::get_directory_listing(self.client.clone(),
                                                                &directory_id,
                                                                versioned,
                                                                share_level)
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
