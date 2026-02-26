use rand::distr::{weighted::WeightedIndex, Distribution};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Row, SqlitePool,
};
use std::{fs, path::PathBuf};

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

const APP_DIR_NAME: &str = "akc";
const DB_FILE_NAME: &str = "akc.db";
const MEMORY_NAMES_SEPARATOR: &str = "\n";

pub struct FriendInfo {
    name: String,
    chance: f64,
    level: String,
}

struct MemoryInfo {
    id: i64,
    kind: String,
    names: Vec<String>,
}

#[derive(Default)]
pub struct AkcConfig {
    friends: Vec<FriendInfo>,
}

fn db_path() -> PathBuf {
    if let Some(mut path) = dirs::config_dir() {
        path.push(APP_DIR_NAME);
        if fs::create_dir_all(&path).is_err() {
            return PathBuf::from(DB_FILE_NAME);
        }
        path.push(DB_FILE_NAME);
        path
    } else {
        PathBuf::from(DB_FILE_NAME)
    }
}

async fn open_pool() -> Result<SqlitePool, sqlx::Error> {
    let options = SqliteConnectOptions::new()
        .filename(db_path())
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await?;
    init_schema(&pool).await?;
    Ok(pool)
}

async fn init_schema(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS friends (
            name TEXT PRIMARY KEY,
            chance REAL NOT NULL,
            level TEXT NOT NULL
        )",
    )
    .execute(pool)
    .await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS memories (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            kind TEXT NOT NULL,
            names TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .execute(pool)
    .await?;
    Ok(())
}

async fn read_config() -> Result<AkcConfig, sqlx::Error> {
    let pool = open_pool().await?;
    let rows = sqlx::query("SELECT name, chance, level FROM friends")
        .fetch_all(&pool)
        .await?;
    let friends = rows
        .into_iter()
        .map(|row| FriendInfo {
            name: row.get("name"),
            chance: row.get("chance"),
            level: row.get("level"),
        })
        .collect();
    Ok(AkcConfig { friends })
}

async fn read_memories() -> Result<Vec<MemoryInfo>, sqlx::Error> {
    let pool = open_pool().await?;
    let rows = sqlx::query("SELECT id, kind, names FROM memories ORDER BY id ASC")
        .fetch_all(&pool)
        .await?;
    let memories = rows
        .into_iter()
        .map(|row| MemoryInfo {
            id: row.get("id"),
            kind: row.get("kind"),
            names: deserialize_memory_names(&row.get::<String, _>("names")),
        })
        .collect();
    Ok(memories)
}

async fn write_config(config: &AkcConfig) -> Result<(), sqlx::Error> {
    let pool = open_pool().await?;
    let mut transaction = pool.begin().await?;
    sqlx::query("DELETE FROM friends")
        .execute(&mut *transaction)
        .await?;

    for friend in &config.friends {
        sqlx::query("INSERT INTO friends (name, chance, level) VALUES (?1, ?2, ?3)")
            .bind(&friend.name)
            .bind(friend.chance)
            .bind(&friend.level)
            .execute(&mut *transaction)
            .await?;
    }

    transaction.commit().await?;
    Ok(())
}

fn get_unit_added_chance(total_reduction: f64, current_total_chance: f64) -> f64 {
    if current_total_chance <= f64::EPSILON {
        0.0
    } else {
        total_reduction / current_total_chance
    }
}

fn serialize_memory_names(names: &[String]) -> String {
    names.join(MEMORY_NAMES_SEPARATOR)
}

fn deserialize_memory_names(names: &str) -> Vec<String> {
    if names.is_empty() {
        return Vec::new();
    }
    names
        .split(MEMORY_NAMES_SEPARATOR)
        .map(ToOwned::to_owned)
        .collect()
}

fn reduction_from_kind(kind: &str) -> Option<f64> {
    match kind {
        "hangout" => Some(default_reduction::HANGOUT),
        "video-call" => Some(default_reduction::VIDEO_CALL),
        "call" => Some(default_reduction::CALL),
        "text" => Some(default_reduction::TEXT),
        _ => None,
    }
}

async fn save_memory(kind: &str, names: &[String]) -> Result<(), sqlx::Error> {
    let pool = open_pool().await?;
    sqlx::query("INSERT INTO memories (kind, names) VALUES (?1, ?2)")
        .bind(kind)
        .bind(serialize_memory_names(names))
        .execute(&pool)
        .await?;
    Ok(())
}

