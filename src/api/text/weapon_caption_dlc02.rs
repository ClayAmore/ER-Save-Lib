use std::{collections::HashMap, sync::LazyLock};

pub static WEAPON_CAPTION_DLC02: LazyLock<HashMap<u32, &'static str>> = LazyLock::new(|| {
    HashMap::from([
		(1,"DLC dummy"),
])});
