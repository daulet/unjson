use colored::Colorize;
use std::{collections::BTreeMap, env::args, io};

const SKIP_KEYS: [&str; 2] = ["timestamp", "level"];

fn runtime_version() -> &'static str {
    option_env!("UNJSON_VERSION_STRING").unwrap_or(env!("CARGO_PKG_VERSION"))
}

#[derive(Debug)]
enum CliCommand {
    Run,
    Version,
}

fn usage(bin: &str) -> String {
    format!("Usage:\n  {bin}\n  {bin} -v|--version")
}

fn parse_cli_command(argv: &[String]) -> Result<CliCommand, String> {
    let bin = argv.first().map_or("unjson", String::as_str);
    match argv {
        [_] => Ok(CliCommand::Run),
        [_, flag] if flag == "-v" || flag == "-V" || flag == "--version" => Ok(CliCommand::Version),
        [_, flag] if flag.starts_with('-') => {
            Err(format!("Unknown flag: {flag}\n\n{}", usage(bin)))
        }
        [_, arg] => Err(format!("Unexpected argument: {arg}\n\n{}", usage(bin))),
        [_, first, ..] => Err(format!(
            "Unexpected arguments starting with: {first}\n\n{}",
            usage(bin)
        )),
        [] => Ok(CliCommand::Run),
    }
}

enum Highlight {
    Color(colored::Color),
    LogLevel,
}

impl Highlight {
    fn output(&self, s: &str) -> String {
        match self {
            Highlight::Color(color) => s.color(*color).to_string(),
            Highlight::LogLevel => match s.to_lowercase().as_str() {
                "info" => s.color(colored::Color::Green).to_string(),
                "warn" => s.color(colored::Color::Yellow).to_string(),
                "error" => s.color(colored::Color::Red).to_string(),
                _ => s.to_string(),
            },
        }
    }
}

fn flatten(highlight_keys: &BTreeMap<&str, Highlight>, json: &serde_json::Value, indent: usize) {
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
            if val.starts_with('"') && val.ends_with('"') {
                val = val.get(1..val.len() - 1).unwrap_or(val);
            }
            if let Some(h) = highlight_keys.get(key) {
                print!(
                    "{}{}",
                    " ".repeat(if first { 0 } else { indent }),
                    h.output(val)
                );
            } else {
                print!("{}{}", " ".repeat(if first { 0 } else { indent }), val);
            }
            first = false;
        }
    }
    for key in keys {
        let mut val: &str = &map[key];
        if val.starts_with('"') && val.ends_with('"') {
            val = val.get(1..val.len() - 1).unwrap_or(val);
        }
        if let Some(h) = highlight_keys.get(key.as_str()) {
            print!(
                "{}{}={}",
                " ".repeat(if first { 0 } else { indent }),
                key,
                h.output(val)
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
            map.insert(
                key.to_string(),
                value.to_string().replace("\\n", "\n").replace("\\\"", "\""),
            );
        }
    }
    map
}

fn main() -> io::Result<()> {
    let argv = args().collect::<Vec<_>>();
    match parse_cli_command(&argv) {
        Ok(CliCommand::Version) => {
            println!("{}", runtime_version());
            return Ok(());
        }
        Ok(CliCommand::Run) => {}
        Err(message) => {
            eprintln!("{message}");
            std::process::exit(2);
        }
    }

    let highlight_keys: BTreeMap<&str, Highlight> = [
        ("level", Highlight::LogLevel),
        ("node", Highlight::Color(colored::Color::BrightBlue)),
    ]
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

#[cfg(test)]
mod tests {
    use super::{parse_cli_command, CliCommand};

    fn argv(parts: &[&str]) -> Vec<String> {
        parts.iter().map(|part| part.to_string()).collect()
    }

    #[test]
    fn parses_version_flags() {
        assert!(matches!(
            parse_cli_command(&argv(&["unjson", "-v"])),
            Ok(CliCommand::Version)
        ));
        assert!(matches!(
            parse_cli_command(&argv(&["unjson", "-V"])),
            Ok(CliCommand::Version)
        ));
        assert!(matches!(
            parse_cli_command(&argv(&["unjson", "--version"])),
            Ok(CliCommand::Version)
        ));
    }

    #[test]
    fn rejects_unknown_flag() {
        let err = parse_cli_command(&argv(&["unjson", "--wat"])).unwrap_err();
        assert!(err.contains("Unknown flag: --wat"));
        assert!(err.contains("Usage:"));
    }

    #[test]
    fn rejects_positional_argument() {
        let err = parse_cli_command(&argv(&["unjson", "file.txt"])).unwrap_err();
        assert!(err.contains("Unexpected argument: file.txt"));
    }
}
