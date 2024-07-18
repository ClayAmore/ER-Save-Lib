use std::{num::ParseIntError, path::Path};

use crate::{save::save::SaveParseError, Save};

use super::event_flags::EventFlagsApi;

#[derive(thiserror::Error, Debug)]
pub enum SaveApiError {
    #[error(transparent)]
    DekuError(#[from] deku::DekuError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    SaveParserError(#[from] SaveParseError),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error("EventId not found!")]
    EventIdNotFound,
}

#[derive(PartialEq)]
pub enum SaveType {
    PC,
    Playstation,
}

pub struct SaveApi {
    raw: Save,
}

impl SaveApi {
    pub fn new(save: Save) -> Self {
        SaveApi { raw: save }
    }

    pub fn to_vec(&self) -> Result<Vec<u8>, SaveApiError> {
        let bytes = self.raw.write_to_vec()?;
        Ok(bytes)
    }

    pub fn write_to_path(&self, path: impl AsRef<Path>) -> Result<(), SaveApiError> {
        Ok(self.raw.write_to_path(path)?)
    }

    pub fn from_slice(bytes: &[u8]) -> Result<Self, SaveApiError> {
        let raw = Save::from_slice(bytes)?;
        Ok(SaveApi { raw })
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, SaveApiError> {
        let raw = Save::from_path(path)?;
        return Ok(SaveApi { raw });
    }

    pub fn platform(&self) -> SaveType {
        if self.raw.header.len() == 0x6c {
            SaveType::Playstation
        } else {
            SaveType::PC
        }
    }

    pub fn character_index_from_name(&self, name: &str) -> Option<usize> {
        self.raw
            .user_data_10
            .profile_summary
            .profiles
            .iter()
            .position(|profile| profile.character_name.contains(name))
    }

    pub fn get_event_flag(
        &self,
        event_id: u32,
        character_index: usize,
    ) -> Result<bool, SaveApiError> {
        EventFlagsApi::get_event_flag(&self.raw, event_id, character_index)
    }

    pub fn set_event_flag(
        &mut self,
        event_id: u32,
        character_index: usize,
        on: bool,
    ) -> Result<(), SaveApiError> {
        EventFlagsApi::set_event_flag(&mut self.raw, event_id, character_index, on)
    }

    pub fn steam_id(&self) -> u64 {
        self.raw.user_data_10.steam_id
    }

    pub fn set_steam_id(&mut self, steam_id: u64) -> Result<(), SaveApiError> {
        for file in &mut self.raw.user_data_x {
            file.steam_id = steam_id;
        }
        self.raw.user_data_10.steam_id = steam_id;
        Ok(())
    }

    pub fn character_name(&self, index: usize) -> String {
        self.raw.user_data_x[index]
            .player_game_data
            .character_name
            .to_string()
    }

    pub fn set_character_name(&mut self, index: usize, new_name: &str) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.character_name = new_name.to_string();
        self.raw.user_data_10.profile_summary.profiles[index].character_name = new_name.to_string();
        Ok(())
    }

    pub fn active_characters(&self) -> [bool; 10] {
        self.raw.user_data_10.profile_summary.active_profiles
    }

    pub fn gender(&self, index: usize) -> u8 {
        self.raw.user_data_x[index].player_game_data.gender
    }

    pub fn set_gender(&mut self, index: usize, gender: u8) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.gender = gender;
        self.raw.user_data_10.profile_summary.profiles[index].gender = gender;
        Ok(())
    }

    pub fn archetype(&self, index: usize) -> u8 {
        self.raw.user_data_x[index].player_game_data.archetype
    }

    pub fn set_archetype(&mut self, index: usize, archetype: u8) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.archetype = archetype;
        self.raw.user_data_10.profile_summary.profiles[index].archetype = archetype;
        Ok(())
    }

    pub fn hp(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.hp
    }

    pub fn set_hp(&mut self, index: usize, hp: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.hp = hp;
        Ok(())
    }

    pub fn max_hp(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.max_hp
    }

    pub fn set_max_hp(&mut self, index: usize, max_hp: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.max_hp = max_hp;
        Ok(())
    }

    pub fn base_max_hp(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.base_max_hp
    }

    pub fn set_base_max_hp(&mut self, index: usize, base_max_hp: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.base_max_hp = base_max_hp;
        Ok(())
    }

    pub fn set_fp(&mut self, index: usize, fp: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.fp = fp;
        Ok(())
    }

    pub fn max_fp(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.max_fp
    }

    pub fn set_max_fp(&mut self, index: usize, max_fp: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.max_fp = max_fp;
        Ok(())
    }

    pub fn base_max_fp(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.base_max_fp
    }

    pub fn set_base_max_fp(&mut self, index: usize, base_max_fp: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.base_max_fp = base_max_fp;
        Ok(())
    }

    pub fn set_sp(&mut self, index: usize, sp: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.sp = sp;
        Ok(())
    }

    pub fn max_sp(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.max_sp
    }

    pub fn set_max_sp(&mut self, index: usize, max_sp: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.max_sp = max_sp;
        Ok(())
    }

    pub fn base_max_sp(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.base_max_sp
    }

    pub fn set_base_max_sp(&mut self, index: usize, base_max_sp: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.base_max_sp = base_max_sp;
        Ok(())
    }

    pub fn level(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.level
    }

    pub fn set_level(&mut self, index: usize, level: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.level = level;
        self.raw.user_data_10.profile_summary.profiles[index].level = level;
        Ok(())
    }

    pub fn vigor(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.vigor
    }

    pub fn set_vigor(&mut self, index: usize, vigor: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.vigor = vigor;
        Ok(())
    }

    pub fn mind(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.mind
    }

    pub fn set_mind(&mut self, index: usize, mind: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.mind = mind;
        Ok(())
    }

    pub fn endurance(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.endurance
    }

    pub fn set_endurance(&mut self, index: usize, endurance: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.endurance = endurance;
        Ok(())
    }

    pub fn strength(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.strength
    }

    pub fn set_strength(&mut self, index: usize, strength: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.strength = strength;
        Ok(())
    }

    pub fn dexterity(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.dexterity
    }

    pub fn set_dexterity(&mut self, index: usize, dexterity: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.dexterity = dexterity;
        Ok(())
    }

    pub fn intelligence(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.intelligence
    }

    pub fn set_intelligence(
        &mut self,
        index: usize,
        intelligence: u32,
    ) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.intelligence = intelligence;
        Ok(())
    }

    pub fn faith(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.faith
    }

    pub fn set_faith(&mut self, index: usize, faith: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.faith = faith;
        Ok(())
    }

    pub fn arcane(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.arcane
    }

    pub fn set_arcane(&mut self, index: usize, arcane: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.arcane = arcane;
        Ok(())
    }

    pub fn runes(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.runes
    }

    pub fn set_runes(&mut self, index: usize, runes: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.runes = runes;
        Ok(())
    }

    pub fn runes_memory(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.runes_memory
    }

    pub fn set_runes_memory(
        &mut self,
        index: usize,
        runes_memory: u32,
    ) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.runes_memory = runes_memory;
        self.raw.user_data_10.profile_summary.profiles[index].runes_memory = runes_memory;
        Ok(())
    }
}
