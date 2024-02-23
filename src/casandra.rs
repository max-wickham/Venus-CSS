// // CSS library for dioxus

// // Provide StyleSheet function


// #[macro_export]
// macro_rules! style_sheet {
//     (@parse { $($pairs:tt)* } $acc:ident) => {};

//     // Parse a key-value pair and recursively call the macro for the rest
//     (@parse { $key:ident : $value:expr, $($rest:tt)* } $acc:ident) => {
//         $acc.insert(stringify!($key).to_string(), $value);
//         css_dict!(@parse { $($rest)* } $acc);
//     };

//     // Entry point of the macro
//     ($($pairs:tt)*) => {
//         {
//             let mut map = HashMap::new();
//             css_dict!(@parse { $($pairs)* } map);
//             map
//         }
//     };
// }
