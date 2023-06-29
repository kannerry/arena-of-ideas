use strum_macros::EnumString;

use super::*;

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct House {
    pub name: HouseName,
    pub color: Rgba<f32>,
    pub abilities: HashMap<AbilityName, Ability>,
    #[serde(default)]
    pub statuses: HashMap<String, Status>,
}

#[derive(
    Deserialize,
    Serialize,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Clone,
    Copy,
    enum_iterator::Sequence,
    EnumString,
    strum_macros::Display,
)]
pub enum HouseName {
    Medics,

    Test,
}

impl HouseName {
    pub fn get_color(&self, resources: &Resources) -> Rgba<f32> {
        HousePool::get_color(self, resources)
    }
}

impl FileWatcherLoader for House {
    fn load(resources: &mut Resources, path: &PathBuf, watcher: &mut FileWatcherSystem) {
        watcher.watch_file(path, Box::new(Self::load));
        debug!("Load house {:?}", path);
        let house: House = futures::executor::block_on(load_json(path)).unwrap();
        house.statuses.iter().for_each(|(name, status)| {
            let mut status = status.clone();
            status.color = house.color;
            StatusLibrary::register(name, status, resources);
        });
        house.abilities.iter().for_each(|(name, ability)| {
            AbilityPool::define_ability(name, ability, house.color, house.name, resources);
        });
        HousePool::insert_house(house.name, house, resources);
    }
}
