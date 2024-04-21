use serde_json::{Error, Value};
use std::fs;

pub fn load_stats(dir: &str) -> Result<Value, Error> {
    let dir = format!("{}/stats", dir);
    let file = fs::read_dir(dir).unwrap().next().unwrap().unwrap().path();
    let contents = fs::read_to_string(file).unwrap();
    let json = serde_json::from_str(&contents);
    json
}

fn picked_up(stats: &Value, item: &str) -> usize {
    let picked_up = match stats.get("stats") {
        Some(stats) => stats.get("minecraft:picked_up"),
        None => return 0,
    };
    let item = match picked_up {
        Some(picked_up) => picked_up.get(item),
        None => return 0,
    };
    match item {
        Some(item) => item.as_u64().unwrap() as usize,
        None => 0,
    }
}

fn killed(stats: &Value, entity: &str) -> usize {
    let killed = match stats.get("stats") {
        Some(stats) => stats.get("minecraft:killed"),
        None => return 0,
    };
    let entity = match killed {
        Some(killed) => killed.get(entity),
        None => return 0,
    };
    match entity {
        Some(entity) => entity.as_u64().unwrap() as usize,
        None => 0,
    }
}

pub fn get_wither_heads(stats: &Value) -> usize {
    picked_up(stats, "minecraft:wither_skeleton_skull")
}

pub fn get_shells(stats: &Value) -> usize {
    picked_up(stats, "minecraft:nautilus_shell")
}

pub fn get_beehives(stats: &Value) -> usize {
    picked_up(stats, "minecraft:bee_nest")
}

pub fn get_tridents(stats: &Value) -> usize {
    picked_up(stats, "minecraft:trident")
}

pub fn get_gold_blocks(stats: &Value) -> usize {
    picked_up(stats, "minecraft:gold_block")
}

pub fn get_wither_skeletons_killed(stats: &Value) -> usize {
    killed(stats, "minecraft:wither_skeleton")
}

pub fn get_drowned_killed(stats: &Value) -> usize {
    killed(stats, "minecraft:drowned")
}