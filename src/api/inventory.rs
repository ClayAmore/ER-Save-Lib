use crate::SaveApiError;

pub const KEY_ITEM_CAPACITY: (usize, usize) = (384, 128);

pub enum StorageType {
    Held,
    StorageBox,
}

pub enum StorageItemType {
    Regular,
    Key,
}

#[derive(Default, Debug, PartialEq, Clone)]
pub enum ItemType {
    #[default]
    None,
    Weapon,
    Armor,
    Accessory,
    Item,
    Aow,
}
impl ItemType {
    #[allow(unused)]
    pub(crate) fn from_item_id(id: u32) -> Result<Self, SaveApiError> {
        let item_type = (id & 0xf0000000) >> 28;
        match item_type {
            0x0 => Ok(ItemType::Weapon),
            0x1 => Ok(ItemType::Armor),
            0x2 => Ok(ItemType::Accessory),
            0x4 => Ok(ItemType::Item),
            0x8 => Ok(ItemType::Aow),
            _ => Err(SaveApiError::UnkownItemType(id)),
        }
    }
    pub(crate) fn from_gaitem_id(id: u32) -> Result<Self, SaveApiError> {
        let item_type = (id & 0xf0000000) >> 28;
        match item_type {
            0x0 => Ok(ItemType::None),
            0x8 => Ok(ItemType::Weapon),
            0x9 => Ok(ItemType::Armor),
            0xa => Ok(ItemType::Accessory),
            0xb => Ok(ItemType::Item),
            0xc => Ok(ItemType::Aow),
            _ => Err(SaveApiError::UnkownItemType(id)),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Item {
    pub item_type: ItemType,
    pub gaitem_handle: u32,
    pub item_id: u32,
    pub item_name: String,
    pub quantity: u32,
    pub equip_index: usize,
    pub aqcuistion_index: u32,
    pub is_dlc_item: bool,
}
