# maidsafe_nfs

**Primary Maintainer:**     Spandan Sharma (spandan.sharma@maidsafe.net)

**Secondary Maintainer:**   Krishna Kumar (krishna.kumar@maidsafe.net)


| [API Documentation - master branch](http://maidsafe.net/maidsafe_nfs/master) | [SAFE Network System Documentation](http://systemdocs.maidsafe.net) | [MaidSafe website](http://maidsafe.net) | [Safe Community site](https://forum.safenetwork.io) |
|:------:|:-------:|:-------:|:-------:|

###Build Instructions:
Maidsafe-Client interfaces conditionally with either the actual routing crate or the Mock used for efficient local testing.

To use it with the Mock (default) do:
```
cargo build
cargo test
etc
```
##TODO (rust_3 sprint)
###Version 0.0.1 (NFS separation)
- [ ] [MAID-1209](https://maidsafe.atlassian.net/browse/MAID-1209) Move NFS API from maidsafe_client

###Version 0.1.0 (Unified Structured Datatype)
- [ ] [MAID-1249](https://maidsafe.atlassian.net/browse/MAID-1249) Implement Unified Structured Datatype
    - [ ] [MAID-1230](https://maidsafe.atlassian.net/browse/MAID-1230) Implement a handler for Storing Versioned Structured Data
    - [ ] [MAID-1231](https://maidsafe.atlassian.net/browse/MAID-1231) Implement a handler for Retrieving Versioned Structured Data
    - [ ] [MAID-1232](https://maidsafe.atlassian.net/browse/MAID-1232) Write Test Cases for Versioned Structured Data handler
    - [ ] [MAID-1233](https://maidsafe.atlassian.net/browse/MAID-1233) Metadata to indicate versioning support and type (Private, Public, Shared)
    - [ ] [MAID-1234](https://maidsafe.atlassian.net/browse/MAID-1234) Update the Hybrid encrypt and decrypt
    - [ ] [MAID-1235](https://maidsafe.atlassian.net/browse/MAID-1235) Handle Container Creation
    - [ ] [MAID-1236](https://maidsafe.atlassian.net/browse/MAID-1236) Update FileHelper and Writer to handle new Structured data changes
    - [ ] [MAID-1237](https://maidsafe.atlassian.net/browse/MAID-1237) Error handling in Version related API
    - [ ] [MAID-1238](https://maidsafe.atlassian.net/browse/MAID-1238) Update the test cases
    - [ ] [MAID-1239](https://maidsafe.atlassian.net/browse/MAID-1239) Update the rest_api_example
