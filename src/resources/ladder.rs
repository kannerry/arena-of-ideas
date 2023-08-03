use super::*;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Ladder {
    #[serde(skip)]
    current: usize,
    pub base: Vec<ReplicatedTeam>,
    #[serde(skip)]
    pub levels: Vec<ReplicatedTeam>,
}

impl Ladder {
    pub fn start_next_battle(world: &mut legion::World, resources: &mut Resources) {
        let light = PackedTeam::pack(Faction::Team, world, resources);
        let dark = Self::current_team(resources);
        BattleSystem::init_battle(&light, &dark.into(), world, resources);
        GameStateSystem::set_transition(GameState::Battle, resources);
    }

    pub fn generate_next_level(
        team: &PackedTeam,
        world: &mut legion::World,
        resources: &mut Resources,
    ) {
        let mut templates = vec![];
        for _ in 0..20 {
            templates.push(EnemyPool::get_random_unit(resources));
        }
        let new_enemy = RatingSystem::generate_weakest_opponent(team, templates, world, resources);
        dbg!(&new_enemy);
        Ladder::push_level(new_enemy, resources);
        Ladder::save(resources);
    }

    pub fn get_score(world: &legion::World) -> usize {
        (UnitSystem::collect_faction(world, Faction::Dark).len() == 0) as usize
    }

    // pub fn set_teams(teams: Vec<PackedTeam>, resources: &mut Resources) {
    //     resources.ladder.levels = teams.into_iter().map(|x| x.into()).collect_vec();
    // }

    pub fn save(resources: &Resources) {
        SaveSystem::save_ladder(resources);
    }

    pub fn current_team(resources: &Resources) -> PackedTeam {
        let ind = Self::current_ind(resources);
        resources
            .ladder
            .base
            .iter()
            .chain(resources.ladder.levels.iter())
            .skip(ind)
            .next()
            .unwrap()
            .clone()
            .into()
    }

    pub fn set_levels(teams: Vec<ReplicatedTeam>, resources: &mut Resources) {
        resources.ladder.levels = teams;
    }

    pub fn get_levels(resources: &Resources) -> Vec<ReplicatedTeam> {
        resources.ladder.levels.clone()
    }

    pub fn current_ind(resources: &Resources) -> usize {
        resources.ladder.current
    }

    pub fn reset(resources: &mut Resources) {
        resources.ladder.current = default();
    }

    pub fn is_last_level(resources: &Resources) -> bool {
        resources.ladder.current + 1 == Self::count(resources)
    }

    pub fn next(resources: &mut Resources) -> bool {
        resources.ladder.current += 1;
        resources.ladder.current < Self::count(resources)
    }

    pub fn need_new_level(resources: &Resources) -> bool {
        resources.ladder.current == Self::count(resources)
    }

    pub fn count(resources: &Resources) -> usize {
        resources.ladder.levels.len() + resources.ladder.base.len()
    }

    pub fn set_level(ind: usize, resources: &mut Resources) {
        resources.ladder.current = ind;
    }

    pub fn push_level(team: ReplicatedTeam, resources: &mut Resources) {
        resources.ladder.levels.push(team);
    }
}

impl FileWatcherLoader for Ladder {
    fn load(resources: &mut Resources, path: &PathBuf, watcher: &mut FileWatcherSystem) {
        watcher.watch_file(path, Box::new(Self::load));
        debug!("Load ladder {path:?}");
        resources.ladder = futures::executor::block_on(load_json(path)).unwrap();
    }
}
