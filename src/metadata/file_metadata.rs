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


/// FileMetadata about a File or a Directory
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct FileMetadata {
    name         : String,
    size         : u64,
    created_time : ::time::Tm,
    modified_time: ::time::Tm,
    user_metadata: Vec<u8>,
}

impl FileMetadata {
    /// Create a new instance of FileMetadata
    pub fn new(name: String, user_metadata: Vec<u8>) -> FileMetadata {
        FileMetadata {
            name         : name,
            size         : 0,
            created_time : ::time::now_utc(),
            modified_time: ::time::now_utc(),
            user_metadata: user_metadata,
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

    /// Get size information
    pub fn get_size(&self) -> u64 {
        self.size
    }

    /// Get user setteble custom metadata
    pub fn get_user_metadata(&self) -> &Vec<u8> {
        &self.user_metadata
    }


    /// Set name associated with the structure (file or directory)
    #[allow(dead_code)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Set the size of file
    pub fn set_size(&mut self, size: u64) {
        self.size = size;
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

impl ::rustc_serialize::Encodable for FileMetadata {
    fn encode<E: ::rustc_serialize::Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        let created_time = self.created_time.to_timespec();
        let modified_time = self.modified_time.to_timespec();
        ::cbor::CborTagEncode::new(5483_000, &(&self.name,
                                               self.size as usize,
                                               &self.user_metadata,
                                               created_time.sec,
                                               created_time.nsec,
                                               modified_time.sec,
                                               modified_time.nsec)).encode(e)
    }
}

impl ::rustc_serialize::Decodable for FileMetadata {
    fn decode<D: ::rustc_serialize::Decoder>(d: &mut D) -> Result<FileMetadata, D::Error> {
        let _ = try!(d.read_u64());

        let (name,
             size,
             meta_data,
             created_sec,
             created_nsec,
             modified_sec,
             modified_nsec) = try!(::rustc_serialize::Decodable::decode(d));

        Ok(FileMetadata {
            name: name,
            user_metadata: meta_data,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialise() {
        let obj_before = FileMetadata::new("hello.txt".to_string(),
                                           "{mime: \"application/json\"}".to_string().into_bytes());
        let serialised_data = eval_result!(::maidsafe_client::utility::serialise(&obj_before));
        let obj_after = eval_result!(::maidsafe_client::utility::deserialise(&serialised_data));
        assert_eq!(obj_before, obj_after);
    }
}
