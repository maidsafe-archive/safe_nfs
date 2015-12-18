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

//! #Safe-Nfs Library
//! [Project github page](https://github.com/maidsafe/safe_nfs)

#![doc(html_logo_url =
           "https://raw.githubusercontent.com/maidsafe/QA/master/Images/maidsafe_logo.png",
       html_favicon_url = "http://maidsafe.net/img/favicon.ico",
       html_root_url = "http://maidsafe.github.io/safe_nfs")]

// For explanation of lint checks, run `rustc -W help` or see
// https://github.com/maidsafe/QA/blob/master/Documentation/Rust%20Lint%20Checks.md
#![forbid(bad_style, exceeding_bitshifts, mutable_transmutes, no_mangle_const_items,
          unknown_crate_types, warnings)]
#![deny(deprecated, drop_with_repr_extern, improper_ctypes, missing_docs,
        non_shorthand_field_patterns, overflowing_literals, plugin_as_library,
        private_no_mangle_fns, private_no_mangle_statics, raw_pointer_derive, stable_features,
        unconditional_recursion, unknown_lints, unsafe_code, unused, unused_allocation,
        unused_attributes, unused_comparisons, unused_features, unused_parens, while_true)]
#![warn(trivial_casts, trivial_numeric_casts, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, variant_size_differences)]
#![allow(box_pointers, fat_ptr_transmutes, missing_copy_implementations,
         missing_debug_implementations)]

extern crate time;
extern crate routing;
extern crate xor_name;
extern crate safe_core;
extern crate sodiumoxide;
extern crate rustc_serialize;
extern crate self_encryption;
#[macro_use] extern crate log;
#[macro_use] extern crate maidsafe_utilities;

/// Module for File struct
pub mod file;
/// Module for Restful interfaces for storage
pub mod rest;
/// Errors
pub mod errors;
/// Helper for directory_listing and File for NFS Low level API
pub mod helper;
/// Directory and File Metadata
pub mod metadata;
/// Module for directory reltaed structs - DirectoryListin, DirectoryInfo
pub mod directory_listing;

/// Root directory name
pub const ROOT_DIRECTORY_NAME: &'static str = "USER_ROOT";
/// Configuration directory Name stored in the session packet
pub const CONFIGURATION_DIRECTORY_NAME: &'static str = "CONFIGURATION_ROOT";
/// Tag representing the Versioned Directory Listing
pub const VERSIONED_DIRECTORY_LISTING_TAG: u64 = safe_core::CLIENT_STRUCTURED_DATA_TAG + 100;
/// Tag representing the Versioned Directory Listing
pub const UNVERSIONED_DIRECTORY_LISTING_TAG: u64 = VERSIONED_DIRECTORY_LISTING_TAG + 1;

/// AccessLevel indicates whether the container is Private or Public shared
#[derive(RustcEncodable, RustcDecodable, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum AccessLevel {
    /// Private Directory where the directory is encrypted with users private keys
    Private,
    /// Public Directory where the directory is not encrypted and anyone can read the contents of it
    Public,
}
