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

/// Generates a nonce based on the directory_id
pub fn generate_nonce(directory_id: &::routing::NameType) -> ::sodiumoxide::crypto::box_::Nonce {
    let mut nonce = [0u8; ::sodiumoxide::crypto::box_::NONCEBYTES];
    for i in 0..nonce.len() {
        nonce[i] = directory_id.0[i];
    }
    ::sodiumoxide::crypto::box_::Nonce(nonce)
}

/// Get DirectoryListing from the network
pub fn get_directory_listing(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
                             directory_id: &::routing::NameType,
                             version: ::routing::NameType) -> Result<::directory_listing::DirectoryListing, ::errors::NfsError> {
    // Get immutable data
    let nonce = generate_nonce(directory_id);
    let immutable_data = try!(::utility::get_immutable_data(client.clone(), version, ::maidsafe_client::client::ImmutableDataType::Normal));
    // Hybrid Decrypt
    let decrypted_data_map = try!(client.lock().unwrap().hybrid_decrypt(&immutable_data.value()[..], Some(&nonce)));
    // Desrialise Datamap
    let datamap: ::self_encryption::datamap::DataMap = try!(::maidsafe_client::utility::deserialise(&decrypted_data_map));
    // read Data
    let mut se = ::self_encryption::SelfEncryptor::new(::maidsafe_client::SelfEncryptionStorage::new(client.clone()), datamap);
    let length = se.len();
    let serialised_directory_listing = se.read(0, length);
    // Desrialise DirectoryListing
    Ok(try!(::maidsafe_client::utility::deserialise(&serialised_directory_listing)))
}

/// Get DirectoryInfo of sub_directory within a DirectoryListing.
/// Returns the Option<DirectoryInfo> for the directory_name from the DirectoryListing
pub fn find_sub_directory(directory_listing: &::directory_listing::DirectoryListing,
                          directory_name: String) -> Option<&::directory_info::DirectoryInfo> {
    directory_listing.get_sub_directories().iter().find(|info| *info.get_name() == directory_name)
}

/// Get DirectoryInfo of sub_directory within a DirectoryListing.
/// Returns the Option<DirectoryInfo> for the directory_name from the DirectoryListing
pub fn find_file(directory_listing: &::directory_listing::DirectoryListing,
                 file_name: String) -> Option<&::file::File> {
    directory_listing.get_files().iter().find(|file| *file.get_name() == file_name)
}

/// Saves the DirectoryListing in the network
pub fn save_directory_listing(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
                              directory: &::directory_listing::DirectoryListing) -> Result<::maidsafe_client::client::StructuredData, ::errors::NfsError> {
    let signing_key = client.lock().unwrap().get_secret_signing_key();
    let owner_key = client.lock().unwrap().get_public_signing_key();
    let share_level = match directory.get_metadata().get_share_level() {
        Some(share_level) => share_level,
        None => return Err(::errors::NfsError::MetaDataMissingOrCorrupted),
    };
    let versioned = match directory.get_metadata().is_versioned() {
        Some(enabled) => enabled,
        None => return Err(::errors::NfsError::MetaDataMissingOrCorrupted),
    };
    let encrypted_data = match share_level {
        ::ShareLevel::PRIVATE => try!(::utility::directory_listing_util::encrypt_directory_listing(client.clone(), &directory)),
        ::ShareLevel::PUBLIC => try!(::maidsafe_client::utility::serialise(&directory)),
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
            let encryption_keys = match share_level {
                PRIVATE => Some((client.lock().unwrap().get_public_encryption_key(),
                                 client.lock().unwrap().get_secret_encryption_key(),
                                 &::utility::directory_listing_util::generate_nonce(directory.get_id()))),
                PUBLIC => None,
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

pub fn encrypt_directory_listing(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
                              directory_listing: &::directory_listing::DirectoryListing) -> Result<Vec<u8>, ::errors::NfsError> {
    let serialised_data = try!(::maidsafe_client::utility::serialise(directory_listing));
    let mut se = ::self_encryption::SelfEncryptor::new(::maidsafe_client::SelfEncryptionStorage::new(client.clone()), ::self_encryption::datamap::DataMap::None);
    se.write(&serialised_data, 0);
    let datamap = se.close();
    let serialised_data_map = try!(::maidsafe_client::utility::serialise(&datamap));
    try!(client.lock().unwrap().hybrid_encrypt(&serialised_data_map, Some(&generate_nonce(directory_listing.get_id()))));
}
