use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Locale {
    pub(crate) locate: String,
    pub(crate) status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Locales {
    pub(crate) locale_list: Vec<Locale>,
}