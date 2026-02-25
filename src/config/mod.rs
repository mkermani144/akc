use confy::{load, store};
use rand::distr::{weighted::WeightedIndex, Distribution};
use serde::{Deserialize, Serialize};

mod utils;

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
    level: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct AkcConfig {
    friends: Vec<FriendInfo>,
}

pub fn read_config() -> AkcConfig {
    load(CONFIG_NAME, None).unwrap_or_else(|_| AkcConfig::default())
}

fn write_config(config: AkcConfig) {
    store(CONFIG_NAME, None, &config).unwrap_or_else(|_| panic!("Failed to write config file"));
}

fn add_friend(friend_info: FriendInfo) {
    let mut config = read_config();
    let is_duplicate = utils::is_name_duplicate(&config, &friend_info.name);

    if is_duplicate {
        println!(
            "Name \"{}\" already exists, please use a different name",
            friend_info.name
        )
    } else {
        config.friends.push(friend_info);
        write_config(config);
    }
}

pub fn add_aji(name: String) {
    add_friend(FriendInfo {
        name,
        chance: default_chance::AJI,
        level: "aji".to_owned(),
    })
}

pub fn add_ki(name: String) {
    add_friend(FriendInfo {
        name,
        chance: default_chance::KI,
        level: "ki".to_owned(),
    })
}

pub fn add_chi(name: String) {
    add_friend(FriendInfo {
        name,
        chance: default_chance::CHI,
        level: "chi".to_owned(),
    })
}

pub fn list_friends() {
    let config = read_config();
    println!("{}", utils::list_friends(&config));
}

pub fn suggest() {
    let config = read_config();
    let filtered_config = utils::filter_config_by_enough_chance(&config);
    let mut rng = rand::rng();

    let weighted_dist =
        WeightedIndex::new(filtered_config.iter().map(|friend_info| friend_info.chance))
            .expect("Failed to suggest a friend");
    println!(
        "Suggested friend: {}",
        config.friends[weighted_dist.sample(&mut rng)].name
    );
}

fn add_memory(reduction: f64, names: &[String]) {
    if names.is_empty() {
        println!("Please specify at least one name");
        return;
    }
    let mut config = read_config();
    let unknown_names = utils::get_unknown_names(&config, names);

    if !unknown_names.is_empty() {
        let unknown_names_string = unknown_names
            .iter()
            .map(|&unknown_name| unknown_name.to_owned())
            .collect::<Vec<String>>()
            .join(", ");
        println!(
            "The following names are not added yet: {}",
            unknown_names_string
        );
    } else {
        let total_reduction = reduction * names.len() as f64;
        let current_total_chance = utils::get_config_total_chance(&config, names);
        let unit_added_chance = total_reduction / current_total_chance;

        utils::increase_chances_by_unit(&mut config, unit_added_chance, names);
        utils::decrease_chances_by_reduction(&mut config, reduction, names);

        write_config(config)
    }
}

pub fn add_hangout(names: &[String]) {
    add_memory(default_reduction::HANGOUT, names)
}

pub fn add_video_call(names: &[String]) {
    add_memory(default_reduction::VIDEO_CALL, names)
}

pub fn add_call(names: &[String]) {
    add_memory(default_reduction::CALL, names)
}

pub fn add_text(names: &[String]) {
    add_memory(default_reduction::TEXT, names)
}
