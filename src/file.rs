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

/// Representation of a File to be put into the network. Could be text, music, video etc any kind
/// of file
#[derive(RustcEncodable, RustcDecodable, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct File {
    metadata: ::metadata::file_metadata::FileMetadata,
    datamap : ::self_encryption::datamap::DataMap,
}

impl File {
    /// Create a new instance of File
    pub fn new(metadata: ::metadata::file_metadata::FileMetadata,
               datamap : ::self_encryption::datamap::DataMap) -> File {
        File {
            metadata: metadata,
            datamap: datamap,
        }
    }

    /// Get the name of the File
    pub fn get_name(&self) -> &String {
        self.metadata.get_name()
    }

    /// Get metadata associated with the file
    pub fn get_metadata(&self) -> &::metadata::file_metadata::FileMetadata {
        &self.metadata
    }

    /// Get metadata associated with the file, with mutability to allow updation
    pub fn get_mut_metadata(&mut self) -> &mut ::metadata::file_metadata::FileMetadata {
        &mut self.metadata
    }

    /// Get the data-map of the File. This is generated by passing the contents of the File to
    /// self-encryption
    pub fn get_datamap(&self) -> &::self_encryption::datamap::DataMap {
        &self.datamap
    }

    /// Set a data-map to be associated with the File
    pub fn set_datamap(&mut self, datamap: ::self_encryption::datamap::DataMap) {
        self.datamap = datamap;
    }
}

impl ::std::fmt::Debug for File {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "File > metadata: {:?}", self.metadata)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialise() {
        let obj_before = File::new(::metadata::file_metadata::FileMetadata::new("Home".to_string(),
                                                                                "{mime:\"application/json\"}".to_string().into_bytes()),
                                                                                ::self_encryption::datamap::DataMap::None);
        let serialised_data = eval_result!(::maidsafe_client::utility::serialise(&obj_before));
        let obj_after = eval_result!(::maidsafe_client::utility::deserialise(&serialised_data));
        assert_eq!(obj_before, obj_after);
    }
}
