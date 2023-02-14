use geng::prelude::itertools::Itertools;

use super::*;

pub struct Cassette {
    pub head: Time,
    queue: Vec<CassetteNode>,
    pub node_template: CassetteNode, // any new node will be cloned from this
    pub parallel_node: CassetteNode,
}

impl Default for Cassette {
    fn default() -> Self {
        Self {
            head: default(),
            queue: vec![default()],
            node_template: default(),
            parallel_node: default(),
        }
    }
}

impl Cassette {
    pub fn close_node(&mut self) {
        let node = self.queue.last_mut().unwrap();
        let start = (node.start + node.duration).max(self.head);
        if node.duration == 0.0 {
            node.start = start;
            self.queue.pop();
        }
        let mut new_node = self.node_template.clone();
        new_node.start = start;
        self.queue.push(new_node);
    }

    pub fn merge_template_into_last(&mut self) {
        let node = self.queue.last_mut().unwrap();
        node.merge(&self.node_template);
    }

    pub fn add_effect(&mut self, effect: VisualEffect) {
        self.queue.last_mut().unwrap().add_effect(effect);
    }

    pub fn add_effect_by_key(&mut self, key: &str, effect: VisualEffect) {
        self.queue
            .last_mut()
            .unwrap()
            .add_effect_by_key(key, effect);
    }

    pub fn add_entity_shader(&mut self, entity: legion::Entity, shader: Shader) {
        self.queue
            .last_mut()
            .unwrap()
            .add_entity_shader(entity, shader);
    }

    pub fn get_shaders(&self) -> Vec<Shader> {
        let node = self.get_node_at_ts(self.head).merge(&self.parallel_node);
        let time = self.head - node.start;
        let mut shaders: Vec<Shader> = default();
        let mut entity_shaders = node.entity_shaders.clone();
        for effect in node.effects.values().flatten().sorted_by_key(|x| x.order) {
            if effect.duration > 0.0 && time > effect.duration {
                continue;
            }
            shaders.extend(
                effect
                    .r#type
                    .process(time / effect.duration, &mut entity_shaders),
            );
        }
        [entity_shaders.into_values().collect_vec(), shaders].concat()
    }

    pub fn length(&self) -> Time {
        let last = self.queue.last().unwrap();
        last.start + last.duration
    }

    pub fn last_start(&self) -> Time {
        self.queue.last().unwrap().start
    }

    pub fn get_skip_ts(&self, from_ts: Time, right: bool) -> Time {
        let node = self.get_node_at_ts(
            from_ts
                + match right {
                    true => 0.001,
                    false => -0.001,
                },
        );
        if right {
            node.start + node.duration
        } else {
            node.start
        }
    }

    pub fn clear(&mut self) {
        self.queue = vec![default()];
        self.head = 0.0;
        self.node_template.clear();
    }

    fn get_node_at_ts(&self, ts: Time) -> &CassetteNode {
        if ts > self.length() {
            return &self.node_template;
        }
        let index = match self
            .queue
            .binary_search_by_key(&r32(ts), |node| r32(node.start))
        {
            Ok(index) => index,
            Err(index) => index - 1,
        };
        if let Some(node) = self.queue.get(index) {
            node
        } else {
            &self.node_template
        }
    }
}

#[derive(Default, Clone, Debug)]

pub struct CassetteNode {
    start: Time,
    duration: Time,
    entity_shaders: HashMap<legion::Entity, Shader>,
    effects: HashMap<String, Vec<VisualEffect>>,
}

impl CassetteNode {
    pub fn add_entity_shader(&mut self, entity: legion::Entity, shader: Shader) {
        self.entity_shaders.insert(entity, shader);
    }
    pub fn add_effect_by_key(&mut self, key: &str, effect: VisualEffect) {
        self.duration = self.duration.max(effect.duration);
        let mut vec = self.effects.remove(key).unwrap_or_default();
        vec.push(effect);
        self.effects.insert(key.to_string(), vec);
    }
    pub fn add_effect(&mut self, effect: VisualEffect) {
        self.add_effect_by_key("default", effect);
    }
    pub fn add_effects_by_key(&mut self, key: &str, effect: Vec<VisualEffect>) {
        effect
            .into_iter()
            .for_each(|effect| self.add_effect_by_key(key, effect))
    }
    pub fn add_effects(&mut self, effects: Vec<VisualEffect>) {
        effects
            .into_iter()
            .for_each(|effect| self.add_effect(effect))
    }
    pub fn clear_key(&mut self, key: &str) {
        self.effects.remove(key);
    }
    pub fn clear(&mut self) {
        self.start = default();
        self.duration = default();
        self.entity_shaders.clear();
        self.effects.clear();
    }
    pub fn clear_entities(&mut self) {
        self.entity_shaders.clear();
    }
    pub fn merge(&self, other: &CassetteNode) -> CassetteNode {
        let mut node = self.clone();
        node.duration = node.duration.max(node.duration);
        for (key, effects) in other.effects.iter() {
            node.effects.insert(key.clone(), effects.clone());
        }
        other.entity_shaders.iter().for_each(|(entity, shader)| {
            node.entity_shaders.insert(*entity, shader.clone());
        });
        node
    }
}
