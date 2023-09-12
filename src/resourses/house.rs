use super::*;

#[derive(Serialize, Deserialize, Clone, Debug, TypeUuid, TypePath)]
#[uuid = "96ef0a80-906b-458c-8c3e-3d91cfb62276"]
pub struct House {
    pub name: String,
    pub color: HexColor,
    pub abilities: Vec<Ability>,
    pub statuses: Vec<PackedStatus>,
}
