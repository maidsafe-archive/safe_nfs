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

/// Reader is used to read contents of a File. It can read in chunks if the file happens to be very
/// large
#[allow(dead_code)]
pub struct Reader<'a> {
    client        : ::std::sync::Arc<::std::sync::Mutex<::safe_client::client::Client>>,
    self_encryptor: ::self_encryption::SelfEncryptor<::safe_client::SelfEncryptionStorage>,
    file          : &'a ::file::File,
}

impl<'a> Reader<'a> {
    /// Create a new instance of Reader
    pub fn new(client: ::std::sync::Arc<::std::sync::Mutex<::safe_client::client::Client>>,
               file  : &'a ::file::File) -> Reader {
        let se_storage = ::safe_client::SelfEncryptionStorage::new(client.clone());

        Reader {
            client        : client.clone(),
            self_encryptor: ::self_encryption::SelfEncryptor::new(se_storage, file.get_datamap().clone()),
            file          : file,
        }
    }

    /// Returns the total size of the file/blob
    pub fn size(&self) -> u64 {
        debug!("Retrieving file length ...");
        self.self_encryptor.len()
    }
    
    /// Read data from file/blob
    pub fn read(&mut self,  position: u64, length: u64) -> Result<Vec<u8>, ::errors::NfsError> {
        if (position + length) > self.size() {
            Err(::errors::NfsError::InvalidRangeSpecified)
        } else {
            debug!("Reading data from file from {pos} to {len} ...", pos = position, len = length);
            Ok(self.self_encryptor.read(position, length))
        }
    }
}
