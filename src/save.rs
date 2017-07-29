
#[derive(Serialize, Deserialize)]
pub struct Settings {
    feed_scale: u8,
    hunger_scale: u8,
    heal_scale: u8,
    damage_scale: u8,
    clean_scale: u8,
    dirty_scale: u8,
    play_scale: u8,
    bore_scale: u8,

    hunger_damage: u8,
    hunger_sicken: u8,
    cleanliness_sicken: u8,
    health_kill: u8,
    age_kill: u8,

    health_message: u8,
    hunger_message: u8,
    bore_message: u8,
    clean_message: u8,
}

// TODO: fill, add ser/de
pub struct Savefile {
}

