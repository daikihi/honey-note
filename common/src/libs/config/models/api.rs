use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Api {
    pub honey: Honey,
    pub prefecture: Prefecture,
    pub beekeeper: Beekeeper,
}

#[derive(Debug, Deserialize)]
pub struct Honey {}

#[derive(Debug, Deserialize)]
pub struct Prefecture {}

#[derive(Debug, Deserialize)]
pub struct Beekeeper {}
