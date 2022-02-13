use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Texts {
    pub actors: ActorsText,
    pub first_period: FirstPeriodTexts,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActorsText {
    pub fda: String,
    pub ttandme: String,
    pub google: String,
    pub scientists: String,
    pub anne_wo1: String,
    pub anne_wo2: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FirstPeriodTexts {
    introduction: String,
    commercialisation: String,
}

pub fn load_texts() -> Texts {
    let file = fs::read_to_string("static/texts.ron").expect("Couldn't load texts file");

    ron::from_str(&file).expect("couldn't deserialize ron file")
}
