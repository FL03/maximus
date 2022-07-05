/*
   Appellation: interface
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Interface;

impl Interface {
    pub fn constructor() -> Self {
        Self
    }
    pub fn new() -> Self {
        Self::constructor()
    }
}

impl std::fmt::Display for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Interface()",)
    }
}
