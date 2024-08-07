use std::{collections::HashMap, sync::LazyLock};

pub static WEAPON_INFO_DLC01: LazyLock<HashMap<u32, &'static str>> = LazyLock::new(|| {
    HashMap::from([
		(4530000,""),
		(4540000,""),
		(4550000,""),
		(50540000,"Fletched bone arrow with putrescence-soaked tip"),
		(50550000,"Animal bone arrow with putrescence-soaked tip"),
		(52520000,"Animal bone bolt with putrescence-soaked tip"),
])});
