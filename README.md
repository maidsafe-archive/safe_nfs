# maidsafe_nfs

[![](https://img.shields.io/badge/Project%20SAFE-Approved-green.svg)](http://maidsafe.net/applications) [![](https://img.shields.io/badge/License-GPL3-green.svg)](https://github.com/maidsafe/maidsafe_nfs/blob/master/COPYING)

**Primary Maintainer:**     Krishna Kumar (krishna.kumar@maidsafe.net)

**Secondary Maintainer:**   Spandan Sharma (spandan.sharma@maidsafe.net)

|Crate|Linux|Windows|OSX|Coverage|Issues|
|:------:|:-------:|:-------:|:-------:|:-------:|:-------:|
|[![](http://meritbadge.herokuapp.com/maidsafe_nfs)](https://crates.io/crates/maidsafe_nfs)|[![Build Status](https://travis-ci.org/maidsafe/maidsafe_nfs.svg?branch=master)](https://travis-ci.org/maidsafe/maidsafe_nfs)|[![Build Status](http://ci.maidsafe.net:8080/buildStatus/icon?job=maidsafe_nfs_win64_status_badge)](http://ci.maidsafe.net:8080/job/maidsafe_nfs_win64_status_badge/)|[![Build Status](http://ci.maidsafe.net:8080/buildStatus/icon?job=maidsafe_nfs_osx_status_badge)](http://ci.maidsafe.net:8080/job/maidsafe_nfs_osx_status_badge/)|[![Coverage Status](https://coveralls.io/repos/maidsafe/maidsafe_nfs/badge.svg)](https://coveralls.io/r/maidsafe/maidsafe_nfs)|[![Stories in Ready](https://badge.waffle.io/maidsafe/maidsafe_nfs.png?label=ready&title=Ready)](https://waffle.io/maidsafe/maidsafe_nfs)

| [API Documentation - master branch](http://maidsafe.net/maidsafe_nfs/master/) | [SAFE Network System Documention](http://systemdocs.maidsafe.net) | [MaidSafe website](http://maidsafe.net) | [Safe Community site](https://forum.safenetwork.io) |
|:------:|:-------:|:-------:|:-------:|

##TODO (rust_3 sprint)
### [0.0.1]
- [X] [MAID-1209](https://maidsafe.atlassian.net/browse/MAID-1209) Move NFS API from maidsafe_client

### [0.0.2]
- [X] [MAID-1220](https://maidsafe.atlassian.net/browse/MAID-1220) Create SAFE-Drive folder in root directory
- [X] [MAID-1221](https://maidsafe.atlassian.net/browse/MAID-1221) Update test cases for SAFE-Drive assertion

### [0.1.0]
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
