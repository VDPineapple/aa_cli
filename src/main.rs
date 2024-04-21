use std::env;
use crossterm::{
    style::Color,
    event::{poll, read, Event, KeyCode},
};
use serde_json::Value;
use aa_cli::{advancements::{
    check_done, check_done_criterion, get_all_advancements, load_advancements, Advancement, Group
}, logs::new_line};
use aa_cli::logs::{print_logs, read_logs, write_logs, write, write_line};
use aa_cli::stats::*;
use aa_cli::bolan::get_bolan;

fn print_advancements(json: &Value, all_advancements: &mut Vec<Group>) {
    let orange = Color::Rgb { r: 255, g: 165, b: 0 };
    let green = Color::Rgb { r: 0, g: 255, b: 0 };
    write_line(&Color::Rgb { r: 0, g: 255, b: 255 }, true, "All Advancements CLI Tracker");
    
    let wrap = 12;
    for group in all_advancements {
        let mut count = 0;
        if group.done {
            write_line(&green.clone(), true, format!("{}:", group.name).as_str());
        } else {
            write_line(&orange, true, format!("{}:", group.name).as_str());
        }
        group.done = true;
        for advancement in &group.advancements {
            let done = check_done(json, &advancement);
            let style = if done {
                green.clone()
            } else {
                group.done = false;
                Color::Rgb { r: 255, g: 0, b: 0 }
            };
            let spaced_name = format!("{:<15}", advancement.short_name);
            write(&style, true, format!("{}", spaced_name).as_str());
            count += 1;
            if count % wrap == 0 {
                new_line();
            }
        }
        if count % wrap != 0 {
            new_line();
        }

        for advancement in &group.tiered_advancements {
            let done = check_done(json, &advancement);
            let style = if done {
                green.clone()
            } else {
                group.done = false;
                Color::Rgb { r: 255, g: 0, b: 0 }
            };
            write(&style, true, format!("{:<15}", advancement.short_name).as_str());
            count = 1;
            for criterion in advancement.clone().criteria {
                let done = check_done_criterion(json, &advancement, &criterion);
                let style = if done {
                    green.clone()
                } else {
                    Color::Rgb { r: 255, g: 60, b: 0 }
                };
                let spaced_name = format!("{:<15}", criterion.name);
                write(&style, false, format!("{}", spaced_name).as_str());
                count += 1;
                if count % wrap == 0 {
                    new_line();
                }
            }
            new_line();
        }
        
        if count % wrap != 0 {
            new_line();
        }
    }
}

// Stats, Notch, etc
fn print_other(json: &Value, stats: &Value) {
    let orange = Color::Rgb { r: 255, g: 165, b: 0 };
    let green = Color::Rgb { r: 0, g: 255, b: 0 };
    
    let gapple_adv: Advancement = Advancement {
        id: "minecraft:recipes/misc/mojang_banner_pattern".to_string(),
        _name: "Has Notch".to_string(),
        short_name: "Has Notch".to_string(),
        criteria: Vec::new(),
    };
    
    let gapple = check_done(json, &gapple_adv) as usize;
    let wither_heads = get_wither_heads(stats);
    let beehives = get_beehives(stats);
    let shells = get_shells(stats);
    let tridents = get_tridents(stats);
    let gold_blocks = get_gold_blocks(stats);
    
    let other_names = ["Notches", "Beehives", "Shells", "Skulls", "Tridents", "G. Blocks"];
    let other_values = [gapple, beehives, shells, wither_heads, tridents, gold_blocks];
    let other_thresholds = [1, 2, 8, 3, 1, 164];
    
    let mut done = true;
    for i in 0..other_names.len() {
        if other_values[i] < other_thresholds[i] {
            done = false;
        }
    }
    
    write_line(if done {&green} else {&orange}, true, format!("Other:").as_str());
    
    for i in 0..other_names.len() {
        let style = if other_values[i] >= other_thresholds[i] {
            green.clone()
        } else {
            Color::Rgb { r: 255, g: 0, b: 0 }
        };
        let spaced_name = format!("{:<15}", format!("{} {}", other_values[i], other_names[i]));
        write(&style, true, format!("{}", spaced_name).as_str());
    }

    let skeletons_killed = get_wither_skeletons_killed(stats);
    let drowned_killed = get_drowned_killed(stats);

    let style = if wither_heads >= 3 {
        green.clone()
    } else {
        Color::Rgb { r: 255, g: 0, b: 0 }
    };
    let spaced_name = format!("{:<15}", format!("{} W. Skeles", skeletons_killed));
    write(&style, true, format!("{}", spaced_name).as_str());

    let style = if tridents >= 1 && shells >= 8 {
        green.clone()
    } else {
        Color::Rgb { r: 255, g: 0, b: 0 }
    };
    let spaced_name = format!("{:<15}", format!("{} Drowned", drowned_killed));
    write(&style, true, format!("{}", spaced_name).as_str());
    
    new_line();
    
}

pub fn print_bolan() {
    let orange = Color::Rgb { r: 255, g: 165, b: 0 };
    write_line(&Color::Rgb { r: 0, g: 255, b: 255 }, true, "Bolan");
    let bolan = get_bolan();
    for (key, value) in bolan {
        write_line(&orange, true, format!("{}:", key).as_str());
        for (key, value) in value {
            let spaced_name = format!("{:<15}", format!("{}: {}", key, value));
            write(&Color::Rgb { r: 255, g: 0, b: 0 }, true, format!("{}", spaced_name).as_str());
        }
        new_line();
    }

    loop {
        if poll(std::time::Duration::from_millis(0)).unwrap() {
            let x = read().unwrap();
            match x {
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Enter => {
                            break;
                        },
                        _ => (),
                    }
                },
                _ => (),
            }
        }
    }
}

fn main() {
    let dir = env::args().nth(1).unwrap_or(".".to_string());
    let mut all_advancements = get_all_advancements();
    let mut prev_adv = load_advancements(dir.as_str()).unwrap();
    let mut prev_stats = load_stats(dir.as_str()).unwrap();
    let mut logs: Vec<String> = read_logs();
    let mut latest = String::new();
    print_advancements(&prev_adv, &mut all_advancements);
    print_other(&prev_adv, &prev_stats);
    print_logs(&logs);
    
    loop {
        let adv = load_advancements(dir.as_str()).unwrap_or(prev_adv.clone());
        let stats = load_stats(dir.as_str()).unwrap_or(prev_stats.clone());
        if poll(std::time::Duration::from_millis(0)).unwrap() {
            let x = read().unwrap();
            // Add the message to the logs
            match x {
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Enter => {
                            println!("{}", latest);
                            if latest == "bolan".to_string() {
                                print_bolan();
                            } else if latest == "clear".to_string() {
                                logs.clear();
                            } else {
                                logs.push(latest.clone());
                            }
                            latest = String::new();
                            print_advancements(&adv, &mut all_advancements);
                            print_other(&adv, &stats);
                            print_logs(&logs);
                            write_logs(&logs);
                        },
                        KeyCode::Backspace => {
                            latest.pop();
                        },
                        KeyCode::Char(c) => {
                            latest.push(c);
                        },
                        _ => (),
                    }
                },
                _ => (),
            }
        }

        if adv != prev_adv || stats != prev_stats {
            prev_adv = adv.clone();
            prev_stats = stats.clone();
            print_advancements(&adv, &mut all_advancements);
            print_other(&adv, &stats);
            print_logs(&logs);
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
