extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use std::env;
use std::fs;
use std::path::Path;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn generate_module_list(__input: TokenStream) -> TokenStream {
    // Parse the input path and list name from the TokenStream
    let __input = parse_macro_input!(__input as LitStr);
    let __input = __input.value();
    let __internal_base_path = Path::new(&__input);

    // Get the project directory
    let __internal_project_dir =
        env::var("CARGO_MANIFEST_DIR").expect("Failed to get project directory");
    let __internal_project_path = Path::new(&__internal_project_dir);

    // Construct the full base path by appending the input path to the project directory
    let __internal_full_base_path = __internal_project_path.join(__internal_base_path);

    // Collect the module names by iterating over the entries in the full base directory
    let __internal_module_names: Vec<String> = fs::read_dir(__internal_full_base_path)
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

    // let __the_inside_typedef = __internal_module_names.len();

    // Generate the static list of string slices with the custom list name
    let __internal_macro_output: proc_macro2::TokenStream = quote! {
        pub const MODULE_LIST: [&str; 1] = [#__internal_module_array];
    };

    __internal_macro_output.into()
}
