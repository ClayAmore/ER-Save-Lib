use deku::ctx::Endian;
use deku::prelude::*;
use deku::{DekuRead, DekuWrite};

use std::io::Cursor;

use super::util::{FloatVector3, FloatVector4, MapId, Util};

#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, end: usize, is_ps: bool")]
pub(crate) struct UserDataX {
    // Checksum (PC only)
    #[deku(skip, cond = "is_ps", count = "0x10")]
    pub(crate) checksum: Vec<u8>,

    // File version
    pub(crate) version: u32,

    // Current Map Id
    pub(crate) map_id: [u8; 4],

    // Could be just random data
    unk0x8: [u8; 0x8],
    // Definetly seemed like random data. Game is using random function to generate.
    unk0x10: [u8; 0x10],

    // Gaitem Map
    #[deku(count = "if *version <= 80 {0x13FE} else {0x1400}")]
    pub(crate) gaitem_map: Vec<Gaitem>,

    // Player data
    pub(crate) player_game_data: PlayerGameData,

    // SPEffects
    #[deku(count = "0xD")]
    pub(crate) sp_effects: Vec<SPEffect>,

    // Equipment data and Inventory data
    pub(crate) equipped_items_equip_index: EquippedItemsEquipIndex,
    pub(crate) active_weapon_slots_and_arm_style: ActiveWeaponSlotsAndArmStyle,
    pub(crate) equipped_items_item_id: EquippedItemsItemIds,
    pub(crate) equipped_items_gaitem_handle: EquppedItemsGaitemHandles,
    #[deku(ctx = "0xa80, 0x180")]
    pub(crate) inventory_held: Invenotry,
    pub(crate) equipped_spells: EquippedSpells,
    pub(crate) equipped_items: EquippedItems,
    pub(crate) equipped_gestures: EquippedGestures,
    pub(crate) acquired_projectiles: AcquiredProjectiles,
    pub(crate) equipped_armaments_and_items: EquippedArmamentsAndItems,
    pub(crate) equipped_physics: EquippedPhysics,

    // Face data
    #[deku(ctx = "false")]
    pub(crate) face_data: FaceData,

    // Inventory Data (Storage Box)
    #[deku(ctx = "0x780, 0x80")]
    pub(crate) inventory_storage_box: Invenotry,

    // Gestures
    pub(crate) gestures: Gestures,

    // Unlocked regions
    pub(crate) unlocked_regions: Regions,

    // Horse Data
    pub(crate) horse: RideGameData,

    #[deku(assert = "*control_byte_maybe == 1 || *control_byte_maybe == 0")]
    control_byte_maybe: u8,

    // Blood Stain
    pub(crate) blood_stain: BloodStain,

    unk_gamedataman_0x120_or_gamedataman_0x130: u32,
    unk_gamedataman_0x88: u32,

    // Menu Profile Save Load
    pub(crate) menu_profile_save_load: MenuSaveLoad,

    // Trophy Equip Data
    pub(crate) trophy_equip_data: TrophyEquipData,

    // Gaitem Game Data
    pub(crate) gaitem_game_data: GaitemGameData,

    // Tutorial Game Data
    pub(crate) tutorial_data: TutorialData,

    gameman_0x8c: u8,
    gameman_0x8d: u8,
    gameman_0x8e: u8,

    // Death Count
    pub(crate) total_deaths_count: u32,

    // Character Type {
    //     None = -1,
    //     Phantom = 1,
    //     Invader = 2,
    //     Ghost = 3,
    //     DeadGhost = 10,
    //     NakedGhost = 11,
    //     Unkown = 13,
    //     NakedGhost2 = 14,
    //     Invader2 = 15,
    //     Invader3 = 16,
    //     Blue = 17,
    //     Invader4 = 18
    // }
    pub(crate) character_type: i32,

    // Deactivates warp, resting at sites of grace, etc..
    pub(crate) in_online_session_flag: u8,

    // For phantom or invader character types this value is 0. Otherwise it's 8.
    #[deku(assert = "*character_type_online == 8 || *character_type_online == 0")]
    pub(crate) character_type_online: u32,

    // Last Grace EntityId + 1000
    pub(crate) last_rested_grace: u32,

    // Seems to indicate wether a player is alone or has someone in their world
    #[deku(assert = "*not_alone_flag == 1 || *not_alone_flag == 0")]
    pub(crate) not_alone_flag: u8,

    // 1 = 10 second
    pub(crate) in_game_countdown_timer: u32,

    // Is set conditionally. Can either be gameman with offset 0x124 or 0x134
    unk_gamedataman_0x124_or_gamedataman_0x134: u32,

