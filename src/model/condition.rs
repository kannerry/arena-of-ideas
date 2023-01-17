use crate::assets::{Clan};

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum Condition {
    Always,
    Not {
        condition: Box<Condition>,
    },
    UnitHasStatus {
        who: Who,
        #[serde(rename = "status")]
        status_type: StatusName,
    },
    InRange {
        max_distance: Coord,
    },
    Chance {
        percent: Expr,
    },
    Equal {
        a: Box<Expr>,
        b: Box<Expr>,
    },
    Less {
        a: Box<Expr>,
        b: Box<Expr>,
    },
    More {
        a: Box<Expr>,
        b: Box<Expr>,
    },
    ClanSize {
        clan: Clan,
        count: usize,
    },
    HasClan {
        who: Who,
        clan: Clan,
    },
    HasVar {
        name: VarName,
    },
    Faction {
        who: Who,
        faction: Faction,
    },
    And {
        conditions: Vec<Box<Condition>>,
    },
    Or {
        conditions: Vec<Box<Condition>>,
    },
    Position {
        who: Who,
        position: i64,
    },
}

impl Default for Condition {
    fn default() -> Self {
        Self::Always
    }
}
