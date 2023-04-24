use geng::prelude::itertools::Itertools;

use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PackedTeam {
    pub units: Vec<PackedUnit>,
    #[serde(default = "default_team_name")]
    pub name: String,
    #[serde(default)]
    pub vars: Vars,
    #[serde(default)]
    pub ability_vars: HashMap<AbilityName, Vars>,
    #[serde(default)]
    pub statuses: HashMap<String, i32>,
    #[serde(default = "default_team_slots")]
    pub slots: usize,
}

fn default_team_name() -> String {
    "unnamed".to_owned()
}

fn default_team_slots() -> usize {
    3
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ReplicatedTeam {
    #[serde(flatten)]
    pub team: PackedTeam,
    #[serde(default)]
    pub replications: usize,
}

impl From<ReplicatedTeam> for PackedTeam {
    fn from(value: ReplicatedTeam) -> Self {
        let mut team = value.team;
        let size = team.units.len();
        for _ in 1..value.replications {
            for i in 0..size {
                team.units.push(team.units[i].clone())
            }
        }
        team.slots = team.units.len();
        team
    }
}

impl PackedTeam {
    pub fn new(name: String, units: Vec<PackedUnit>) -> Self {
        Self {
            units,
            name,
            vars: default(),
            ability_vars: default(),
            statuses: default(),
            slots: default(),
        }
    }

    pub fn unpack(
        &self,
        faction: &Faction,
        world: &mut legion::World,
        resources: &mut Resources,
    ) -> legion::Entity {
        let mut entities = vec![];
        let mut state = ContextState::new(self.name.clone(), Some(WorldSystem::entity(world)));
        state.ability_vars = self.ability_vars.clone();
        state.vars = self.vars.clone();
        state.statuses = self.statuses.clone();
        state.vars.set_int(&VarName::Slots, self.slots as i32);
        state.vars.set_faction(&VarName::Faction, *faction);
        resources.logger.log(
            || {
                format!(
                    "Unpack team {} {:?} {}",
                    self,
                    &state,
                    entities.iter().map(|x| format!("{:?}", x)).join(", ")
                )
            },
            &LogContext::UnitCreation,
        );
        let team = world.push((TeamComponent {}, state));
        world
            .entry(team)
            .unwrap()
            .add_component(EntityComponent::new(team));
        for (slot, unit) in self.units.iter().enumerate() {
            entities.push(unit.unpack(world, resources, slot + 1, None, team));
        }
        team
    }

    pub fn pack(faction: &Faction, world: &legion::World, resources: &Resources) -> PackedTeam {
        let units = UnitSystem::collect_faction_states(world, *faction)
            .iter()
            .sorted_by_key(|(_, state)| state.vars.get_int(&VarName::Slot))
            .map(|(entity, _)| PackedUnit::pack(*entity, world, resources))
            .collect_vec();
        let state = TeamSystem::get_state(faction, world);
        let name = state.name.clone();
        let vars = state.vars.clone();
        let ability_vars = state.ability_vars.clone();
        let statuses = state.statuses.clone();
        let slots = state.vars.get_int(&VarName::Slots) as usize;
        PackedTeam {
            units,
            name,
            vars,
            ability_vars,
            statuses,
            slots,
        }
    }
}

impl fmt::Display for PackedTeam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = self.units.iter().map(|x| x.to_string()).join(", ");
        write!(f, "{}[{}]", self.name, text)
    }
}