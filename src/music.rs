use crate::menu::select;
use std::env;
use std::process::Command;

const STOP: &str = "---> STOP";

fn music_dir() -> String {
    let home = env::var("HOME").unwrap();
    format!("{}/Music", home)
}

fn music_entries() -> Vec<String> {
    let entries = String::from_utf8(
        Command::new("rg")
            .current_dir(music_dir())
            .arg("--files")
            .output()
            .expect("Failed to read music directory")
            .stdout
    ).unwrap();
    let mut output: Vec<String> = entries
        .split("\n")
        .map(str::to_string)
        .collect();
    output.push(STOP.to_string());
    output
}

pub fn run() {
    let entries = music_entries();
    let selected = select(&entries.join("\n"), None);
    if selected.is_empty() {
        println!("No entry selected");
        std::process::exit(1);
    }
    let mpv_cmd = "mpv";
    match selected.as_str() {
        STOP => {
            Command::new("killall")
                .arg(&mpv_cmd)
                .spawn()
                .unwrap();
        }
        _ => {
            let mpv = Command::new(&mpv_cmd)
                .current_dir(music_dir())
                .arg("--really-quiet")
                .arg("--audio-display=no")
                .arg(selected)
                .spawn()
                .unwrap();
            let out = mpv.wait_with_output().unwrap();
            if !out.stderr.is_empty() {
                println!("{}", String::from_utf8(out.stderr).unwrap());
                std::process::exit(1);
            }
        }
    }
}
