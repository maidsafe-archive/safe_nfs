# Safe nfs - Change Log

## [0.2.1]
- Routing crate updated to version 0.4.*

## [0.2.0]
- [MAID-1305](https://maidsafe.atlassian.net/browse/MAID-1305) Expand test cases
- [MAID-1306](https://maidsafe.atlassian.net/browse/MAID-1306) Remove all unwraps() AND Check for Ok(try!( and see if really required (ie., for error conversion etc)
- [MAID-1307](https://maidsafe.atlassian.net/browse/MAID-1307) Address the TODO’s and make temporary fixes as permanent
- [MAID-1308](https://maidsafe.atlassian.net/browse/MAID-1308) Test cases for addressed TODO's and permanent fixes
- [MAID-1309](https://maidsafe.atlassian.net/browse/MAID-1309) Check for all muts (eg., response_getter etc) and validate if really required
- [MAID-1310](https://maidsafe.atlassian.net/browse/MAID-1310) Remove unwanted errors and Unexpected should take an &str instead of String
- [MAID-1311](https://maidsafe.atlassian.net/browse/MAID-1311) Put debug statements
- [MAID-1312](https://maidsafe.atlassian.net/browse/MAID-1312) Changes due to client
- [MAID-1313](https://maidsafe.atlassian.net/browse/MAID-1313) Add exit and help to rest_api example

## [0.1.0]
- [MAID-1260](https://maidsafe.atlassian.net/browse/MAID-1260) Refactor to interface with safe_client (0.1.3)
- [MAID-1249](https://maidsafe.atlassian.net/browse/MAID-1249) Implement Unified Structured Datatype
    - [MAID-1233](https://maidsafe.atlassian.net/browse/MAID-1233) Metadata to indicate versioning support and type (Private, Public, Shared)
    - [MAID-1235](https://maidsafe.atlassian.net/browse/MAID-1235) Handle Container Creation
    - [MAID-1236](https://maidsafe.atlassian.net/browse/MAID-1236) Update FileHelper and Writer to handle new Structured data changes
    - [MAID-1237](https://maidsafe.atlassian.net/browse/MAID-1237) Error handling in NFS API
    - [MAID-1238](https://maidsafe.atlassian.net/browse/MAID-1238) Update the test cases
    - [MAID-1239](https://maidsafe.atlassian.net/browse/MAID-1239) Update the rest_api_example

## [0.0.2]
- [MAID-1209](https://maidsafe.atlassian.net/browse/MAID-1209) Move NFS API from safe_client
