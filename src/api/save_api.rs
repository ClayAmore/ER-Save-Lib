use std::{
    collections::{BTreeMap, HashMap},
    num::ParseIntError,
    path::Path,
    sync::LazyLock,
};

use crate::{
    regulation::{params::param_structs, regulation::RegulationParseError},
    save::{save::SaveParseError, user_data_10::Profile, user_data_x::UserDataX},
    Save,
};

use super::{
    event_flags::EventFlagsApi,
    inventory::{Item, ItemType, StorageItemType, StorageType, KEY_ITEM_CAPACITY},
    text::{
        accessory_name::ACCESSORY_NAME, accessory_name_dlc01::ACCESSORY_NAME_DLC01,
        arts_name::ARTS_NAME, arts_name_dlc01::ARTS_NAME_DLC01, goods_name::GOODS_NAME,
        goods_name_dlc01::GOODS_NAME_DLC01, protector_name::PROTECTOR_NAME,
        protector_name_dlc01::PROTECTOR_NAME_DLC01, weapon_name::WEAPON_NAME,
        weapon_name_dlc01::WEAPON_NAME_DLC01,
    },
    vanilla_check::VanillaCheck,
};

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
    #[error("EventId {} not found!", .0)]
    EventIdNotFound(u32),
    #[error(transparent)]
    RegulationParseError(#[from] RegulationParseError),
    #[error("Character index is out of range for the limit is 10 and the index is {}.", .0)]
    CharacterIndexOutOfRange(usize),
    #[error("Failed to determine item type from id: {:#X}", .0)]
    UnkownItemType(u32),
    #[error("Failed to find item id from gaitem handle: {:#X}", .0)]
    ItemIdNotFound(u32),
    #[error("Failed to import character to a playstation save!\nReason: {}", .0)]
    CheatedCharacterImport(String),
}

#[derive(PartialEq)]
pub enum SaveType {
    PC,
    Playstation,
}

#[derive(Default)]
pub struct Param<P: param_structs::param_trait::Param> {
    pub rows: HashMap<i32, P::ParamType>,
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

    pub fn get_param_bytes_map(&self) -> Result<&BTreeMap<String, Vec<u8>>, SaveApiError> {
        Ok(&self
            .raw
            .user_data_11
            .regulation
            .content
            .data
            .file_data
            .param_files)
    }

