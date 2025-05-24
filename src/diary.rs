use std::str::FromStr;
use crate::menu::select;
use std::{collections::HashMap, process::Command};

enum ACTIONS {
    START,
    STOP,
    ARCHIVE,
    OPEN,
    UNKNOWN
}

impl FromStr for ACTIONS {
    type Err = ();

    fn from_str(input: &str) -> Result<ACTIONS, ()> {
        match input {
            "START" => Ok(ACTIONS::START),
            "STOP" => Ok(ACTIONS::STOP),
            "ARCHIVE" => Ok(ACTIONS::ARCHIVE),
            "OPEN" => Ok(ACTIONS::OPEN),
            _ => Err(())
        }
    }
}

impl ACTIONS {
    fn to_string(&self) -> String {
        match &self {
            ACTIONS::START => "START".to_string(),
            ACTIONS::STOP => "STOP".to_string(),
            ACTIONS::ARCHIVE => "ARCHIVE".to_string(),
            ACTIONS::OPEN => "OPEN".to_string(),
            ACTIONS::UNKNOWN => "UNKNOWN".to_string()
        }
    }
}
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
        if value.starts_with("/") { 
            out.insert(format!("*{}", key), String::from(value));
        }
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
    let actions: [&str; 4] = [
        &ACTIONS::START.to_string(), 
        &ACTIONS::STOP.to_string(), 
        &ACTIONS::ARCHIVE.to_string(),
        &ACTIONS::OPEN.to_string()
    ];
    select(&actions.join("\n"), Some(&prompt))
}


fn do_action(action: &str, entry: (&String, &String)) {
    let cmd = match ACTIONS::from_str(action).unwrap_or(ACTIONS::UNKNOWN) {
        ACTIONS::START => "start",
        ACTIONS::STOP => "stop",
        ACTIONS::ARCHIVE => "archive",
        ACTIONS::OPEN => {
           Command::new("foot")
                .arg("nvim")
                .arg(entry.1)
                .output()
                .expect("Cannot open terminal");
            std::process::exit(0)
        }
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
