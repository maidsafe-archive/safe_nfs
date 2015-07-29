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


/// Container Repersents a Directory.
/// Container can have its own metadata, sub-containers and files
pub struct Container {
    client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
    directory_listing: ::directory_listing::DirectoryListing,
}

impl Container {
    /// Authorises the directory access and returns the Container, if authorisation is successful.
    /// Operations can be performed only after the authorisation is successful.
    pub fn authorise(client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
                     container_info: Option<::rest::ContainerInfo>) -> Result<Container, ::errors::NfsError> {
        let directory_helper = ::helper::directory_helper::DirectoryHelper::new(client.clone());
        let directory = match container_info {
            Some(container_info) => {
                let dir_info = container_info.convert_to_directory_info();
                let metadata = dir_info.get_metadata();
                try!(directory_helper.get(dir_info.get_key(), metadata.is_versioned(), metadata.get_access_level().clone()))
            },
            None => try!(directory_helper.get_user_root_directory_listing()),
        };
        Ok(Container {
            client: client,
            directory_listing: directory,
        })
    }

    /// Creates a Container
    pub fn create(&mut self, name: String, versioned: bool, access_level: ::AccessLevel) -> Result<::rest::ContainerInfo, ::errors::NfsError> {
        if name.is_empty() {
            return Err(::errors::NfsError::NameIsEmpty);
        }
        // TODO add metadata support to containers
        let metadata = None;
        let user_metadata = try!(self.validate_metadata(metadata));
        let tag_type = if versioned {
            ::VERSION_DIRECTORY_LISTING_TAG
        } else {
            ::UNVERSION_DIRECTORY_LISTING_TAG
        };
        match self.directory_listing.find_sub_directory(&name) {
            Some(_) => Err(::errors::NfsError::AlreadyExists),
            None => {
                let directory_helper = ::helper::directory_helper::DirectoryHelper::new(self.client.clone());
                let directory_created = try!(directory_helper.create(name, tag_type, user_metadata, versioned, access_level, Some(&mut self.directory_listing)));
                Ok(::rest::ContainerInfo::convert_from_directory_info(directory_created.get_info().clone()))
            }
        }
    }

    /// Returns the Created time of the container
    pub fn get_created_time(&self) -> &::time::Tm {
        self.directory_listing.get_metadata().get_created_time()
    }

    /// Return the unique id of the container
    pub fn get_info(&self) -> ::rest::ContainerInfo {
        ::rest::ContainerInfo::convert_from_directory_info(self.directory_listing.get_info().clone())
    }

    /// Returns the user metadata saved as String.
    pub fn get_metadata(&self) -> String {
        match String::from_utf8(self.directory_listing.get_metadata().get_user_metadata().clone()) {
            Ok(data) => data,
            Err(_) => "".to_string(),
        }
    }

    /// Returns the name of the container
    pub fn get_name(&self) -> &String {
        self.directory_listing.get_metadata().get_name()
    }

    /// Returns the list of Blobs in the container
    pub fn get_blobs(&self) -> Vec<::rest::Blob> {
        self.directory_listing.get_files().iter().map(|x| ::rest::Blob::convert_from_file(x.clone())).collect()
    }

    /// Returns a Blob from the container
    pub fn get_blob(&self, name: String, version: Option<[u8; 64]>) -> Result<::rest::blob::Blob, ::errors::NfsError> {
        match version {
            Some(version_id) => {
                let directory_helper = ::helper::directory_helper::DirectoryHelper::new(self.client.clone());
                let directory_listing_version = try!(directory_helper.get_by_version(self.directory_listing.get_key(),
                                                                                     self.directory_listing.get_metadata().get_access_level().clone(),
                                                                                     ::routing::NameType(version_id)));
                match directory_listing_version.find_file(&name) {
                    Some(file) => Ok(::rest::blob::Blob::convert_from_file(file.clone())),
                    None => Err(::errors::NfsError::NotFound),
                }
            },
            None => match self.directory_listing.find_file(&name) {
                Some(file) => Ok(::rest::blob::Blob::convert_from_file(file.clone())),
                None => Err(::errors::NfsError::NotFound),
            },
        }
    }

    /// Returns the list of child containers
    pub fn get_containers(&self) -> Vec<::rest::ContainerInfo> {
        self.directory_listing.get_sub_directories().iter().map(|info| {
                ::rest::ContainerInfo::convert_from_directory_info(info.clone())
            }).collect()
    }

