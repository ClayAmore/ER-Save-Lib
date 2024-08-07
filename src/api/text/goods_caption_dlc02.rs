use std::{collections::HashMap, sync::LazyLock};

pub static GOODS_CAPTION_DLC02: LazyLock<HashMap<u32, &'static str>> = LazyLock::new(|| {
    HashMap::from([
		(12,"DLC dummy"),
])});
