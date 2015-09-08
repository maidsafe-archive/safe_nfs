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

/// Metadata about a File or a Directory
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct DirectoryMetadata {
    name          : String,
    created_time  : ::time::Tm,
    modified_time : ::time::Tm,
    user_metadata : Vec<u8>,
    versioned     : bool,
    access_level  : ::AccessLevel,
    parent_dir_key: Option<(::routing::NameType, u64, bool, ::AccessLevel)>,
}

impl DirectoryMetadata {
    /// Create a new instance of Metadata
    pub fn new(name          : String,
               user_metadata : Vec<u8>,
               versioned     : bool,
               access_level  : ::AccessLevel,
               parent_dir_key: Option<(::routing::NameType, u64, bool, ::AccessLevel)>) -> DirectoryMetadata {
        DirectoryMetadata {
            name          : name,
            created_time  : ::time::now_utc(),
            modified_time : ::time::now_utc(),
            user_metadata : user_metadata,
            versioned     : versioned,
            access_level  : access_level,
            parent_dir_key: parent_dir_key,
        }
    }

    /// Get time of creation
    pub fn get_created_time(&self) -> &::time::Tm {
        &self.created_time
    }

    /// Get time of modification
    pub fn get_modified_time(&self) -> &::time::Tm {
        &self.modified_time
    }

    /// Get name associated with the structure (file or directory) that this metadata is a part
    /// of
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Returns the AccessLevel
    pub fn get_access_level(&self) -> &::AccessLevel {
        &self.access_level
    }

    /// Returns the Parent dir id
    pub fn get_parent_dir_key(&self) -> Option<(&::routing::NameType, u64, bool, &::AccessLevel)> {
        self.parent_dir_key.iter().next().map(|a| (&a.0, a.1, a.2, &a.3))
    }

    /// Get user setteble custom metadata
    pub fn get_user_metadata(&self) -> &Vec<u8> {
        &self.user_metadata
    }

    /// Returns whther the DirectoryListing is versioned or not
    pub fn is_versioned(&self) -> bool {
        self.versioned
    }

    /// Set name associated with the structure (file or directory) that this metadata is a part
    /// of
    #[allow(dead_code)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Set time of modification
    pub fn set_modified_time(&mut self, modified_time: ::time::Tm) {
        self.modified_time = modified_time
    }

    /// User setteble metadata for custom metadata
    pub fn set_user_metadata(&mut self, user_metadata: Vec<u8>) {
        self.user_metadata = user_metadata;
    }
}

impl ::rustc_serialize::Encodable for DirectoryMetadata {
    fn encode<E: ::rustc_serialize::Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        let created_time = self.created_time.to_timespec();
        let modified_time = self.modified_time.to_timespec();

        e.emit_struct("DirectoryMetadata", 9, |e| {
            try!(e.emit_struct_field("name",               0, |e| self.name.encode(e)));
            try!(e.emit_struct_field("created_time_sec",   1, |e| created_time.sec.encode(e)));
            try!(e.emit_struct_field("created_time_nsec",  2, |e| created_time.nsec.encode(e)));
            try!(e.emit_struct_field("modified_time_sec",  3, |e| modified_time.sec.encode(e)));
            try!(e.emit_struct_field("modified_time_nsec", 4, |e| modified_time.nsec.encode(e)));
            try!(e.emit_struct_field("user_metadata",      5, |e| self.user_metadata.encode(e)));
            try!(e.emit_struct_field("versioned",          6, |e| self.versioned.encode(e)));
            try!(e.emit_struct_field("access_level",       7, |e| self.access_level.encode(e)));
            try!(e.emit_struct_field("parent_dir_key",     8, |e| self.parent_dir_key.encode(e)));

            Ok(())
        })
    }
}

impl ::rustc_serialize::Decodable for DirectoryMetadata {
    fn decode<D: ::rustc_serialize::Decoder>(d: &mut D) -> Result<DirectoryMetadata, D::Error> {
        d.read_struct("DirectoryMetadata", 9, |d| {
            Ok(DirectoryMetadata {
                name          : try!(d.read_struct_field("name", 0, |d| ::rustc_serialize::Decodable::decode(d))),
                created_time  : ::time::at_utc(::time::Timespec {
                                                   sec : try!(d.read_struct_field("created_time_sec",  1, |d| ::rustc_serialize::Decodable::decode(d))),
                                                   nsec: try!(d.read_struct_field("created_time_nsec", 2, |d| ::rustc_serialize::Decodable::decode(d))),
                                               }),
                modified_time : ::time::at_utc(::time::Timespec {
                                                   sec : try!(d.read_struct_field("modified_time_sec",  3, |d| ::rustc_serialize::Decodable::decode(d))),
                                                   nsec: try!(d.read_struct_field("modified_time_nsec", 4, |d| ::rustc_serialize::Decodable::decode(d))),
                                               }),
                user_metadata : try!(d.read_struct_field("user_metadata",  5, |d| ::rustc_serialize::Decodable::decode(d))),
                versioned     : try!(d.read_struct_field("versioned",      6, |d| ::rustc_serialize::Decodable::decode(d))),
                access_level  : try!(d.read_struct_field("access_level",   7, |d| ::rustc_serialize::Decodable::decode(d))),
                parent_dir_key: try!(d.read_struct_field("parent_dir_key", 8, |d| ::rustc_serialize::Decodable::decode(d))),
            })
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialise() {
        let obj_before = DirectoryMetadata::new("hello.txt".to_string(),
                                                "{mime: \"application/json\"}".to_string().into_bytes(),
                                                true,
                                                ::AccessLevel::Private,
                                                None);
        let serialised_data = eval_result!(::safe_client::utility::serialise(&obj_before));
        let obj_after = eval_result!(::safe_client::utility::deserialise(&serialised_data));
        assert_eq!(obj_before, obj_after);
    }
}
