extern crate cargo;
extern crate clap;
extern crate home;

use cargo::*;
use clap::{App, Arg};
use std::process::Command;

fn main() {
    let matches = App::new("Cargo-debug")
        .version("0.1")
        .author("Jasper Bekkers. <bekkers@gmail.com>")
        .about("Launch Visual Studio for the current rust project")
        .arg(Arg::with_name("release").long("release"))
        .get_matches();

    let repo_path = std::env::current_dir().unwrap();

    let shell = cargo::core::shell::Shell::new();

    let mut config = cargo::util::config::Config::new(
        shell,
        repo_path.to_path_buf(),
        ::home::cargo_home_with_cwd(&repo_path).unwrap(),
    );

    config
        .configure(
            0,          // verbose,
            Some(true), // quiet
            &None,      // color
            false,      // frozen
            false,      // locked
            &None,      // target_dir
            &[],        // unstable_flags
        ).unwrap();

    let workspace = core::Workspace::new(&repo_path.join("Cargo.toml"), &config).unwrap();

    let targets = workspace
        .current()
        .unwrap()
        .manifest()
        .targets()
        .iter()
        .filter(|target| target.is_bin())
        .collect::<Vec<&cargo::core::Target>>();

    let target_name = targets[0].crate_name().replace("_", "-");
    let mut output_path = workspace
        .target_dir()
        .into_path_unlocked()
        .join(if matches.is_present("release") {
            "release"
        } else {
            "debug"
        }).join(target_name);

    output_path.set_extension("exe");

    if let Ok(vs_2017) = std::env::var("VS2017INSTALLDIR") {
        println!("Debugging: {:?}", output_path);

        let ide_path = std::path::Path::new(&vs_2017)
            .join("Common7")
            .join("IDE")
            .join("devenv.exe");

        let child = Command::new(ide_path)
            .args(&["/DebugExe", output_path.to_str().unwrap()])
            .spawn()
            .unwrap();

        child.wait_with_output().unwrap();
    } else {
        panic!("Failed to find IDE");
    }
}
