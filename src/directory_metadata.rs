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

use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
/// Metadata about a File or a Directory
pub struct DirectoryMetadata {
    name: String,
    created_time:  ::time::Tm,
    modified_time: ::time::Tm,
    parent_dir: Option<(::routing::NameType, u64)>,
    user_metadata: Option<Vec<u8>>,
    versioned: bool,
    access_level: ::AccessLevel,
}

impl DirectoryMetadata {
    /// Create a new instance of Metadata
    pub fn new(name: String,
               user_metadata: Option<Vec<u8>>,
               versioned: bool,
               access_level: ::AccessLevel,
               parent_dir: Option<(&::routing::NameType, u64)>) -> DirectoryMetadata {
        DirectoryMetadata {
            name: name,
            created_time:  ::time::now_utc(),
            modified_time: ::time::now_utc(),
            user_metadata: user_metadata,
            parent_dir: parent_dir.map(|key| { (key.0.clone(), key.1)}),
            versioned: versioned,
            access_level: access_level,
        }
    }

    /// Get time of creation
    pub fn get_created_time(&self) -> ::time::Tm {
        self.created_time
    }

    /// Get time of modification
    pub fn get_modified_time(&self) -> ::time::Tm {
        self.modified_time
    }

    /// Get name associated with the structure (file or directory) that this metadata is a part
    /// of
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Returns the AccessLevel
    pub fn get_access_level(&self) -> ::AccessLevel {
        self.access_level.clone()
    }

    /// Get user setteble custom metadata
    pub fn get_user_metadata(&self) -> Option<Vec<u8>> {
        self.user_metadata.clone()
    }

    /// Returns whther the DirectoryListing is versioned or not
    pub fn is_versioned(&self) -> bool {
        self.versioned
    }

    #[allow(dead_code)]
    /// Set name associated with the structure (file or directory) that this metadata is a part
    /// of
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Set time of modification
    pub fn set_modified_time(&mut self, modified_time: ::time::Tm) {
        self.modified_time = modified_time
    }

    /// User setteble metadata for custom metadata
    pub fn set_user_metadata(&mut self, user_metadata: Option<Vec<u8>>) {
        self.user_metadata = user_metadata;
    }

}

impl Encodable for DirectoryMetadata {
    fn encode<E: Encoder>(&self, e: &mut E)->Result<(), E::Error> {
        let created_time = self.created_time.to_timespec();
        let modified_time = self.modified_time.to_timespec();
        ::cbor::CborTagEncode::new(5483_000, &(self.name.clone(),
                                               self.parent_dir.clone(),
                                               self.user_metadata.clone(),
                                               created_time.sec,
                                               created_time.nsec,
                                               modified_time.sec,
                                               modified_time.nsec,
                                               self.versioned,
                                               self.access_level.clone())).encode(e)
    }
}

impl Decodable for DirectoryMetadata {
    fn decode<D: Decoder>(d: &mut D)->Result<DirectoryMetadata, D::Error> {
        try!(d.read_u64());
        let (name, parent_dir, meta, created_sec, created_nsec, modified_sec,
            modified_nsec, versioned, access_level) = try!(Decodable::decode(d));

        Ok(DirectoryMetadata {
                name: name,
                user_metadata: meta,
                parent_dir: parent_dir,
                created_time: ::time::at_utc(::time::Timespec {
                        sec: created_sec,
                        nsec: created_nsec
                    }),
                modified_time: ::time::at_utc(::time::Timespec {
                        sec: modified_sec,
                        nsec: modified_nsec
                    }),
                versioned: versioned,
                access_level: access_level,
            })
    }
}

impl ::std::fmt::Debug for DirectoryMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "name: {}, versioned: {}", self.name, self.versioned)
    }
}

impl ::std::fmt::Display for DirectoryMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "name: {}, versioned: {}", self.name, self.versioned)
    }
}
