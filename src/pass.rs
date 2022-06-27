use crate::menu::select;
use std::env;
use std::process::Command;

fn pass_dir() -> String {
    let home = env::var("HOME").unwrap();
    format!("{}/.password-store", home)
}

fn pass_entries() -> Vec<String> {
    let entries = String::from_utf8(
        Command::new("ls")
            .arg(pass_dir())
            .output()
            .expect("Failed to read ~/.password-store")
            .stdout
    ).unwrap();
    entries
        .split("\n")
        .filter(|&x| x.contains(".gpg"))
        .map(|x| x.replace(".gpg", ""))
        .collect()
}

pub fn run() {
    let entries = pass_entries();
    let selected = select(&entries.join("\n"));
    if selected.is_empty() {
        println!("No entry selected");
        std::process::exit(1);
    }
    let pass = Command::new("pass")
        .arg("-c")
        .arg(selected)
        .spawn()
        .unwrap();
    let out = pass.wait_with_output().unwrap();
    if !out.stderr.is_empty() {
        println!("{}", String::from_utf8(out.stderr).unwrap());
        std::process::exit(1);
    }
}
