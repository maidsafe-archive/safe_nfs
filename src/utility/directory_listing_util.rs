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

/// Decrypts a directory listing
pub fn decrypt_directory_listing(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
                                 directory_id: &::routing::NameType,
                                 share_level: ::ShareLevel,
                                 data: Vec<u8>) -> Result<::directory_listing::DirectoryListing, ::errors::NfsError> {
     let decrypted_data_map = match share_level {
         ::ShareLevel::Private => try!(client.lock().unwrap().hybrid_decrypt(&data[..],
                                                                             Some(&::utility::directory_listing_util::generate_nonce(directory_id)))),
         ::ShareLevel::Public => data,
     };
     let datamap: ::self_encryption::datamap::DataMap = try!(::maidsafe_client::utility::deserialise(&decrypted_data_map));
     let mut se = ::self_encryption::SelfEncryptor::new(::maidsafe_client::SelfEncryptionStorage::new(client.clone()), datamap);
     let length = se.len();
     let serialised_directory_listing = se.read(0, length);
     Ok(try!(::maidsafe_client::utility::deserialise(&serialised_directory_listing)))
}

/// Encrypts a directory listing
pub fn encrypt_directory_listing(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
                              directory_listing: &::directory_listing::DirectoryListing) -> Result<Vec<u8>, ::errors::NfsError> {
    let serialised_data = try!(::maidsafe_client::utility::serialise(directory_listing));
    let mut se = ::self_encryption::SelfEncryptor::new(::maidsafe_client::SelfEncryptionStorage::new(client.clone()), ::self_encryption::datamap::DataMap::None);
    se.write(&serialised_data, 0);
    let datamap = se.close();
    let serialised_data_map = try!(::maidsafe_client::utility::serialise(&datamap));
    Ok(try!(client.lock().unwrap().hybrid_encrypt(&serialised_data_map, Some(&::utility::directory_listing_util::generate_nonce(directory_listing.get_id())))))
}

/// Get DirectoryInfo of sub_directory within a DirectoryListing.
/// Returns the Option<DirectoryInfo> for the directory_name from the DirectoryListing
pub fn find_file(directory_listing: &::directory_listing::DirectoryListing,
                 file_name: String) -> Option<&::file::File> {
    directory_listing.get_files().iter().find(|file| *file.get_name() == file_name)
}


/// Get DirectoryInfo of sub_directory within a DirectoryListing.
/// Returns the Option<DirectoryInfo> for the directory_name from the DirectoryListing
pub fn find_sub_directory(directory_listing: &::directory_listing::DirectoryListing,
                          directory_name: String) -> Option<&::directory_info::DirectoryInfo> {
    directory_listing.get_sub_directories().iter().find(|info| *info.get_name() == directory_name)
}

pub fn get_sub_directory_index(directory_listing: &::directory_listing::DirectoryListing,
                          directory_name: String) -> Option<usize> {
    directory_listing.get_sub_directories().iter().position(|dir_info| *dir_info.get_name() == directory_name)
}

/// Generates a nonce based on the directory_id
pub fn generate_nonce(directory_id: &::routing::NameType) -> ::sodiumoxide::crypto::box_::Nonce {
    let mut nonce = [0u8; ::sodiumoxide::crypto::box_::NONCEBYTES];
    for i in 0..nonce.len() {
        nonce[i] = directory_id.0[i];
    }
    ::sodiumoxide::crypto::box_::Nonce(nonce)
}

/// Get DirectoryListing from the network
pub fn get_directory_listing_for_version(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
                                         directory_id: &::routing::NameType,
                                         share_level: ::ShareLevel,
                                         version: ::routing::NameType) -> Result<::directory_listing::DirectoryListing, ::errors::NfsError> {
    // Get immutable data
    let immutable_data = try!(::utility::get_immutable_data(client.clone(), version, ::maidsafe_client::client::ImmutableDataType::Normal));
    ::utility::directory_listing_util::decrypt_directory_listing(client.clone(), directory_id, share_level, immutable_data.value().clone())
}

