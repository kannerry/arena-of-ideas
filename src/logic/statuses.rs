use geng::prelude::itertools::Itertools;

use super::*;

impl Logic {
    fn is_status_expired(status: &AttachedStatus) -> bool {
        status.time.map(|time| time == 0).unwrap_or(false)
            || status
                .vars
                .get(&VarName::StackCounter)
                .map(|count| *count <= 0)
                .unwrap_or(false)
    }

    pub fn collect_modifier_targets(
        &mut self,
        unit: &Unit,
    ) -> Vec<(EffectContext, ModifierTarget)> {
        let mut modifier_targets: Vec<(EffectContext, ModifierTarget)> = Vec::new();
        for status in &unit.all_statuses {
            if !Self::is_status_expired(status) {
                if let StatusEffect::Modifier(modifier) = &status.status.effect {
                    let context = EffectContext {
                        caster: Some(unit.id),
                        from: status.caster,
                        target: Some(unit.id),
                        vars: status.vars.clone(),
                        status_id: Some(status.id),
                        color: None,
                    };
                    if let ModifierTarget::List { targets } = &modifier.target {
                        if match &modifier.condition {
                            Some(condition) => {
                                self.model.units.insert(unit.clone());
                                let result = self.check_condition(condition, &context);
                                self.model.units.remove(&unit.id);
                                result
                            }
                            None => true,
                        } {
                            modifier_targets.extend(
                                targets
                                    .iter()
                                    .map(|target| (context.clone(), target.clone())),
                            );
                        }
                    } else if match &modifier.condition {
                        Some(condition) => {
                            self.model.units.insert(unit.clone());
                            let result = self.check_condition(condition, &context);
                            self.model.units.remove(&unit.id);
                            result
                        }
                        None => true,
                    } {
                        modifier_targets.push((context.clone(), modifier.target.clone()));
                    }
                }
            }
        }
        modifier_targets
    }

    pub fn process_modifiers(&mut self, unit: &mut Unit) {
        let modifier_targets = self.collect_modifier_targets(&unit);
        unit.stats = unit.permanent_stats.clone();

        for (context, target) in &modifier_targets {
            if let ModifierTarget::Stat { stat, value } = target {
                self.model.units.insert(unit.clone());
                let stat_value = value.calculate(&context, self);
                self.model.units.remove(&unit.id);
                *unit.stats.get_mut(*stat) = stat_value;
            }
        }
        unit.modifier_targets = modifier_targets;
    }

    pub fn process_unit_statuses(&mut self, unit: &mut Unit) {
        let mut expired: Vec<(Option<Id>, Id, String)> = Vec::new();
        unit.all_statuses
            .sort_by(|a, b| a.status.order.cmp(&b.status.order));
        for status in &mut unit.all_statuses {
            if !status.is_inited {
                for (effect, trigger, vars, status_id, status_color) in
                    status.trigger(|trigger| matches!(trigger, StatusTriggerType::Init))
                {
                    self.effects.push_front(QueuedEffect {
                        effect,
                        context: EffectContext {
                            caster: status.caster,
                            from: Some(unit.id),
                            target: Some(unit.id),
                            vars,
                            status_id: Some(status_id),
                            color: Some(status_color),
                        },
                    })
                }
                status.is_inited = true;
            }
            if let Some(time) = &mut status.time {
                *time = time.saturating_sub(1);
            }
            for listener in &mut status.status.listeners {
                let ticks = listener
                    .triggers
                    .iter_mut()
                    .map(|trigger| match &mut trigger.trigger_type {
                        StatusTriggerType::Repeating {
                            tick_time,
                            next_tick,
                        } => {
                            let cur_tick = self.model.current_tick.tick_num;
                            let mut ticks = 0;
                            if *next_tick <= 0 {
                                *next_tick += *tick_time;
                                ticks += 1;
                            } else {
                                *next_tick -= 1;
                            }

                            ticks
                        }
                        _ => 0,
                    })
                    .sum();
                for _ in 0..ticks {
                    self.effects.push_back(QueuedEffect {
                        effect: listener.effect.clone(),
                        context: EffectContext {
                            caster: status.caster.or(Some(unit.id)),
                            from: Some(unit.id),
                            target: Some(unit.id),
                            vars: status.vars.clone(),
                            status_id: Some(status.id),
                            color: Some(status.status.color),
                        },
                    });
                }
            }
            if Self::is_status_expired(status) {
                expired.push((status.caster, status.id, status.status.name.clone()));
                for (effect, trigger, vars, status_id, status_color) in
                    status.trigger(|trigger| matches!(trigger, StatusTriggerType::Break))
                {
                    self.effects.push_back(QueuedEffect {
                        effect,
                        context: EffectContext {
                            caster: Some(unit.id),
                            from: Some(unit.id),
                            target: Some(unit.id),
                            vars,
                            status_id: Some(status_id),
                            color: Some(status_color),
                        },
                    })
                }
            }
        }

        unit.all_statuses
            .retain(|status| !Self::is_status_expired(status));

        unit.flags = unit
            .all_statuses
            .iter()
            .flat_map(|status| status.status.flags.iter())
            .copied()
            .collect();

        // Detect expired statuses
        for (caster_id, status_id, detect_status) in expired {
            self.trigger_status_drop(UnitRef::Ref(unit), caster_id, status_id, &detect_status)
        }
    }

    pub(super) fn trigger_status_drop(
        &mut self,
        unit: UnitRef<'_>,
        caster_id: Option<Id>,
        status_id: Id,
        status_name: &StatusName,
    ) {
        let unit = match unit {
            UnitRef::Id(id) => self
                .model
                .units
                .get(&id)
                .expect("Failed to find unit by id"),
            UnitRef::Ref(unit) => unit,
        };
        for (effect, trigger, vars, status_id, status_color) in
            unit.all_statuses.iter().flat_map(|status| {
                status.trigger(|trigger| match trigger {
                    StatusTriggerType::SelfDetectAttach {
                        status_name: name,
                        status_action: StatusAction::Remove,
                    } => status_name == name,
                    _ => false,
                })
            })
        {
            self.effects.push_back(QueuedEffect {
                effect,
                context: EffectContext {
                    caster: caster_id,
                    from: Some(unit.id),
                    target: Some(unit.id),
                    vars,
                    status_id: Some(status_id),
                    color: Some(status_color),
                },
            })
        }

        for other in &self.model.units {
            for (effect, trigger, vars, status_id, status_color) in
                other.all_statuses.iter().flat_map(|status| {
                    status.trigger(|trigger| match trigger {
                        StatusTriggerType::DetectAttach {
                            status_name: name,
                            filter,
                            status_action: StatusAction::Remove,
                        } => {
                            other.id != unit.id
                                && status_name == name
                                && filter.matches(unit.faction, other.faction)
                        }
                        _ => false,
                    })
                })
            {
                self.effects.push_back(QueuedEffect {
                    effect,
                    context: EffectContext {
                        caster: caster_id,
                        from: Some(other.id),
                        target: Some(unit.id),
                        vars,
                        status_id: Some(status_id),
                        color: Some(status_color),
                    },
                })
            }
        }
    }
}
