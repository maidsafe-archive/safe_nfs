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

/// Get StructuredData
pub fn get_structured_data(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
                           id: ::routing::NameType,
                           type_tag: u64) -> Result<::maidsafe_client::client::StructuredData, ::errors::NFSError> {

    let mut response_getter = try!(client.lock().unwrap().get(id, ::maidsafe_client::client::DataRequest::StructuredData(type_tag)));
    let data = try!(response_getter.get());
    match data {
        ::maidsafe_client::client::Data::StructuredData(structured_data) => Ok(structured_data),
        _ => Err(::errors::NFSError::ReceivedUnexpectedData),
    }
}

/// Get ImmutableData
pub fn get_immutable_data(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
                           id: ::routing::NameType,
                           data_type: ::maidsafe_client::client::ImmutableDataType) -> Result<::maidsafe_client::client::ImmutableData, ::errors::NFSError> {
    let mut response_getter = try!(client.lock().unwrap().get(id, ::maidsafe_client::client::DataRequest::ImmutableData(data_type)));
    let data = try!(response_getter.get());
    match data {
        ::maidsafe_client::client::Data::ImmutableData(immutable_data) => Ok(immutable_data),
        _ => Err(::errors::NFSError::ReceivedUnexpectedData),
    }
}
