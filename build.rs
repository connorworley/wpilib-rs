extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // HAL
    Command::new("mvn")
        .arg("dependency:get")
        .arg(format!("-Ddest={}", out_path.join("hal.zip").as_os_str().to_str().unwrap()))
        .arg("-DremoteRepositories=http://first.wpi.edu/FRC/roborio/maven/release")
        .arg("-DgroupId=edu.wpi.first.wpilib")
        .arg("-DartifactId=hal")
        .arg("-Dversion=2017.3.1")
        .arg("-Dpackaging=zip")
        .output()
        .expect("Failed to download FRC HAL via Maven");
    Command::new("unzip")
        .arg(out_path.join("hal.zip").as_os_str().to_str().unwrap())
        .arg("-d")
        .arg(out_path.join("hal").as_os_str().to_str().unwrap())
        .output()
        .expect("Failed to unzip FRC HAL");
    println!("cargo:rustc-link-search={}", out_path.join("hal/lib").as_os_str().to_str().unwrap());
    
    // Athena runtime libraries
    Command::new("mvn")
        .arg("dependency:get")
        .arg(format!("-Ddest={}", out_path.join("athena-runtime.zip").as_os_str().to_str().unwrap()))
        .arg("-DremoteRepositories=http://first.wpi.edu/FRC/roborio/maven/release")
        .arg("-DgroupId=edu.wpi.first.wpilib")
        .arg("-DartifactId=athena-runtime")
        .arg("-Dversion=2017.3.1")
        .arg("-Dpackaging=zip")
        .output()
        .expect("Failed to download Athena Runtime via Maven");
    Command::new("unzip")
        .arg(out_path.join("athena-runtime.zip").as_os_str().to_str().unwrap())
        .arg("-d")
        .arg(out_path.join("athena-runtime").as_os_str().to_str().unwrap())
        .output()
        .expect("Failed to unzip Athena Runtime");
    println!("cargo:rustc-link-search={}", out_path.join("athena-runtime/lib").as_os_str().to_str().unwrap());

    println!("cargo:rustc-link-lib=nilibraries");
    println!("cargo:rustc-link-lib=wpiutil");
    println!("cargo:rustc-link-lib=HALAthena");
    
    let bindings = bindgen::Builder::default()
        .header("hal-wrapper.hpp")
        .whitelisted_type("HAL_.*")
        .whitelisted_function("HAL_.*")
        .whitelisted_var("HAL_.*")
        .trust_clang_mangling(false)
        .clang_arg("-std=c++14")
        .clang_arg(format!("-I{}", out_path.join("hal/include").as_os_str().to_str().unwrap()))
        .generate()
        .expect("Unable to generate bindings for FRC HAL");
    bindings.write_to_file(out_path.join("hal-bindings.rs"))
        .expect("Failed to write bindings to file");
}
