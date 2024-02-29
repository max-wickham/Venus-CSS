#![allow(unused_imports)]
#![allow(non_snake_case)]
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use rand::Rng;

use css_rules::{
    assert_valid_border, assert_valid_color, assert_valid_dimensions, assert_valid_length,
    assert_valid_number, check_string_in_list,
};

mod css_passing;

struct GlobalCSS {
    pub css_string: String,
}

impl GlobalCSS {
    pub fn append_value(&mut self, value: String) {
        self.css_string += &value;
    }
}

lazy_static! {
    static ref GLOBAL_CSS: Mutex<GlobalCSS> = Mutex::new(GlobalCSS {
        css_string: String::new(),
    });
}
// const GOAL: Goal = Goal {
//     value: String::new(),
// };

macro_rules! create_object {

    ($key:literal, $hashmap:expr, $value:expr) => {{
        $hashmap.insert($key.to_string(), $value.to_string());
    }};

    ($key:ident, $hashmap:expr, $value:literal) => {{
        css_key!($key, $hashmap, $value);
    }};

    ($key:ident, $hashmap:expr, $value:expr) => {{
        $hashmap.insert(stringify!($key).to_string(), $value.to_string());
    }};

}

macro_rules! css_style {
    ($($key:tt : $value:expr),* $(,)?)  => {
        {
            let mut map : HashMap<String,String> = HashMap::new();
            $(
                create_object!($key, map, $value);
            )*
            map
        }
    };
}

fn convert_map_to_css_string(map: HashMap<String, String>) -> String {
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

fn generate_css_style_name() -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const LENGTH: usize = 16;

    let mut rng = rand::thread_rng();
    let name: String = (0..LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    String::from("venus-") + &name
}

macro_rules! css_stylesheet {
    ({} => {}) => {};

    ($($key:ident : $value:tt),* $(,)?)  => {
        {
            #[derive(Debug)]
            #[allow(dead_code)]
            struct Stylesheet {
                $(
                    pub $key: String,
                )*
            }
            let result = Stylesheet {
                $(
                    $key:  || -> String {
                        let id = generate_css_style_name();
                        let css_map = css_style! $value;
                        GLOBAL_CSS.lock().unwrap().append_value(id.clone() + ": " + &convert_map_to_css_string(css_map) + "\n");
                        id
                    }(),
                )*
            };
            $(
                css_style! $value;
            )*
            result
        }
    };
}

pub fn css_text() -> String {
    GLOBAL_CSS.lock().unwrap().css_string.clone()
}
