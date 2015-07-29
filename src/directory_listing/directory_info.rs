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

#[derive(RustcEncodable, RustcDecodable, PartialEq, Eq, PartialOrd, Ord, Clone)]
/// Contains Information pertaining to a Directory
pub struct DirectoryInfo {
    id: ::routing::NameType,
    type_tag: u64,
    metadata: ::directory_metadata::DirectoryMetadata,
}

impl DirectoryInfo {
    /// Create a new DirectoryInfo
    pub fn new(metadata: ::directory_metadata::DirectoryMetadata, type_tag: u64) -> DirectoryInfo {
        DirectoryInfo {
            id: ::routing::test_utils::Random::generate_random(),
            type_tag: type_tag,
            metadata: metadata,
        }
    }

    /// Get the unique ID representing this directory in the network
    pub fn get_key(&self) -> (&::routing::NameType, u64) {
        (&self.id, self.type_tag)
    }

    #[allow(dead_code)]
    /// Get the metadata of this directory. Since return value is mutable it can also be used to
    /// update the metadata
    pub fn get_mut_metadata(&mut self) -> &mut ::directory_metadata::DirectoryMetadata {
        &mut self.metadata
    }

    /// Get the metadata of this directory
    pub fn get_metadata(&self) -> &::directory_metadata::DirectoryMetadata {
        &self.metadata
    }

    /// Get the name of this directory
    pub fn get_name(&self) -> &String {
        self.metadata.get_name()
    }

    // pub fn get_parent_dir_id(&self) -> &::routing::NameType {
    //     &self.parent_dir_id
    // }
}

impl ::std::fmt::Debug for DirectoryInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "metadata: {}, key: {}, {}", *self.get_metadata(), *self.get_key().0, self.get_key().1)
    }
}

impl ::std::fmt::Display for DirectoryInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "metadata: {}, key: {}, {}", *self.get_metadata(), *self.get_key().0, self.get_key().1)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cbor;

    #[test]
    fn serialise() {
        let metadata = ::directory_metadata::DirectoryMetadata::new("Hello.txt".to_string(), None, true, ::AccessLevel::Public, None);
        let obj_before = DirectoryInfo::new(metadata, ::VERSION_DIRECTORY_LISTING_TAG);

        let mut e = cbor::Encoder::from_memory();
        e.encode(&[&obj_before]).unwrap();

        let mut d = cbor::Decoder::from_bytes(e.as_bytes());
        let obj_after: DirectoryInfo = d.decode().next().unwrap().unwrap();

        assert_eq!(obj_before, obj_after);
    }
}
