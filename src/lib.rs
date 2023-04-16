extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use std::env;
use std::fs;
use std::path::Path;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn generate_module_list(input: TokenStream) -> TokenStream {
    // Parse the input path and list name from the TokenStream
    let input = parse_macro_input!(input as LitStr);
    let input = input.value();
    let base_path = Path::new(&input);

    // Get the project directory
    let project_dir = env::var("CARGO_MANIFEST_DIR").expect("Failed to get project directory");
    let project_path = Path::new(&project_dir);

    // Construct the full base path by appending the input path to the project directory
    let full_base_path = project_path.join(base_path);

    // Collect the module names by iterating over the entries in the full base directory
    let module_names: Vec<String> = fs::read_dir(full_base_path)
        .expect("Failed to read directory")
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                if let Some(entry_name) = entry.file_name().to_str() {
                    if entry_name.ends_with(".rs") && entry_name != "mod.rs" {
                        return Some(entry_name[..entry_name.len() - 3].to_owned());
                    } else {
                        return Some(entry_name.to_owned());
                    }
                }
            }
            None
        })
        .collect();

    // Generate array of module names
    let module_array = quote! {
        [
            #(#module_names),*
        ]
    };

    // Generate the static list of string slices with the custom list name
    let output = quote! {
        pub const MODULE_LIST: [&str; #module_array.len()] = [#module_array];
    };

    output.into()
}