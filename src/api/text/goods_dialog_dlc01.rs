use std::{collections::HashMap, sync::LazyLock};

pub static GOODS_DIALOG_DLC01: LazyLock<HashMap<u32, &'static str>> = LazyLock::new(|| {
    HashMap::from([
		(10010850,"Use <?gdsparam@2002900?>?"),
		(10010851,"Use <?gdsparam@2002901?>?"),
		(10010852,"Use <?gdsparam@2002902?>?"),
		(10010853,"Use <?gdsparam@2002903?>?"),
		(10010854,"Use <?gdsparam@2002904?>?"),
		(10010855,"Use <?gdsparam@2002905?>?"),
		(10010856,""),
		(10010857,"Use <?gdsparam@2002907?>?"),
		(10010858,"Use <?gdsparam@2002908?>?"),
		(10010859,"Use <?gdsparam@2002909?>?"),
		(10010860,"Use <?gdsparam@2002910?>?"),
		(20000810,"The spectral steed is frightened and cannot be summoned."),
])});
