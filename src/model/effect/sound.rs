use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct SoundEffect {
    pub name: String,
    #[serde(default)]
    pub r#loop: bool,
}

impl EffectContainer for SoundEffect {
    fn walk_effects_mut(&mut self, f: &mut dyn FnMut(&mut Effect)) {}
}

impl EffectImpl for SoundEffect {
    fn process(self: Box<Self>, context: EffectContext, logic: &mut logic::Logic) {
        let effect = *self;
        logic.sound_controller.play_sound(effect.name.clone());
    }
}