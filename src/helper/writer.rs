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

/// Mode of the writter
pub enum Mode {
    /// Will create a new data
    Overwrite,
    /// Will modify the existing data
    Modify,
}

/// Writer is used to write contents to a File and especially in chunks if the file happens to be
/// too large
pub struct Writer {
    client        : ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
    file          : ::file::File,
    directory     : ::directory_listing::DirectoryListing, // TODO call it parent_directory
    self_encryptor: ::self_encryption::SelfEncryptor<::maidsafe_client::SelfEncryptionStorage>,
}

impl Writer {
    /// Create new instance of Writer
    pub fn new(client   : ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
               mode     : Mode,
               directory: ::directory_listing::DirectoryListing, // TODO call it parent_directory
               file     : ::file::File) -> Writer {
        let datamap = match mode {
                Mode::Modify    => file.get_datamap().clone(),
                Mode::Overwrite => ::self_encryption::datamap::DataMap::None,
        };

        // TODO Pls define in order in structure above, not randomly, difficult to verify in review
        Writer {
            file: file.clone(), // TODO Why
            directory: directory,
            self_encryptor: ::self_encryption::SelfEncryptor::new(::maidsafe_client::SelfEncryptionStorage::new(client.clone()), datamap),
            client: client,
        }
    }

    /// Data of a file/blob can be written in smaller chunks
    pub fn write(&mut self, data: &[u8], position: u64) {
        self.self_encryptor.write(data, position);
    }

    // TODO return (updated and written to network) parent dir-listing
    /// close is invoked only after alll the data is completely written
    /// The file/blob is saved only when the close is invoked.
    pub fn close(mut self) -> Result<(), ::errors::NfsError> {
        let ref mut file = self.file;
        let ref mut directory = self.directory;
        let size = self.self_encryptor.len();

        file.set_datamap(self.self_encryptor.close());

        file.get_mut_metadata().set_modified_time(::time::now_utc());
        file.get_mut_metadata().set_size(size);

        directory.upsert_file(file.clone());

        let directory_helper = ::helper::directory_helper::DirectoryHelper::new(self.client.clone());
        directory_helper.update(&directory)
    }
}
