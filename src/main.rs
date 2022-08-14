use std::env;

#[tokio::main]
async fn main() {
    let future = check_version();

    // body of program here

    future.await;
}

async fn check_version() {
    let body = reqwest::get("https://raw.githubusercontent.com/mwrietz/yfa-quotes/main/Cargo.toml")
        .await
        .expect("error 1")
        .text()
        .await
        .expect("error 2");

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
        if _buffer.contains("version") {
            _buffer = _buffer.replace(" ", "");
            github_version = _buffer.replace("version=", "");
        }
    }

    let local_version = env!("CARGO_PKG_VERSION");

    if local_version != github_version {
        if local_version < github_version.as_str() {
            println!("The github version of this program is newer than the local version. Consider upgrading to the newer version.");
        } else {
            println!("The local version of this program is newer than the github version. Consider a commit.");
        }
        println!("local version  = {}", local_version);
        println!("github version = {}", github_version);
    }

}
