mod casandra;
// mod css_rules;
use css_rules::{
    assert_valid_border, assert_valid_color, assert_valid_dimensions, assert_valid_length,
    assert_valid_number, check_string_in_list,
};

mod css_passing;
// use css_passing::css_key;
// use casandra::style_sheet;
use std::collections::HashMap;
use uuid::Uuid;
extern crate proc_macro;


macro_rules! create_object {
    ($key:ident, $hashmap:expr, $value:expr) => {
        {
            css_key!($key, $hashmap, $value);
        }
    };

    ($key:literal, $hashmap:expr, $value:expr) => {
        {
            $hashmap.insert($key.to_string(), $value.to_string());
        }
    }

}

macro_rules! css_style {
    ($($key:tt : $value:expr),* $(,)?)  => {
        {
            let mut map = HashMap::new();
            $(
                create_object!($key, map, $value);
            )*
            map
        }
    };
}

macro_rules! css_stylesheet {
    ({} => {}) => {};
    // ($($key:tt : {$value:tt}),* $(,)?) => {
    //     // css_style!({$value});
    // }
    ($($key:ident : $value:tt),* $(,)?)  => {
        {
            #[derive(Debug)]
            #[allow(dead_code)]
            struct Stylesheet {
                $(
                    $key: String,
                )*
            }
            let result = Stylesheet {
                $(
                    $key: Uuid::new_v4().to_string(),
                )*
            };
            $(
                css_style! $value;
            )*
            result
        }
    };
}

fn main() {
    // let x: i32 = 15;
    let styles = css_stylesheet! {
        button : {
            color: "black",
            width: "10em 10px 10% 10",
            align_content: "initial",
            "stsg":"10"
        },

        header_1: {
            color: "black",
        }
    };
    let x = styles.button.as_str();
    if x == "aca" {
        println!("hello");
    }
    println!("{}", styles.button);

    // Display the generated HashMap
    println!("{:?}", styles);
}

// procedural macro
// can be used to check the keys are valid
// can be used to check the result is valid
// provide warning if not valid
// output final hashmap
// convert hashmap to css string

// return object with list of css class names