    // Event Flags
    #[deku(bytes_read = "0x1BF99F")]
    pub(crate) event_flags: Vec<u8>,
    #[deku(assert_eq = "0")]
    event_flags_terminator: u8,

    // Field Area
    pub(crate) field_area: FieldArea,

    // World Area
    pub(crate) world_area: WorldArea,

    // World Geom Man (GEOM)
    pub(crate) world_geom_man: WorldGeomMan,

    // World Geom Man (GEOF)
    pub(crate) world_geom_man2: WorldGeomMan,

    // RendMan
    pub(crate) rend_man: RendMan,

    // Player Coordinates
    pub(crate) player_coordinates: PlayerCoordinates,

    // GameMan 0x5BE
    game_man_0x5be: u8,

    // GameMan 0x5BF
    game_man_0x5bf: u8,

    // Spawn Point Entity Id
    pub(crate) spawn_point_entity_id: u32,

    // GameMan 0x5BF
    game_man_0xb64: u32,

    // Temp spawn point entity id, only for post 65  save version
    #[deku(skip, cond = "*version < 65")]
    pub(crate) temp_spawn_point_entity_id: u32,

    // Only for post 66 save version
    #[deku(skip, cond = "*version < 66")]
    game_man_0xcb3: u8,

    // NetMan
    pub(crate) net_man: NetMan,

    // World Area Weather
    pub(crate) world_area_weather: WorldAreaWeather,

    // World Area Time
    pub(crate) world_area_time: WorldAreaTime,

    // Base Version
    pub(crate) base_version: BaseVersion,

    // SteamId
    pub(crate) steam_id: u64,

    // PS5Activity
    pub(crate) ps5_activity: PS5Activity,

    // DLC
    pub(crate) dlc: DLC,

    // Player Data Hash
    pub(crate) player_data_hash: PlayerDataHash,

    #[deku(count = "end - deku::byte_offset")]
    pub(crate) rest: Vec<u8>,
}

impl UserDataX {
    pub(crate) fn read<R: std::io::Read>(
        reader: &mut deku::reader::Reader<R>,
        endian: Endian,
        start: usize,
        size: usize,
        count: usize,
        is_ps: bool,
    ) -> Result<Vec<Self>, DekuError> {
        let mut user_data_x_vec: Vec<Self> = Vec::with_capacity(count);
        for i in 0..count {
            let end = (start + size * i) + size;
            let user_data_x = Self::from_reader_with_ctx(reader, (endian, end, is_ps))?;
            user_data_x_vec.push(user_data_x)
        }
        Ok(user_data_x_vec)
    }

    pub(crate) fn write<W: std::io::Write>(
        writer: &mut deku::writer::Writer<W>,
        endian: Endian,
        start: usize,
        size: usize,
        is_ps: bool,
        user_data_x_vec: &Vec<Self>,
    ) -> Result<(), DekuError> {
        for (i, user_data_x) in user_data_x_vec.iter().enumerate() {
            let end = (start + size * i) + size;

            if is_ps {
                user_data_x.to_writer(writer, (endian, end, is_ps))?;
                continue;
            }

            let mut buffer = Vec::new();
            {
                let mut temp_writer = Writer::new(Cursor::new(&mut buffer));
                user_data_x.to_writer(&mut temp_writer, (endian, start + size * i, is_ps))?;
            }

            Util::update_checksum(&mut buffer);

            writer.write_bytes(&buffer)?;
        }
        Ok(())
    }
}

// #[deku(skip, cond = "true", default = "deku::byte_offset")]
// pub(crate) test: usize,

// Gaitem Map
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct Gaitem {
    #[deku(assert = "
            (*gaitem_handle & 0xf0000000) == 0 ||
            (*gaitem_handle & 0xf0000000) == 0x80000000 ||
            (*gaitem_handle & 0xf0000000) == 0x90000000 ||
            (*gaitem_handle & 0xf0000000) == 0xc0000000")]
    pub(crate) gaitem_handle: u32,
    #[deku(assert = "
            (*item_id == 0xFFFFFFFF) ||
            (*item_id & 0xf0000000) == 0 ||
            (*item_id & 0xf0000000) == 0x10000000 ||
            (*item_id & 0xf0000000) == 0x80000000
        ")]
    pub(crate) item_id: u32,
    #[deku(
        skip,
        cond = "*gaitem_handle == 0 || 
        *gaitem_handle & 0xf0000000 == 0xc0000000"
    )]
    pub(crate) unk0x10: Option<i32>,
    #[deku(
        skip,
        cond = "*gaitem_handle == 0 || 
        *gaitem_handle & 0xf0000000 == 0xC0000000"
    )]
    pub(crate) unk0x14: Option<i32>,
    #[deku(
        skip,
        cond = "*gaitem_handle == 0 || 
        *gaitem_handle & 0xf0000000 != 0x80000000"
    )]
    pub(crate) gem_gaitem_handle: Option<i32>,
    #[deku(
        skip,
        cond = "*gaitem_handle == 0 || 
        *gaitem_handle & 0xf0000000 != 0x80000000",
        assert = "(
            (
                (
                    *gaitem_handle == 0 || 
                    *gaitem_handle & 0xf0000000 != 0x80000000
                ) && unk0x1c.is_none()
            )
            ||
            (
                (
                    *gaitem_handle != 0 || 
                    *gaitem_handle & 0xf0000000 == 0x80000000
                ) && *unk0x1c == Some(0)
            )
        )"
    )]
    pub(crate) unk0x1c: Option<u8>,
}

