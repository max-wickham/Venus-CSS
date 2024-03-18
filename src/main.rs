#![allow(unused_imports)]
#![allow(non_snake_case)]
use lazy_static::lazy_static;
use phf::phf_map;
use rand::Rng;
use random_string::generate;
use std::collections::HashMap;
use std::sync::Mutex;

use css_rules::{
    assert_valid_border, assert_valid_color, assert_valid_dimensions, assert_valid_length,
    assert_valid_number, check_string_in_list,
};

mod css_passing;

// #[doc(hidden)]
// pub struct GlobalCSS {
//     pub css_string: String,
// }

// impl GlobalCSS {
//     pub fn append_value(&mut self, value: String) {
//         self.css_string += &value;
//     }
// }

// lazy_static! {
//     #[doc(hidden)]
//     pub static ref GLOBAL_CSS: Mutex<GlobalCSS> = Mutex::new(GlobalCSS {
//         css_string: String::new(),
//     });
// }
// const GOAL: Goal = Goal {
//     value: String::new(),
// };

#[doc(hidden)]
#[macro_export]
macro_rules! create_object {
    // ($key:literal, $value:expr) => {
    //     // $hashmap.insert($keyuse const_random::const_random  ;.to_string(), $value.to_string());
    //     // $key => $value,
    // };
    ($key:ident, $value:literal) => {
        css_key!($key, $value);
    }; // ($key:ident, $value:expr) => {{
       //     stringify!($key) => $value,
       //     // $hashmap.insert(stringify!($key).to_string(), $value.to_string());
       // }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! css_style {
    ($($key:tt : $value:expr),* $(,)?)  => {
    // ($($key:ident : $value:tt),* $(,)?) => {
        {
            $(
                create_object!($key, $value);
            )*
        }
    };
        // {
            // let mut map : HashMap<String,String> = HashMap::new();
            // $(
            //     create_object!($key, map, $value);
            // )*
            // const map : phf::Map<String,String> = phf_map! {
                // $(
                //     // create_object!($key, $value);
                //     // stringify!($key) => $value,
                // )*
            // };
            // map
        // }
    // };
}

// #[doc(hidden)]
// pub fn convert_map_to_css_string(map: HashMap<String, String>) -> String {
//     let mut css_string = String::new();
//     css_string += "{ \n";
//     for (key, value) in map.iter() {
//         let new_key = key.replace("_", "-");
//         css_string += &new_key;
//         css_string += ": ";
//         css_string += value;
//         css_string += "\n";
//     }
//     css_string += "}";
//     css_string
// }

// #[doc(hidden)]
// pub fn generate_css_style_name() -> String {
//     const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
//     const LENGTH: usize = 16;

//     let mut rng = rand::thread_rng();
//     let name: String = (0..LENGTH)
//         .map(|_| {
//             let idx = rng.gen_range(0..CHARSET.len());
//             CHARSET[idx] as char
//         })
//         .collect();

//     String::from("venus-") + &name
// }

// #[doc(hidden)]
// const fn generate_css_style_name(length: usize) -> &'static str {
//     let random_string: &'static str = (0..length)
//         .map(|_| thread_rng().sample(Alphanumeric));

//     // Use concat! and format! to convert the String to a &'static str at compile time
//     &*format!("{}", concat!("\"", random_string, "\""))
// }

// #[doc(hidden)]
// #[macro_export]
// macro_rules! generate_random_css_name {
//     ($len:expr) => {{
//         const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
//         const LEN: usize = $len;
//         let mut rng = rand::thread_rng();
//         let random_string: String = (0..LEN)
//             .map(|_| {
//                 let idx = rng.gen_range(0..CHARSET.len());
//                 CHARSET[idx] as char
//             })
//             .collect();
//         random_string
//     }};
// }

#[macro_export]
macro_rules! css_stylesheet {
    ({} => {}) => {};

    ($($key:ident : $value:tt),* $(,)?)  => {
        // {
            // #[cfg(not(feature = "module_imported"))]
            // use venus_css;
            lazy_static!{
                $(static ref $key : &'static str =
                {
                    const ID : &'static str = stringify!(ident);
                    // need to write to a file at this point
                    // const css_map: phf::Map<String,String> = css_style! $value;
                    // const css_string: &str = &convert_map_to_css_string(css_map);
                    // Make compile time
                    // GLOBAL_CSS.lock().unwrap().append_value(format!("{}: {}\n",
                    //     id.clone(),
                    //     &convert_map_to_css_string(css_map)
                    // ));
                    css_style! $value;
                    ID
                };
                )*
            }

    };
}

css_stylesheet! {

    BUTTON : {
        color:"red",

    },

    NAME : {

    },
}

pub fn main() {}
