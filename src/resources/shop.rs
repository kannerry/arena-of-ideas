use geng::prelude::itertools::Itertools;

use super::*;

pub const INITIAL_POOL_COUNT_PER_HERO: usize = 5;

#[derive(Default)]
pub struct Shop {
    pub pool: Vec<PackedUnit>,
    pub level_extensions: Vec<Vec<PackedUnit>>,
    pub money: usize,
    pub drop_entity: Option<legion::Entity>,
    pub drag_entity: Option<legion::Entity>,
    pub refresh_btn: Option<legion::Entity>,
    pub load_new_hero: bool,
}

impl Shop {
    pub fn load_pool(resources: &mut Resources) {
        resources.shop.pool.clear();
        let mut sorted_by_power = VecDeque::from_iter(resources.hero_pool.all_sorted());
        let heroes_per_extension = (sorted_by_power.len() as f32
            / (10.0_f32.min(resources.floors.count() as f32)))
        .ceil() as usize;
        let mut cur_level = 0;
        resources.shop.level_extensions = vec![default()];
        while let Some(unit) = sorted_by_power.pop_front() {
            if resources
                .shop
                .level_extensions
                .get(cur_level)
                .unwrap()
                .len()
                >= heroes_per_extension
                    + (cur_level == 0) as usize * resources.options.initial_shop_fill
            {
                cur_level += 1;
                resources.shop.level_extensions.push(default());
            }
            resources
                .shop
                .level_extensions
                .get_mut(cur_level)
                .unwrap()
                .push(unit);
        }
        resources
            .shop
            .level_extensions
            .iter()
            .for_each(|x| debug!("{}", x.iter().map(|x| x.to_string()).join(", ")));
    }

    pub fn load_level(resources: &mut Resources, level: usize) {
        if let Some(new_units) = resources.shop.level_extensions.get(level) {
            resources.shop.pool.extend(
                new_units
                    .iter()
                    .map(|unit| {
                        (0..INITIAL_POOL_COUNT_PER_HERO)
                            .map(|_| unit.clone())
                            .collect_vec()
                    })
                    .flatten(),
            )
        }
    }
}