// Player
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct PlayerGameData {
    #[deku(assert_eq = "0")]
    unk0x0: u32,
    #[deku(assert_eq = "0")]
    unk0x4: u32,
    pub(crate) hp: u32,
    pub(crate) max_hp: u32,
    pub(crate) base_max_hp: u32,
    pub(crate) fp: u32,
    pub(crate) max_fp: u32,
    pub(crate) base_max_fp: u32,
    #[deku(assert_eq = "0")]
    unk0x20: u32,
    pub(crate) sp: u32,
    pub(crate) max_sp: u32,
    pub(crate) base_max_sp: u32,
    #[deku(assert_eq = "0")]
    unk0x30: u32,
    pub(crate) vigor: u32,
    pub(crate) mind: u32,
    pub(crate) endurance: u32,
    pub(crate) strength: u32,
    pub(crate) dexterity: u32,
    pub(crate) intelligence: u32,
    pub(crate) faith: u32,
    pub(crate) arcane: u32,
    #[deku(assert_eq = "0")]
    unk0x54: u32,
    #[deku(assert_eq = "0")]
    unk0x58: u32,
    #[deku(assert_eq = "0")]
    unk0x5c: u32,
    pub(crate) level: u32,
    pub(crate) souls: u32,
    pub(crate) souls_memory: u32,
    unk0x6c: u32,
    pub(crate) poison_buildup: u32,
    pub(crate) rot_buildup: u32,
    pub(crate) bleed_buildup: u32,
    pub(crate) death_buildup: u32,
    pub(crate) frost_buildup: u32,
    pub(crate) sleep_buildup: u32,
    pub(crate) madness_buildup: u32,
    unk0x8c: u32,
    #[deku(assert_eq = "0")]
    unk0x90: u32,
    #[deku(
        reader = "Util::read_wstring(deku::reader, 32)",
        writer = "Util::write_wstring(deku::writer, &character_name, 32)"
    )]
    pub(crate) character_name: String,
    #[deku(assert_eq = "0")]
    pub(crate) terminator: u16,
    pub(crate) gender: u8,
    pub(crate) arche_type: u8,
    unk0xb8: u8,
    unk0xb9: u8,
    pub(crate) voice_type: u8,
    pub(crate) gift: u8,
    unk0xbc: u8,
    unk0xbd: u8,
    pub(crate) additional_talisman_slot_count: u8,
    pub(crate) summon_spirit_level: u8,
    unk0xc0: [u8; 0x18],
    pub(crate) furl_calling_finger_on: bool,
    unk0xd9: u8,
    pub(crate) matchmaking_weapon_level: u8,
    pub(crate) white_chipher_ring_on: bool,
    pub(crate) blue_cipher_ring_on: bool,
    unk0xdd: [u8; 0x1a],
    pub(crate) great_rune_on: bool,
    unk0xf8: u8,
    pub(crate) max_crimson_flask_count: u8,
    pub(crate) max_cerulean_flask_count: u8,
    unk0xfb: [u8; 0x15],
    #[deku(
        reader = "Util::read_wstring(deku::reader, 16)",
        writer = "Util::write_wstring(deku::writer, &password, 16)"
    )]
    pub(crate) password: String,
    #[deku(assert_eq = "0")]
    password_terminator: u16,
    #[deku(
        reader = "Util::read_wstring(deku::reader, 16)",
        writer = "Util::write_wstring(deku::writer, &group_password1, 16)"
    )]
    pub(crate) group_password1: String,
    #[deku(assert_eq = "0")]
    group_password1d_terminator: u16,
    #[deku(
        reader = "Util::read_wstring(deku::reader, 16)",
        writer = "Util::write_wstring(deku::writer, &group_password2, 16)"
    )]
    pub(crate) group_password2: String,
    #[deku(assert_eq = "0")]
    group_password2d_terminator: u16,
    #[deku(
        reader = "Util::read_wstring(deku::reader, 16)",
        writer = "Util::write_wstring(deku::writer, &group_password3, 16)"
    )]
    pub(crate) group_password3: String,
    #[deku(assert_eq = "0")]
    group_password3d_terminator: u16,
    #[deku(
        reader = "Util::read_wstring(deku::reader, 16)",
        writer = "Util::write_wstring(deku::writer, &group_password4, 16)"
    )]
    pub(crate) group_password4: String,
    #[deku(assert_eq = "0")]
    group_password4d_terminator: u16,
    #[deku(
        reader = "Util::read_wstring(deku::reader, 16)",
        writer = "Util::write_wstring(deku::writer, &group_password5, 16)"
    )]
    pub(crate) group_password5: String,
    #[deku(assert_eq = "0")]
    group_password5d_terminator: u16,
    unk0x17c: [u8; 0x34],
}

