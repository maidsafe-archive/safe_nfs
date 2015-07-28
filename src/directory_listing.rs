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
/// DirectoryListing is the representation of a deserialised Directory in the network
pub struct DirectoryListing {
    info: ::directory_info::DirectoryInfo,
    sub_directories: Vec<::directory_info::DirectoryInfo>,
    files: Vec<::file::File>,
}

impl DirectoryListing {
    /// Create a new DirectoryListing
    pub fn new(name: String, user_metadata: Option<Vec<u8>>,
               versioned: bool, share_level: ::ShareLevel) -> DirectoryListing {
        DirectoryListing {
            info: ::directory_info::DirectoryInfo::new(::metadata::Metadata::new(name,
                                                                                 user_metadata,
                                                                                 Some(versioned),
                                                                                 Some(share_level))),
            sub_directories: Vec::new(),
            files: Vec::new(),
        }
    }

    /// Get ::directory_info::DirectoryInfo
    pub fn get_info(&self) -> &::directory_info::DirectoryInfo {
        &self.info
    }

    #[allow(dead_code)]
    /// Get Directory metadata in mutable format so that it can also be updated
    pub fn get_mut_metadata(&mut self) -> &mut ::metadata::Metadata {
        self.info.get_mut_metadata()
    }

    /// Get Directory metadata
    pub fn get_metadata(&self) -> &::metadata::Metadata {
        self.info.get_metadata()
    }

    // pub fn get_parent_dir_id(&self) -> &::routing::NameType {
    //     self.info.get_parent_dir_id()
    // }

    /// If file is present in the DirectoryListing then replace it else insert it
    pub fn upsert_file(&mut self, file: ::file::File) {
        match self.files.iter().position(|entry| entry.get_name() == file.get_name()) {
            Some(pos) => *self.files.get_mut(pos).unwrap() = file,
            None => self.files.push(file),
        }
    }

    /// Get all files in this DirectoryListing
    pub fn get_files(&self) -> &Vec<::file::File> {
        &self.files
    }

    /// Get all files in this DirectoryListing with mutability to update the listing of files
    pub fn get_mut_files(&mut self) -> &mut Vec<::file::File> {
        &mut self.files
    }

    /// Get all subdirectories in this DirectoryListing
    pub fn get_sub_directories(&self) -> &Vec<::directory_info::DirectoryInfo> {
        &self.sub_directories
    }

    /// Get all subdirectories in this DirectoryListing with mutability to update the listing of subdirectories
    pub fn get_mut_sub_directories(&mut self) -> &mut Vec<::directory_info::DirectoryInfo> {
        &mut self.sub_directories
    }

    /// Get the unique ID that represents this DirectoryListing in the network
    pub fn get_id(&self) -> &::routing::NameType {
        self.info.get_id()
    }
}

impl ::std::fmt::Debug for DirectoryListing {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "id: {}, metadata: {}", *self.info.get_id(), *self.info.get_metadata())
    }
}

impl ::std::fmt::Display for DirectoryListing {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "id: {}, metadata: {}", *self.info.get_id(), *self.info.get_metadata())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cbor;

    #[test]
    fn serialise() {
        let obj_before = DirectoryListing::new("Home".to_string(), Some("{mime:\"application/json\"}".to_string().into_bytes()), true, ::ShareLevel::Private);

        let mut e = cbor::Encoder::from_memory();
        e.encode(&[&obj_before]).unwrap();

        let mut d = cbor::Decoder::from_bytes(e.as_bytes());
        let obj_after: DirectoryListing = d.decode().next().unwrap().unwrap();

        assert_eq!(obj_before, obj_after);
    }
}