    // /// Updates the metadata of the container
    // pub fn update_metadata(&mut self, metadata: Option<String>) -> Result<(), String>{
    //     match self.validate_metadata(metadata) {
    //         Ok(user_metadata) => {
    //             self.directory_listing.get_mut_metadata().set_user_metadata(user_metadata);
    //             let mut directory_helper = ::helper::DirectoryHelper::new(self.client.clone());
    //             match directory_helper.update(&self.directory_listing) {
    //                 Ok(_) => Ok(()),
    //                 Err(_) => Err("Error".to_string()),
    //             }
    //         },
    //         Err(err) => Err(err),
    //     }
    // }

    /// Retrieves Versions for the container
    pub fn get_versions(&self) -> Result<Vec<[u8; 64]>, ::errors::NfsError> {
        self.list_container_versions(self.directory_listing.get_key())
    }

    /// Retrieves Versions for the container being referred by the container_id
    pub fn get_container_versions(&self, container_info: &::rest::container_info::ContainerInfo) -> Result<Vec<[u8; 64]>, ::errors::NfsError> {
        let directory_info = container_info.convert_to_directory_info();
        self.list_container_versions(directory_info.get_key())
    }

    /// Fetches the latest version of the child container.
    /// Can fetch a specific version of the Container by passing the corresponding VersionId.
    pub fn get_container(&mut self, name: String, version: Option<[u8; 64]>) -> Result<Container, ::errors::NfsError> {
        let dir_info = try!(self.directory_listing.find_sub_directory(&name).ok_or(::errors::NfsError::NotFound));
        let directory_helper = ::helper::directory_helper::DirectoryHelper::new(self.client.clone());
        let dir_listing = match version {
            Some(version_id) => try!(directory_helper.get_by_version(self.directory_listing.get_key(),
                                                                     self.directory_listing.get_metadata().get_access_level().clone(),
                                                                     ::routing::NameType(version_id))),
            None =>  try!(directory_helper.get(dir_info.get_key(),
                                               dir_info.get_metadata().is_versioned(),
                                               dir_info.get_metadata().get_access_level().clone())),
        };
        Ok(Container {
            client: self.client.clone(),
            directory_listing: dir_listing,
        })
    }

   /// Deletes the child container
    pub fn delete_container(&mut self, name: &String) -> Result<(), ::errors::NfsError> {
        let directory_helper = ::helper::directory_helper::DirectoryHelper::new(self.client.clone());
        directory_helper.delete(&mut self.directory_listing, name)
    }

    /// Creates a Blob within the container
    /// Returns a Writter object
    /// The content of the blob is written using the writter.
    /// The blob is created only after the writter.close() is invoked
    pub fn create_blob(&mut self, name: String, metadata: Option<String>) -> Result<::helper::writer::Writer, ::errors::NfsError> {
        if name.is_empty() {
            return Err(::errors::NfsError::NameIsEmpty);
        }
        let user_metadata = try!(self.validate_metadata(metadata));
        let file_helper = ::helper::file_helper::FileHelper::new(self.client.clone());
        file_helper.create(name, user_metadata, self.directory_listing.clone())
    }

    /// Updates the blob content. Writes the complete data and updates the Blob
    pub fn update_blob_content(&mut self, blob: &::rest::Blob, data: &[u8]) -> Result<::directory_listing::DirectoryListing, ::errors::NfsError> {
        let mut writer = try!(self.get_writer_for_blob(blob, ::helper::writer::Mode::Overwrite));
        writer.write(data, 0);
        writer.close()
    }

    /// Return a writter object for the Blob, through which the content of the blob can be updated
    /// This is useful while handling larger files, to enable writting content in parts
    pub fn get_blob_writer(&mut self, blob: &::rest::Blob) -> Result<::helper::writer::Writer, ::errors::NfsError> {
        self.get_writer_for_blob(blob, ::helper::writer::Mode::Modify)
    }

    /// Reads the content of the blob and returns the complete content
    pub fn get_blob_content(&self, blob: &::rest::Blob) -> Result<Vec<u8>, ::errors::NfsError> {
        let mut reader = try!(self.get_reader_for_blob(blob));
        let size = reader.size();
        reader.read(0, size)
    }

