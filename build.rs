
use css_file_gen::extract_css_macros_from_files;

fn main(){
    let directory: &str = "src"; // Directory containing Rust source files
    let macros = extract_css_macros_from_files(directory);

    // println!("vssvsvbsb");
    // for macro_call in macros {
    //     println!("cargo:warning={}", macro_call);
    // }
}
