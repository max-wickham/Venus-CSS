use std::fs;
use std::path::Path;

pub fn extract_css_macros_from_files(directory: &str) -> Vec<String> {
    let mut macros = Vec::new();

    // Iterate through files in the given directory
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if extension == "rs" {
                        if let Ok(contents) = fs::read_to_string(&path) {
                            let extracted_macros = extract_css_macros(&contents);
                            macros.extend(extracted_macros);
                        }
                    }
                }
            }
        }
    }

    macros
}

fn extract_css_macros(contents: &str) -> Vec<String> {
    let mut macros = Vec::new();

    let mut iter = contents.match_indices("css_stylesheet! {");

    while let Some((start_index, _)) = iter.next() {
        let mut depth = 0;
        let mut end_index = start_index;

        for (i, c) in contents[start_index..].char_indices() {
            end_index = start_index + i;
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                _ => {}
            }
        }

        if depth == 0 {
            // Found a complete macro call, extract it
            let macro_call = &contents[start_index..=end_index];
            macros.push(macro_call.to_string());
        }
    }

    macros
}
