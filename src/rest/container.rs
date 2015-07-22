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
                    dir_id: Option<[u8; 64]>) -> Result<Container, ::errors::NFSError> {
        let directory_id = match dir_id {
            Some(id) => ::routing::NameType(id),
            None => try!(::utility::get_user_root_directory_id(client.clone())),
        };
        let mut directory_helper = ::helper::DirectoryHelper::new(client.clone());
        Ok(Container {
            client: client,
            directory_listing: try!(directory_helper.get(&directory_id))
        })
    }

    /// Creates a Container
    pub fn create(&mut self, name: String) -> Result<(), String> {
        if name.is_empty() {
            return Err("Name can not be empty".to_string());
        }
        // TODO add metadata support to containers
        let metadata = None;
        match self.validate_metadata(metadata) {
            Ok(user_metadata) => {
                match self.directory_listing.get_sub_directories().iter().find(|&entry| *entry.get_name() == name) {
                    Some(_) => Err("Container already exists".to_string()),
                    None => {
                        let mut directory_helper = ::helper::DirectoryHelper::new(self.client.clone());
                        match directory_helper.create(name, user_metadata) {
                            Ok(dir_id) => {
                                let mut directory_helper = ::helper::DirectoryHelper::new(self.client.clone());
                                match directory_helper.get(&dir_id) {
                                    Ok(created_directory) => {
                                        self.directory_listing.get_mut_sub_directories().push(created_directory.get_info().clone());
                                        let mut directory_helper = ::helper::DirectoryHelper::new(self.client.clone());
                                        match directory_helper.update(&self.directory_listing) {
                                            Ok(_) => Ok(()),
                                            Err(_) => Err("Error".to_string())
                                        }
                                    },
                                    Err(_) => Err("Error".to_string())
                                }
                            },
                            Err(_) => Err("Error".to_string())
                        }
                    }
                }
            },
            Err(err) => Err(err),
        }
    }

    /// Returns the Created time of the container
    pub fn get_created_time(&self) -> ::time::Tm {
        self.directory_listing.get_metadata().get_created_time()
    }

    /// Return the unique id of the container
    pub fn get_id(&self) -> [u8; 64] {
        self.directory_listing.get_id().0
    }

    /// Returns the user metadata saved as String.
    /// None can be passed to clear the metadata
    pub fn get_metadata(&self) -> Option<String> {
        let metadata = self.directory_listing.get_metadata().get_user_metadata();
        match metadata {
            Some(data) => Some(String::from_utf8(data.clone()).unwrap()),
            None => None,
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
    pub fn get_blob(&self, name: String, version: Option<[u8; 64]>) -> Result<::rest::Blob, String> {
        match version {
            Some(version_id) => {
                let dir_id = self.directory_listing.get_id();
                let mut directory_helper = ::helper::DirectoryHelper::new(self.client.clone());
                match directory_helper.get_by_version(dir_id, &::routing::NameType(version_id)) {
                    Ok(listing) => match self.find_file(&name, &listing){
                        Some(blob) => Ok(blob),
                        None => Err("Blob not found for the version specified".to_string())
                    },
                    Err(_) => Err("Error".to_string())
                }
            },
            None => match self.find_file(&name, &self.directory_listing) {
                Some(blob) => Ok(blob),
                None => Err("Blob not found for the version specified".to_string())
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
    pub fn get_versions(&mut self) -> Result<Vec<[u8; 64]>, String> {
        let id = self.directory_listing.get_id().0;
        self.list_container_versions(&::routing::NameType(id))
    }

    /// Retrieves Versions for the container being referred by the container_id
    pub fn get_container_versions(&mut self, container_id: [u8; 64]) -> Result<Vec<[u8; 64]>, String> {
        self.list_container_versions(&::routing::NameType(container_id))
    }

    /// Fetches the latest version of the child container.
    /// Can fetch a specific version of the Container by passing the corresponding VersionId.
    pub fn get_container(&mut self, name: String, version: Option<[u8; 64]>) -> Result<Container, String> {
        let sub_dirs = self.directory_listing.get_sub_directories();
        match sub_dirs.iter().find(|&entry| *entry.get_name() == name) {
            Some(dir_info) => {
                let mut directory_helper = ::helper::DirectoryHelper::new(self.client.clone());
                let get_dir_listing_result = match version {
                    Some(version_id) => directory_helper.get_by_version(dir_info.get_id(), &::routing::NameType(version_id)),
                    None =>  directory_helper.get(dir_info.get_id())
                };
                match get_dir_listing_result {
                    Ok(dir_listing) => Ok(Container {
                        client: self.client.clone(),
                        directory_listing: dir_listing
                    }),
                    Err(_) => Err("Error".to_string())
                }
            },
            None => Err("Container not found".to_string()),
        }
    }

    /// Deletes the child container
    pub fn delete_container(&mut self, name: String) -> Result<(), String> {
        match self.directory_listing.get_sub_directories().iter().position(|entry| *entry.get_name() == name) {
            Some(pos) => {
                self.directory_listing.get_mut_sub_directories().remove(pos);
                let mut directory_helper = ::helper::DirectoryHelper::new(self.client.clone());
                match directory_helper.update(&self.directory_listing) {
                    Ok(_) => Ok(()),
                    Err(_) => Err("Error".to_string())
                }
            },
            None => {
                Err("Container not found".to_string())
            },
        }
    }

    /// Creates a Blob within the container
    /// Returns a Writter object
    /// The content of the blob is written using the writter.
    /// The blob is created only after the writter.close() is invoked
    pub fn create_blob(&mut self, name: String, metadata: Option<String>) -> Result<::io::Writer, String> {
        if name.is_empty() {
            return Err("Name can not be empty".to_string());
        }
        match self.validate_metadata(metadata) {
            Ok(user_metadata) => {
                let mut file_helper = ::helper::FileHelper::new(self.client.clone());
                file_helper.create(name, user_metadata, &self.directory_listing)
            },
            Err(err) => Err(err),
        }
    }

    /// Updates the blob content. Writes the complete data and updates the Blob
    pub fn update_blob_content(&mut self, blob: &::rest::Blob, data: &[u8]) -> Result<(), String> {
        match self.get_writer_for_blob(blob, ::io::writer::Mode::Overwrite) {
            Ok(mut writer) => {
                writer.write(data, 0);
                writer.close()
            },
            Err(err) => Err(err),
        }
    }

    /// Return a writter object for the Blob, through which the content of the blob can be updated
    /// This is useful while handling larger files, to enable writting content in parts
    pub fn get_blob_writer(&mut self, blob: &::rest::Blob) -> Result<::io::Writer, String> {
        self.get_writer_for_blob(blob, ::io::writer::Mode::Modify)
    }

    /// Reads the content of the blob and returns the complete content
    pub fn get_blob_content(&mut self, blob: &::rest::Blob) -> Result<Vec<u8>, String> {
        match self.get_reader_for_blob(blob) {
            Ok(mut reader) => {
                let size = reader.size();
                reader.read(0, size)
            },
            Err(_) => Err("Error".to_string()),
        }
    }

    /// Returns a reader for the blob
    /// Using a Reader helps in handling large file contents and also fetch data in a specific range
    pub fn get_blob_reader(&mut self, blob: &::rest::blob::Blob) -> Result<::io::reader::Reader, String> {
        self.get_reader_for_blob(blob)
    }

    /// Returns the list of versions_id for the blob
    pub fn get_blob_versions(&mut self, name: String) -> Result<Vec<[u8; 64]>, String>{
        match self.find_file(&name, &self.directory_listing) {
            Some(blob) => {
                let mut file_helper = ::helper::FileHelper::new(self.client.clone());
                match file_helper.get_versions(self.directory_listing.get_id(), &blob.convert_to_file()) {
                    Ok(versions) => {
                        Ok(versions.iter().map(|x| x.0).collect())
                    },
                    Err(_) => Err("Error".to_string()),
                }
            },
            None => Err("Blob not found".to_string()),
        }
    }

    /// Update the metadata of the Blob in the container
    pub fn update_blob_metadata(&mut self, name: String, metadata: Option<String>) ->Result<(), String> {
        match self.validate_metadata(metadata) {
            Ok(user_metadata) => {
                match self.find_file(&name, &self.directory_listing) {
                    Some(mut blob) => {
                        let mut file_helper = ::helper::FileHelper::new(self.client.clone());
                        file_helper.update_metadata(blob.convert_to_mut_file(), &mut self.directory_listing, &user_metadata)
                    },
                    None => Err("Blob not found".to_string()),
                }
            },
            Err(_) => Err("Error".to_string()),
        }
    }

    /// Delete blob from the container
    pub fn delete_blob(&mut self, name: String) -> Result<(), String> {
        match self.directory_listing.get_files().iter().position(|file| *file.get_name() == name) {
            Some(pos) => {
                self.directory_listing.get_mut_files().remove(pos);
                let mut directory_helper = ::helper::DirectoryHelper::new(self.client.clone());
                match directory_helper.update(&self.directory_listing) {
                    Ok(_) => Ok(()),
                    Err(_) => Err("Error".to_string()),
                }
            },
            None => Err("Blob not found".to_string()),
        }
    }

    /// Copies the latest blob version from the container to the specified destination container
    pub fn copy_blob(&mut self, blob_name: String, to_container: [u8; 64]) -> Result<(), String> {
        let to_dir_id = ::routing::NameType(to_container);
        if *self.directory_listing.get_id() == to_dir_id {
            return Err("Destination and Source containers are the same".to_string());
        }
        let mut directory_helper = ::helper::DirectoryHelper::new(self.client.clone());
        match self.directory_listing.get_files().iter().position(|file| *file.get_name() == blob_name) {
            Some(file_pos) => {
                match directory_helper.get(&to_dir_id) {
                    Ok(mut to_dir_listing) => {
                        match self.find_file(&blob_name, &to_dir_listing) {
                            Some(_) => Err("File already exists in the destination Container".to_string()),
                            None => {
                                let file = self.directory_listing.get_files()[file_pos].clone();
                                to_dir_listing.get_mut_files().push(file);
                                match  directory_helper.update(&to_dir_listing) {
                                    Ok(_) => Ok(()),
                                    Err(_) => Err("Error".to_string()),
                                }
                            }
                        }
                    },
                    Err(_) => Err("Error".to_string()),
                }
            },
            None => Err("Blob not found".to_string()),
        }
    }

    fn get_writer_for_blob(&self, blob: &::rest::blob::Blob, mode: ::io::writer::Mode) -> Result<::io::Writer, String> {
        let mut helper = ::helper::FileHelper::new(self.client.clone());
        match helper.update(blob.convert_to_file(), &self.directory_listing, mode) {
            Ok(writter) => Ok(writter),
            Err(_) => Err("Blob not found".to_string()),
        }
    }

    fn get_reader_for_blob(&self, blob: &::rest::blob::Blob) -> Result<::io::Reader, String> {
        match self.find_file(blob.get_name(), &self.directory_listing) {
            Some(_) => {
                Ok(::io::Reader::new(blob.convert_to_file().clone(), self.client.clone()))
            },
            None => Err("Blob not found".to_string()),
        }
    }

    fn validate_metadata(&self, metadata: Option<String>) -> Result<Vec<u8>, String> {
        match metadata {
            Some(data) => {
                if data.len() == 0 {
                    Err("Metadata cannot be empty".to_string())
                } else {
                    Ok(data.into_bytes())
                }
            },
            None => Ok(Vec::new()),
        }
    }

    fn find_file(&self, name: &String, directory_listing: &::directory_listing::DirectoryListing) -> Option<::rest::Blob> {
        match directory_listing.get_files().iter().find(|file| file.get_name() == name) {
            Some(file) => Some(::rest::Blob::convert_from_file(file.clone())),
            None => None,
        }
    }

    fn list_container_versions(&mut self, container_id: &::routing::NameType) -> Result<Vec<[u8; 64]>, String> {
        let mut directory_helper = ::helper::DirectoryHelper::new(self.client.clone());
        match directory_helper.get_versions(container_id) {
            Ok(versions) => {
                Ok(versions.iter().map(|v| v.0).collect())
            },
            Err(_) => Err("Error".to_string()),
        }
    }
}

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
        let mut public_container = container.get_container("Public".to_string(), None).unwrap();
        assert_eq!(public_container.get_blobs().len(), 0);
        let _ = home_container.copy_blob("sample.txt".to_string(), public_container.get_id());
        public_container = container.get_container("Public".to_string(), None).unwrap();
        assert_eq!(public_container.get_blobs().len(), 1);

        let _ = home_container.delete_blob("sample.txt".to_string());
        assert_eq!(home_container.get_blobs().len(), 0);
    }

}