// SPeffects
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct SPEffect {
    sp_effect_id: i32,
    remaining_time: f32,
    unk0x8: u32,
    unk0x10: u32,
}

// Equipped Items Equip Indexes
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct EquippedItemsEquipIndex {
    pub(crate) left_hand_armament1: u32,
    pub(crate) right_hand_armament1: u32,
    pub(crate) left_hand_armament2: u32,
    pub(crate) right_hand_armament2: u32,
    pub(crate) left_hand_armament3: u32,
    pub(crate) right_hand_armament3: u32,
    pub(crate) arrows1: u32,
    pub(crate) bolts1: u32,
    pub(crate) arrows2: u32,
    pub(crate) bolts2: u32,
    unk0x28: u32,
    unk0x2c: u32,
    pub(crate) head: u32,
    pub(crate) chest: u32,
    pub(crate) arms: u32,
    pub(crate) legs: u32,
    unk0x40: u32,
    pub(crate) talisman1: u32,
    pub(crate) talisman2: u32,
    pub(crate) talisman3: u32,
    pub(crate) talisman4: u32,
    unk0x54: u32,
}

// Active weapon slot, arrow and bolt
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct ActiveWeaponSlotsAndArmStyle {
    pub(crate) arm_style: u32,
    #[deku(assert = "*left_hand_weapon_active_slot < 3")]
    pub(crate) left_hand_weapon_active_slot: u32,
    #[deku(assert = "*right_hand_weapon_active_slot < 3")]
    pub(crate) right_hand_weapon_active_slot: u32,
    #[deku(assert = "*left_arrow_active_slot < 2")]
    pub(crate) left_arrow_active_slot: u32,
    #[deku(assert = "*right_arrow_active_slot < 2")]
    pub(crate) right_arrow_active_slot: u32,
    #[deku(assert = "*left_bolt_active_slot < 2")]
    pub(crate) left_bolt_active_slot: u32,
    #[deku(assert = "*right_bolt_active_slot < 2")]
    pub(crate) right_bolt_active_slot: u32,
}

// Equipped Items Param Ids
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct EquippedItemsItemIds {
    pub(crate) left_hand_armament1: u32,
    pub(crate) right_hand_armament1: u32,
    pub(crate) left_hand_armament2: u32,
    pub(crate) right_hand_armament2: u32,
    pub(crate) left_hand_armament3: u32,
    pub(crate) right_hand_armament3: u32,
    pub(crate) arrows1: u32,
    pub(crate) bolts1: u32,
    pub(crate) arrows2: u32,
    pub(crate) bolts2: u32,
    unk0x28: u32,
    unk2c: u32,
    pub(crate) head: u32,
    pub(crate) chest: u32,
    pub(crate) arms: u32,
    pub(crate) legs: u32,
    unk40: u32,
    pub(crate) talisman1: u32,
    pub(crate) talisman2: u32,
    pub(crate) talisman3: u32,
    pub(crate) talisman4: u32,
    unk0x54: u32,
}

// Equipped Items GaitemHandles
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct EquppedItemsGaitemHandles {
    pub(crate) left_hand_armament1: u32,
    pub(crate) right_hand_armament1: u32,
    pub(crate) left_hand_armament2: u32,
    pub(crate) right_hand_armament2: u32,
    pub(crate) left_hand_armament3: u32,
    pub(crate) right_hand_armament3: u32,
    pub(crate) arrows1: u32,
    pub(crate) bolts1: u32,
    pub(crate) arrows2: u32,
    pub(crate) bolts2: u32,
    unk0x44: u32,
    unk48: u32,
    pub(crate) head: u32,
    pub(crate) chest: u32,
    pub(crate) arms: u32,
    pub(crate) legs: u32,
    unk5c: u32,
    pub(crate) talisman1: u32,
    pub(crate) talisman2: u32,
    pub(crate) talisman3: u32,
    pub(crate) talisman4: u32,
    unk0x54: u32,
}

