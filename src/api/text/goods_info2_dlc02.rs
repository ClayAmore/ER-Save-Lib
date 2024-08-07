use std::{collections::HashMap, sync::LazyLock};

pub static GOODS_INFO2_DLC02: LazyLock<HashMap<u32, &'static str>> = LazyLock::new(|| {
    HashMap::from([
		(1760,"DLC dummy"),
])});
