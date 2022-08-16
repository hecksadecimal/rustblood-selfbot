use std::{collections::BTreeMap, io::Read};
use std::env::current_dir;
use relative_path::RelativePath;
use std::fs::File;
use rand::seq::SliceRandom;
use regex::Regex;

use serde::{Serialize, Deserialize};
use serde_json::Value;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Character {
    pub handle: String,
    pub acronym: String,
    pub quirks: Vec<BTreeMap<String, Value>>,
}
impl Character {
    pub fn quirked(&self, s: &str) -> String {
        return quirked(s, self);
    }

    pub fn from_name(n: &str) -> Option<Character> {
        let root = current_dir().unwrap();
        let rel_path_string = format!("./quirks/{}.json", n);

        let rel_path = RelativePath::new(rel_path_string.as_str());
        let full_path = rel_path.to_path(&root);
        let file_exists: bool = full_path.is_file();
        if !file_exists {
            None
        } else {
            let mut file = File::open(full_path).unwrap();
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();

            let c = parse_safe(data);

            Some(c)
        }
    }
}

#[derive(Debug, Default)]
pub struct Characters {
    pub string: String,
    pub characters: BTreeMap<String, Character>
}

impl Characters {
    pub fn from_string(s: &str) -> Characters {
        let mut cs = Characters::default();
        cs.string = s.to_owned();
        let prefix_regex_string = r"^((?P<name>[A-Za-z]{1,})?: )";
        let regex = Regex::new(prefix_regex_string).unwrap();
        let lines = s.split("\n");
        for line in lines {
            for caps in regex.captures_iter(line) {
                let c = Character::from_name(&caps["name"]);
                if c.is_some(){
                    cs.characters.entry(caps["name"].to_string()).or_insert_with(|| {
                        c.unwrap()
                    });
                }
            }
        }

        cs
    }

    pub fn quirked(&self) -> String {
        let mut string = String::new();
        let prefix_regex_string = r"^(?P<to_remove>(?P<name>[A-Za-z]{1,})?: )";
        let regex = Regex::new(prefix_regex_string).unwrap();
        let lines = self.string.split("\n");
        for line in lines {
            if regex.captures_iter(line).count() > 0 {
                for caps in regex.captures_iter(line) {
                    if self.characters.contains_key(&caps["name"]) {
                        let line_trimmed = line.replace(&caps["to_remove"], "");
                        string = string + &self.characters[&caps["name"]].quirked(&line_trimmed) + "\n";
                    } else {
                        string = string + line + "\n";
                    }
                }
            } else {
                string = string + line + "\n";
            }
        }

        string.trim_end().to_string()
    }
}

pub fn parse_safe(s: String) -> Character {
    let s: &str = &s;
    let c: Character = serde_json::from_str(&strip_jsonc_comments(s, true)).unwrap();
    c
}

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn invert_capitalization(s: &str) -> String {
    let c = s.chars();
    let mut new_string = String::new();
    for ch in c {
        if ch.is_uppercase() {
            new_string = new_string + ch.to_lowercase().to_string().as_str();
        } else if ch.is_lowercase() {
            new_string = new_string + ch.to_uppercase().to_string().as_str();
        } else {
            new_string = new_string + ch.to_string().as_str();
        }
    }
    new_string
}


/// Takes a string of jsonc content and returns a comment free version
/// which should parse fine as regular json.
/// Nested block comments are supported.
/// preserve_locations will replace most comments with spaces, so that JSON parsing
/// errors should point to the right location.
pub fn strip_jsonc_comments(jsonc_input: &str, preserve_locations: bool) -> String {
    let mut json_output = String::new();

    let mut block_comment_depth: u8 = 0;
    let mut is_in_string: bool = false; // Comments cannot be in strings

    for line in jsonc_input.split('\n') {
        let mut last_char: Option<char> = None;
        for cur_char in line.chars() {
            // Check whether we're in a string
            if block_comment_depth == 0 && last_char != Some('\\') && cur_char == '"' {
                is_in_string = !is_in_string;
            }

            // Check for line comment start
            if !is_in_string && last_char == Some('/') && cur_char == '/' {
                last_char = None;
                if preserve_locations {
                    json_output.push_str("  ");
                }
                break; // Stop outputting or parsing this line
            }
            // Check for block comment start
            if !is_in_string && last_char == Some('/') && cur_char == '*' {
                block_comment_depth += 1;
                last_char = None;
                if preserve_locations {
                    json_output.push_str("  ");
                }
            // Check for block comment end
            } else if !is_in_string && last_char == Some('*') && cur_char == '/' {
                if block_comment_depth > 0 {
                    block_comment_depth -= 1;
                }
                last_char = None;
                if preserve_locations {
                    json_output.push_str("  ");
                }
            // Output last char if not in any block comment
            } else {
                if block_comment_depth == 0 {
                    if let Some(last_char) = last_char {
                        json_output.push(last_char);
                    }
                } else {
                    if preserve_locations {
                        json_output.push_str(" ");
                    }
                }
                last_char = Some(cur_char);
            }
        }

        // Add last char and newline if not in any block comment
        if let Some(last_char) = last_char {
            if block_comment_depth == 0 {
                json_output.push(last_char);
            } else if preserve_locations {
                json_output.push(' ');
            }
        }

        // Remove trailing whitespace from line
        while json_output.ends_with(' ') {
            json_output.pop();
        }
        json_output.push('\n');
    }

    json_output
}

