//! # List-Modules Procedural Macro
//!
//! **WARNING: This crate is domain-specific. The only thing that makes it so
//! is that you cannot name one of the directory items you are trying to list
//! "archetypes.rs" (a module folder named "archetypes" is fine). I will try
//! to fix this ASAP.**
//!
//! This crate creates a constant string slice list of all the module names
//! which are children of the crate of the module folder it was called from.
//! Note that it will only have the desired function if it is called from the
//! `mod.rs` file of a module structured in a folder (not a file).
//!
//! For example, calling this macro from `mod.rs` in the following file tree:
//!
//! ```none
//! parent/
//!     mod.rs
//!     child_1.rs
//!     child_2/
//!         mod.rs
//!         internal.rs
//!         other_internal/
//!             ...
//!         ...
//!     child_3.rs
//!     child_4.rs
//!     ...
//!     child_N/
//!         mod.rs
//! ```
//!
//! ...will result in the following list expansion:
//!
//! ```rust
//! pub const LIST: [&str; N] = [
//!     "child_1",
//!     "child_2",
//!     "child_3",
//!     ...
//!     "child_n",
//! ];
//! ```
//!
//! Note that this is the only guaranteed behavior.

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use std::env;
use std::fs;

/// # List-Modules Procedural Macro
///
/// **WARNING: This crate is domain-specific. The only thing that makes it so
/// is that you cannot name one of the directory items you are trying to list
/// "archetypes.rs" (a module folder named "archetypes" is fine). I will try
/// to fix this ASAP.**
///
/// This crate creates a constant string slice list of all the module names
/// which are children of the crate of the module folder it was called from.
/// Note that it will only have the desired function if it is called from the
/// `mod.rs` file of a module structured in a folder (not a file).
///
/// For example, calling this macro from `mod.rs` in the following file tree:
///
/// ```none
/// parent/
///     mod.rs
///     child_1.rs
///     child_2/
///         mod.rs
///         internal.rs
///         other_internal/
///             ...
///         ...
///     child_3.rs
///     child_4.rs
///     ...
///     child_N/
///         mod.rs
/// ```
///
/// ...will result in the following list expansion:
///
/// ```rust
/// pub const LIST: [&str; N] = [
///     "child_1",
///     "child_2",
///     "child_3",
///     ...
///     "child_n",
/// ];
/// ```
///
/// Note that this is the only guaranteed behavior.
#[proc_macro]
pub fn here(_: TokenStream) -> TokenStream {
    // Get the absolute path of the directory where the macro was called from
    let mut __macro_call_path = env::current_dir().expect("Failed to get current directory");

    // Collect the module names by iterating over the entries in the full base directory
    let __internal_module_names: Vec<String> = fs::read_dir(__macro_call_path)
        .expect("Failed to read directory")
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                if let Some(entry_name) = entry.file_name().to_str() {
                    if entry_name.ends_with(".rs")
                        && entry_name != "mod.rs"
                        && entry_name != "archetypes.rs"
                    {
                        return Some(entry_name[..entry_name.len() - 3].to_owned());
                    } else if entry_name != "mod.rs" && entry_name != "archetypes.rs" {
                        return Some(entry_name.to_owned());
                    }
                }
            }
            None
        })
        .collect();

    // Generate array of module names
    let __internal_module_array = quote! {
        [
            #(#__internal_module_names),*
        ]
    };

    let __number_of_modules_in_array = __internal_module_names.len();

    // Generate the static list of string slices with the custom list name
    let __internal_macro_output: proc_macro2::TokenStream = quote! {
        pub const LIST: [&str; #__number_of_modules_in_array] = #__internal_module_array;
    };

    __internal_macro_output.into()
}
