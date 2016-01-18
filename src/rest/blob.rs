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

#[allow(dead_code)]
/// Blob represents a File - Music, Video, Text etc
pub struct Blob {
    file: ::file::File,
}

impl Blob {
    /// Get the name of the Blob
    pub fn get_name(&self) -> &String {
        self.file.get_metadata().get_name()
    }

    // TODO metadata is convering utf8 string - better to do no conversion and just send the vec<u8> to the caller
    /// Get the user settable Metadata of the Blob
    pub fn get_metadata(&self) -> String {
        match String::from_utf8(self.file.get_metadata().get_user_metadata().clone()) {
            Ok(data) => data,
            Err(_) => "".to_string(),
        }
    }

    /// Get the creation time for Blob
    pub fn get_created_time(&self) -> &::time::Tm {
        self.file.get_metadata().get_created_time()
    }

    /// Get the last modified time for the Blob
    pub fn get_modified_time(&self) -> &::time::Tm {
        self.file.get_metadata().get_modified_time()
    }

    /// Get the Blob size in bytes
    pub fn get_size(&self) -> u64 {
        self.file.get_metadata().get_size()
    }

    /// Convert the Blob to the format acceptable to the lower level Api's
    pub fn convert_to_file(&self) -> &::file::File {
        &self.file
    }

    // TODO Implement from trait for coversion
    /// Convert the Blob to the format acceptable to the lower level Api's
    /// This can also be modified on the fly as the return is a mutable value
    pub fn convert_to_mut_file(&mut self) -> &mut ::file::File {
        &mut self.file
    }

    /// Convert the format acceptable to the lower level Api's into a Blob for more restful
    /// interface
    pub fn convert_from_file(file: ::file::File) -> Blob {
        Blob {
            file: file,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create() {
        let datamap = ::self_encryption::datamap::DataMap::None;
        let metadata = ::metadata::file_metadata::FileMetadata::new("blob".to_string(), Vec::new());
        let file = ::file::File::new(metadata.clone(), datamap.clone());

        let blob = Blob{file: file.clone() };

        assert_eq!(*blob.get_name(), *metadata.get_name());
        assert_eq!(blob.get_created_time(), metadata.get_created_time());
        assert_eq!(blob.get_modified_time(), metadata.get_modified_time());
        assert_eq!(blob.get_size(), metadata.get_size());
        assert!(blob.get_metadata().is_empty());

        let file = blob.convert_to_file();

        assert_eq!(file.get_name(), metadata.get_name());
        assert_eq!(file.get_metadata().get_created_time(), metadata.get_created_time());
        assert_eq!(file.get_metadata().get_modified_time(), metadata.get_modified_time());
        assert_eq!(file.get_metadata().get_size(), metadata.get_size());
        assert_eq!(file.get_datamap().len(), datamap.len());
        assert!(!file.get_datamap().has_chunks());
    }

    #[test]
    fn create_from_file() {
        let datamap = ::self_encryption::datamap::DataMap::None;
        let metadata = ::metadata::file_metadata::FileMetadata::new("blob".to_string(), Vec::new());
        let file = ::file::File::new(metadata.clone(), datamap.clone());

        let blob = Blob::convert_from_file(file.clone());

        assert_eq!(*blob.get_name(), *file.get_name());
        assert_eq!(blob.get_created_time(), file.get_metadata().get_created_time());
        assert_eq!(blob.get_modified_time(), file.get_metadata().get_modified_time());
        assert_eq!(blob.get_size(), file.get_metadata().get_size());
        assert!(blob.get_metadata().is_empty());
    }

    #[test]
    fn convert_to_file() {
        let datamap = ::self_encryption::datamap::DataMap::None;
        let metadata = ::metadata::file_metadata::FileMetadata::new("blob".to_string(), Vec::new());
        let file = ::file::File::new(metadata.clone(), datamap.clone());

        let blob = Blob{ file: file.clone() };

        assert_eq!(*blob.get_name(), *file.get_name());
        assert_eq!(blob.get_created_time(), file.get_metadata().get_created_time());
        assert_eq!(blob.get_modified_time(), file.get_metadata().get_modified_time());
        assert_eq!(blob.get_size(), file.get_metadata().get_size());
        assert!(blob.get_metadata().is_empty());
        assert!(file.get_metadata().get_user_metadata().is_empty());

        let file = blob.convert_to_file();

        assert_eq!(*blob.get_name(), *file.get_name());
        assert_eq!(blob.get_created_time(), file.get_metadata().get_created_time());
        assert_eq!(blob.get_modified_time(), file.get_metadata().get_modified_time());
        assert_eq!(blob.get_size(), file.get_metadata().get_size());
        assert!(file.get_metadata().get_user_metadata().is_empty());
    }

    #[test]
    fn compare() {
        let first_datamap = ::self_encryption::datamap::DataMap::None;
        let first_metadata = ::metadata::file_metadata::FileMetadata::new("first_blob".to_string(), Vec::new());
        let first_file = ::file::File::new(first_metadata.clone(), first_datamap.clone());

        let first_blob = Blob::convert_from_file(first_file.clone());
        let second_blob = Blob{file: first_file.clone() };

        // allow 'times' to be sufficiently distinct
        ::std::thread::sleep_ms(1000u32);

        let second_datamap = ::self_encryption::datamap::DataMap::None;
        let second_metadata = ::metadata::file_metadata::FileMetadata::new("second_blob".to_string(), Vec::new());
        let second_file = ::file::File::new(second_metadata, second_datamap.clone());

        let third_blob = Blob::convert_from_file(second_file.clone());

        assert_eq!(*first_blob.get_name(), *second_blob.get_name());
        assert_eq!(first_blob.get_created_time(), second_blob.get_created_time());
        assert_eq!(first_blob.get_modified_time(), second_blob.get_modified_time());

        assert!(*first_blob.get_name() != *third_blob.get_name());
        assert!(first_blob.get_created_time() != third_blob.get_created_time());
        assert!(first_blob.get_modified_time() != third_blob.get_modified_time());
    }
}