    /// Returns a reader for the blob
    /// Using a Reader helps in handling large file contents and also fetch data in a specific range
    pub fn get_blob_reader(&self, blob: &::rest::blob::Blob) -> Result<::helper::reader::Reader, ::errors::NfsError> {
        self.get_reader_for_blob(blob)
    }

    /// Returns the list of versions_id for the blob
    pub fn get_blob_versions(&self, name: &String) -> Result<Vec<::rest::blob::Blob>, ::errors::NfsError>{
        let file = try!(self.directory_listing.find_file(name).ok_or(::errors::NfsError::NotFound));
        let file_helper = ::helper::file_helper::FileHelper::new(self.client.clone());
        let versions = try!(file_helper.get_versions(&file, &self.directory_listing));
        Ok(versions.iter().map(|file| { ::rest::blob::Blob::convert_from_file(file.clone()) }).collect())
    }

    /// Update the metadata of the Blob in the container
    pub fn update_blob_metadata(&mut self, name: &String, metadata: Option<String>) ->Result<(), ::errors::NfsError> {
        let user_metadata = try!(self.validate_metadata(metadata));
        let file = try!(self.directory_listing.get_mut_files().iter().find(|file| *file.get_name() == *name).ok_or(::errors::NfsError::FileNotFound));
        let mut file_helper = ::helper::file_helper::FileHelper::new(self.client.clone());
        try!(file_helper.update_metadata(file.clone(), user_metadata, &mut self.directory_listing));
        Ok(())
    }

    /// Delete blob from the container
    pub fn delete_blob(&mut self, name: String) -> Result<(), ::errors::NfsError> {
        let mut file_helper = ::helper::file_helper::FileHelper::new(self.client.clone());
        try!(file_helper.delete(name, &mut self.directory_listing.clone()));
        Ok(())
    }

    /// Copies the latest blob version from the container to the specified destination container
    pub fn copy_blob(&mut self, blob_name: &String, to_container: &::rest::container_info::ContainerInfo) -> Result<(), ::errors::NfsError> {
        let to_dir_info = to_container.convert_to_directory_info();
        if self.directory_listing.get_key() == to_dir_info.get_key() {
            return Err(::errors::NfsError::DestinationAndSourceAreSame);
        }
        let file = try!(self.directory_listing.find_file(blob_name).ok_or(::errors::NfsError::NotFound));
        let directory_helper = ::helper::directory_helper::DirectoryHelper::new(self.client.clone());
        let mut destination = try!(directory_helper.get(to_dir_info.get_key(),
                                                   to_dir_info.get_metadata().is_versioned(),
                                                   to_dir_info.get_metadata().get_access_level().clone()));
        if destination.find_file(blob_name).is_some() {
           return Err(::errors::NfsError::FileExistsInDestination);
        }
        destination.get_mut_files().push(file.clone());
        try!(directory_helper.update(&destination));
        Ok(())
    }

    fn get_writer_for_blob(&self, blob: &::rest::blob::Blob, mode: ::helper::writer::Mode) -> Result<::helper::writer::Writer, ::errors::NfsError> {
        let helper = ::helper::file_helper::FileHelper::new(self.client.clone());
        helper.update(blob.convert_to_file().clone(), mode, self.directory_listing.clone())
    }

    fn get_reader_for_blob(&self, blob: &::rest::blob::Blob) -> Result<::helper::reader::Reader, ::errors::NfsError> {
        match self.directory_listing.find_file(blob.get_name()) {
            Some(_) => {
                Ok(::helper::reader::Reader::new(self.client.clone(), blob.convert_to_file().clone()))
            },
            None => Err(::errors::NfsError::NotFound),
        }
    }

    fn list_container_versions(&self, dir_key: (::routing::NameType, u64)) -> Result<Vec<[u8; 64]>, ::errors::NfsError> {
        let directory_helper = ::helper::directory_helper::DirectoryHelper::new(self.client.clone());
        let versions = try!(directory_helper.get_versions(dir_key));
        Ok(versions.iter().map(|v| v.0).collect())
    }

    fn validate_metadata(&self, metadata: Option<String>) -> Result<Vec<u8>, ::errors::NfsError> {
        match metadata {
            Some(data) => {
                if data.len() == 0 {
                    Err(::errors::NfsError::MetadataIsEmpty)
                } else {
                    Ok(data.into_bytes())
                }
            },
            None => Ok(Vec::new()),
        }
    }
}

