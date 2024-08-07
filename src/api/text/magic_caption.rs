use std::{collections::HashMap, sync::LazyLock};

pub static MAGIC_CAPTION: LazyLock<HashMap<u32, &'static str>> = LazyLock::new(|| {
    HashMap::from([
		(0,"<<NEWLINE>>"),
])});
