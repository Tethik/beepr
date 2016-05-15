use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let profile = env::var("PROFILE").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("build_config.rs");


    macro_rules! format_string {() => ("
        pub fn sound_directory() -> &'static str {{
            \"{}\"
        }}")};


    let output_string = match profile.trim() {
        "debug" => format!(format_string!(), ""),
        "dev" => format!(format_string!(), ""),
        "bench" => format!(format_string!(), ""),
        "test" => format!(format_string!(), ""),
        "release" => format!(format_string!(), "/usr/share/beepr/"),
        _=> panic!("Unknown build profile!")
    };

    let mut f = File::create(&dest_path).unwrap();
    f.write_all(output_string.to_string().as_bytes()).unwrap();

}
