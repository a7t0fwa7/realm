#[cfg(target_os = "windows")]
fn build_bin_create_file_dll(){
    use std::{process::{Command, Stdio}, path::Path, io::{BufReader, BufRead}};

    // Define which files should cause this section to be rebuilt.
    println!("cargo:rerun-if-changed=..\\..\\bin\\create_file_dll\\src\\lib.rs");
    println!("cargo:rerun-if-changed=..\\..\\bin\\create_file_dll\\src\\main.rs");
    println!("cargo:rerun-if-changed=..\\..\\bin\\create_file_dll\\Cargo.toml");

    // Get the path of the create_file_dll workspace member
    let cargo_root = env!("CARGO_MANIFEST_DIR");
    let relative_path_to_test_dll = "..\\..\\bin\\create_file_dll\\";
    let test_dll_path = Path::new(cargo_root).join(relative_path_to_test_dll);
    assert!(test_dll_path.is_dir());

    println!("Starting cargo build lib");
    let res = Command::new("cargo").args(&["build","--lib"])
        .current_dir(test_dll_path)
        .stderr(Stdio::piped())
        .spawn().unwrap().stderr.unwrap();

    let reader = BufReader::new(res);
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("cargo dll build: {}", line));

    let relative_path_to_test_dll_file = "..\\..\\bin\\create_file_dll\\target\\debug\\create_file_dll.dll";
    let test_dll_path = Path::new(cargo_root).join(relative_path_to_test_dll_file);
    assert!(test_dll_path.is_file());
}


#[cfg(target_os = "windows")]
fn build_bin_reflective_loader(){
    use std::{process::{Command, Stdio}, path::Path, io::{BufReader, BufRead}};

    // Define which files should cause this section to be rebuilt.
    println!("cargo:rerun-if-changed=..\\..\\bin\\reflective_loader\\src\\lib.rs");
    println!("cargo:rerun-if-changed=..\\..\\bin\\reflective_loader\\src\\loader.rs");
    println!("cargo:rerun-if-changed=..\\..\\bin\\reflective_loader\\Cargo.toml");

    // Get the path of the create_file_dll workspace member
    let cargo_root = env!("CARGO_MANIFEST_DIR");
    let relative_path_to_test_dll = "..\\..\\bin\\reflective_loader\\";
    let test_dll_path = Path::new(cargo_root).join(relative_path_to_test_dll);
    assert!(test_dll_path.is_dir());

    println!("Starting cargo build lib");
    let res_build = Command::new("cargo").args(&["build","--release"])
        .current_dir(test_dll_path.clone())
        .stderr(Stdio::piped())
        .spawn().unwrap().stderr.unwrap();

    let reader = BufReader::new(res_build);
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("cargo dll build: {}", line));    

    let res_pack = Command::new("upx").args(&["--best","--lzma",".\\target\\release\\reflective_loader.dll"])
        .current_dir(test_dll_path.clone())
        .stderr(Stdio::piped())
        .spawn().unwrap().stderr.unwrap();

    let reader = BufReader::new(res_pack);
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("cargo dll upx: {}", line));    
    
    let relative_path_to_test_dll_file = "..\\..\\bin\\reflective_loader\\target\\release\\reflective_loader.dll";
    let test_dll_path = Path::new(cargo_root).join(relative_path_to_test_dll_file);
    assert!(test_dll_path.is_file());
}


fn main() {
    #[cfg(target_os = "windows")]
    build_bin_create_file_dll();
    #[cfg(target_os = "windows")]
    build_bin_reflective_loader();
}