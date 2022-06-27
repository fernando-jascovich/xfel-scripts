use std::io::Write;
use std::process::Stdio;
use std::process::Command;


fn bemenu_args<'a>(cmd: &'a mut Command) -> &'a mut Command {
    cmd
        .arg("-m").arg("1")
        .arg("--fn").arg("Iosevka 25")
}


pub fn select(input_data: &str) -> String {
    let mut cmd = Command::new("bemenu");
    let mut proc = bemenu_args(&mut cmd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    proc.stdin.as_mut().unwrap().write_all(
        input_data.as_bytes()
    ).unwrap();
    String::from_utf8(
        proc.wait_with_output().unwrap().stdout
    ).unwrap().replace("\n", "")
}

pub fn run() {
    let mut cmd = Command::new("bemenu-run");
    bemenu_args(&mut cmd)
        .output()
        .expect("Failed to execute bemenu");
}

