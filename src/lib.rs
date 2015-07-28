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

#![crate_name = "maidsafe_nfs"]
#![crate_type = "lib"]
#![doc(html_logo_url = "http://maidsafe.net/img/Resources/branding/maidsafe_logo.fab2.png",
       html_favicon_url = "http://maidsafe.net/img/favicon.ico",
              html_root_url = "http://dirvine.github.io/dirvine/maidsafe_nfs/")]
/*
///////////////////////////////////////////////////
//               LINT
///////////////////////////////////////////////////

#![forbid(bad_style, warnings)]

#![deny(deprecated, improper_ctypes, missing_docs, non_shorthand_field_patterns,
overflowing_literals, plugin_as_library, Private_no_mangle_fns, Private_no_mangle_statics,
raw_pointer_derive, stable_features, unconditional_recursion, unknown_lints, unsafe_code,
unsigned_negation, unused, unused_allocation, unused_attributes, unused_comparisons,
unused_features, unused_parens, while_true)]

#![warn(trivial_casts, trivial_numeric_casts, unused_extern_crates, unused_import_braces,
unused_qualifications, variant_size_differences)]

///////////////////////////////////////////////////
*/
//! #Maidsafe-Nfs Library
//! [Project github page](https://github.com/maidsafe/maidsafe_nfs)


extern crate time;
extern crate self_encryption;
extern crate cbor;
extern crate routing;
extern crate sodiumoxide;
extern crate rustc_serialize;
extern crate maidsafe_client;

pub mod file;
pub mod errors;
pub mod helper;
pub mod directory_metadata;
pub mod metadata;
pub mod directory_listing;

/// Module for input/output to network/file
pub mod io;
/// Module for Restful interfaces for storage
// pub mod rest;
/// Root directory name
pub const ROOT_DIRECTORY_NAME: &'static str = "root";
/// Configuration directory Name stored in the session packet
pub const CONFIGURATION_DIRECTORY_NAME: &'static str = "MaidSafe_Configuration";
/// Tag representing the Versioned Directory Listing
pub const VERSION_DIRECTORY_LISTING_TAG: u64 = ::maidsafe_client::MAIDSAFE_TAG + 100;
/// Tag representing the Versioned Directory Listing
pub const UNVERSION_DIRECTORY_LISTING_TAG: u64 = ::maidsafe_client::MAIDSAFE_TAG + 101;

/// ShareLebvel indicates whether the container is Private or Public shared
#[derive(RustcEncodable, RustcDecodable, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum AccessLevel {
    Private,
    Public,
}
