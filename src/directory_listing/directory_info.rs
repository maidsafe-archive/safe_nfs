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

/// Contains Information pertaining to a Directory
#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct DirectoryInfo {
    id      : ::routing::NameType,
    type_tag: u64,
    metadata: ::metadata::directory_metadata::DirectoryMetadata,
}

impl DirectoryInfo {
    /// Create a new DirectoryInfo
    pub fn new(metadata: ::metadata::directory_metadata::DirectoryMetadata, type_tag: u64) -> DirectoryInfo {
        DirectoryInfo {
            id      : ::routing::test_utils::Random::generate_random(),
            type_tag: type_tag,
            metadata: metadata,
        }
    }

    /// Get the unique ID representing this directory in the network
    pub fn get_key(&self) -> (&::routing::NameType, u64) {
        (&self.id, self.type_tag)
    }

    /// Get the metadata of this directory
    pub fn get_metadata(&self) -> &::metadata::directory_metadata::DirectoryMetadata {
        &self.metadata
    }

    /// Get the metadata of this directory. Since return value is mutable it can also be used to
    /// update the metadata
    #[allow(dead_code)]
    pub fn get_mut_metadata(&mut self) -> &mut ::metadata::directory_metadata::DirectoryMetadata {
        &mut self.metadata
    }

    /// Get the name of this directory
    pub fn get_name(&self) -> &String {
        self.metadata.get_name()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialise() {
        let metadata = ::metadata::directory_metadata::DirectoryMetadata::new("Hello.txt".to_string(), Vec::new(), true, ::AccessLevel::Public, None);
        let obj_before = DirectoryInfo::new(metadata, ::VERSIONED_DIRECTORY_LISTING_TAG);
        let serialised_data = eval_result!(::maidsafe_client::utility::serialise(&obj_before));
        let obj_after = eval_result!(::maidsafe_client::utility::deserialise(&serialised_data));

        assert_eq!(obj_before, obj_after);
    }
}
