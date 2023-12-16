use super::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LadderClimb {
    pub team: PackedTeam,
    pub levels: Vec<String>,
    pub owner_team: Option<PackedTeam>,
    pub defeated: usize,
    pub shop: ShopState,
}
