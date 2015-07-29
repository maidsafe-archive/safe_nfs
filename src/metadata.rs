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
pub struct Metadata {
    name: String,
    size: u64,
    created_time:  ::time::Tm,
    modified_time: ::time::Tm,
    user_metadata: Option<Vec<u8>>,
}

impl Metadata {
    /// Create a new instance of Metadata
    pub fn new(name: String, user_metadata: Option<Vec<u8>>) -> Metadata {
        Metadata {
            name: name,
            size: 0,
            created_time:  ::time::now_utc(),
            modified_time: ::time::now_utc(),
            user_metadata: user_metadata,
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

    /// Get size information
    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn set_size(&mut self, size: u64){
        self.size = size;
    }

    /// Get user setteble custom metadata
    pub fn get_user_metadata(&self) -> Option<Vec<u8>> {
        self.user_metadata.clone()
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

impl Encodable for Metadata {
    fn encode<E: Encoder>(&self, e: &mut E)->Result<(), E::Error> {
        let created_time = self.created_time.to_timespec();
        let modified_time = self.modified_time.to_timespec();
        ::cbor::CborTagEncode::new(5483_000, &(self.name.clone(),
                                               self.size as usize,
                                               self.user_metadata.clone(),
                                               created_time.sec,
                                               created_time.nsec,
                                               modified_time.sec,
                                               modified_time.nsec)).encode(e)
    }
}

impl Decodable for Metadata {
    fn decode<D: Decoder>(d: &mut D)->Result<Metadata, D::Error> {
        try!(d.read_u64());
        let (name, size, meta, created_sec, created_nsec, modified_sec, modified_nsec) = try!(Decodable::decode(d));

        Ok(Metadata {
                name: name,
                user_metadata: meta,
                size: size,
                created_time: ::time::at_utc(::time::Timespec {
                        sec: created_sec,
                        nsec: created_nsec
                    }),
                modified_time: ::time::at_utc(::time::Timespec {
                        sec: modified_sec,
                        nsec: modified_nsec
                    }),
            })
    }
}

impl ::std::fmt::Debug for Metadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "name: {}, size: {}, user_metadata: {:?}", self.name, self.size, self.user_metadata)
    }
}

impl ::std::fmt::Display for Metadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "name: {}, size: {}, user_metadata: {:?}", self.name, self.size, self.user_metadata)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cbor;

    #[test]
    fn serialise() {
        // TODO Elaborate test cases
        let obj_before = Metadata::new("hello.txt".to_string(),
                                       Some("{mime: \"application/json\"}".to_string().into_bytes()));

        let mut e = cbor::Encoder::from_memory();
        e.encode(&[&obj_before]).unwrap();

        let mut d = cbor::Decoder::from_bytes(e.as_bytes());
        let obj_after: Metadata = d.decode().next().unwrap().unwrap();

        assert_eq!(obj_before, obj_after);
    }
}
