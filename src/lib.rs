#![allow(unused_imports)]
#![allow(non_snake_case)]
use lazy_static::lazy_static;
use rand::Rng;
use std::collections::HashMap;
use std::sync::Mutex;
use random_string::generate;
use phf::phf_map;

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
    ($key:literal, $hashmap:expr, $value:expr) => {{
        // $hashmap.insert($keyuse const_random::const_random  ;.to_string(), $value.to_string());
    }};

    ($key:ident, $hashmap:expr, $value:literal) => {{
        css_key!($key, $hashmap, $value);
    }};

    ($key:ident, $hashmap:expr, $value:expr) => {{
        $hashmap.insert(stringify!($key).to_string(), $value.to_string());
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! css_style {
    ($($key:tt : $value:expr),* $(,)?)  => {
        {
            // let mut map : HashMap<String,String> = HashMap::new();
            // $(
            //     create_object!($key, map, $value);
            // )*
            const map : HashMap<String,String> = phf_map!{
                $(
                    create_object!($key, map, $value);
                )*
            };
            map
        }
    };
}

#[doc(hidden)]
pub fn convert_map_to_css_string(map: HashMap<String, String>) -> String {
    let mut css_string = String::new();
    css_string += "{ \n";
    for (key, value) in map.iter() {
        let new_key = key.replace("_", "-");
        css_string += &new_key;
        css_string += ": ";
        css_string += value;
        css_string += "\n";
    }
    css_string += "}";
    css_string
}

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

#[doc(hidden)]
#[macro_export]
macro_rules! generate_random_css_name {
    ($len:expr) => {{
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        const LEN: usize = $len;
        let mut rng = rand::thread_rng();
        let random_string: String = (0..LEN)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        random_string
    }};
}

#[macro_export]
macro_rules! css_stylesheet {
    ({} => {}) => {};

    {$($key:ident : $value:tt),* $(,)?}  => {
        // {
            use venus_css::{css_style, create_object, css_key, generate_random_css_name, convert_map_to_css_string};
            use std::collections::HashMap;
            use rand::Rng;
            use random_string::generate;
            const charset : &str = "abcdefghijklmnopqrstuvwxyz";
            lazy_static!{
                $(static ref $key : &'static str =
                {
                    const id : &'static str = stringify!(ident);
                    const css_map: HashMap<String,String> = css_style! $value;
                    // const css_string: &str = &convert_map_to_css_string(css_map);
                    // Make compile time
                    // GLOBAL_CSS.lock().unwrap().append_value(format!("{}: {}\n",
                    //     id.clone(),
                    //     &convert_map_to_css_string(css_map)
                    // ));
                    // css_style! $value;
                    id
                };
                )*
            }
            // $(static $key : &'static str = || -> &'static str {
            //         const id : &'static str = generate_random_css_name!(16);
            //         // let css_map = css_style! $value;
            //         // GLOBAL_CSS.lock().unwrap().append_value(format!("{}: {}\n",
            //         // id.clone(),
            //         // &convert_map_to_css_string(css_map)
            //         // ));
            //         id
            //     }();
            //     // css_style! $value;
            // )*
            // $(
            //     css_style! $value;
            // )*
        // }

    };
}

// pub fn css_text() -> String {
//     GLOBAL_CSS.lock().unwrap().css_string.clone()
// }
