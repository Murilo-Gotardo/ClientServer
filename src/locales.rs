use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Locale {
    pub(crate) locate: String,
    pub(crate) status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Locales {
    pub(crate) locales: Vec<Locale>,
}