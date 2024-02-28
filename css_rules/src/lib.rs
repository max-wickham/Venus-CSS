extern crate proc_macro;
use std::collections::HashMap;

use lazy_static::lazy_static;
use proc_macro::{token_stream, TokenStream};
use quote::quote;
use regex::Regex;
use syn::{braced, parse_macro_input, punctuated::Punctuated, Expr, LitStr, Result, Token}; // 1.3.0

lazy_static! {
    static ref LENGTH_RE: Regex = Regex::new(r"^(\d+(\.\d+)?)(px|em|rem|%|vw|vh|vmin|vmax|cm|mm|in|pt|pc)?$").unwrap();
    static ref DIMENSIONS_RE: Regex = Regex::new(r"^(\d+(\.\d+)?(px|em|rem|%|vw|vh|vmin|vmax|cm|mm|in|pt|pc)?)\s+(\d+(\.\d+)?(px|em|rem|%|vw|vh|vmin|vmax|cm|mm|in|pt|pc)?)\s+(\d+(\.\d+)?(px|em|rem|%|vw|vh|vmin|vmax|cm|mm|in|pt|pc)?)\s+(\d+(\.\d+)?(px|em|rem|%|vw|vh|vmin|vmax|cm|mm|in|pt|pc)?)$").unwrap();
    static ref COLOR_RE: Regex = Regex::new(r"^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$|^(rgb|hsl)a?\(\s*\d+\s*,\s*\d+\s*,\s*\d+\s*(,\s*(\d+(\.\d+)?)\s*)?\)$|^(red|blue|green|yellow|magenta|cyan|black|white|gray)$").unwrap();
    static ref NUMBER_RE: Regex = Regex::new(r"^-?\d+(\.\d+)?$").unwrap();
    static ref BORDER_RE: Regex = Regex::new(r"^(?:(?:\d+(?:px|em|%|rem|vw|vh|vmin|vmax)?|thin|medium|thick)\s+)?(?:solid|dashed|dotted|double|groove|ridge|inset|outset)\s+(?:#[0-9a-fA-F]{3,6}|[a-zA-Z]+)$").unwrap();

    static ref REGEX_DICT : HashMap<String,Regex> = HashMap::new();
}

struct CheckStringInput(LitStr, Vec<LitStr>);
// Implementing the syn::parse::Parse trait for CheckStringInput
impl syn::parse::Parse for CheckStringInput {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        // Parse the string literal
        let search_string = input.parse::<LitStr>()?;

        // Ensure a comma follows the string literal
        input.parse::<Token![,]>()?;

        // Parse the list of string literals inside square brackets
        let content;
        braced!(content in input);
        let allowed_strings = Punctuated::<LitStr, Token![,]>::parse_terminated(&content)?;

        Ok(CheckStringInput(
            search_string,
            allowed_strings.into_iter().collect(),
        ))
    }
}

macro_rules! remove_quotations {
    ($string:expr) => {{
        if $string.starts_with('"') && $string.ends_with('"') {
            $string.remove(0);
            $string.remove($string.len() - 1);
        }
    }};
}

macro_rules! deconstruct_token_stream {
    ($tokens:expr) => {
        {
            let tokens = $tokens;
            let input = parse_macro_input!(tokens as CheckStringInput); // Assuming CheckStringInput is a custom type
            let mut search_string = input.0.value().to_string();
            remove_quotations!(search_string);
            let allowed_strings: Vec<_> = input.1.into_iter().map(|lit| lit.value()).collect();
            (search_string, allowed_strings)
        }
    };
}

macro_rules! check_contains_string {
    ($search_string:expr, $allowed_strings:expr) => {{
        // Use the contains method to check if the string is in the vector
        if $allowed_strings.contains(&$search_string) {
            return quote! {}.into();
        }
    }};
}

// TODO macro to reduce code amount
#[proc_macro]
pub fn assert_valid_length(input: TokenStream) -> TokenStream {
    // let expr = parse_macro_input!(input as Expr);
    // match &expr {
    //     Expr::Lit(_) => {},
    //     // TODO add runtime checking here of the type
    //     _ => return  quote! {}.into(),
    // };
    let (search_string, allowed_strings) = deconstruct_token_stream!(input);
    check_contains_string!(search_string, allowed_strings);

    if !LENGTH_RE.is_match(&search_string) {
        return quote! {
            compile_error!("Invalid number");
        }
        .into();
    }

    quote! {}.into()
}

#[proc_macro]
pub fn assert_valid_number(input: TokenStream) -> TokenStream {
    let (search_string, allowed_strings) = deconstruct_token_stream!(input);
    check_contains_string!(search_string, allowed_strings);
    if !NUMBER_RE.is_match(&search_string) {
        return quote! {
            compile_error!("Invalid number");
        }
        .into();
    }
    quote! {}.into()
}

#[proc_macro]
pub fn assert_valid_dimensions(input: TokenStream) -> TokenStream {
    // Convert the expression to a string
    let (search_string, allowed_strings) = deconstruct_token_stream!(input);
    check_contains_string!(search_string, allowed_strings);

    if !DIMENSIONS_RE.is_match(&search_string) {
        return quote! {
            compile_error!("Invalid input");
        }
        .into();
    }

    quote! {}.into()
}

#[proc_macro]
pub fn assert_valid_color(input: TokenStream) -> TokenStream {
    let (search_string, allowed_strings) = deconstruct_token_stream!(input);
    check_contains_string!(search_string, allowed_strings);

    if !COLOR_RE.is_match(&search_string) {
        return quote! {
           compile_error!("Invalid CSS color, must be of form HEX, RGBA, RGB, HSL, HSLA, Keyword");
        }
        .into();
    }

    quote! {}.into()
}

#[proc_macro]
pub fn assert_valid_border(input: TokenStream) -> TokenStream {
    let (search_string, allowed_strings) = deconstruct_token_stream!(input);
    check_contains_string!(search_string, allowed_strings);

    if !BORDER_RE.is_match(&search_string) {
        return quote! {
           compile_error!("Invalid CSS color, must be of form HEX, RGBA, RGB, HSL, HSLA, Keyword");
        }
        .into();
    }

    quote! {}.into()
}

#[proc_macro]
pub fn check_string_in_list(input: TokenStream) -> TokenStream {
    let (search_string, allowed_strings) = deconstruct_token_stream!(input);
    check_contains_string!(search_string, allowed_strings);
    quote! {
        let error_message = format!(
            "String '{}' is not allowed. Allowed values are: {:?}",
            search_string, allowed_strings
        );
        compile_error!(error_message);
    }
    .into()
}

// #[proc_macro]
// pub fn check_value(input: TokenStream) -> TokenStream {
//     let (search_string, allowed_strings) = deconstruct_token_stream!(input);
//     if !REGEX_DICT.contains_key(search_string) {
//         REGEX_DICT.insert(search_string, Regex::new(&search_string).unwrap());
//     }

//     if !REGEX_DICT.get(search_string).is_match(&search_string) {
//         return quote! {
//             compile_error!("Invalid input");
//         }
//         .into();
//     }

//     quote! {}.into()
// }

// give a list of values and a
