use colored::*;
use std::{collections::BTreeMap, io};

const SKIP_KEYS: [&str; 2] = ["timestamp", "level"];

fn flatten(
    highlight_keys: &BTreeMap<&str, colored::Color>,
    json: &serde_json::Value,
    indent: usize,
) {
    let map = walk_json(json);
    let mut keys = map
        .keys()
        .filter(|k| !SKIP_KEYS.contains(&k.as_str()))
        .collect::<Vec<_>>();
    keys.sort();

    let mut first = true;
    for key in SKIP_KEYS {
        if map.contains_key(key) {
            let mut val: &str = &map[key];
            val = val.get(1..val.len() - 1).unwrap_or(val);
            if let Some(color) = highlight_keys.get(key) {
                print!(
                    "{}{}",
                    " ".repeat(if first { 0 } else { indent }),
                    val.color(*color)
                );
            } else {
                print!("{}{}", " ".repeat(if first { 0 } else { indent }), val);
            }
            first = false;
        }
    }
    for key in keys {
        let mut val: &str = &map[key];
        val = val.get(1..val.len() - 1).unwrap_or(val);
        if let Some(color) = highlight_keys.get(key.as_str()) {
            print!(
                "{}{}={}",
                " ".repeat(if first { 0 } else { indent }),
                key,
                val.color(*color)
            );
        } else {
            print!(
                "{}{}={}",
                " ".repeat(if first { 0 } else { indent }),
                key,
                val
            );
        }
        first = false;
    }
    println!();
}

fn walk_json(json: &serde_json::Value) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for (key, value) in json.as_object().unwrap() {
        if value.is_object() {
            map.extend(walk_json(value));
        } else {
            // TODO escape string brackets
            map.insert(key.to_string(), value.to_string());
        }
    }
    map
}

fn main() -> io::Result<()> {
    // TODO special mode for level to change color based on value
    let highlight_keys: BTreeMap<&str, colored::Color> =
        [("level", Color::Green), ("node", Color::BrightBlue)]
            .into_iter()
            .collect();

    for line in io::stdin().lines() {
        let line = line?;
        if let Ok(json) = serde_json::from_str(&line) {
            flatten(&highlight_keys, &json, 2);
        } else {
            println!("{}", line);
        }
    }
    Ok(())
}
