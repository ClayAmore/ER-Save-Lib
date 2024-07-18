use std::{collections::HashMap, sync::OnceLock};

use crate::{Save, SaveApiError};

// BST tree dump
const EVENTFLAG_BST: &str = include_str!("../res/eventflag_bst.txt");

pub(crate) struct EventFlagsApi;

const FLAG_DIVISOR: u32 = 1000;
const BLOCK_SIZE: u32 = 125;

impl EventFlagsApi {
    pub(crate) fn get_event_flag(
        raw: &Save,
        event_id: u32,
        character_index: usize,
    ) -> Result<bool, SaveApiError> {
        let block = event_id / FLAG_DIVISOR;
        let index = event_id - block * FLAG_DIVISOR;

        if let Some(res) = Self::event_flag_map().get(&block) {
            let offset = res * BLOCK_SIZE;
            let byte_index = index / 8;
            let mut bit_index = index - byte_index * 8;
            bit_index = 7 - bit_index;
            let eventflag_byte =
                raw.user_data_x[character_index].event_flags[(offset + byte_index) as usize];
            let eventflag_bit = ((eventflag_byte >> bit_index) & 1) == 1;
            return Ok(eventflag_bit);
        }
        return Err(SaveApiError::EventIdNotFound);
    }

    pub(crate) fn set_event_flag(
        raw: &mut Save,
        event_id: u32,
        character_index: usize,
        on: bool,
    ) -> Result<(), SaveApiError> {
        let block = event_id / FLAG_DIVISOR;
        let index = event_id - block * FLAG_DIVISOR;

        if let Some(res) = Self::event_flag_map().get(&block) {
            let offset = res * BLOCK_SIZE;
            let byte_index = index / 8;
            let mut bit_index = index - byte_index * 8;
            bit_index = 7 - bit_index;
            let mut eventflag_byte =
                raw.user_data_x[character_index].event_flags[(offset + byte_index) as usize];
            if on {
                eventflag_byte = eventflag_byte | (1 << bit_index);
            } else {
                eventflag_byte = eventflag_byte & !(1 << bit_index);
            }
            raw.user_data_x[character_index].event_flags[(offset + byte_index) as usize] =
                eventflag_byte;
            return Ok(());
        }
        return Err(SaveApiError::EventIdNotFound);
    }

    // BST tree turned into a static hashmap
    fn event_flag_map() -> &'static HashMap<u32, u32> {
        static MAP: OnceLock<HashMap<u32, u32>> = OnceLock::new();
        MAP.get_or_init(|| {
            let mut map: HashMap<u32, u32> = HashMap::new();
            for line in EVENTFLAG_BST.lines() {
                let mut key_value = line.split(",");

                if let (Some(key), Some(value)) = (key_value.next(), key_value.next()) {
                    let key = key.parse::<u32>().unwrap();
                    let value = value.parse::<u32>().unwrap();
                    map.insert(key, value);
                }
            }
            map
        })
    }
}

#[test]
fn get_event_flag() {
    let first_step_grace = 76101;
    let save = Save::from_path("./test/ER0000.sl2").unwrap();
    let on = EventFlagsApi::get_event_flag(&save, first_step_grace, 0);
    assert!(on.is_ok());
}

#[test]
fn set_event_flag() {
    let first_step_grace = 76101;
    let mut save = Save::from_path("./test/ER0000.sl2").unwrap();
    let _ = EventFlagsApi::set_event_flag(&mut save, first_step_grace, 0, true);
    save.write_to_path("./test/copy.sl2")
        .expect("Failed to write save file!");
}
