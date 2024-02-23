extern crate proc_macro;
use proc_macro::{token_stream, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Expr, LitStr, Token, braced, Result, punctuated::Punctuated};
use regex::Regex;
use lazy_static::lazy_static; // 1.3.0




lazy_static! {
    static ref LENGTH_RE: Regex = Regex::new(r"^(\d+(\.\d+)?)(px|em|rem|%|vw|vh|vmin|vmax|cm|mm|in|pt|pc)?$").unwrap();
    static ref DIMENSIONS_RE: Regex = Regex::new(r"^(\d+(\.\d+)?(px|em|rem|%|vw|vh|vmin|vmax|cm|mm|in|pt|pc)?)\s+(\d+(\.\d+)?(px|em|rem|%|vw|vh|vmin|vmax|cm|mm|in|pt|pc)?)\s+(\d+(\.\d+)?(px|em|rem|%|vw|vh|vmin|vmax|cm|mm|in|pt|pc)?)\s+(\d+(\.\d+)?(px|em|rem|%|vw|vh|vmin|vmax|cm|mm|in|pt|pc)?)$").unwrap();
    static ref COLOR_RE: Regex = Regex::new(r"^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$|^(rgb|hsl)a?\(\s*\d+\s*,\s*\d+\s*,\s*\d+\s*(,\s*(\d+(\.\d+)?)\s*)?\)$|^(red|blue|green|yellow|magenta|cyan|black|white|gray)$").unwrap();
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

        Ok(CheckStringInput(search_string, allowed_strings.into_iter().collect()))
    }
}

// macro_rules! deconstruct_token_stream {
//     ($tokens:expr) => {
//         {
//             let input = parse_macro_input!($tokens as CheckStringInput);
//             let search_string = input.0.value().to_string();
//             let allowed_strings: Vec<_> = input.1.into_iter().map(|lit| lit.value()).collect();
//             (search_string, allowed_strings)
//         }
//     };
// }

macro_rules! remove_quotations {
    ($string:expr) => {
        {
            if $string.starts_with('"') && $string.ends_with('"'){
                $string.remove(0);
                $string.remove($string.len() - 1);
            }
        }
    }
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
    ($search_string:expr, $allowed_strings:expr) => {
        {
            // Use the contains method to check if the string is in the vector
            if $allowed_strings.contains(&$search_string) {
                return quote! {}.into();
            }
        }
    };
}

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
        }.into();
    }

    quote! {}.into()
}

#[proc_macro]
pub fn assert_valid_dimensions(input: TokenStream) -> TokenStream {
    // let expr = parse_macro_input!(input as Expr);
    // match &expr {
    //     Expr::Lit(_) => return  quote! {}.into(),
    //     // TODO add runtime checking here of the type
    //     _ => {},
    // };

    // Convert the expression to a string
    let (search_string, allowed_strings) = deconstruct_token_stream!(input);
    check_contains_string!(search_string, allowed_strings);

    if !DIMENSIONS_RE.is_match(&search_string){
        return quote! {
            compile_error!("Invalid input");
        }.into();
    }

    quote! {}.into()
}

#[proc_macro]
pub fn assert_valid_color(input: TokenStream) -> TokenStream {
    // Parse the input expression
    // let expr = parse_macro_input!(input as Expr);
    // // TODO change this to work correctly with string literal
    // match &expr {
    //     Expr::Lit(_) => return  quote! {}.into(),
    //     // TODO add runtime checking here of the type
    //     _ => {},
    // };
    // Convert the expression to a string
    // let mut expr_str = quote!(#expr).to_string();
    // Check that the input was a string
    // if expr_str.starts_with('"') && expr_str.ends_with('"'){
    //     expr_str.remove(0);
    //     expr_str.remove(expr_str.len() - 1);
    // } else {
    //     return quote! {
    //         compile_error!("Invalid value");
    //     }.into();
    // }
    let (search_string, allowed_strings) = deconstruct_token_stream!(input);
    check_contains_string!(search_string, allowed_strings);

    if !COLOR_RE.is_match(&search_string){
        return quote! {
           compile_error!("Invalid CSS color, must be of form HEX, RGBA, RGB, HSL, HSLA, Keyword");
        }.into();
    }

    quote!{}.into()
}


#[proc_macro]
pub fn check_string_in_list(input: TokenStream) -> TokenStream {
    let (search_string, allowed_strings) = deconstruct_token_stream!(input);
    check_contains_string!(search_string, allowed_strings);
    quote!{
        let error_message = format!(
            "String '{}' is not allowed. Allowed values are: {:?}",
            search_string, allowed_strings
        );
        compile_error!(error_message);
    }.into()
    // Parse the input manually as a string literal and a list of string literals
    // let input = parse_macro_input!(input as CheckStringInput);
    // let search_string = input.0.value();
    // let allowed_strings: Vec<_> = input.1.into_iter().map(|lit| lit.value()).collect();

    // // Check if the search string is in the list
    // if allowed_strings.contains(&String::from(search_string.as_str())) {
    //     // If found, return an empty TokenStream
    //     TokenStream::new()
    // } else {
    //     // If not found, generate a compile-time error with a custom message
    //     let error_message = format!(
    //         "String '{}' is not allowed. Allowed values are: {:?}",
    //         search_string, allowed_strings
    //     );
    //     TokenStream::from(quote! {
    //         compile_error!(#error_message);
    //     })
    // }
}


// give a list of values and a
