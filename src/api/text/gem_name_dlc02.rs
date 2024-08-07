use std::{collections::HashMap, sync::LazyLock};

pub static GEM_NAME_DLC02: LazyLock<HashMap<u32, &'static str>> = LazyLock::new(|| {
    HashMap::from([
		(10,"DLC dummy"),
])});
