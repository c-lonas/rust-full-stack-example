use std::env;
// use std::fs::File;
// use std::io::prelude::*;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let scss_path = manifest_dir.join("sass").join("main.scss");
    let sass_load_path = manifest_dir.join("sass");

    let compiled_css = grass::from_path(
        &scss_path,
        &grass::Options::default().load_path(sass_load_path),
    )
    .expect("Failed to compile SCSS");

    let out_path = PathBuf::from("./main.css");

    if let Ok(existing_content) = std::fs::read_to_string(&out_path) {
        if existing_content == compiled_css {
            return;
        }
    }

    std::fs::write(&out_path, compiled_css.as_bytes())
        .expect("Failed to write compiled CSS");
}



// Previous version to use if the above version breaks

// use std::env;
// use std::fs::File;
// use std::io::prelude::*;
// use std::path::Path;

// fn main() {
//     let scss_path = format!("{}/sass/main.scss", env!("CARGO_MANIFEST_DIR"));
//     let compiled_css = grass::from_path(
//         &scss_path,
//         &grass::Options::default().load_path(format!("{}/sass", env!("CARGO_MANIFEST_DIR"))),
//     )
//     .unwrap();

//     let out_path = Path::new("./main.css");
//     let mut should_write = true;

//     if let Ok(mut existing_file) = File::open(&out_path) {
//         let mut existing_content = String::new();
//         if existing_file.read_to_string(&mut existing_content).is_ok() {
//             if existing_content == compiled_css {
//                 should_write = false;
//             }
//         }
//     }

//     if should_write {
//         let mut file = File::create(&out_path).unwrap();
//         file.write_all(compiled_css.as_bytes()).unwrap();
//     }
// }