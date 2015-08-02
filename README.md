# safe_nfs

[![](https://img.shields.io/badge/Project%20SAFE-Approved-green.svg)](http://maidsafe.net/applications) [![](https://img.shields.io/badge/License-GPL3-green.svg)](https://github.com/maidsafe/safe_nfs/blob/master/COPYING)

**Primary Maintainer:**     Krishna Kumar (krishna.kumar@maidsafe.net)

**Secondary Maintainer:**   Spandan Sharma (spandan.sharma@maidsafe.net)

|Crate|Linux|Windows|OSX|Coverage|Issues|
|:------:|:-------:|:-------:|:-------:|:-------:|:-------:|
|[![](http://meritbadge.herokuapp.com/safe_nfs)](https://crates.io/crates/safe_nfs)|[![Build Status](https://travis-ci.org/maidsafe/safe_nfs.svg?branch=master)](https://travis-ci.org/maidsafe/safe_nfs)|[![Build Status](http://ci.maidsafe.net:8080/buildStatus/icon?job=safe_nfs_win64_status_badge)](http://ci.maidsafe.net:8080/job/safe_nfs_win64_status_badge/)|[![Build Status](http://ci.maidsafe.net:8080/buildStatus/icon?job=safe_nfs_osx_status_badge)](http://ci.maidsafe.net:8080/job/safe_nfs_osx_status_badge/)|[![Coverage Status](https://coveralls.io/repos/maidsafe/safe_nfs/badge.svg)](https://coveralls.io/r/maidsafe/safe_nfs)|[![Stories in Ready](https://badge.waffle.io/maidsafe/safe_nfs.png?label=ready&title=Ready)](https://waffle.io/maidsafe/safe_nfs)

| [API Documentation - master branch](http://maidsafe.net/safe_nfs/master/) | [SAFE Network System Documention](http://systemdocs.maidsafe.net) | [MaidSafe website](http://maidsafe.net) | [Safe Community site](https://forum.safenetwork.io) |
|:------:|:-------:|:-------:|:-------:|

##TODO (rust_3 sprint)
### [0.0.2]
- [X] [MAID-1209](https://maidsafe.atlassian.net/browse/MAID-1209) Move NFS API from safe_client

### [0.1.0]
- [X] [MAID-1260](https://maidsafe.atlassian.net/browse/MAID-1260) Refactor to interface with safe_client (0.1.3)
- [X] [MAID-1249](https://maidsafe.atlassian.net/browse/MAID-1249) Implement Unified Structured Datatype
    - [X] [MAID-1233](https://maidsafe.atlassian.net/browse/MAID-1233) Metadata to indicate versioning support and type (Private, Public, Shared)
    - [X] [MAID-1235](https://maidsafe.atlassian.net/browse/MAID-1235) Handle Container Creation
    - [X] [MAID-1236](https://maidsafe.atlassian.net/browse/MAID-1236) Update FileHelper and Writer to handle new Structured data changes
    - [X] [MAID-1237](https://maidsafe.atlassian.net/browse/MAID-1237) Error handling in NFS API
    - [X] [MAID-1238](https://maidsafe.atlassian.net/browse/MAID-1238) Update the test cases
    - [X] [MAID-1239](https://maidsafe.atlassian.net/browse/MAID-1239) Update the rest_api_example