// Inventory (Held and Storage Box)
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: Endian, common_items_capacity: u32, key_items_capacity: u32"
)]
pub(crate) struct Invenotry {
    #[deku(assert = "*common_item_count <= common_items_capacity")]
    common_item_count: u32,
    #[deku(count = "common_items_capacity")]
    common_items: Vec<InvenotryItem>,
    #[deku(assert = "*key_item_count <= key_items_capacity")]
    key_item_count: u32,
    #[deku(count = "key_items_capacity")]
    key_items: Vec<InvenotryItem>,
    equip_index_counter: u32,
    aquistion_index_counter: u32,
}
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct InvenotryItem {
    gaitem_handle: u32,
    #[deku(assert = "*quantity <= 999")]
    quantity: u32,
    aqcuistion_index: u32,
}

// Equipped Spells
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct EquippedSpells {
    #[deku(count = "0xc")]
    pub(crate) spellslot: Vec<Spell>,
    unk0x60: u32,
    unk0x64: u32,
    unk0x68: u32,
    unk0x70: u32,
    #[deku(assert = "*active_index < 0xc || *active_index == 0xffffffff")]
    pub(crate) active_index: u32,
}
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct Spell {
    pub(crate) spell_id: u32,
    unk0x4: u32,
}

// Equipped Items
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct EquippedItems {
    #[deku(count = "0xa")]
    pub(crate) quick_items: Vec<EquippedItem>,
    #[deku(assert = "*active_quick_item_index < 10 || *active_quick_item_index == 0xffffffff")]
    pub(crate) active_quick_item_index: u32,
    #[deku(count = "0x6")]
    pub(crate) pouch_items: Vec<EquippedItem>,
    unk0x84: u32,
    unk0x88: u32,
}
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct EquippedItem {
    pub(crate) gaitem_handle: u32,
    pub(crate) equip_index: u32,
}

// Equipped Gestures
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct EquippedGestures {
    #[deku(count = "0x6")]
    equipped_gesture: Vec<u32>,
}

// Aquired Projectiles
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct AcquiredProjectiles {
    pub(crate) count: u32,
    #[deku(count = "*count")]
    projectiles: Vec<Projectile>,
}
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct Projectile {
    pub(crate) id: u32,
    unk0x4: u32,
}

// Equipped Weapons, Amor, Talisman and Items
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct EquippedArmamentsAndItems {
    pub(crate) left_hand_armament1: u32,
    pub(crate) right_hand_armament1: u32,
    pub(crate) left_hand_armament2: u32,
    pub(crate) right_hand_armament2: u32,
    pub(crate) left_hand_armament3: u32,
    pub(crate) right_hand_armament3: u32,
    pub(crate) arrows1: u32,
    pub(crate) bolts1: u32,
    pub(crate) arrows2: u32,
    pub(crate) bolts2: u32,
    unk0x28: u32,
    unk0x2c: u32,
    pub(crate) head: u32,
    pub(crate) chest: u32,
    pub(crate) arms: u32,
    pub(crate) legs: u32,
    unk0x40: u32,
    pub(crate) talisman1: u32,
    pub(crate) talisman2: u32,
    pub(crate) talisman3: u32,
    pub(crate) talisman4: u32,
    unk0x54: u32,
    pub(crate) quickitem1: u32,
    pub(crate) quickitem2: u32,
    pub(crate) quickitem3: u32,
    pub(crate) quickitem4: u32,
    pub(crate) quickitem5: u32,
    pub(crate) quickitem6: u32,
    pub(crate) quickitem7: u32,
    pub(crate) quickitem8: u32,
    pub(crate) quickitem9: u32,
    pub(crate) quickitem10: u32,
    pub(crate) pouch1: u32,
    pub(crate) pouch2: u32,
    pub(crate) pouch3: u32,
    pub(crate) pouch4: u32,
    pub(crate) pouch5: u32,
    pub(crate) pouch6: u32,
    unk0x98: u32,
}

// Equipped Physics
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct EquippedPhysics {
    pub(crate) slot1: u32,
    pub(crate) slot2: u32,
    unk0x8: u32,
}

