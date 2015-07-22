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

pub fn get_secret_signing_key(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>) -> ::sodiumoxide::crypto::sign::SecretKey {
    client.lock().unwrap().get_secret_signing_key().clone()
}

pub fn get_public_signing_key(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>) -> ::sodiumoxide::crypto::sign::PublicKey {
    client.lock().unwrap().get_public_signing_key().clone()
}

pub fn get_secret_encryption_key(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>) -> ::sodiumoxide::crypto::box_::SecretKey {
    client.lock().unwrap().get_secret_encryption_key().clone()
}

pub fn get_public_encryption_key(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>) -> ::sodiumoxide::crypto::box_::PublicKey {
    client.lock().unwrap().get_public_encryption_key().clone()
}

pub fn get_user_root_directory_id(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>) -> Result<::routing::NameType, ::errors::NFSError> {
    let root_directory;
    {
        root_directory = match client.lock().unwrap().get_user_root_directory_id() {
            Some(id) => Some(id.clone()),
            None => None,
        }
    }
    match root_directory {
        Some(id) => Ok(id.clone()),
        None => {
            let mut directory_helper = ::helper::DirectoryHelper::new(client.clone());
            let created_directory_id = try!(directory_helper.create("root".to_string(), Vec::new()));
            let _ = try!(client.lock().unwrap().set_user_root_directory_id(created_directory_id.clone()));
            Ok(created_directory_id.clone())
        }
    }
}

/// Generates a nonce based on the directory_id
pub fn generate_nonce(directory_id: &::routing::NameType) -> ::sodiumoxide::crypto::box_::Nonce {
    let mut nonce = [0u8; ::sodiumoxide::crypto::box_::NONCEBYTES];
    for i in 0..nonce.len() {
        nonce[i] = directory_id.0[i];
    }
    ::sodiumoxide::crypto::box_::Nonce(nonce)
}

/// Saves the DirectoryListing in the network
pub fn save_directory_listing(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
                              directory_listing: &::directory_listing::DirectoryListing) -> Result<::routing::NameType, ::errors::NFSError> {
    let serialised_data = try!(::maidsafe_client::utility::serialise(directory_listing));
    let mut se = ::self_encryption::SelfEncryptor::new(::maidsafe_client::SelfEncryptionStorage::new(client.clone()), ::self_encryption::datamap::DataMap::None);
    se.write(&serialised_data, 0);
    let datamap = se.close();
    let serialised_data_map = try!(::maidsafe_client::utility::serialise(&datamap));
    let encrypted_data_map = try!(client.lock().unwrap().hybrid_encrypt(&serialised_data_map[..],
                                                                        Some(&generate_nonce(directory_listing.get_id()))));
    save_as_immutable_data(client.clone(), encrypted_data_map, ::maidsafe_client::client::ImmutableDataType::Normal)
}

/// Get DirectoryListing from the network
pub fn get_directory_listing(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
                              directory_id: &::routing::NameType,
                              version: ::routing::NameType) -> Result<::directory_listing::DirectoryListing, ::errors::NFSError> {
    // Get immutable data
    let nonce = generate_nonce(directory_id);
    let immutable_data = try!(get_immutable_data(client.clone(), version, ::maidsafe_client::client::ImmutableDataType::Normal));
    // Hybrid Decrypt
    let decrypted_data_map = try!(client.lock().unwrap().hybrid_decrypt(&immutable_data.value()[..], Some(&nonce)));
    // Desriase Datamap
    let datamap: ::self_encryption::datamap::DataMap = try!(::maidsafe_client::utility::deserialise(&decrypted_data_map));
    // read Data
    let mut se = ::self_encryption::SelfEncryptor::new(::maidsafe_client::SelfEncryptionStorage::new(client.clone()), datamap);
    let length = se.len();
    let serialised_directory_listing = se.read(0, length);
    // Desrialise Directorylisting
    Ok(try!(::maidsafe_client::utility::deserialise(&serialised_directory_listing)))
}

/// Get StructuredData from the Network
pub fn get_structured_data(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
                           id: ::routing::NameType,
                           type_tag: u64) -> Result<::maidsafe_client::client::StructuredData, ::errors::NFSError> {

    let mut response_getter = try!(client.lock().unwrap().get(id, ::maidsafe_client::client::DataRequest::StructuredData(type_tag)));
    let data = try!(response_getter.get());
    match data {
        ::maidsafe_client::client::Data::StructuredData(structured_data) => Ok(structured_data),
        _ => Err(::errors::NFSError::from(::maidsafe_client::errors::ClientError::ReceivedUnexpectedData)),
    }
}

/// Gets ImmutableData from the Network
pub fn get_immutable_data(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
                           id: ::routing::NameType,
                           data_type: ::maidsafe_client::client::ImmutableDataType) -> Result<::maidsafe_client::client::ImmutableData, ::errors::NFSError> {
    let mut response_getter = try!(client.lock().unwrap().get(id, ::maidsafe_client::client::DataRequest::ImmutableData(data_type)));
    let data = try!(response_getter.get());
    match data {
        ::maidsafe_client::client::Data::ImmutableData(immutable_data) => Ok(immutable_data),
        _ => Err(::errors::NFSError::from(::maidsafe_client::errors::ClientError::ReceivedUnexpectedData)),
    }
}

/// Saves the data as ImmutableData in the network and returns the name
pub fn save_as_immutable_data(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
                             data: Vec<u8>,
                             data_type: ::maidsafe_client::client::ImmutableDataType) -> Result<::routing::NameType, ::errors::NFSError> {
    let immutable_data = ::maidsafe_client::client::ImmutableData::new(data_type, data);
    let name = immutable_data.name();
    client.lock().unwrap().put(name.clone(),
                               ::maidsafe_client::client::Data::ImmutableData(immutable_data));
    Ok(name)
}
