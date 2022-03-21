use super::*;

impl Alliance {
    pub fn apply(&self, template: &mut UnitTemplate, party_members: usize) {
        match self {
            Self::Assassins => {
                if party_members >= 2 {
                    let crit_percent = 15.0;
                    template
                        .action
                        .effect
                        .walk_mut(&mut |effect| match &effect {
                            Effect::Damage(damage) => {
                                *effect = Effect::Random {
                                    choices: vec![
                                        WeighedEffect {
                                            weight: 100.0 - crit_percent,
                                            effect: effect.clone(),
                                        },
                                        WeighedEffect {
                                            weight: crit_percent,
                                            effect: Effect::List {
                                                effects: {
                                                    let mut effects = Vec::new();
                                                    effects.push(Effect::Damage(Box::new(
                                                        DamageEffect {
                                                            hp: damage.hp * r32(3.0),
                                                            lifesteal: damage.lifesteal,
                                                            types: {
                                                                let mut types =
                                                                    damage.types.clone();
                                                                types.insert("Crit".to_owned());
                                                                types
                                                            },
                                                            on: damage.on.clone(),
                                                        },
                                                    )));
                                                    if party_members >= 4 {
                                                        effects.push(Effect::AttachStatus(
                                                            Box::new(AttachStatusEffect {
                                                                who: Who::Target,
                                                                status: AttachedStatus {
                                                                    status: Status::Slow {
                                                                        percent: 70.0,
                                                                    },
                                                                    time: Some(r32(3.0)),
                                                                },
                                                            }),
                                                        ));
                                                    }
                                                    if party_members >= 6 {
                                                        effects.push(Effect::AttachStatus(
                                                            Box::new(AttachStatusEffect {
                                                                who: Who::Caster,
                                                                status: AttachedStatus {
                                                                    status: Status::Shield,
                                                                    time: None,
                                                                },
                                                            }),
                                                        ));
                                                    }
                                                    effects
                                                },
                                            },
                                        },
                                    ],
                                };
                            }
                            _ => {}
                        });
                }
            }
            Self::Spawners => {
                if party_members >= 4 {
                    template.statuses.push(AttachedStatus {
                        status: Status::Kill(UnitKillTrigger {
                            damage_type: None,
                            effect: Effect::Spawn(Box::new(SpawnEffect {
                                unit_type: "critter".to_owned(),
                            })),
                        }),
                        time: None,
                    });
                }
                if party_members >= 6 {
                    let big_critter_percent = 10.0;
                    template.walk_effects_mut(&mut |effect| match effect {
                        Effect::Spawn(spawn) => {
                            if spawn.unit_type == "critter" {
                                *effect = Effect::Random {
                                    choices: vec![
                                        WeighedEffect {
                                            weight: 100.0 - big_critter_percent,
                                            effect: Effect::Spawn(Box::new(SpawnEffect {
                                                unit_type: "critter".to_owned(),
                                            })),
                                        },
                                        WeighedEffect {
                                            weight: big_critter_percent,
                                            effect: Effect::Spawn(Box::new(SpawnEffect {
                                                unit_type: "big_critter".to_owned(),
                                            })),
                                        },
                                    ],
                                }
                            }
                        }
                        _ => {}
                    });
                }
                if party_members >= 2 {
                    template.statuses.push(AttachedStatus {
                        status: Status::Aura(Aura {
                            distance: None,
                            alliance: Some(Alliance::Critters),
                            status: Box::new(Status::Modifier(Modifier::Strength(
                                StrengthModifier {
                                    multiplier: r32(1.0),
                                    add: r32(2.0),
                                },
                            ))),
                        }),
                        time: None,
                    });
                }
            }
            Self::Archers => {
                template.walk_effects_mut(&mut |effect| match effect {
                    Effect::Projectile(projectile) => {
                        *effect = if party_members >= 6 {
                            Effect::AddTargets(Box::new(AddTargetsEffect {
                                effect: effect.clone(),
                                condition: Condition::Always,
                                additional_targets: None,
                            }))
                        } else if party_members >= 4 {
                            Effect::AddTargets(Box::new(AddTargetsEffect {
                                effect: effect.clone(),
                                condition: Condition::Always,
                                additional_targets: Some(4),
                            }))
                        } else if party_members >= 2 {
                            Effect::AddTargets(Box::new(AddTargetsEffect {
                                effect: effect.clone(),
                                condition: Condition::Always,
                                additional_targets: Some(2),
                            }))
                        } else {
                            effect.clone()
                        };
                    }
                    _ => {}
                });
            }
            Self::Critters => {}
            Self::Freezers => {
                if party_members >= 2 {
                    template.walk_effects_mut(&mut |effect| match effect {
                        Effect::Damage(damage) => {
                            *effect = Effect::MaybeModify(Box::new(MaybeModifyEffect {
                                base_effect: effect.clone(),
                                condition: Condition::UnitHasStatus {
                                    who: Who::Target,
                                    status_type: StatusType::Freeze,
                                },
                                modifier: Modifier::Strength(StrengthModifier {
                                    multiplier: r32(2.0),
                                    add: R32::ZERO,
                                }),
                            }))
                        }
                        _ => {}
                    })
                }

                if party_members >= 4 {
                    template.statuses.push(AttachedStatus {
                        status: Status::Injured(UnitTakeDamageTrigger {
                            damage_type: None,
                            effect: Effect::AttachStatus(Box::new(AttachStatusEffect {
                                who: Who::Caster,
                                status: AttachedStatus {
                                    status: Status::Freeze,
                                    time: None,
                                },
                            })),
                        }),
                        time: None,
                    });
                }

                if party_members >= 6 {
                    template.statuses.push(AttachedStatus {
                        status: Status::Kill(UnitKillTrigger {
                            damage_type: None,
                            effect: Effect::If(Box::new(IfEffect {
                                condition: Condition::UnitHasStatus {
                                    who: Who::Target,
                                    status_type: StatusType::Freeze,
                                },
                                then: {
                                    Effect::AOE(Box::new(AoeEffect {
                                        filter: TargetFilter::Enemies,
                                        skip_current_target: true,
                                        radius: r32(0.5),
                                        effect: Effect::Projectile(Box::new(ProjectileEffect {
                                            speed: r32(10.0),
                                            effect: Effect::Damage(Box::new(DamageEffect {
                                                hp: DamageValue::absolute(1.0),
                                                lifesteal: DamageValue::default(),
                                                types: {
                                                    let mut types = HashSet::new();
                                                    types.insert("Ranged".to_owned());
                                                    types
                                                },
                                                on: HashMap::new(),
                                            })),
                                        })),
                                    }))
                                },
                                r#else: Effect::Noop,
                            })),
                        }),
                        time: None,
                    });
                }
            }
            Self::Warriors => {
                let mut protection = 0.0;
                if party_members >= 3 {
                    protection = 30.0;
                } else if party_members >= 6 {
                    protection = 50.0;
                }
                if protection != 0.0 {
                    template.statuses.push(AttachedStatus {
                        status: Status::Protection {
                            percent: protection,
                        },
                        time: None,
                    });
                }
            }
        }
    }
}
