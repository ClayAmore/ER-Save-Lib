use std::{collections::HashMap, sync::LazyLock};

pub static GOODS_DIALOG_DLC02: LazyLock<HashMap<u32, &'static str>> = LazyLock::new(|| {
    HashMap::from([
		(10010670,"DLC dummy"),
])});
