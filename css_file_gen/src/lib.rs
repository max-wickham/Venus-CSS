use std::fs;
extern crate syn;
use quote::ToTokens;
use proc_macro2::{TokenStream, TokenTree, Ident};
use syn::{Item};

fn visit_items(items: Vec<Item>) {
    for item in items {
        match item {
            Item::Macro(item_mod) => {
                let macro_name = item_mod
                    .mac
                    .path
                    .segments
                    .iter()
                    .map(|segment| segment.ident.to_string())
                    .collect::<Vec<_>>()
                    .join("::");
                match macro_name.as_str() {
                    "css_stylesheet" => {
                        let body = item_mod.to_token_stream().into_iter().nth(2);
                        match body.unwrap() {
                            TokenTree::Group(group) => {
                                // for tt in group.stream().into_iter().enumerate() {
                                //     // Process each token tree in the group
                                //     println!("{:?}", tt);
                                // }

                                let mut token_iter = group.stream().into_iter().peekable();
                                let index = 0;
                                // Iterate through the tokens with lookahead
                                while let Some((index, token)) = token_iter.next().map(|t| (index, t)) {
                                    println!("Token {}: {:?}", index, token);

                                    // Look ahead by 2 tokens
                                    if let Some((next_index, next_token)) = token_iter.peek().map(|t| (index + 2, t)) {
                                        println!("Next token {}: {:?}", next_index, next_token);
                                    }
                                }
                            },
                            _ => {}
                        }
                        // println!("{}", item_mod.to_token_stream().to_string());
                        // for token in body. {
                        //     println!("{}", token.to_string());
                        // }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

pub fn extract_css_macros_from_files(directory: &str) -> Vec<String> {
    let macros = Vec::new();

    // Iterate through files in the given directory
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if extension == "rs" {
                        if let Ok(contents) = fs::read_to_string(&path) {
                            let ast = syn::parse_file(&contents);
                            visit_items(ast.unwrap().items);
                        }
                    }
                }
            }
        }
    }

    macros
}
