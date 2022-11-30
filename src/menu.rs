use std::io::Write;
use std::process::Command;
use std::process::Stdio;

fn bemenu_args() -> Vec<&'static str> {
    return vec![
        "-l",
        "5",
        "-p",
        "->",
        "-i",
        "-w",
        "-m",
        "1",
        "--fn",
        "Iosevka 33",
    ];
}

pub fn select(input_data: &str, prompt: Option<&str>) -> String {
    let mut cmd = Command::new("bemenu")
        .args(bemenu_args())
        .arg("-p")
        .arg(prompt.unwrap_or("->"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    cmd.stdin
        .as_mut()
        .unwrap()
        .write_all(input_data.as_bytes())
        .unwrap();
    String::from_utf8(cmd.wait_with_output().unwrap().stdout)
        .unwrap()
        .replace("\n", "")
}

pub fn run() {
    let desktop_files = String::from_utf8(
        Command::new("ls")
            .arg("/usr/share/applications")
            .output()
            .expect("Failed to execute ls command")
            .stdout,
    ).unwrap();
    let arr: Vec<String> = desktop_files
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| {
            let parts: Vec<&str> = x.split(".").collect();
            parts.first().unwrap().to_string()
        })
        .collect();
    let selected = select(&arr.join("\n"), None);
    Command::new("gtk-launch")
        .arg(selected)
        .output()
        .expect("Failed to execute selected program");
}