    pub fn get_param<P: param_structs::param_trait::Param>(
        &self,
    ) -> Result<Param<P>, SaveApiError> {
        let rows = self.raw.user_data_11.regulation.get_param::<P>()?;
        Ok(Param::<P> { rows })
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

    pub fn import_character(
        &mut self,
        to_index: usize,
        from: &SaveApi,
        from_index: usize,
    ) -> Result<(), SaveApiError> {
        // Validate character
        if self.platform() == SaveType::Playstation {
            VanillaCheck::is_vanilla_save(&from.raw, from_index)?;
        }

        // Validate index is within range
        if to_index > 9 {
            return Err(SaveApiError::CharacterIndexOutOfRange(to_index));
        }

        // Count active characters
        let active_character_count = self.active_characters().into_iter().filter(|b| *b).count();

        // Update active characters flag if neccessary
        if to_index > active_character_count - 1 {
            self.raw.user_data_10.profile_summary.active_profiles[active_character_count] = true;
        }

        // Set profile
        Self::set_profile(self, to_index, from.get_profile(from_index)?)?;

        // Set character
        Self::set_character(self, to_index, from.get_character(from_index)?)?;

        Ok(())
    }

    fn get_profile(&self, index: usize) -> Result<&Profile, SaveApiError> {
        Ok(&self.raw.user_data_10.profile_summary.profiles[index])
    }

    fn set_profile(&mut self, index: usize, profile: &Profile) -> Result<(), SaveApiError> {
        self.raw.user_data_10.profile_summary.profiles[index] = profile.clone();
        Ok(())
    }

    fn get_character(&self, index: usize) -> Result<&UserDataX, SaveApiError> {
        Ok(&self.raw.user_data_x[index])
    }

    fn set_character(&mut self, index: usize, character: &UserDataX) -> Result<(), SaveApiError> {
        // Clone character
        let mut character = character.clone();

        // Update character steam ID
        character.steam_id = match self.platform() {
            SaveType::PC => self.steam_id(),
            SaveType::Playstation => 0,
        };

        // Insert the character
        self.raw.user_data_x[index] = character;
        Ok(())
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

    pub fn regions(&self, index: usize) -> Result<&Vec<u32>, SaveApiError> {
        Ok(&self.raw.user_data_x[index].unlocked_regions.ids)
    }

    pub fn regions_count(&self, index: usize) -> Result<u32, SaveApiError> {
        Ok(self.raw.user_data_x[index].unlocked_regions.count)
    }

    pub fn add_region(&mut self, index: usize, region_id: u32) -> Result<(), SaveApiError> {
        let user_data_x = &mut self.raw.user_data_x[index];
        if user_data_x
            .unlocked_regions
            .ids
            .iter()
            .position(|id| *id == region_id)
            .is_none()
        {
            user_data_x.unlocked_regions.ids.push(region_id);
            user_data_x.unlocked_regions.count += 1;
            let rest_len = user_data_x.rest.len();
            user_data_x.rest.truncate(rest_len - 4);
        }
        Ok(())
    }

    pub fn remove_region(&mut self, index: usize, region_id: u32) -> Result<(), SaveApiError> {
        let user_data_x = &mut self.raw.user_data_x[index];
        if let Some(region_index) = user_data_x
            .unlocked_regions
            .ids
            .iter()
            .position(|id| *id == region_id)
        {
            user_data_x.unlocked_regions.ids.remove(region_index);
            user_data_x.unlocked_regions.count -= 1;
            user_data_x.rest.extend(vec![0; 4]);
        }
        Ok(())
    }

    pub fn get_inventory(
        &self,
        index: usize,
        storage_type: StorageType,
        storage_item_type: StorageItemType,
    ) -> Result<Vec<Item>, SaveApiError> {
        // Get save item list
        let items = match (&storage_type, &storage_item_type) {
            (StorageType::Held, StorageItemType::Regular) => {
                &self.raw.user_data_x[index].inventory_held.common_items
            }
            (StorageType::Held, StorageItemType::Key) => {
                &self.raw.user_data_x[index].inventory_held.key_items
            }
            (StorageType::StorageBox, StorageItemType::Regular) => {
                &self.raw.user_data_x[index]
                    .inventory_storage_box
                    .common_items
            }
            (StorageType::StorageBox, StorageItemType::Key) => {
                &self.raw.user_data_x[index].inventory_storage_box.key_items
            }
        };

        // Create item list
        let items: Result<Vec<Item>, SaveApiError> = items
            .iter()
            .enumerate()
            .map(|(inventory_index, item)| {
                let item_type = ItemType::from_gaitem_id(item.gaitem_handle)?;
                let gaitem_handle = item.gaitem_handle;
                let item_id = self.item_id_from_gaitem_handle(index, item.gaitem_handle)?;
                let text_repo_id = Self::text_repo_id(item_id, &item_type);
                let item_name =
                    if let Some(item_name) = Self::get_item_name(text_repo_id, &item_type) {
                        item_name
                    } else {
                        format!("Unk_{}", item_id)
                    };
                let equip_index = match &storage_item_type {
                    StorageItemType::Regular => match &storage_type {
                        StorageType::Held => KEY_ITEM_CAPACITY.0 + inventory_index,
                        StorageType::StorageBox => KEY_ITEM_CAPACITY.1 + inventory_index,
                    },
                    StorageItemType::Key => 0,
                };
                let quantity = item.quantity;
                let aqcuistion_index = item.aqcuistion_index;
                let is_dlc_item = Self::is_dlc_item(item_id, &item_type);

                Ok(Item {
                    item_type,
                    gaitem_handle,
                    item_id,
                    item_name,
                    quantity,
                    aqcuistion_index,
                    equip_index,
                    is_dlc_item,
                })
            })
            .collect();

        items
    }

    fn item_id_from_gaitem_handle(
        &self,
        index: usize,
        gaitem_handle: u32,
    ) -> Result<u32, SaveApiError> {
        // Get item type
        let item_type = ItemType::from_gaitem_id(gaitem_handle)?;

        // Get item id
        match item_type {
            ItemType::None => Ok(0),
            ItemType::Aow | ItemType::Weapon | ItemType::Armor => {
                let entry = self.raw.user_data_x[index]
                    .gaitem_map
                    .iter()
                    .find(|entry| entry.gaitem_handle == gaitem_handle);

                if entry.is_none() {
                    return Err(SaveApiError::ItemIdNotFound(gaitem_handle));
                }

                Ok(entry.unwrap().item_id & 0x0fffffff)
            }
            ItemType::Item | ItemType::Accessory => Ok(gaitem_handle & 0x0fffffff),
        }
    }

    fn get_text_repos_from_item_type<'a>(
        item_type: &ItemType,
    ) -> Option<(
        &'a LazyLock<HashMap<u32, &'static str>>,
        &'a LazyLock<HashMap<u32, &'static str>>,
    )> {
        match item_type {
            ItemType::None => None,
            ItemType::Weapon => Some((&WEAPON_NAME, &WEAPON_NAME_DLC01)),
            ItemType::Armor => Some((&PROTECTOR_NAME, &PROTECTOR_NAME_DLC01)),
            ItemType::Accessory => Some((&ACCESSORY_NAME, &ACCESSORY_NAME_DLC01)),
            ItemType::Item => Some((&GOODS_NAME, &GOODS_NAME_DLC01)),
            ItemType::Aow => Some((&ARTS_NAME, &ARTS_NAME_DLC01)),
        }
    }

    fn text_repo_id(item_id: u32, item_type: &ItemType) -> u32 {
        // If item is a weapon then remove the weapon level.
        if item_type == &ItemType::Weapon {
            item_id / 100 * 100
        }
        // Aows in inventory has two additional zeros that need to be removed
        // in order to look it up in the text repository
        else if item_type == &ItemType::Aow {
            item_id / 100
        } else {
            item_id
        }
    }

    pub fn get_item_name(text_repo_id: u32, item_type: &ItemType) -> Option<String> {
        // Try to get name from text repos
        if let Some((regular, dlc)) = Self::get_text_repos_from_item_type(item_type) {
            if let Some(name) = regular.get(&text_repo_id) {
                return Some(name.to_string());
            } else if let Some(name) = dlc.get(&text_repo_id) {
                return Some(name.to_string());
            }
        }

        None
    }

    pub fn is_dlc_item(text_repo_id: u32, item_type: &ItemType) -> bool {
        if let Some((_, dlc)) = Self::get_text_repos_from_item_type(item_type) {
            if let Some(_) = dlc.get(&text_repo_id) {
                return true;
            }
        }

        false
    }

    pub fn next_equip_index(
        &self,
        index: usize,
        storage_type: StorageType,
    ) -> Result<u32, SaveApiError> {
        Ok(match storage_type {
            StorageType::Held => {
                self.raw.user_data_x[index]
                    .inventory_held
                    .equip_index_counter
            }
            StorageType::StorageBox => {
                self.raw.user_data_x[index]
                    .inventory_storage_box
                    .equip_index_counter
            }
        })
    }

    pub fn next_acquistion_index(
        &self,
        index: usize,
        storage_type: StorageType,
    ) -> Result<u32, SaveApiError> {
        Ok(match storage_type {
            StorageType::Held => {
                self.raw.user_data_x[index]
                    .inventory_held
                    .aquistion_index_counter
            }
            StorageType::StorageBox => {
                self.raw.user_data_x[index]
                    .inventory_storage_box
                    .aquistion_index_counter
            }
        })
    }

    pub fn common_items_count(
        &self,
        index: usize,
        storage_type: StorageType,
    ) -> Result<u32, SaveApiError> {
        Ok(match storage_type {
            StorageType::Held => self.raw.user_data_x[index].inventory_held.common_item_count,
            StorageType::StorageBox => {
                self.raw.user_data_x[index]
                    .inventory_storage_box
                    .common_item_count
            }
        })
    }

    pub fn key_items_count(
        &self,
        index: usize,
        storage_type: StorageType,
    ) -> Result<u32, SaveApiError> {
        Ok(match storage_type {
            StorageType::Held => self.raw.user_data_x[index].inventory_held.key_item_count,
            StorageType::StorageBox => {
                self.raw.user_data_x[index]
                    .inventory_storage_box
                    .key_item_count
            }
        })
    }
}
