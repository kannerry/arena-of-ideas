use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetectStatus {
    pub on: TargetFilter,
    pub detect_type: StatusName,
    pub effect: Effect,
}

impl EffectContainer for DetectStatus {
    fn walk_effects_mut(&mut self, f: &mut dyn FnMut(&mut Effect)) {
        self.effect.walk_mut(f);
    }
}

impl StatusImpl for DetectStatus {}
