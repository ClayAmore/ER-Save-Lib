use std::{collections::HashMap, sync::LazyLock};

pub static GEM_EFFECT_DLC02: LazyLock<HashMap<u32, &'static str>> = LazyLock::new(|| {
    HashMap::from([
		(0,"DLC dummy"),
])});
