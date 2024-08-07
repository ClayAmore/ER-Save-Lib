use std::{collections::HashMap, sync::LazyLock};

pub static MAGIC_CAPTION_DLC01: LazyLock<HashMap<u32, &'static str>> = LazyLock::new(|| {
    HashMap::from([
		(1,"dummy"),
])});
