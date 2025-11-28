use serde::Deserialize;

/// A universal query parameter for getting json response
#[derive(Deserialize)]
#[allow(dead_code)] // 允许结构体中的字段不被读取
pub struct GetJsonParam {
    /// simpleified version of json response
    pub simple: Option<bool>,
    /// force parse the wz file alogn the way
    pub force_parse: Option<bool>,
    /// sort the json response
    pub sort: Option<bool>,
    pub resolve_uol: Option<bool>,
    /// how long to cache for resoponse header
    pub cache: Option<u32>,
}

#[derive(Deserialize)]
pub struct GetEquipListParam {
    pub extra: Option<bool>,
}
