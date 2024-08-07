use std::{collections::HashMap, sync::LazyLock};

pub static PROTECTOR_INFO_DLC01: LazyLock<HashMap<u32, &'static str>> = LazyLock::new(|| {
    HashMap::from([
		(200,"DLC dummy"),
])});