async fn delete_memory(id: i64) -> Result<bool, sqlx::Error> {
    let pool = open_pool().await?;
    let result = sqlx::query("DELETE FROM memories WHERE id = ?1")
        .bind(id)
        .execute(&pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

fn reset_chances_to_level_defaults(config: &mut AkcConfig) {
    for friend in &mut config.friends {
        friend.chance = level_default_chance(&friend.level).unwrap_or(0.0);
    }
}

fn apply_memory_to_config(config: &mut AkcConfig, kind: &str, names: &[String]) {
    let Some(reduction) = reduction_from_kind(kind) else {
        return;
    };
    if names.is_empty() {
        return;
    }
    let unknown_names = utils::get_unknown_names(config, names);
    if !unknown_names.is_empty() {
        return;
    }

    let total_reduction = reduction * names.len() as f64;
    let current_total_chance = utils::get_config_total_chance(config, names);
    let unit_added_chance = get_unit_added_chance(total_reduction, current_total_chance);
    utils::increase_chances_by_unit(config, unit_added_chance, names);
    utils::decrease_chances_by_reduction(config, reduction, names);
}

async fn rebuild_chances_from_memories() -> Result<(), sqlx::Error> {
    let mut config = read_config().await?;
    let memories = read_memories().await?;
    reset_chances_to_level_defaults(&mut config);

    for memory in memories {
        apply_memory_to_config(&mut config, &memory.kind, &memory.names);
    }

    write_config(&config).await
}

fn level_default_chance(level: &str) -> Option<f64> {
    match level {
        "aji" => Some(default_chance::AJI),
        "ki" => Some(default_chance::KI),
        "chi" => Some(default_chance::CHI),
        _ => None,
    }
}

fn make_friend(name: String, level: &str) -> Option<FriendInfo> {
    level_default_chance(level).map(|chance| FriendInfo {
        name,
        chance,
        level: level.to_owned(),
    })
}

async fn add_friend(friend_info: FriendInfo) {
    let mut config = match read_config().await {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Failed to read data: {err}");
            return;
        }
    };
    let is_duplicate = utils::is_name_duplicate(&config, &friend_info.name);

    if is_duplicate {
        println!(
            "Name \"{}\" already exists, please use a different name",
            friend_info.name
        );
    } else {
        config.friends.push(friend_info);
        if let Err(err) = write_config(&config).await {
            eprintln!("Failed to write data: {err}");
        }
    }
}

pub async fn remove_friend(name: String) {
    let mut config = match read_config().await {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Failed to read data: {err}");
            return;
        }
    };

    let old_len = config.friends.len();
    config.friends.retain(|friend| friend.name != name);
    if old_len == config.friends.len() {
        println!("Name \"{name}\" not found");
        return;
    }

    if let Err(err) = write_config(&config).await {
        eprintln!("Failed to write data: {err}");
    }
}

pub async fn edit_friend(name: String, new_name: Option<String>, new_level: Option<String>) {
    if new_name.is_none() && new_level.is_none() {
        println!("No changes requested");
        return;
    }

    let mut config = match read_config().await {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Failed to read data: {err}");
            return;
        }
    };

    let Some(index) = config.friends.iter().position(|friend| friend.name == name) else {
        println!("Name \"{name}\" not found");
        return;
    };

    if let Some(ref new_name_value) = new_name {
        let name_taken = config
            .friends
            .iter()
            .any(|friend| friend.name == *new_name_value && friend.name != name);
        if name_taken {
            println!("Name \"{new_name_value}\" already exists, please use a different name");
            return;
        }
        config.friends[index].name = new_name_value.clone();
    }

    if let Some(new_level_value) = new_level {
        if let Some(default_level_chance) = level_default_chance(&new_level_value) {
            config.friends[index].level = new_level_value;
            config.friends[index].chance = default_level_chance;
        } else {
            println!("Invalid level");
            return;
        }
    }

    if let Err(err) = write_config(&config).await {
        eprintln!("Failed to write data: {err}");
    }
}

pub async fn add_aji(name: String) {
    if let Some(friend_info) = make_friend(name, "aji") {
        add_friend(friend_info).await;
    }
}

pub async fn add_ki(name: String) {
    if let Some(friend_info) = make_friend(name, "ki") {
        add_friend(friend_info).await;
    }
}

pub async fn add_chi(name: String) {
    if let Some(friend_info) = make_friend(name, "chi") {
        add_friend(friend_info).await;
    }
}