pub fn quirked(s: &str, c: &Character) -> String {
    let mut new_string;
    new_string = mutate_line_multi(s, &c.quirks);
    new_string = format!("{}: {}", c.acronym, new_string);
    new_string
}

pub fn mutate_line_multi(s: &str, d: &Vec<BTreeMap<String, Value>>) -> String {
    let mut new_string = s.to_owned();
    for quirk in d {
        new_string = mutate_line(new_string.as_str(), quirk);
    }

    new_string.to_string()
}

pub fn mutate_line(s: &str, d: &BTreeMap<String, Value>) -> String {
    let mut rng = rand::thread_rng();
    let mut string: String = s.to_owned().clone();
    let first = d.iter().next().unwrap();
    let t = first.0.clone();
    let v = first.1.clone();
    match t.as_str() {
        "prefix" => {format!("{}{}", &v.as_str().unwrap(), s)},
        "suffix" => {format!("{}{}", s, &v.as_str().unwrap())},
        "simple_replacements" => {
            for replacement in v.as_array().unwrap() {
                string = string.replace(replacement[0].as_str().unwrap(), replacement[1].as_str().unwrap());
            }
            string
        },
        "random_replacements" => {
            for replacement in v.as_array().unwrap() {
                string = string.replace(replacement[0].as_str().unwrap(), replacement[1].as_array().unwrap().choose(&mut rng).unwrap().as_str().unwrap());
            }
            string
        },
        "regex_replacements" => {
            for replacement in v.as_array().unwrap() {
                let re = Regex::new(replacement[0].as_str().unwrap()).unwrap();
                let after = re.replace_all(string.as_str(), replacement[1].as_str().unwrap());
                string = after.to_string();
            }
            string
        }
        "scramble" => {
            for replacement in v.as_array().unwrap() {
                let scrambler = replacement[1].as_str().unwrap();
                let scrambler_graphemes = scrambler.graphemes(true);
                let mut list_graphemes: Vec<&str> = Vec::new();
                for g in scrambler_graphemes {
                    list_graphemes.push(g);
                }
                let cloned_string = string.to_owned();
                let ms = cloned_string.matches(replacement[0].as_str().unwrap());
                for m in ms {
                    list_graphemes.shuffle(&mut rng);
                    let scrambler = String::from_iter(list_graphemes.to_owned());
                    string = string.replacen(m, &scrambler, 1);
                }
            }
            string
        },
        "style" => {
            match v.as_str().unwrap() {
                "lowercase" => {s.to_lowercase()},
                "uppercase" => {s.to_uppercase()},
                "alternating" => {
                    let mut new_string = "".to_owned();
                    let graphemes = string.graphemes(true);
                    for (i, g) in graphemes.enumerate() {
                        if i % 2 == 0 {
                            new_string.push_str(&g.to_uppercase());
                        } else {
                            new_string.push_str(&g.to_lowercase());
                        }
                    }
                    new_string
                },
                "camelcase" => {
                    let mut new_string = "".to_owned();
                    let words = string.split_word_bounds();
                    for word in words {
                        new_string.push_str(&capitalize(word));
                    }
                    new_string
                },
                "reverse" => {
                    let mut new_string = "".to_owned();
                    for g in string.graphemes(true).rev() {
                        new_string.push_str(g);
                    }
                    new_string
                },
                "inverted" => {
                    let res = invert_capitalization(string.as_str());
                    res
                },
                &_ => {s.to_string()}
            }
        },
        &_ => {s.to_string()},
    }
}
