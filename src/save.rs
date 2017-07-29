
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Settings {
    pub feed_scale: u8,
    pub hunger_scale: u8,
    pub heal_scale: u8,
    pub damage_scale: u8,
    pub clean_scale: u8,
    pub dirty_scale: u8,
    pub play_scale: u8,
    pub bore_scale: u8,

    pub hunger_damage: u8,
    pub hunger_sicken: u8,
    pub cleanliness_sicken: u8,
    pub health_kill: u8,
    pub age_kill: u8,

    pub health_message: u8,
    pub hunger_message: u8,
    pub bore_message: u8,
    pub clean_message: u8,
}

// TODO: fill, add ser/de
pub struct Savefile {
}

