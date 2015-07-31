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
pub enum NfsError {
    /// Client Error
    ClientError(::safe_client::errors::ClientError),
    // TODO remove already exists
    /// If Directory already exists
    AlreadyExists,
    /// Destonation is Same as the Source
    DestinationAndSourceAreSame,
    /// Directory not found
    DirectoryNotFound,
    /// Failed to update directory
    FailedToUpdateDirectory,
    /// Failed to update file
    FailedToUpdateFile,
    /// File already present in the destonation specified
    FileExistsInDestination,
    /// File not found
    FileNotFound,
    /// Invalid byte range specified
    InvalidRangeSpecified,
    // TODO remove MetadataIsEmpty
    /// Metadata can not be empty
    MetadataIsEmpty,
    /// Metadata for the directory is missing or may be corrupted
    MetaDataMissingOrCorrupted,
    /// Name can not be empty
    NameIsEmpty,
    // TODO remove not found
    /// General
    NotFound,
}

impl From<::safe_client::errors::ClientError> for NfsError {
    fn from(error: ::safe_client::errors::ClientError) -> NfsError {
        NfsError::ClientError(error)
    }
}

impl ::std::fmt::Debug for NfsError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            NfsError::ClientError(ref error)        => write!(f, "NfsError::ClientError -> {:?}", error),
            NfsError::AlreadyExists                 => write!(f, "NfsError::AlreadyExists"),
            NfsError::DestinationAndSourceAreSame   => write!(f, "NfsError::DestinationAndSourceAreSame"),
            NfsError::DirectoryNotFound             => write!(f, "NfsError::DirectoryNotFound"),
            NfsError::FailedToUpdateDirectory       => write!(f, "NfsError::FailedToUpdateDirectory"),
            NfsError::FailedToUpdateFile            => write!(f, "NfsError::FailedToUpdateFile"),
            NfsError::FileExistsInDestination       => write!(f, "NfsError::FileExistsInDestination"),
            NfsError::FileNotFound                  => write!(f, "NfsError::FileNotFound"),
            NfsError::InvalidRangeSpecified         => write!(f, "NfsError::InvalidRangeSpecified"),
            NfsError::MetadataIsEmpty               => write!(f, "NfsError::MetadataIsEmpty"),
            NfsError::MetaDataMissingOrCorrupted    => write!(f, "NfsError::MetaDataMissingOrCorrupted"),
            NfsError::NameIsEmpty                   => write!(f, "NfsError::NameIsEmpty"),
            NfsError::NotFound                      => write!(f, "NfsError::NotFound"),
        }
    }
}
