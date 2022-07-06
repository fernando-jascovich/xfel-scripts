use crate::menu::select;
use std::collections::HashMap;
use std::process::Command;

const MIN_OVERRIDE_COLS: &str ="rc.report.minimal.columns=id,description.desc";
const MIN_OVERRIDE_VALS: &str = "rc.report.minimal.labels=a,b";
const ACTIVE_OVERRIDE_COLS: &str = "rc.report.active.columns=id,description.desc";
const ACTIVE_OVERRIDE_VALS: &str = "rc.report.active.labels=a,b";
const ACTIONS: [&str; 5] = ["START", "STOP", "DETAILS", "ANNOTATE", "DONE"];

fn add_task_entry(input: &&str, map: &mut HashMap<String, String>) {
    if let Some(break_index) = input.find(" ") {
        let key: String = input.chars().skip(break_index + 1).collect();
        let value: String = input.chars().take(break_index).collect();
        map.insert(key, value);
    }
}

fn map_for_cmd(stdout: Vec<u8>) -> HashMap::<String, String> {
    let input = String::from_utf8(stdout).unwrap();
    let mut parts: Vec<&str> = input
        .split("\n")
        .map(|x| x.trim())
        .skip(3)
        .filter(|x| !x.is_empty())
        .collect();
    parts.truncate(parts.len() - 1);
    let mut out = HashMap::<String, String>::new();
    parts.iter().for_each(|x: &&str| add_task_entry(x, &mut out));
    out
}

fn entries() -> HashMap::<String, String> {
    map_for_cmd(
        Command::new("task")
            .arg("minimal")
            .arg(MIN_OVERRIDE_COLS)
            .arg(MIN_OVERRIDE_VALS)
            .output()
            .expect("Failed to execute task minimal")
            .stdout
    )
}

fn active() -> HashMap::<String, String> {
    map_for_cmd(
        Command::new("task")
            .arg("active")
            .arg(ACTIVE_OVERRIDE_COLS)
            .arg(ACTIVE_OVERRIDE_VALS)
            .output()
            .expect("Failed to execute task active")
            .stdout
    )
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

fn task_warrior(task_id: &str, args: Vec<&str>) {
    let task_out = Command::new("task")
        .arg(task_id)
        .args(args)
        .output()
        .expect("Cannot execute task warrior")
        .stdout;
    Command::new("notify-send")
        .arg("Task")
        .arg(String::from_utf8(task_out).unwrap())
        .output()
        .expect("Cannot execute notify-send");
}

fn do_action(action: &str, entry: (&String, &String)) {
    let task_cmd = match action {
        "START" => "start",
        "STOP" => "stop",
        "DETAILS" => "list",
        "ANNOTATE" => "annotate",
        "DONE" => "done",
        _ => ""
    };
    if task_cmd.is_empty() {
        std::process::exit(1);
    }
    if task_cmd != "annotate" {
        task_warrior(entry.1, vec![task_cmd]);
        return
    }
    let prompt = format!("{} | {} ->", entry.0, task_cmd);
    let extra_arg = select("", Some(&prompt));
    task_warrior(entry.1, vec![task_cmd, &extra_arg]);
}

pub fn run() {
    let entries = entries();
    let selected = show_entries(&entries);
    if selected.is_empty() {
        std::process::exit(1);
    }
    let entry = entries.get_key_value(&selected).unwrap();
    let action = prompt_for_action(entry);
    if action.is_empty() {
        std::process::exit(1);
    }
    do_action(&action, entry);
}
