use std::{collections::HashMap, sync::LazyLock};

pub static GOODS_INFO_DLC02: LazyLock<HashMap<u32, &'static str>> = LazyLock::new(|| {
    HashMap::from([
		(9999099,"DLC dummy"),
])});
