use std::{collections::HashMap, sync::LazyLock};

pub static GEM_EFFECT: LazyLock<HashMap<u32, &'static str>> = LazyLock::new(|| {
    HashMap::from([
		(0,""),
])});