// Face Data
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, in_profile_summary: bool")]
pub(crate) struct FaceData {
    #[deku(assert = "*facedata0x150 == 0 || *facedata0x150 == -1")]
    facedata0x150: i32,
    #[deku(assert = "magic == &[0x46, 0x41, 0x43, 0x45] || magic == &[0, 0, 0, 0]")]
    magic: [u8; 4],
    alignment: u32,
    size: u32,
    #[deku(pad_bytes_after = "3")]
    facemodel: u8,
    #[deku(pad_bytes_after = "3")]
    hairmodel: u8,
    #[deku(pad_bytes_after = "3")]
    unk0x14: u8,
    #[deku(pad_bytes_after = "3")]
    eye_brow_model: u8,
    #[deku(pad_bytes_after = "3")]
    beardmodel: u8,
    #[deku(pad_bytes_after = "3")]
    eyepatchmodel: u8,
    #[deku(pad_bytes_after = "3")]
    unk0x24: u8,
    #[deku(pad_bytes_after = "3")]
    unk0x28: u8,
    apparent_age: u8,
    facial_aesthetic: u8,
    form_emphasis: u8,
    unk0xf: u8,
    _brow_ridge_height: u8,
    inner_brow_ridge: u8,
    outer_brow_ridge: u8,
    cheek_bone_height: u8,
    cheek_bone_depth: u8,
    cheek_bone_width: u8,
    cheek_bone_prostrution: u8,
    cheeks: u8,
    chin_tip_position: u8,
    chin_length: u8,
    chin_prostrusion: u8,
    chin_depth: u8,
    chin_size: u8,
    chin_height: u8,
    chin_width: u8,
    eye_position: u8,
    eye_size: u8,
    eye_slant: u8,
    eye_spacing: u8,
    nose_size: u8,
    nose_forehead_ratio: u8,
    unk0x45: u8,
    face_protrusion: u8,
    vertical_face_ratio: u8,
    facial_features_lant: u8,
    horizontal_face_ratio: u8,
    unk0x4a: u8,
    forehead_depth: u8,
    forehead_protrusion: u8,
    unk0x4d: u8,
    jaw_prostrusion: u8,
    jaw_width: u8,
    lower_jaw: u8,
    jaw_contour: u8,
    lip_shape: u8,
    lip_size: u8,
    lip_fullness: u8,
    mouth_expression: u8,
    lip_prostrusion: u8,
    lip_thickness: u8,
    mouth_prostrusion: u8,
    mout_hslant: u8,
    occulsion: u8,
    mouth_position: u8,
    mouth_width: u8,
    mouth_chin_distance: u8,
    nose_ridge_depth: u8,
    nose_ridge_length: u8,
    nose_position: u8,
    nose_tip_height: u8,
    nostril_slant: u8,
    nostril_size: u8,
    nostril_width: u8,
    nose_prostrution: u8,
    nose_bridge_height: u8,
    bridge_protrusion1: u8,
    bridge_protrusion2: u8,
    nose_bridge_width: u8,
    nose_height: u8,
    nose_slant: u8,
    unk0x6c: [u8; 64],
    head_size: u8,
    chest_size: u8,
    abdomen_size: u8,
    arms_size: u8,
    legs_size: u8,
    unk0xb1: [u8; 2],
    skin_color_r: u8,
    skin_color_g: u8,
    skin_color_b: u8,
    skin_luster: u8,
    pores: u8,
    stubble: u8,
    dark_circles: u8,
    dark_circle_color_r: u8,
    dark_circle_color_g: u8,
    dark_circle_color_b: u8,
    cheeks_color_intensity: u8,
    cheek_color_r: u8,
    cheek_color_g: u8,
    cheek_color_b: u8,
    eyeliner: u8,
    eyeliner_color_r: u8,
    eyeliner_color_g: u8,
    eyeliner_color_b: u8,
    eye_shadow_lower: u8,
    eye_shadow_lower_color_r: u8,
    eye_shadow_lower_color_g: u8,
    eye_shadow_lower_color_b: u8,
    eye_shadow_upper: u8,
    eye_shadow_upper_color_r: u8,
    eye_shadow_upper_color_g: u8,
    eye_shadow_upper_color_b: u8,
    lipstick: u8,
    lipstick_color_r: u8,
    lipstick_color_g: u8,
    lipstick_color_b: u8,
    tatto_markposition_horizontal: u8,
    tatto_markposition_vertical: u8,
    tatto_markangle: u8,
    tatto_markexpansion: u8,
    tatto_mark_color_r: u8,
    tatto_mark_color_g: u8,
    tatto_mark_color_b: u8,
    unk0xd8: u8,
    tatto_mark_flip: u8,
    bodyhair: u8,
    body_hair_color_r: u8,
    body_hair_color_g: u8,
    body_hair_color_b: u8,
    right_iris_color_r: u8,
    right_iris_color_g: u8,
    right_iris_color_b: u8,
    right_iris_size: u8,
    right_eye_clouding: u8,
    right_eye_clouding_color_r: u8,
    right_eye_clouding_color_g: u8,
    right_eye_clouding_color_b: u8,
    right_eye_white_color_r: u8,
    right_eye_white_color_g: u8,
    right_eye_white_color_b: u8,
    right_eye_position: u8,
    left_iris_color_r: u8,
    left_iris_color_g: u8,
    left_iris_color_b: u8,
    left_iris_size: u8,
    left_eye_clouding: u8,
    left_eye_clouding_color_r: u8,
    left_eye_clouding_color_g: u8,
    left_eye_clouding_color_b: u8,
    left_eye_white_color_r: u8,
    left_eye_white_color_g: u8,
    left_eye_white_color_b: u8,
    left_eye_position: u8,
    hair_color_r: u8,
    hair_color_g: u8,
    hari_color_b: u8,
    luster: u8,
    hair_root_darkness: u8,
    white_hairs: u8,
    beard_color_r: u8,
    beard_color_g: u8,
    beard_color_b: u8,
    beard_luster: u8,
    hair_root_darkness2: u8,
    beard_white_hairs: u8,
    brow_color_r: u8,
    brow_color_g: u8,
    brow_color_b: u8,
    brow_luster: u8,
    brow_rootdarkness: u8,
    brow_whitehairs: u8,
    eyeleash_color_r: u8,
    eyeleash_color_g: u8,
    eyeleash_color_b: u8,
    eyepatch_color_r: u8,
    eyepatch_color_g: u8,
    eyepatch_color_b: u8,
    unk0x10e: [u8; 0x12],

