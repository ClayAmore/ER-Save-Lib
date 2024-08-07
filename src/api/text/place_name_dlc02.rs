use std::{collections::HashMap, sync::LazyLock};

pub static PLACE_NAME_DLC02: LazyLock<HashMap<u32, &'static str>> = LazyLock::new(|| {
    HashMap::from([
		(90000000,"DLC dummy"),
])});
