use core::panic;
use serde::{Deserialize, Serialize};
use rand::prelude::*;
use rand::distributions::WeightedIndex;
use confy::{load, store};

mod default_chance {
    pub const AJI: f64 = 50.0;
    pub const KI: f64 = 5.0;
    pub const CHI: f64 = 1.0;
}

mod default_reduction {
    pub const HANGOUT: f64 = 2.0;
    pub const VIDEO_CALL: f64 = 1.0;
    pub const CALL: f64 = 0.5;
    pub const TEXT: f64 = 0.25;
}

const CONFIG_NAME: &str = "akc";


#[derive(Serialize, Deserialize)]
pub struct FriendInfo {
    name: String,
    chance: f64,
    level: String
}

#[derive(Serialize, Deserialize, Default)]
pub struct AkcConfig {
    friends: Vec<FriendInfo>
}

pub fn read_config() -> AkcConfig {
    load(CONFIG_NAME).unwrap_or_else(|_| AkcConfig::default())
}

fn write_config(config: AkcConfig) {
    store(CONFIG_NAME, &config).unwrap_or_else(|_| panic!("Failed to write config file"));
}

fn is_name_duplicate(name: &str) -> bool {
    let config = read_config();
    config.friends.iter().any(| friend_info | friend_info.name == name)
}

fn add_friend(friend_info: FriendInfo) {
    let mut config = read_config();
    let is_duplicate = is_name_duplicate(&friend_info.name);

    if is_duplicate {
        println!("Name \"{}\" already exists, please use a different name", friend_info.name)
    } else {
        config.friends.push(friend_info);
        write_config(config);
    }
}

pub fn add_aji(name: String) {
    add_friend(FriendInfo {
        name,
        chance: default_chance::AJI,
        level: "aji".to_owned()
    })
}

pub fn add_ki(name: String) {
    add_friend(FriendInfo {
        name,
        chance: default_chance::KI,
        level: "ki".to_owned()
    })
}

pub fn add_chi(name: String) {
    add_friend(FriendInfo {
        name,
        chance: default_chance::CHI,
        level: "chi".to_owned()
    })
}

pub fn suggest() {
    let config = read_config();
    let filtered_config: Vec<&FriendInfo> = config.friends.iter().filter(| friend_info | friend_info.chance > default_reduction::TEXT ).collect();
    let mut rng = thread_rng();

    let weighted_dist = WeightedIndex::new(filtered_config.iter().map(| friend_info | friend_info.chance)).expect("Failed to suggest a friend");
    println!("Suggested friend: {}", config.friends[weighted_dist.sample(&mut rng)].name);
}

fn add_memory(reduction: f64, names: Vec<String>) {
    let mut config = read_config();
    let all_names = config.friends.iter().map(| friend_info | friend_info.name.clone()).collect::<Vec<String>>();
    let unknown_names = names.iter().filter(| name | !all_names.iter().any(| inner_name | inner_name == *name )).collect::<Vec<&String>>();

    if !unknown_names.is_empty() {
        let unknown_names_string = unknown_names.iter()
            .map(| &unknown_name | unknown_name.to_owned())
            .collect::<Vec<String>>()
            .join(", ");
        println!("The following names are not added yet: {}", unknown_names_string);
    } else {
        let total_reduction = reduction * names.len() as f64;
        let current_total_chance = config.friends.iter()
            .filter(| friend_info | !names.contains(&friend_info.name))
            .map(| friend_info | friend_info.chance)
            .sum::<f64>();
        let unit_added_chance = total_reduction / current_total_chance;

        config.friends.iter_mut()
            .filter(| friend_info | !names.contains(&friend_info.name))
            .for_each(| friend_info | {
                let level_chance = match friend_info.level.as_str() {
                    "aji" => default_chance::AJI,
                    "ki" => default_chance::KI,
                    "chi" => default_chance::CHI,
                    _ => 0.0
                };
                friend_info.chance += level_chance * unit_added_chance;
            });

        config.friends.iter_mut()
            .filter(| friend_info | names.contains(&friend_info.name))
            .for_each(| friend_info | {
                friend_info.chance -= reduction;
            });

        write_config(config)
    }
}

pub fn add_hangout(names: Vec<String>) {
    add_memory(default_reduction::HANGOUT, names)
}

pub fn add_video_call(names: Vec<String>) {
    add_memory(default_reduction::VIDEO_CALL, names)
}

pub fn add_call(names: Vec<String>) {
    add_memory(default_reduction::CALL, names)
}

pub fn add_text(names: Vec<String>) {
    add_memory(default_reduction::TEXT, names)
}
