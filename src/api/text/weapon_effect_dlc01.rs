use std::{collections::HashMap, sync::LazyLock};

pub static WEAPON_EFFECT_DLC01: LazyLock<HashMap<u32, &'static str>> = LazyLock::new(|| {
    HashMap::from([
		(1200,"Boosts finger sorcery"),
		(1201,"Boosts Miquella's incantations"),
		(1202,"Boosts Messmer's flame incantations"),
		(1203,"Boosts spiral incantations"),
		(6800,"Causes eternal sleep buildup (<?sleepATKpwr?>)"),
])});