/// Returns Directorylisting
/// Fetches the directory listing based on the versioning support and also on share_level
pub fn get_directory_listing(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
                             directory_id: &::routing::NameType,
                             versioned: bool,
                             share_level: ::ShareLevel) -> Result<::directory_listing::DirectoryListing, ::errors::NfsError> {
        // get structured_data
        let tag = get_tag_type(versioned);
        let structured_data = try!(::utility::get_structured_data(client.clone(),
                                                                  ::maidsafe_client::client::StructuredData::compute_name(tag, directory_id),
                                                                  tag));
        match versioned {
            true => {
                let versions = try!(::maidsafe_client::structured_data_operations::versioned::get_all_versions(&mut *client.lock().unwrap(), &structured_data));
                let latest_version = versions.last().unwrap();
                get_directory_listing_for_version(client.clone(), directory_id, share_level, latest_version.clone())
            },
            false => {
                let private_key = client.lock().unwrap().get_public_encryption_key().clone();
                let secret_key = client.lock().unwrap().get_secret_encryption_key().clone();
                let nonce = ::utility::directory_listing_util::generate_nonce(directory_id);
                let encryption_keys = match share_level {
                    ::ShareLevel::Private => Some((&private_key,
                                                   &secret_key,
                                                   &nonce)),
                    ::ShareLevel::Public => None,
                };
                let structured_data = try!(::maidsafe_client::structured_data_operations::unversioned::get_data(client.clone(),
                                                                                                                &structured_data,
                                                                                                                encryption_keys));
                ::utility::directory_listing_util::decrypt_directory_listing(client.clone(),
                                                                             directory_id,
                                                                             share_level,
                                                                             structured_data)
            },
        }
}

/// Returns the type tag for versioend and unversioned types
pub fn get_tag_type(versioned: bool) -> u64 {
    match versioned {
        true => ::VERSION_DIRECTORY_LISTING_TAG,
        false => ::UNVERSION_DIRECTORY_LISTING_TAG,
    }
}

/// Saves the DirectoryListing in the network
pub fn save_directory_listing(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
                              directory: &::directory_listing::DirectoryListing) -> Result<::maidsafe_client::client::StructuredData, ::errors::NfsError> {
    let signing_key = client.lock().unwrap().get_secret_signing_key().clone();
    let owner_key = client.lock().unwrap().get_public_signing_key().clone();
    let share_level = try!(directory.get_metadata().get_share_level().ok_or(::errors::NfsError::MetaDataMissingOrCorrupted));
    let versioned = try!(directory.get_metadata().is_versioned().ok_or(::errors::NfsError::MetaDataMissingOrCorrupted));
    let encrypted_data = match share_level {
        ::ShareLevel::Private => try!(::utility::directory_listing_util::encrypt_directory_listing(client.clone(), &directory)),
        ::ShareLevel::Public => try!(::maidsafe_client::utility::serialise(&directory)),
    };
    match versioned {
        true => {
            let version = try!(::utility::save_as_immutable_data(client.clone(),
                                                                 encrypted_data,
                                                                 ::maidsafe_client::client::ImmutableDataType::Normal));
            Ok(try!(::maidsafe_client::structured_data_operations::versioned::create(&mut *client.lock().unwrap(),
                                                                                     version,
                                                                                     ::VERSION_DIRECTORY_LISTING_TAG,
                                                                                     directory.get_id().clone(),
                                                                                     0,
                                                                                     vec![owner_key.clone()],
                                                                                     Vec::new(),
                                                                                     &signing_key)))
        },
        false => {
            let private_key = client.lock().unwrap().get_public_encryption_key().clone();
            let secret_key = client.lock().unwrap().get_secret_encryption_key().clone();
            let nonce = ::utility::directory_listing_util::generate_nonce(directory.get_id());
            let encryption_keys = match share_level {
                ::ShareLevel::Private => Some((&private_key,
                                               &secret_key,
                                               &nonce)),
                ::ShareLevel::Public => None,
            };
            Ok(try!(::maidsafe_client::structured_data_operations::unversioned::create(client.clone(),
                                                                                       ::UNVERSION_DIRECTORY_LISTING_TAG,
                                                                                       directory.get_id().clone(),
                                                                                       0,
                                                                                       encrypted_data,
                                                                                       vec![owner_key.clone()],
                                                                                       Vec::new(),
                                                                                       &signing_key,
                                                                                       encryption_keys)))
        },
    }
}