/*
#[cfg(test)]
mod test {
    use super::*;
    use ::maidsafe_client::client::Client;
    use ::std::sync::Arc;
    use ::std::sync::Mutex;

    fn test_client() -> Client {
        ::maidsafe_client::utility::test_utils::get_client().ok().unwrap()
    }

    #[test]
    fn authorise_container() {
        let client = Arc::new(Mutex::new(test_client()));
        assert!(Container::authorise(client.clone(), None).is_ok(), true);
    }

    #[test]
    fn create_container() {
        let client = Arc::new(Mutex::new(test_client()));
        let mut container = Container::authorise(client.clone(), None).ok().unwrap();
        container.create("Home".to_string()).unwrap();

        assert_eq!(container.get_containers().len(), 1);
        assert_eq!(container.get_containers()[0].get_name(), "Home");
    }


    #[test]
    fn delete_container() {
        let client = Arc::new(Mutex::new(test_client()));
        let mut container = Container::authorise(client, None).ok().unwrap();
        container.create("Home".to_string()).unwrap();

        assert_eq!(container.get_containers().len(), 1);
        assert_eq!(container.get_containers()[0].get_name(), "Home");

        container.delete_container("Home".to_string()).unwrap();

        assert_eq!(container.get_containers().len(), 0);
        assert_eq!(container.get_versions().unwrap().len(), 3);
    }

    #[test]
    fn create_update_delete_blob() {
        let client = Arc::new(Mutex::new(test_client()));
        let mut container = Container::authorise(client.clone(), None).ok().unwrap();
        container.create("Home".to_string()).unwrap();

        assert_eq!(container.get_containers().len(), 1);
        assert_eq!(container.get_containers()[0].get_name(), "Home");

        let mut home_container = container.get_container("Home".to_string(), None).unwrap();
        let mut writer = home_container.create_blob("sample.txt".to_string(), None).unwrap();
        let data = "Hello World!".to_string().into_bytes();
        writer.write(&data[..], 0);
        writer.close().unwrap();
        home_container = container.get_container("Home".to_string(), None).unwrap();
        assert_eq!(home_container.get_blob_versions("sample.txt".to_string()).unwrap().len(), 1);
        let blob = home_container.get_blob("sample.txt".to_string(), None).unwrap();
        assert_eq!(home_container.get_blob_content(&blob).unwrap(), data);


        let data_updated = "Hello World updated!".to_string().into_bytes();
        let _ = home_container.update_blob_content(&blob, &data_updated[..]).unwrap();
        home_container = container.get_container("Home".to_string(), None).unwrap();
        let blob = home_container.get_blob("sample.txt".to_string(), None).unwrap();
        assert_eq!(home_container.get_blob_content(&blob).unwrap(), data_updated);

        let versions = home_container.get_blob_versions("sample.txt".to_string()).unwrap();
        assert_eq!(versions.len(), 2);
        for i in 0..2 {
            let blob = home_container.get_blob("sample.txt".to_string(), Some(versions[i])).unwrap();
            if i == 0 {
                assert_eq!(home_container.get_blob_content(&blob).unwrap(), data);
            } else {
                assert_eq!(home_container.get_blob_content(&blob).unwrap(), data_updated);
            }
        }
        let metadata = "{\"purpose\": \"test\"}".to_string();
        home_container.update_blob_metadata("sample.txt".to_string(), Some(metadata.clone())).unwrap();
        home_container = container.get_container("Home".to_string(), None).unwrap();
        assert_eq!(home_container.get_blob("sample.txt".to_string(), None).unwrap().get_metadata().unwrap(), metadata);

        container.create("Public".to_string()).unwrap();
        let mut Public_container = container.get_container("Public".to_string(), None).unwrap();
        assert_eq!(Public_container.get_blobs().len(), 0);
        let _ = home_container.copy_blob("sample.txt".to_string(), Public_container.get_id());
        Public_container = container.get_container("Public".to_string(), None).unwrap();
        assert_eq!(Public_container.get_blobs().len(), 1);

        let _ = home_container.delete_blob("sample.txt".to_string());
        assert_eq!(home_container.get_blobs().len(), 0);
    }

}
*/
