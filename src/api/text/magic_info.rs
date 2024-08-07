use std::{collections::HashMap, sync::LazyLock};

pub static MAGIC_INFO: LazyLock<HashMap<u32, &'static str>> = LazyLock::new(|| {
    HashMap::from([
		(0,""),
])});
