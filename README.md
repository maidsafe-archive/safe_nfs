# safe_nfs

[![](https://img.shields.io/badge/Project%20SAFE-Approved-green.svg)](http://maidsafe.net/applications) [![](https://img.shields.io/badge/License-GPL3-green.svg)](https://github.com/maidsafe/safe_nfs/blob/master/COPYING)

**Primary Maintainer:**     Krishna Kumar (krishna.kumar@maidsafe.net)

**Secondary Maintainer:**   Spandan Sharma (spandan.sharma@maidsafe.net)

|Crate|Linux/OS X|Windows|Coverage|Issues|
|:---:|:--------:|:-----:|:------:|:----:|
|[![](http://meritbadge.herokuapp.com/safe_nfs)](https://crates.io/crates/safe_nfs)|[![Build Status](https://travis-ci.org/maidsafe/safe_nfs.svg?branch=master)](https://travis-ci.org/maidsafe/safe_nfs)|[![Build status](https://ci.appveyor.com/api/projects/status/tg0kg4bnkyh6lm48/branch/master?svg=true)](https://ci.appveyor.com/project/MaidSafe-QA/safe-nfs/branch/master)|[![Coverage Status](https://coveralls.io/repos/maidsafe/safe_nfs/badge.svg)](https://coveralls.io/r/maidsafe/safe_nfs)|[![Stories in Ready](https://badge.waffle.io/maidsafe/safe_nfs.png?label=ready&title=Ready)](https://waffle.io/maidsafe/safe_nfs)|

| [API Documentation - master branch](http://maidsafe.net/safe_nfs/master/) | [SAFE Network System Documention](http://systemdocs.maidsafe.net) | [MaidSafe website](http://maidsafe.net) | [Safe Community site](https://forum.safenetwork.io) |
|:------:|:-------:|:-------:|:-------:|

###Pre-requisite:
libsodium is a native dependency for [sodiumxoide](https://github.com/dnaq/sodiumoxide). Thus, install sodium by following the instructions [here](http://doc.libsodium.org/installation/index.html).

For windows:

- Download [prebuilt libsodium library](https://download.libsodium.org/libsodium/releases/libsodium-1.0.2-mingw.tar.gz)
- Extract `libsodium.a` for x86/x64 from the corresponding folder in the archive to your local filesystem
- Add this local path to `%PATH%`. (`PATH=%PATH%;<path to extracted libsodium.a dir>`)

###Build Instructions:
`safe_nfs` depends on `safe_client` which can interface conditionally against either the routing crate or a mock used for local testing.

To use it with the Mock:
```
cargo build --features "use-mock-routing"
cargo test --features "use-mock-routing"
```

To interface it with actual routing (default):
```
cargo build
cargo test
```

## TODO

### [0.1.1]
- [X] [MAID-1305](https://maidsafe.atlassian.net/browse/MAID-1305) Expand test cases
- [X] [MAID-1306](https://maidsafe.atlassian.net/browse/MAID-1306) Remove all unwraps() AND Check for Ok(try!( and see if really required (ie., for error conversion etc)
- [X] [MAID-1307](https://maidsafe.atlassian.net/browse/MAID-1307) Address the TODO’s and make temporary fixes as permanent
- [X] [MAID-1308](https://maidsafe.atlassian.net/browse/MAID-1308) Test cases for addressed TODO's and permanent fixes
- [X] [MAID-1309](https://maidsafe.atlassian.net/browse/MAID-1309) Check for all muts (eg., response_getter etc) and validate if really required
- [X] [MAID-1310](https://maidsafe.atlassian.net/browse/MAID-1310) Remove unwanted errors and Unexpected should take an &str instead of String
- [X] [MAID-1311](https://maidsafe.atlassian.net/browse/MAID-1311) Put debug statements
- [X] [MAID-1312](https://maidsafe.atlassian.net/browse/MAID-1312) Changes due to client
- [X] [MAID-1313](https://maidsafe.atlassian.net/browse/MAID-1313) Add exit and help to rest_api example
