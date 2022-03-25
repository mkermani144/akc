use core::panic;
use std::path::PathBuf;
use std::{io::ErrorKind};
use std::fs;
use serde::{Deserialize, Serialize};

use dirs::config_dir;

mod default_chance {
    pub const AJI: f64 = 50.;
    pub const KI: f64 = 5.;
    pub const CHI: f64 = 1.;
}

const CONFIG_FILE_NAME: &str = "akc.json";


#[derive(Serialize, Deserialize, Debug)]
pub struct FriendInfo {
    name: String,
    chance: f64,
    level: String
}

fn get_config_path() -> PathBuf {
    config_dir().unwrap_or_else(|| std::path::PathBuf::from(".")).join(CONFIG_FILE_NAME)
}

pub fn read_config() -> Vec<FriendInfo> {
    let config_path = get_config_path();
    let raw_config = fs::read_to_string(&config_path).unwrap_or_else(| error | {
        if error.kind() == ErrorKind::NotFound {
            return "[]".to_owned();
        }
        panic!("Failed to read from config file")
    });
    let config: Vec<FriendInfo> = serde_json::from_str(&raw_config).expect("Invalid config file");
    config
}

fn write_config(config: Vec<FriendInfo>) {
    let config_path = get_config_path();
    let config_string = serde_json::to_string(&config).expect("Failed to serialize friend");
    fs::write(&config_path, &config_string).unwrap_or_else(| error | {
        if error.kind() == ErrorKind::NotFound {
            fs::File::create(&config_path).expect("Failed to create config file");
            fs::write(&config_path, &config_string).expect("Failed to write to config file");
        } else {
            panic!("Failed to write to config file")
        }
    });
}

fn add_friend(friend_info: FriendInfo) {
    let mut config = read_config();
    let is_duplicate = config.iter().any(| friend_info_inner | friend_info_inner.name == friend_info.name);

    if is_duplicate {
        println!("Name \"{}\" already exists, please use a different name", friend_info.name)
    } else {
        config.push(friend_info);
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