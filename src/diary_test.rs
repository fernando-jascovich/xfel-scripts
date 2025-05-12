use std::collections::HashMap;
use crate::diary::{map_for_cmd, add_task_entry};

#[test]
fn test_map_for_cmd() {
    let output = b"Task 1\nTask 2\nTask 3\n";
    let map = map_for_cmd(output.to_vec());
    assert_eq!(map.len(), 3);
    assert_eq!(map["1"], "Task 1");
    assert_eq!(map["2"], "Task 2");
    assert_eq!(map["3"], "Task 3");
}

#[test]
fn test_add_task_entry() {
    let mut map = HashMap::new();
    add_task_entry(&"Task 1", &mut map);
    assert_eq!(map.len(), 1);
    assert_eq!(map["Task 1"], "Task 1");
}