pub async fn add_many_friends(level: String, names: Vec<String>) {
    if names.is_empty() {
        println!("Please specify at least one name");
        return;
    }

    let mut config = match read_config().await {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Failed to read data: {err}");
            return;
        }
    };

    let mut duplicate_names = Vec::new();
    let mut added_count = 0;

    for name in names {
        if utils::is_name_duplicate(&config, &name) {
            duplicate_names.push(name);
            continue;
        }

        if let Some(friend_info) = make_friend(name, &level) {
            config.friends.push(friend_info);
            added_count += 1;
        }
    }

    if added_count > 0 {
        if let Err(err) = write_config(&config).await {
            eprintln!("Failed to write data: {err}");
            return;
        }
    }

    if !duplicate_names.is_empty() {
        println!(
            "Skipped existing names: {}",
            duplicate_names.join(", ")
        );
    }
}

pub async fn list_friends(level_filter: Option<String>, sort_chance: bool) {
    let config = match read_config().await {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Failed to read data: {err}");
            return;
        }
    };
    let mut filtered_config = config;
    if let Some(level_filter) = level_filter {
        filtered_config.friends.retain(|friend| friend.level == level_filter);
    }

    let rendered_list = if sort_chance {
        utils::list_friends_by_chance(&filtered_config)
    } else {
        utils::list_friends(&filtered_config)
    };
    println!("{rendered_list}");
}

pub async fn search_friends(query: String) {
    let config = match read_config().await {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Failed to read data: {err}");
            return;
        }
    };

    println!("{}", utils::search_friends(&config, &query));
}

pub async fn suggest() {
    let config = match read_config().await {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Failed to read data: {err}");
            return;
        }
    };
    let filtered_config = utils::filter_config_by_enough_chance(&config);
    if filtered_config.is_empty() {
        println!("No friend to suggest");
        return;
    }

    let mut rng = rand::rng();
    let weighted_dist = match WeightedIndex::new(filtered_config.iter().map(|friend| friend.chance))
    {
        Ok(weighted_dist) => weighted_dist,
        Err(err) => {
            eprintln!("Failed to suggest a friend: {err}");
            return;
        }
    };
    let suggested_friend = filtered_config[weighted_dist.sample(&mut rng)];
    println!("Suggested friend: {}", suggested_friend.name);
}

async fn add_memory(kind: &str, reduction: f64, names: &[String]) {
    if names.is_empty() {
        println!("Please specify at least one name");
        return;
    }

    let mut config = match read_config().await {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Failed to read data: {err}");
            return;
        }
    };
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
        let unit_added_chance = get_unit_added_chance(total_reduction, current_total_chance);

        utils::increase_chances_by_unit(&mut config, unit_added_chance, names);
        utils::decrease_chances_by_reduction(&mut config, reduction, names);

        if let Err(err) = write_config(&config).await {
            eprintln!("Failed to write data: {err}");
            return;
        }

        if let Err(err) = save_memory(kind, names).await {
            eprintln!("Failed to save memory: {err}");
        }
    }
}

pub async fn add_hangout(names: &[String]) {
    add_memory("hangout", default_reduction::HANGOUT, names).await
}

pub async fn add_video_call(names: &[String]) {
    add_memory("video-call", default_reduction::VIDEO_CALL, names).await
}

pub async fn add_call(names: &[String]) {
    add_memory("call", default_reduction::CALL, names).await
}

pub async fn add_text(names: &[String]) {
    add_memory("text", default_reduction::TEXT, names).await
}

pub async fn undo_memory() {
    let memories = match read_memories().await {
        Ok(memories) => memories,
        Err(err) => {
            eprintln!("Failed to read memories: {err}");
            return;
        }
    };
    let Some(last_memory) = memories.last() else {
        println!("No memory to undo");
        return;
    };

    match delete_memory(last_memory.id).await {
        Ok(true) => {}
        Ok(false) => {
            println!("No memory to undo");
            return;
        }
        Err(err) => {
            eprintln!("Failed to delete memory: {err}");
            return;
        }
    }

    if let Err(err) = rebuild_chances_from_memories().await {
        eprintln!("Failed to rebuild chances: {err}");
    }
}

#[cfg(test)]
mod test {
    use super::{deserialize_memory_names, get_unit_added_chance, serialize_memory_names};

    #[test]
    fn test_get_unit_added_chance_when_total_is_zero() {
        assert_eq!(get_unit_added_chance(1.0, 0.0), 0.0);
    }

    #[test]
    fn test_get_unit_added_chance_when_total_is_positive() {
        assert_eq!(get_unit_added_chance(1.0, 2.0), 0.5);
    }

    #[test]
    fn test_serialize_memory_names() {
        let names = vec!["A".to_owned(), "B".to_owned()];
        assert_eq!(serialize_memory_names(&names), "A\nB");
    }

    #[test]
    fn test_deserialize_memory_names() {
        assert_eq!(
            deserialize_memory_names("A\nB"),
            vec!["A".to_owned(), "B".to_owned()]
        );
    }
}
