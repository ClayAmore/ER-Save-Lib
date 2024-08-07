use std::collections::HashSet;

use crate::{
    param_trait::Param, EquipParamGem::EquipParamGem, EquipParamGoods::EquipParamGoods,
    EquipParamProtector::EquipParamProtector, EquipParamWeapon::EquipParamWeapon, Save,
    SaveApiError,
};

enum ArmorCategory {
    Head = 0,
    Chest = 1,
    Arms = 2,
    Legs = 3,
}

pub(crate) struct VanillaCheck;

impl VanillaCheck {
    pub(crate) fn is_vanilla_save(save: &Save, index: usize) -> Result<bool, SaveApiError> {
        Self::validate_inventory(save, index)?;
        Self::validate_equipment(save, index)?;
        Ok(false)
    }

    fn validate_inventory(save: &Save, index: usize) -> Result<(), SaveApiError> {
        let weapon_param = save
            .user_data_11
            .regulation
            .get_param::<EquipParamWeapon>()?;
        let gem_param = save.user_data_11.regulation.get_param::<EquipParamGem>()?;
        let goods_param = save
            .user_data_11
            .regulation
            .get_param::<EquipParamGoods>()?;
        let protector_param = save
            .user_data_11
            .regulation
            .get_param::<EquipParamProtector>()?;
        let mut item_ids = HashSet::new();

        for inventory_item in save.user_data_x[index]
            .inventory_held
            .common_items
            .iter()
            .chain(
                save.user_data_x[index]
                    .inventory_storage_box
                    .common_items
                    .iter(),
            )
        {
            // If empty. Skip.
            if inventory_item.gaitem_handle == 0 {
                continue;
            }

            // Items
            if inventory_item.gaitem_handle & 0xf0000000 == 0xB {
                // If item not listed in regulation, then not valid
                if let Some(_) =
                    goods_param.get(&((inventory_item.gaitem_handle & 0x0fffffff) as i32))
                {
                    // If item already in inventory then it's a duplicate and that's not valid
                    if let Some(_existing_id) = item_ids.get(&inventory_item.gaitem_handle) {
                        return Err(SaveApiError::CheatedCharacterImport(format!(
                            "Found duplicate item in character inventory. Id: {}!",
                            _existing_id
                        )));
                    }
                    item_ids.insert(inventory_item.gaitem_handle);
                } else {
                    return Err(SaveApiError::CheatedCharacterImport(format!(
                        "Failed to find inventory item with id {} in the goods param!",
                        inventory_item.gaitem_handle & 0x0fffffff
                    )));
                }
            }

            // Armor
            if inventory_item.gaitem_handle & 0xf0000000 == 0x9 {
                // Find armor in gaitem_map. If not found means not valid.
                if let Some(gaitem_map_entry) = save.user_data_x[index]
                    .gaitem_map
                    .iter()
                    .find(|gaitem_entry| gaitem_entry.gaitem_handle == inventory_item.gaitem_handle)
                {
                    // Validate that armor piece is in regulation. If not that means armor is not valid.
                    if protector_param
                        .get(&((gaitem_map_entry.item_id & 0x0fffffff) as i32))
                        .is_none()
                    {
                    } else {
                        return Err(SaveApiError::CheatedCharacterImport(format!(
                            "Failed to find armor with id {} in the protector param!",
                            gaitem_map_entry.item_id & 0x0fffffff
                        )));
                    }
                } else {
                    return Err(SaveApiError::CheatedCharacterImport(format!(
                        "Failed to find armor with gaitem handle {} in the gaitem map. Cannot get item id of the armor!",
                        inventory_item.gaitem_handle & 0x0fffffff
                    )));
                }
            }

            // Weapon
            if inventory_item.gaitem_handle & 0xf0000000 == 0x8 {
                // Find weapon in gaitem_map. If not found means not valid.
                if let Some(gaitem_map_entry) = save.user_data_x[index]
                    .gaitem_map
                    .iter()
                    .find(|gaitem_entry| gaitem_entry.gaitem_handle == inventory_item.gaitem_handle)
                {
                    // Find weapon param from weapon id. If not found means not valid.
                    if let Some(weapon_param_entry) =
                        weapon_param.get(&((gaitem_map_entry.item_id / 100 * 100) as i32))
                    {
                        // Check for gems if no gems then valid.
                        if let Some(weapon_gem) = gaitem_map_entry.gem_gaitem_handle {
                            // If gem found but weapon is not infusable then not valid.
                            if weapon_param_entry.gemMountType == 0 {
                                return Err(SaveApiError::CheatedCharacterImport(format!(
                                    "Weapon with id {} is infused with gem but it's not infusable.",
                                    gaitem_map_entry.item_id / 100 * 100
                                )));
                            }

                            // If weapon has higher upgrade then allowed then not valid.
                            if weapon_param_entry.materialSetId == 2200
                                && gaitem_map_entry.item_id % 100 > 10
                            {
                                return Err(SaveApiError::CheatedCharacterImport(format!(
                                    "Weapon with id {} is upgraded beyond it it's limit. Current weapon level {}, Max weapon level: 10",
                                    gaitem_map_entry.item_id / 100 * 100,
                                    gaitem_map_entry.item_id % 100
                                )));
                            }

                            // Validate infusion
                            if let Some(gem_param_entry) = gem_param.get(&weapon_gem) {
                                if !Self::infusion_valid(
                                    weapon_param_entry.wepType,
                                    gem_param_entry,
                                ) {
                                    return Err(SaveApiError::CheatedCharacterImport(format!(
                                        "Weapon with id {} cannot have Ash of war with id {}!",
                                        gaitem_map_entry.item_id / 100 * 100,
                                        weapon_gem
                                    )));
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn validate_equipment(save: &Save, index: usize) -> Result<(), SaveApiError> {
        let head = save.user_data_x[index].equipped_items_gaitem_handle.head;
        if !Self::is_armor_piece_valid(save, index, head, ArmorCategory::Head)? {
            return Err(SaveApiError::CheatedCharacterImport(format!(
                "Item with id {} cannot have be equipped as head armor!",
                head,
            )));
        }

        let chest = save.user_data_x[index].equipped_items_gaitem_handle.chest;
        if !Self::is_armor_piece_valid(save, index, chest, ArmorCategory::Chest)? {
            return Err(SaveApiError::CheatedCharacterImport(format!(
                "Item with id {} cannot have be equipped as chest armor!",
                chest,
            )));
        }

        let arms = save.user_data_x[index].equipped_items_gaitem_handle.arms;
        if !Self::is_armor_piece_valid(save, index, arms, ArmorCategory::Arms)? {
            return Err(SaveApiError::CheatedCharacterImport(format!(
                "Item with id {} cannot have be equipped as arms armor!",
                arms,
            )));
        }

        let legs = save.user_data_x[index].equipped_items_gaitem_handle.legs;
        if !Self::is_armor_piece_valid(save, index, legs, ArmorCategory::Legs)? {
            return Err(SaveApiError::CheatedCharacterImport(format!(
                "Item with id {} cannot have be equipped as legs armor!",
                legs,
            )));
        }

        Ok(())
    }

    fn is_armor_piece_valid(
        save: &Save,
        index: usize,
        gaitem_handle: u32,
        category: ArmorCategory,
    ) -> Result<bool, SaveApiError> {
        let protector_param = save
            .user_data_11
            .regulation
            .get_param::<EquipParamProtector>()?;
        if save.user_data_x[index]
            .inventory_held
            .common_items
            .iter()
            .any(|inventory_item| inventory_item.gaitem_handle == gaitem_handle)
        {
            let armor_id = save.user_data_x[index]
                .gaitem_map
                .iter()
                .find(|gaitem_map_entry| gaitem_map_entry.gaitem_handle == gaitem_handle)
                .unwrap()
                .item_id;
            let armor_param = &protector_param[&((armor_id & 0x0fffffff) as i32)];
            return Ok(match category {
                ArmorCategory::Head => {
                    armor_param.protectorCategory == 4 || armor_param.protectorCategory == 0
                }
                ArmorCategory::Chest => armor_param.protectorCategory == 1,
                ArmorCategory::Arms => armor_param.protectorCategory == 2,
                ArmorCategory::Legs => armor_param.protectorCategory == 3,
            });
        }

        Ok(false)
    }

    fn infusion_valid(wep_type: i16, gem_param: &<EquipParamGem as Param>::ParamType) -> bool {
        match wep_type {
            0 => true,
            1 => gem_param.canMountWep_Dagger == 1,
            3 => gem_param.canMountWep_SwordNormal == 1,
            5 => gem_param.canMountWep_SwordLarge == 1,
            7 => gem_param.canMountWep_SwordGigantic == 1,
            9 => gem_param.canMountWep_SaberNormal == 1,
            11 => gem_param.canMountWep_SaberLarge == 1,
            13 => gem_param.canMountWep_katana == 1,
            14 => gem_param.canMountWep_SwordDoubleEdge == 1,
            15 => gem_param.canMountWep_SwordPierce == 1,
            16 => gem_param.canMountWep_RapierHeavy == 1,
            17 => gem_param.canMountWep_AxeNormal == 1,
            19 => gem_param.canMountWep_AxeLarge == 1,
            21 => gem_param.canMountWep_HammerNormal == 1,
            23 => gem_param.canMountWep_HammerLarge == 1,
            24 => gem_param.canMountWep_Flail == 1,
            25 => gem_param.canMountWep_SpearNormal == 1,
            28 => gem_param.canMountWep_SpearHeavy == 1,
            29 => gem_param.canMountWep_SpearAxe == 1,
            31 => gem_param.canMountWep_Sickle == 1,
            35 => gem_param.canMountWep_Knuckle == 1,
            37 => gem_param.canMountWep_Claw == 1,
            39 => gem_param.canMountWep_Whip == 1,
            41 => gem_param.canMountWep_AxhammerLarge == 1,
            50 => gem_param.canMountWep_BowSmall == 1,
            51 => gem_param.canMountWep_BowNormal == 1,
            53 => gem_param.canMountWep_BowLarge == 1,
            55 => gem_param.canMountWep_ClossBow == 1,
            56 => gem_param.canMountWep_Ballista == 1,
            57 => gem_param.canMountWep_Staff == 1,
            61 => gem_param.canMountWep_Talisman == 1,
            65 => gem_param.canMountWep_ShieldSmall == 1,
            67 => gem_param.canMountWep_ShieldNormal == 1,
            69 => gem_param.canMountWep_ShieldLarge == 1,
            87 => gem_param.canMountWep_Torch == 1,
            88 => gem_param.canMountWep_HandToHand == 1,
            90 => gem_param.canMountWep_ThrustingShield == 1,
            91 => gem_param.canMountWep_ThrowingWeapon == 1,
            92 => gem_param.canMountWep_ReverseHandSword == 1,
            93 => gem_param.canMountWep_LightGreatsword == 1,
            94 => gem_param.canMountWep_GreatKatana == 1,
            95 => gem_param.canMountWep_BeastClaw == 1,
            _ => true,
        }
    }
}
