use std::io::Cursor;

use deku::ctx::Endian;
use deku::prelude::*;
use deku::{DekuRead, DekuWrite};

use super::user_data_x::{
    ActiveWeaponSlotsAndArmStyle, EquippedItemsItemIds, EquppedItemsGaitemHandles, FaceData,
};
use super::util::{MapId, Util};

#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: Endian, start: usize, size: usize, is_ps: bool"
)]
pub(crate) struct UserData10 {
    // Checksum (PC only)
    #[deku(skip, cond = "is_ps", count = "0x10")]
    checksum: Vec<u8>,

    // File version
    pub(crate) version: u32,

    // SteamId
    pub(crate) steam_id: u64,

    // Settings
    pub(crate) settings: Settings,

    // #[deku(skip, cond = "true", default = "deku::byte_offset")]
    // pub(crate) byte_offset: usize,

    // Menu System Save Load
    pub(crate) menu_system_save_load: MenuSystemSaveLoad,

    // Profile Summary
    pub(crate) profile_summary: ProfileSummary,

    gamedataman0xd0: u32,
    gamedataman0x75: u8,

    // PCOptionData (PC ONLY)
    #[deku(skip, cond = "is_ps")]
    pub(crate) pc_option_data: PCOptionData,

    // Key Config Save Load
    pub(crate) key_config_save_load: KeyConfigSaveLoad,

    game_man_0x118: u64,

    // Empty calories
    #[deku(count = "size - (deku::byte_offset - start)")]
    pub(crate) rest: Vec<u8>,
}

// Settings
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct Settings {
    pub(crate) camera_speed: u8,
    pub(crate) controller_vibration: u8,
    pub(crate) brightness: u8,
    pub(crate) unk0x3: u8,
    pub(crate) music_volume: u8,
    pub(crate) sound_effects_volume: u8,
    pub(crate) voice_volume: u8,
    pub(crate) display_blood: u8,
    pub(crate) subtitles: u8,
    pub(crate) hud: u8,
    pub(crate) camera_x_axis: u8,
    pub(crate) camera_y_axis: u8,
    pub(crate) toggle_auto_lockon: u8,
    pub(crate) camera_auto_wall_recovery: u8,
    pub(crate) unk0xe: u8,
    pub(crate) unk0xf: u8,
    pub(crate) reset_camera_y_axis: u8,
    pub(crate) cinematic_effects: u8,
    pub(crate) unk0x12: u8,
    pub(crate) perform_matchmaking: u8,
    pub(crate) unk0x14: u8,
    pub(crate) unk0x15: u8,
    pub(crate) manual_attack_aim: u8,
    pub(crate) autotarget: u8,
    pub(crate) launchsettings: u8,
    pub(crate) send_summon_sign: u8,
    pub(crate) unk0x1a: u8,
    pub(crate) hdr: u8,
    pub(crate) hdr_adjust_brightness: u8,
    pub(crate) hdr_maximum_brightness: u8,
    pub(crate) hdr_adjust_saturation: u8,
    pub(crate) unk0x1f: u8,
    pub(crate) master_volume: u8,
    pub(crate) is_raytracing_on: u8,
    pub(crate) mark_new_items: u8,
    pub(crate) show_recent_tabs: u8,
    #[deku(assert_eq = "0")]
    unk0x24: u64,
    #[deku(assert_eq = "0")]
    unk0x2c: u16,
    pub(crate) show_tutorials: u8,
    pub(crate) camera_auto_rotation: u8,
    #[deku(count = "0x110")]
    pub(crate) justzero: Vec<u8>,
}

impl UserData10 {
    pub(crate) fn read<R: std::io::Read>(
        reader: &mut deku::reader::Reader<R>,
        endian: Endian,
        start: usize,
        size: usize,
        is_ps: bool,
    ) -> Result<Self, DekuError> {
        let user_data_10 = Self::from_reader_with_ctx(reader, (endian, start, size, is_ps))?;
        Ok(user_data_10)
    }

    pub(crate) fn write<W: std::io::Write>(
        writer: &mut deku::writer::Writer<W>,
        endian: Endian,
        start: usize,
        size: usize,
        is_ps: bool,
        user_data_10: &Self,
    ) -> Result<(), DekuError> {
        if is_ps {
            user_data_10.to_writer(writer, (endian, start, size, is_ps))?;
            return Ok(());
        }

        let mut buffer = Vec::new();
        {
            let mut temp_writer = Writer::new(Cursor::new(&mut buffer));
            user_data_10.to_writer(&mut temp_writer, (endian, start, size, is_ps))?;
        }

        Util::update_checksum(&mut buffer);

        writer.write_bytes(&buffer)?;
        Ok(())
    }
}

// Menu System Save Load
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct MenuSystemSaveLoad {
    unk0x0: u16,
    unk0x2: u16,
    pub(crate) size: u32,
    #[deku(count = "size")]
    pub(crate) data: Vec<u8>,
}

// Profile Summary
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct ProfileSummary {
    pub(crate) active_profiles: [bool; 10],
    #[deku(count = "10")]
    pub(crate) profiles: Vec<Profile>,
}
// Profile
#[derive(PartialEq, Debug, DekuRead, DekuWrite, Clone)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct Profile {
    #[deku(
        reader = "Util::read_wstring(deku::reader, 32)",
        writer = "Util::write_wstring(deku::writer, &character_name, 32)"
    )]
    pub(crate) character_name: String,
    #[deku(assert_eq = "0")]
    character_name_terminator: u16,
    pub(crate) level: u32,
    pub(crate) seconds_played: u32,
    pub(crate) runes_memory: u32,
    pub(crate) map_id: MapId,
    pub(crate) unk0x34: u32,
    #[deku(ctx = "true")]
    pub(crate) face_data: FaceData,
    // #[deku(skip, cond = "true", default = "deku::byte_offset")]
    // pub(crate) byte_offset: usize,
    pub(crate) equipment: ProfileEquipment,
    pub(crate) gender: u8,
    pub(crate) archetype: u8,
    pub(crate) starting_gift: u8,
    profile_summary_character_0x293: u8,
    profile_summary_character_0x294: u8,
    profile_summary_character_0x295: u8,
    profile_summary_character_0x298: u32,
}

// Profile Equipment
#[derive(PartialEq, Debug, DekuRead, DekuWrite, Clone)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct ProfileEquipment {
    unk0x0: u64,
    pub(crate) active_weapon_slots_and_arm_style: ActiveWeaponSlotsAndArmStyle,
    pub(crate) equipped_items_gaitem_handle: EquppedItemsGaitemHandles,
    pub(crate) equipped_items_item_id: EquippedItemsItemIds,
    unk0xd4: i32,
    unk0xd8: u32,
    unk0xdc: [u32; 3],
}

// PCOptionData
#[derive(PartialEq, Debug, DekuRead, DekuWrite, Default)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct PCOptionData {
    unk0x0: u32,
    unk0xc: u8,
    unk0xd: u8,
    unk0xe: u8,
    unk0xf: u8,
    unk0x10: u64,
    unk0x18: u16,
    #[deku(count = "0xa0/2")]
    unk0x12: Vec<u16>,
}

// KeyConfigSaveLoad
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct KeyConfigSaveLoad {
    unk0x0: u16,
    unk0x2: u16,
    pub(crate) size: u32,
    #[deku(count = "*size")]
    pub(crate) data: Vec<u8>,
}
