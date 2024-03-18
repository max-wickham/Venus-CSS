# VenusCSS

The goal of the library is to allow the definition of CSS classes in rust files similar to the aphrodite.js library used in React. The main object of this library is to be used with the Dioxus framework but can be used with any Rust GUI framework.

Currently there is still a lot of work before a stable release but the first features will be as following.

- Generate a single CSS file from all css_stylesheet! macro calls throughout project at build time.
- Check at compile time that CSS rules have been followed (only a subset of CSS features will be present at first)
- Allow definition without strings of CSS rules in code in the global scope using the css_stylesheet! macro.
