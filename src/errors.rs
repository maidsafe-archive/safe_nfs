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


/// NFS Errors
pub enum NFSError {
    /// Data Size large
    DataHeaderSizeProhibitive,
    /// Could not Serialise or Deserialise
    UnsuccessfulEncodeDecode,
    /// Asymmetric Key Decryption Failed
    AsymmetricDecipherFailure,
    /// Symmetric Key Decryption Failed
    SymmetricDecipherFailure,
    /// Routing GET, PUT, POST, DELETE Immediate Failure
    RoutingFailure(::std::io::Error),
    /// ReceivedUnexpectedData
    ReceivedUnexpectedData,
    /// No such data found in local version cache
    VersionCacheMiss,
    /// No such data found in routing-filled cache
    RoutingMessageCacheMiss,
    // Network operation failed
    NetworkOperationFailure(::routing::error::ResponseError),
    /// Cannot overwrite a root directory if it already exists
    RootDirectoryAlreadyExists,
    /// Generic I/O Error
    GenericIoError(::std::io::Error),
    /// Dummy
    UnhandledError,
}

impl From<::cbor::CborError> for NFSError {
    fn from(_: ::cbor::CborError) -> NFSError {
        NFSError::UnsuccessfulEncodeDecode
    }
}

impl From<::maidsafe_client::errors::ClientError> for NFSError {
    fn from(error: ::maidsafe_client::errors::ClientError) -> NFSError {
        match error {
            ::maidsafe_client::errors::ClientError::StructuredDataHeaderSizeProhibitive => NFSError::DataHeaderSizeProhibitive,
            /// Could not Serialise or Deserialise
            ::maidsafe_client::errors::ClientError::UnsuccessfulEncodeDecode => NFSError::UnsuccessfulEncodeDecode,
            /// Asymmetric Key Decryption Failed
            ::maidsafe_client::errors::ClientError::AsymmetricDecipherFailure => NFSError::AsymmetricDecipherFailure,
            /// Symmetric Key Decryption Failed
            ::maidsafe_client::errors::ClientError::SymmetricDecipherFailure => NFSError::SymmetricDecipherFailure,
            /// Routing GET, PUT, POST, DELETE Immediate Failure
            ::maidsafe_client::errors::ClientError::RoutingFailure(err) => NFSError::RoutingFailure(err),
            /// ReceivedUnexpectedData
            ::maidsafe_client::errors::ClientError::ReceivedUnexpectedData => NFSError::ReceivedUnexpectedData,
            /// No such data found in local version cache
            ::maidsafe_client::errors::ClientError::VersionCacheMiss => NFSError::VersionCacheMiss,
            /// No such data found in routing-filled cache
            ::maidsafe_client::errors::ClientError::RoutingMessageCacheMiss => NFSError::RoutingMessageCacheMiss,
            /// Network operation failed
            ::maidsafe_client::errors::ClientError::NetworkOperationFailure(err) => NFSError::NetworkOperationFailure(err),
            /// Cannot overwrite a root directory if it already exists
            ::maidsafe_client::errors::ClientError::RootDirectoryAlreadyExists => NFSError::RootDirectoryAlreadyExists,
            /// Generic I/O Error
            ::maidsafe_client::errors::ClientError::GenericIoError(err) => NFSError::GenericIoError(err),
        }
    }
}

impl ::std::fmt::Display for NFSError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            NFSError::DataHeaderSizeProhibitive           => ::std::fmt::Display::fmt("NFSError::DataHeaderSizeProhibitive", f),
            NFSError::UnsuccessfulEncodeDecode            => ::std::fmt::Display::fmt("NFSError::UnsuccessfulEncodeDecode", f),
            NFSError::AsymmetricDecipherFailure           => ::std::fmt::Display::fmt("NFSError::AsymmetricDecipherFailure", f),
            NFSError::SymmetricDecipherFailure            => ::std::fmt::Display::fmt("NFSError::SymmetricDecipherFailure", f),
            NFSError::RoutingFailure(_)                   => ::std::fmt::Display::fmt("NFSError::RoutingFailure", f), // TODO Improve these containing nested stuff to print as well
            NFSError::ReceivedUnexpectedData              => ::std::fmt::Display::fmt("NFSError::ReceivedUnexpectedData", f),
            NFSError::VersionCacheMiss                    => ::std::fmt::Display::fmt("NFSError::VersionCacheMiss", f),
            NFSError::RoutingMessageCacheMiss             => ::std::fmt::Display::fmt("NFSError::RoutingMessageCacheMiss", f),
            NFSError::NetworkOperationFailure(_)          => ::std::fmt::Display::fmt("NFSError::NetworkOperationFailure", f),
            NFSError::RootDirectoryAlreadyExists          => ::std::fmt::Display::fmt("NFSError::RootDirectoryAlreadyExists", f),
            NFSError::GenericIoError(_)                   => ::std::fmt::Display::fmt("NFSError::GenericIoError", f),
            NFSError::UnhandledError                      => ::std::fmt::Display::fmt("NFSError::UnhandledError", f),
        }
    }
}