    #[deku(skip, cond = "in_profile_summary")]
    unk0x124: bool,
    #[deku(skip, cond = "in_profile_summary")]
    unk0x125: u16,
    #[deku(skip, cond = "in_profile_summary")]
    unk0x127: u64,
}

// Gestures
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct Gestures {
    #[deku(count = "0x40")]
    ids: Vec<u32>,
}

// Regions
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct Regions {
    pub(crate) count: u32,
    #[deku(count = "*count")]
    pub(crate) ids: Vec<u32>,
}

// Ride Game Data
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct RideGameData {
    pub(crate) coordinates: FloatVector3,
    pub(crate) map_id: MapId,
    pub(crate) angle: FloatVector4,
    pub(crate) hp: i32,
    pub(crate) state: u32,
}

// BloodStain
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct BloodStain {
    pub(crate) coordinates: FloatVector3,
    pub(crate) angle: FloatVector4,
    unk0x1c: u32,
    unk0x20: u32,
    unk0x24: u32,
    unk0x28: u32,
    unk0x2c: u32,
    unk0x30: i32,
    pub(crate) souls: i32,
    pub(crate) map_id: MapId,
    unk0x3c: u32,
    unk0x38: u32,
}

// Menu Save Load
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct MenuSaveLoad {
    unk0x0: u16,
    unk0x2: u16,
    pub(crate) size: u32,
    #[deku(count = "*size")]
    pub(crate) data: Vec<u8>,
}

// Trophy Equip Data
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct TrophyEquipData {
    unk0x0: u32,
    unk0x4: [u8; 0x10],
    unk0x14: [u8; 0x10],
    unk0x24: [u8; 0x10],
}

// Gaitem Data
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct GaitemGameDataEntry {
    pub(crate) id: u32,
    #[deku(pad_bytes_after = "3")]
    unk0x4: u8,
    pub(crate) next_item_id: u32,
    #[deku(pad_bytes_after = "3")]
    unk0xc: u8,
}
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct GaitemGameData {
    pub(crate) count: i64,
    #[deku(count = "7000")]
    pub(crate) entries: Vec<GaitemGameDataEntry>,
}

// Tutorial Data
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, total_count: u32")]
pub(crate) struct TutorialDataChunk {
    pub(crate) count: u32,
    #[deku(skip, cond = "*count == 0", count = "(total_count-0x4)/4")]
    pub(crate) ids: Vec<u32>,
}
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct TutorialData {
    unk0x0: u16,
    unk0x2: u16,
    pub(crate) size: u32,
    #[deku(ctx = "*size")]
    pub(crate) data: TutorialDataChunk,
}

// Event Flags
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct EventFlag {
    #[deku(bits = 1)]
    pub(crate) bit_0: u8,
    #[deku(bits = 1)]
    pub(crate) bit_1: u8,
    #[deku(bits = 1)]
    pub(crate) bit_2: u8,
    #[deku(bits = 1)]
    pub(crate) bit_3: u8,
    #[deku(bits = 1)]
    pub(crate) bit_4: u8,
    #[deku(bits = 1)]
    pub(crate) bit_5: u8,
    #[deku(bits = 1)]
    pub(crate) bit_6: u8,
    #[deku(bits = 1)]
    pub(crate) bit_7: u8,
}

