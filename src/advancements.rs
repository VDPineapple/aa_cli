use quick_xml::events;
use quick_xml::reader::Reader;
use serde_json::{Error, Value};
use std::fs;

pub fn load_advancements(dir: &str) -> Result<Value, Error> {
    let dir = format!("{}/advancements", dir);
    let file = fs::read_dir(dir).unwrap().next().unwrap().unwrap().path();
    let contents = fs::read_to_string(file).unwrap();
    let json = serde_json::from_str(&contents);
    json
}

#[derive(Debug, Clone)]
pub struct Group {
    pub name: String,
    pub advancements: Vec<Advancement>,
    pub tiered_advancements: Vec<Advancement>,
    pub done: bool,
}

#[derive(Debug, Clone)]
pub struct Advancement {
    pub id: String,
    pub _name: String,
    pub short_name: String,
    pub criteria: Vec<Criteria>,
}

#[derive(Debug, Clone)]
pub struct Criteria {
    pub id: String,
    pub name: String,
}

pub fn get_all_advancements() -> Vec<Group> {
    // Read advancements/adventure.xml
    // let file = fs::read_to_string("/home/user/Games/aa_cli/advancements/everything.xml").unwrap();
    let file = include_str!("../advancements/everything.xml");
    let mut reader = Reader::from_str(file);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut groups: Vec<Group> = Vec::new();

    loop {
        // NOTE: this is the generic case when we don't know about the input BufRead.
        // when the input is a &str or a &[u8], we don't actually need to use another
        // buffer, we could directly call `reader.read_event()`
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(events::Event::Eof) => break,

            Ok(events::Event::Start(e)) => {
                match e.name().as_ref() {
                    b"group" => {
                        for a in e.attributes().map(|a| a.unwrap().value) {
                            let name = std::str::from_utf8(&a).unwrap();
                            groups.push(Group {
                                name: name.to_string(),
                                advancements: Vec::new(),
                                tiered_advancements: Vec::new(),
                                done: false,
                            });
                        }
                    }
                    b"advancement" => {
                        let mut id = String::new();
                        let mut name = String::new();
                        let mut short_name = String::new();
                        for a in e.attributes().map(|a| a.unwrap()) {
                            let key = std::str::from_utf8(a.key.as_ref()).unwrap();
                            let value = std::str::from_utf8(&a.value).unwrap();
                            match key {
                                "id" => id = value.to_string(),
                                "name" => {
                                    name = value.to_string();
                                    short_name = value.to_string();
                                },
                                "short_name" => short_name = value.to_string(),
                                _ => (),
                            }
                        }
                        let advancement = Advancement {
                            id,
                            _name: name,
                            short_name,
                            criteria: Vec::new(),
                        };
                        groups.last_mut().unwrap().tiered_advancements.push(advancement);
                    }
                    _ => (),
                }
            }
            Ok(events::Event::Empty(e)) => {
                match e.name().as_ref() {
                    b"advancement" => {
                        let mut id = String::new();
                        let mut name = String::new();
                        let mut short_name = String::new();
                        for a in e.attributes().map(|a| a.unwrap()) {
                            let key = std::str::from_utf8(a.key.as_ref()).unwrap();
                            let value = std::str::from_utf8(&a.value).unwrap();
                            match key {
                                "id" => id = value.to_string(),
                                "name" => {
                                    name = value.to_string();
                                    short_name = value.to_string();
                                },
                                "short_name" => short_name = value.to_string(),
                                _ => (),
                            }
                        }
                        let advancement = Advancement {
                            id,
                            _name: name,
                            short_name,
                            criteria: Vec::new(),
                        };
                        groups.last_mut().unwrap().advancements.push(advancement);
                    }
                    b"criterion" => {
                        let mut id = String::new();
                        let mut name = String::new();
                        for a in e.attributes().map(|a| a.unwrap()) {
                            let key = std::str::from_utf8(a.key.as_ref()).unwrap();
                            let value = std::str::from_utf8(&a.value).unwrap();
                            match key {
                                "id" => {
                                    id = value.to_string();
                                    // capitalize the first letter of the id
                                    let id_short = id.split(":").last().unwrap();
                                    let spaced = id_short.replace("_", " ");
                                    let mut chars = spaced.chars();
                                    let capital = match chars.next() {
                                        None => String::new(),
                                        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
                                    };

                                    name = capital;
                                },
                                "name" => name = value.to_string(),
                                "short_name" => name = value.to_string(),
                                _ => (),
                            }
                        }
                        let criterion = Criteria {
                            id,
                            name,
                        };
                        groups.last_mut().unwrap().tiered_advancements.last_mut().unwrap().criteria.push(criterion);
                    }
                    _ => (),
                }
            }

            // There are several other `Event`s we do not consider here
            _ => (),
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }

    groups
}

pub fn check_done(json: &Value, advancement: &Advancement) -> bool {
    // The advancement.id is the key to the json object
    match json.get(advancement.id.as_str()) {
        Some(obj) => {
            let done = obj.get("done");
            if done.is_none() {
                return false;
            }
            done.unwrap().as_bool().unwrap()
        },
        None => false,
    }
}

pub fn check_done_criterion(json: &Value, advancement: &Advancement, criterion: &Criteria) -> bool {
    // The criterion.id is the key to the "criteria" object in the advancement object
    match json.get(advancement.id.as_str()) {
        Some(obj) => {
            let criteria = obj.get("criteria");
            if criteria.is_none() {
                return false;
            }
            let done = criteria.unwrap().get(criterion.id.as_str());
            done.is_some()
        },
        None => false,
    }
}