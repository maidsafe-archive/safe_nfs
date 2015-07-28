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

/// Utility function related to DirectoryListing
pub mod directory_listing_util;

/// Returns the Root Directory
pub fn get_user_root_directory_id(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>) -> Result<::directory_listing::DirectoryListing, ::errors::NfsError> {
    let root_directory;
    {
        root_directory = match client.lock().unwrap().get_user_root_directory_id() {
            Some(id) => Some(id.clone()),
            None => None,
        }
    }
    let mut directory_helper = ::helper::DirectoryHelper::new(client.clone());
    match root_directory {
        Some(id) => {
            directory_helper.get(id, false, ::ShareLevel::PRIVATE)
        },
        None => {
            let created_directory = try!(directory_helper.create(::ROOT_DIRECTORY_NAME.to_string(), None, false, ::ShareLevel::PRIVATE));
            let _ = try!(client.lock().unwrap().set_user_root_directory_id(created_directory.get_id().clone()));
            Ok(created_directory)
        }
    }
}

/// Returns the Configuration DirectoryListing from the configuration root folder
/// Creates the directory if the directory does not exists
#[allow(dead_code)]
pub fn get_configuration_directory(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
                                      directory_name: String) -> Result<::directory_listing::DirectoryListing, ::errors::NfsError> {
    let config_root_directory;
    {
        config_root_directory = match client.lock().unwrap().get_configuration_root_directory_id() {
            Some(id) => Some(id.clone()),
            None => None,
        }
    }
    let mut directory_helper = ::helper::DirectoryHelper::new(client.clone());
    let mut config_directory_listing = match config_root_directory {
        Some(id) => try!(directory_helper.get(id, false, ::ShareLevel::PRIVATE)),
        None => {
            let created_directory = try!(directory_helper.create(::CONFIGURATION_DIRECTORY_NAME.to_string(), None, false, ::ShareLevel::PRIVATE));
            try!(client.lock().unwrap().set_configuration_root_directory_id(created_directory.get_id().clone()));
            created_directory
        }
    };
    match config_directory_listing.get_sub_directories().iter().position(|dir_info| *dir_info.get_name() == directory_name.clone()) {
        Some(index) => Ok(try!(directory_helper.get(config_directory_listing.get_sub_directories()[index].get_id().clone(),
                                                    false,
                                                    ::ShareLevel::PRIVATE))),
        None => {
            let new_dir_listing = try!(directory_helper.create(directory_name, None, false, ::ShareLevel::PRIVATE));
            config_directory_listing.get_mut_sub_directories().push(new_dir_listing.get_info().clone());
            try!(directory_helper.update(&config_directory_listing));
            Ok(new_dir_listing)
        },
    }
}

/// Get StructuredData from the Network
pub fn get_structured_data(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
                           id: ::routing::NameType,
                           type_tag: u64) -> Result<::maidsafe_client::client::StructuredData, ::errors::NfsError> {
    // TODO Wrong - location of structured data is StructuredData::compute_location(tag, &id)
    let mut response_getter = try!(client.lock().unwrap().get(id, ::maidsafe_client::client::DataRequest::StructuredData(type_tag)));
    let data = try!(response_getter.get());
    match data {
        ::maidsafe_client::client::Data::StructuredData(structured_data) => Ok(structured_data),
        _ => Err(::errors::NfsError::from(::maidsafe_client::errors::ClientError::ReceivedUnexpectedData)),
    }
}

/// Get ImmutableData from the Network
pub fn get_immutable_data(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
                           id: ::routing::NameType,
                           data_type: ::maidsafe_client::client::ImmutableDataType) -> Result<::maidsafe_client::client::ImmutableData, ::errors::NfsError> {
    let mut response_getter = try!(client.lock().unwrap().get(id, ::maidsafe_client::client::DataRequest::ImmutableData(data_type)));
    let data = try!(response_getter.get());
    match data {
        ::maidsafe_client::client::Data::ImmutableData(immutable_data) => Ok(immutable_data),
        _ => Err(::errors::NfsError::from(::maidsafe_client::errors::ClientError::ReceivedUnexpectedData)),
    }
}

/// Saves the data as ImmutableData in the network and returns the name
pub fn save_as_immutable_data(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
                             data: Vec<u8>,
                             data_type: ::maidsafe_client::client::ImmutableDataType) -> Result<::routing::NameType, ::errors::NfsError> {
    let immutable_data = ::maidsafe_client::client::ImmutableData::new(data_type, data);
    let name = immutable_data.name();
    let _ = client.lock().unwrap().put(name.clone(),
                               ::maidsafe_client::client::Data::ImmutableData(immutable_data));
    Ok(name)
}
