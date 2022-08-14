
//!  verify if CARGO_PKG_VERSION matches Cargo.toml version currently on GitHub

///
///  Basic usage:
///
///  ```
///  mod gh_status;
///
///  gh_status::check_version()
///      .expect("check_version error");
///  ```
///

use std::env;
use std::io::Read;

pub fn check_version() -> Result<(), Box<dyn std::error::Error>> {

    let url = format!("https://raw.githubusercontent.com/mwrietz/{}/main/Cargo.toml", get_prog_name());

    let mut res = reqwest::blocking::get(url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;
/*
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);
*/
    // split body into vector of lines
    let lines: Vec<&str> = body.split("\n").collect();

    // remove "\"" from each line
    let mut clean_lines = Vec::new();
    let mut github_version = String::new();
    let mut _buffer = String::new();
    for line in lines {
        clean_lines.push(line.replace("\"", ""));
        _buffer = clean_lines
            .last()
            .expect("expected string")
            .to_string();
        if _buffer.starts_with("version") {
            _buffer = _buffer.replace(" ", "");
            github_version = _buffer.replace("version=", "");
        }
    }

    let local_version = env!("CARGO_PKG_VERSION");

    if local_version != github_version {
        println!();
        println!("The local version of '{}' is different than the GitHub version.", get_prog_name());
        println!("    Local version  = {}", local_version);
        println!("    GitHub version = {}", github_version);
        if local_version < github_version.as_str() {
            println!("The GitHub version is newer.  Consider upgrading to the newer version.");
        } else {
            println!("The GitHub version is older.  Consider a commit.");
        }
        println!();
    }

    Ok(())
}

fn get_prog_name() -> String {
    let prog_name = env::current_exe()
        .expect("Can't get the exec path")
        .file_name()
        .expect("Can't get the exec name")
        .to_string_lossy()
        .into_owned();
    prog_name
}
