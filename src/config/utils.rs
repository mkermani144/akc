use super::{AkcConfig, FriendInfo, default_chance, default_reduction};

pub fn is_name_duplicate(config: &AkcConfig, name: &str) -> bool {
  config.friends.iter().any(| friend_info | friend_info.name == name)
}

pub fn filter_config_by_enough_chance(config: &AkcConfig) -> Vec<&FriendInfo> {
  config.friends.iter().filter(| friend_info | friend_info.chance >= default_reduction::TEXT).collect()
}

pub fn get_unknown_names<'a>(config: &AkcConfig, names: &'a[String]) -> Vec<&'a String> {
  let all_names = config.friends.iter().map(| friend_info | friend_info.name.clone()).collect::<Vec<String>>();
  names.iter().filter(| name | !all_names.iter().any(| inner_name | inner_name == *name )).collect::<Vec<&String>>()
}

pub fn get_config_total_chance(config: &AkcConfig, excluded_names: &[String]) -> f64 {
  config.friends.iter()
    .filter(| friend_info | !excluded_names.contains(&friend_info.name))
    .map(| friend_info | friend_info.chance)
    .sum::<f64>()
}

pub fn increase_chances_by_unit(config: &mut AkcConfig, unit_added_chance: f64, excluded_names: &[String]) {
  config.friends.iter_mut()
    .filter(| friend_info | !excluded_names.contains(&friend_info.name))
    .for_each(| friend_info | {
        let level_chance = match friend_info.level.as_str() {
            "aji" => default_chance::AJI,
            "ki" => default_chance::KI,
            "chi" => default_chance::CHI,
            _ => 0.0
        };
        friend_info.chance += level_chance * unit_added_chance;
    })
}

pub fn decrease_chances_by_reduction(config: &mut AkcConfig, reduction: f64, names: &[String]) {
  config.friends.iter_mut()
    .filter(| friend_info | names.contains(&friend_info.name))
    .for_each(| friend_info | {
        friend_info.chance -= reduction;
    })
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_is_name_duplicate() {
    let config = AkcConfig {
      friends: vec![
        FriendInfo {
          name: "John".to_owned(),
          chance: default_chance::AJI,
          level: "aji".to_owned()
        },
        FriendInfo {
          name: "Doe".to_owned(),
          chance: default_chance::KI,
          level: "ki".to_owned()
        }
      ]
    };

    assert!(is_name_duplicate(&config, "John"));
    assert!(is_name_duplicate(&config, "Doe"));
    assert!(!is_name_duplicate(&config, "John Doe"));
  }

  #[test]
  fn test_filter_config_by_enough_chance() {
    let config = AkcConfig {
      friends: vec![
        FriendInfo {
          name: "John".to_owned(),
          chance: default_chance::AJI,
          level: "aji".to_owned()
        },
        FriendInfo {
          name: "Doe".to_owned(),
          chance: default_reduction::TEXT,
          level: "ki".to_owned()
        },
        FriendInfo {
          name: "Jane".to_owned(),
          chance: 0.0,
          level: "chi".to_owned()
        }
      ]
    };

    let filtered_config = filter_config_by_enough_chance(&config);
    assert_eq!(filtered_config.len(), 2);
  }

  #[test]
  fn test_get_unknown_names() {
    let config = AkcConfig {
      friends: vec![
        FriendInfo {
          name: "John".to_owned(),
          chance: default_chance::AJI,
          level: "aji".to_owned()
        },
        FriendInfo {
          name: "Doe".to_owned(),
          chance: default_chance::KI,
          level: "ki".to_owned()
        }
      ]
    };

    let names = vec![
      "John".to_owned(),
      "Doe".to_owned(),
      "Jane".to_owned()
    ];
    let unknown_names = get_unknown_names(&config, &names);

    assert_eq!(unknown_names.len(), 1);
    assert_eq!(unknown_names[0], "Jane");
  }

  #[test]
  fn test_get_config_total_chance() {
    let config = AkcConfig {
      friends: vec![
        FriendInfo {
          name: "John".to_owned(),
          chance: default_chance::AJI,
          level: "aji".to_owned()
        },
        FriendInfo {
          name: "Doe".to_owned(),
          chance: default_chance::KI,
          level: "ki".to_owned()
        },
        FriendInfo {
          name: "Doe2".to_owned(),
          chance: default_chance::CHI,
          level: "chi".to_owned()
        },
      ]
    };

    let total_chance = get_config_total_chance(&config, &["Doe".to_owned()]);
    assert_eq!(total_chance, default_chance::AJI + default_chance::CHI);
  }

  #[test]
  fn test_increase_chances_by_unit() {
    let mut config = AkcConfig {
      friends: vec![
        FriendInfo {
          name: "John".to_owned(),
          chance: default_chance::AJI,
          level: "aji".to_owned()
        },
        FriendInfo {
          name: "Doe".to_owned(),
          chance: default_chance::KI,
          level: "ki".to_owned()
        },
        FriendInfo {
          name: "Doe2".to_owned(),
          chance: default_chance::CHI,
          level: "chi".to_owned()
        },
      ]
    };

    increase_chances_by_unit(&mut config, 0.1, &["Doe".to_owned()]);
    // println("{}", config.friends[0].chance);
    assert_eq!(config.friends[0].chance, default_chance::AJI + 0.1 * default_chance::AJI);
    assert_eq!(config.friends[1].chance, default_chance::KI);
    assert_eq!(config.friends[2].chance, default_chance::CHI + 0.1 * default_chance::CHI);
  }

  #[test]
  fn test_decrease_chances_by_reduction() {
    let mut config = AkcConfig {
      friends: vec![
        FriendInfo {
          name: "John".to_owned(),
          chance: default_chance::AJI,
          level: "aji".to_owned()
        },
        FriendInfo {
          name: "Doe".to_owned(),
          chance: default_chance::KI,
          level: "ki".to_owned()
        },
        FriendInfo {
          name: "Doe2".to_owned(),
          chance: default_chance::CHI,
          level: "chi".to_owned()
        },
      ]
    };

    decrease_chances_by_reduction(&mut config, 1.0, &["Doe".to_owned()]);
    assert_eq!(config.friends[0].chance, default_chance::AJI);
    assert_eq!(config.friends[1].chance, default_chance::KI - 1.0);
    assert_eq!(config.friends[2].chance, default_chance::CHI);
  }
}