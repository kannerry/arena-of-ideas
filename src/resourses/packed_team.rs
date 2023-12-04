use super::*;

#[derive(Deserialize, Serialize, TypeUuid, TypePath, Debug, Clone, Default)]
#[uuid = "cb5457bc-b429-4af8-8d92-bf141a80020b"]
pub struct PackedTeam {
    pub units: Vec<PackedUnit>,
    #[serde(default)]
    pub state: VarState,
}

impl PackedTeam {
    pub fn new(units: Vec<PackedUnit>) -> Self {
        Self {
            units,
            state: default(),
        }
    }
    pub fn pack(faction: Faction, world: &mut World) -> Self {
        let state = VarState::get(Self::entity(faction, world).unwrap(), world).clone();
        let units = UnitPlugin::collect_factions(HashSet::from([faction]), world)
            .into_iter()
            .map(|(u, _)| PackedUnit::pack(u, world))
            .sorted_by_key(|u| u.state.get_int(VarName::Slot).unwrap_or_default())
            .collect_vec();
        PackedTeam { units, state }
    }
    pub fn unpack(mut self, faction: Faction, world: &mut World) {
        self.state
            .init(VarName::Faction, VarValue::Faction(faction));
        let team = Self::spawn(faction, world);
        self.state.attach(team, world);
        for (i, unit) in self.units.into_iter().enumerate() {
            unit.unpack(team, Some(i + 1), world);
        }
    }
    pub fn spawn(faction: Faction, world: &mut World) -> Entity {
        Self::despawn(faction, world);
        let team = world
            .spawn((
                VarState::new_with(VarName::Faction, VarValue::Faction(faction)),
                Team,
                Transform::default(),
                GlobalTransform::default(),
                VisibilityBundle::default(),
            ))
            .id();
        match faction {
            Faction::Team => {
                for slot in 1..=10 {
                    UnitPlugin::spawn_slot(slot, Faction::Team, world);
                }
            }
            _ => {}
        }
        team
    }
    pub fn despawn(faction: Faction, world: &mut World) {
        if let Some(team) = Self::entity(faction, world) {
            world.entity_mut(team).despawn_recursive();
        }
    }
    pub fn entity(faction: Faction, world: &mut World) -> Option<Entity> {
        world
            .query_filtered::<(Entity, &VarState), With<Team>>()
            .iter(world)
            .find_map(
                |(e, s)| match s.get_faction(VarName::Faction).unwrap().eq(&faction) {
                    true => Some(e),
                    false => None,
                },
            )
    }
    pub fn state(faction: Faction, world: &mut World) -> Option<&VarState> {
        Self::entity(faction, world).and_then(|e| Some(VarState::get(e, world)))
    }
    pub fn state_mut(faction: Faction, world: &mut World) -> Option<Mut<VarState>> {
        Self::entity(faction, world).and_then(|e| Some(VarState::get_mut(e, world)))
    }
}

impl ToString for PackedTeam {
    fn to_string(&self) -> String {
        let mut result = String::with_capacity(30);
        let mut i = 0;
        while i < self.units.len() {
            if !result.is_empty() {
                result.push_str(", ");
            }
            let name = self.units[i].name.clone();
            let statuses = self.units[i].statuses_string();
            let mut count = 1;
            for c in i + 1..self.units.len() {
                count = c - i + 1;
                if !self.units[c].name.eq(&name) || !self.units[c].statuses_string().eq(&statuses) {
                    break;
                }
            }
            if count > 1 {
                result.push_str(&format!("{name} x{count}"));
            } else {
                result.push_str(&format!("{name}"));
            }
            if !statuses.is_empty() {
                result.push_str(&format!(" with {statuses}"));
            }
            i += count;
        }
        result
    }
}

#[derive(Component)]
pub struct Team;