// Field Area
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct FieldArea {
    pub(crate) size: i32,
    #[deku(bytes_read = "size")]
    pub(crate) data: Option<Vec<u32>>,
}

// World Area
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct WorldBlockChrData {
    magic: [u8; 4],
    pub(crate) map_id: MapId,
    pub(crate) size: i32,
    unk0xc: u32,
    #[deku(skip, cond = "*size < 1", count = "*size - 0x10")]
    pub(crate) data: Vec<u8>,
}
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct WorldAreaChrData {
    magic: [u8; 4],
    #[deku(assert = "*unk_0x21042700 == 0x21042700 || *unk_0x21042700 == 0")]
    unk_0x21042700: u32,
    unk0x8: u32,
    unk0xc: u32,
    #[deku(until = "|d: &WorldBlockChrData| d.size < 1")]
    pub(crate) data: Vec<WorldBlockChrData>,
}
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct WorldArea {
    pub(crate) size: i32,
    pub(crate) data: WorldAreaChrData,
}

// World Geom Man
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct WorldGeomDataChunk {
    map_id: MapId,
    pub(crate) size: i32,
    unk_0x8: u64,
    #[deku(skip, cond = "*size < 1", count = "*size-0x10")]
    pub(crate) data: Vec<u8>,
}
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct WorldGeomData {
    magic: [u8; 4],
    unk_0x4: u32,
    #[deku(until = "|d: &WorldGeomDataChunk| d.size < 1")]
    pub(crate) data: Vec<WorldGeomDataChunk>,
}
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct WorldGeomMan {
    pub(crate) size: i32,
    pub(crate) data: WorldGeomData,
}

// RendMan
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, size: i32")]
pub(crate) struct StageManEntry {
    #[deku(skip, cond = "size < 1", count = "size")]
    pub(crate) data: Vec<u8>,
}
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, size: i32")]
pub(crate) struct StageMan {
    count: i32,
    #[deku(skip, cond = "*count < 1", count = "*count", ctx = "(size-4)/(*count)")]
    pub(crate) data: Vec<StageManEntry>,
}
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct RendMan {
    pub(crate) size: i32,
    #[deku(ctx = "*size")]
    pub(crate) data: StageMan,
}

// Player Coordinates
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct PlayerCoordinates {
    pub(crate) coordinates: FloatVector3,
    pub(crate) map_id: MapId,
    pub(crate) angle: FloatVector4,
    #[deku(assert = "*game_man_0xbf0 == 0 || *game_man_0xbf0 == 1")]
    game_man_0xbf0: u8,
    pub(crate) unk_coordinates: FloatVector3,
    pub(crate) unk_angle: FloatVector4,
}

// NetMan
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct NetMan {
    #[deku(assert = "*unk0x0 == 2 || *unk0x0 == 0")]
    unk0x0: u32,
    #[deku(count = "0x20000")]
    pub(crate) data: Vec<u8>,
}

// World Area Weather
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct WorldAreaWeather {
    pub(crate) area_id: u16,
    pub(crate) weather_type: u16,
    pub(crate) timer: u32,
    padding: u32,
}

// World Area Time
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct WorldAreaTime {
    pub(crate) hour: u32,
    pub(crate) minute: u32,
    pub(crate) second: u32,
}

// Base Version
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct BaseVersion {
    pub(crate) base_version_copy: u32,
    pub(crate) base_version: u32,
    #[deku(assert = "*is_latest_version == 0 || *is_latest_version == 1")]
    pub(crate) is_latest_version: u32,
    unk0xc: u32,
}

// PS5Activity
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct PS5Activity {
    data: [u8; 0x20],
}

// DLC
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct DLC {
    data: [u8; 0x32],
}

// Player Data Hash
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub(crate) struct PlayerDataHash {
    pub(crate) level: u32,
    pub(crate) stats: u32,
    pub(crate) archetype: u32,
    pub(crate) playergame_data_0xc0: u32,
    pub(crate) padding: u32,
    pub(crate) souls: u32,
    pub(crate) souls_memory: u32,
    pub(crate) equipped_weapons: u32,
    pub(crate) equipped_armors_and_talismans: u32,
    pub(crate) equipped_items: u32,
    pub(crate) equipped_spells: u32,

    #[deku(count = "0x54")]
    rest: Vec<u8>,
}
