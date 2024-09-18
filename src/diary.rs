use crate::menu::select;
use std::{collections::HashMap, process::Command};

const ACTIONS: [&str; 3] = ["START", "STOP", "ARCHIVE"];

fn add_task_entry(input: &&str, map: &mut HashMap<String, String>) {
    println!("{}", input);
    if let Some(k) = input.trim().rsplit_once("/") {
        map.insert(k.1.to_string(), input.to_string());
    }
}

fn map_for_cmd(stdout: Vec<u8>) -> HashMap::<String, String> {
    let input = String::from_utf8(stdout).unwrap();
    let parts: Vec<&str> = input
        .split("\n")
        .filter(|x| !x.is_empty())
        .collect();
    let mut out = HashMap::<String, String>::new();
    parts.iter().for_each(|x: &&str| add_task_entry(x, &mut out));
    out
}


fn entries() -> HashMap<String, String> {
    map_for_cmd(
        Command::new("xfel-worklog")
            .arg("browse")
            .output()
            .expect("Failed to execute xfel-worklog")
            .stdout
    )
}

fn active() -> HashMap::<String, String> {
    let active = map_for_cmd(
        Command::new("xfel-worklog")
            .arg("browse")
            .arg("-a")
            .output()
            .expect("Failed to execute xfel-worklog")
            .stdout
    );
    let mut out = HashMap::new();
    for (key, value) in active {
        out.insert(format!("*{}", key), String::from(value));
    }
    out
}

fn show_entries(entries: &HashMap<String, String>) -> String {
    let active_entries = active();
    let normal_entries_keys = entries.keys().filter(|&x| !active_entries.contains_key(x));
    let mut entries_keys = active_entries.keys().chain(normal_entries_keys);
    let mut for_select = "".to_owned();
    while let Some(entry) = entries_keys.next() {
        for_select.push_str(entry);
        for_select.push_str("\n");
    }
    select(&for_select, None)
}

fn prompt_for_action(entry: (&String, &String)) -> String {
    let prompt = format!("{} ->", entry.0);
    select(&ACTIONS.join("\n"), Some(&prompt))
}


fn do_action(action: &str, entry: (&String, &String)) {
    let cmd = match action {
        "START" => "start",
        "STOP" => "stop",
        "ARCHIVE" => "archive",
        _ => std::process::exit(1)
    };
    let task_out = Command::new("xfel-worklog")
        .arg("action")
        .arg(entry.1)
        .arg(cmd)
        .output()
        .expect("Cannot execute xfel-worklog")
        .stdout;
    Command::new("notify-send")
        .arg("Task")
        .arg(String::from_utf8(task_out).unwrap())
        .output()
        .expect("Cannot execute notify-send");
}

pub fn run () {
    let entries = entries();
    let selected = show_entries(&entries);
    if selected.is_empty() {
        std::process::exit(1);
    }
    let key = selected.replace("*", "");
    let entry = entries.get_key_value(&key).unwrap();
    let action = prompt_for_action(entry);
    if action.is_empty() {
        std::process::exit(1);
    }
    do_action(&action, entry);
}
