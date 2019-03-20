# ***This repository is no longer maintained***
# It has been moved to the maidsafe-archive organisation for reference only
#
#
#
#
# safe_nfs

[![](https://img.shields.io/badge/Project%20SAFE-Approved-green.svg)](http://maidsafe.net/applications) [![](https://img.shields.io/badge/License-GPL3-green.svg)](https://github.com/maidsafe/safe_nfs/blob/master/COPYING)

**Primary Maintainer:**     Krishna Kumar (krishna.kumar@maidsafe.net)

**Secondary Maintainer:**   Spandan Sharma (spandan.sharma@maidsafe.net)

|Crate|Linux/OS X|Windows|Coverage|Issues|
|:---:|:--------:|:-----:|:------:|:----:|
|[![](http://meritbadge.herokuapp.com/safe_nfs)](https://crates.io/crates/safe_nfs)|[![Build Status](https://travis-ci.org/maidsafe/safe_nfs.svg?branch=master)](https://travis-ci.org/maidsafe/safe_nfs)|[![Build status](https://ci.appveyor.com/api/projects/status/tg0kg4bnkyh6lm48/branch/master?svg=true)](https://ci.appveyor.com/project/MaidSafe-QA/safe-nfs/branch/master)|[![Coverage Status](https://coveralls.io/repos/maidsafe/safe_nfs/badge.svg)](https://coveralls.io/r/maidsafe/safe_nfs)|[![Stories in Ready](https://badge.waffle.io/maidsafe/safe_nfs.png?label=ready&title=Ready)](https://waffle.io/maidsafe/safe_nfs)|

| [API Documentation - master branch](http://maidsafe.net/safe_nfs/master) | [SAFE Network System Documentation](http://systemdocs.maidsafe.net) | [MaidSafe website](http://maidsafe.net) | [SAFE Network Forum](https://forum.safenetwork.io) |
|:------:|:-------:|:-------:|:-------:|

## Prerequisite

[libsodium](https://github.com/jedisct1/libsodium) is a native dependency, and can be installed by following the instructions [for Windows](https://github.com/maidsafe/QA/blob/master/Documentation/Install%20libsodium%20for%20Windows.md) or [for OS X and Linux](https://github.com/maidsafe/QA/blob/master/Documentation/Install%20libsodium%20for%20OS%20X%20or%20Linux.md).

## Build Instructions

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
