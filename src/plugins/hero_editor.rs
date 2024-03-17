use std::fmt::Display;

use bevy_egui::egui::{DragValue, Frame, Key, ScrollArea, Sense, Shape, SidePanel};
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::de::DeserializeOwned;

use super::*;

pub struct HeroEditorPlugin;

impl Plugin for HeroEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (Self::input, Self::update, Self::ui.after(PanelsPlugin::ui))
                .run_if(in_state(GameState::HeroEditor)),
        )
        .add_systems(OnEnter(GameState::HeroEditor), Self::on_enter)
        .add_systems(OnExit(GameState::HeroEditor), Self::on_exit);
    }
}

impl HeroEditorPlugin {
    fn on_enter(world: &mut World) {
        let mut pd = PersistentData::load(world);
        pd.hero_editor_data.active = None;
        pd.hero_editor_data.load(world);
        pd.save(world).unwrap();
        Pools::get_mut(world).only_local_cache = true;
    }

    fn on_exit(world: &mut World) {
        Self::save(world);
        UnitPlugin::despawn_all_teams(world);
    }

    fn update(world: &mut World) {
        let mut pd = PersistentData::load(world);
        let ed = &mut pd.hero_editor_data;
        let pos = if let Some((entity, _)) = ed.active {
            VarState::get(entity, world)
                .get_vec2(VarName::Position)
                .unwrap_or(ed.camera_need_pos)
        } else {
            default()
        };
        ed.camera_need_pos = pos;
        ed.apply_camera(world);
        pd.save(world).unwrap();
    }

    fn input(world: &mut World) {
        let mut pd = PersistentData::load(world);
        let ed = &mut pd.hero_editor_data;
        if world.resource::<Input<KeyCode>>().just_pressed(KeyCode::Up) {
            ed.camera_scale *= 1.2;
            pd.save(world).unwrap();
        } else if world
            .resource::<Input<KeyCode>>()
            .just_pressed(KeyCode::Down)
        {
            ed.camera_scale /= 1.2;
            pd.save(world).unwrap();
        }
    }

    fn save(world: &mut World) {
        debug!("Saving.");
        let mut pd = PersistentData::load(world);
        pd.hero_editor_data.save(world);
        pd.save(world).unwrap();
    }

    pub fn ui(world: &mut World) {
        let ctx = &if let Some(context) = egui_context(world) {
            context
        } else {
            return;
        };
        let mut pd = PersistentData::load(world);
        let ed = &mut pd.hero_editor_data.clone();

        let hovered = UnitPlugin::get_hovered(world);
        let mut delete: Option<Entity> = None;
        for unit in UnitPlugin::collect_all(world) {
            let hovered = hovered == Some(unit);
            if hovered {
                entity_window(unit, vec2(0.0, 0.0), None, &format!("{unit:?}"), world)
                    .frame(Frame::none())
                    .show(ctx, |ui| {
                        let button = ui.button("Edit");
                        if button.clicked() {
                            ed.active = Some((unit, PackedUnit::pack(unit, world)));
                        }
                        ui.add_space(5.0);
                        if ui.button_red("Delete").clicked() {
                            delete = Some(unit);
                        }
                    });
            }
        }
        if let Some(unit) = delete {
            world.entity_mut(unit).despawn_recursive();
            UnitPlugin::fill_gaps_and_translate(world);
            ed.save(world);
        }
        Self::show_edit_panel(ed, world);
        if ed.active.is_none() {
            for faction in [Faction::Left, Faction::Right] {
                let offset: Vec2 = match faction {
                    Faction::Left => [-1.0, 0.0],
                    _ => [1.0, 0.0],
                }
                .into();
                window(&format!("spawn {faction}"))
                    .fixed_pos(world_to_screen(
                        (UnitPlugin::get_slot_position(faction, 0) + offset).extend(0.0),
                        world,
                    ))
                    .title_bar(false)
                    .stroke(false)
                    .set_width(60.0)
                    .show(ctx, |ui| {
                        if ui.button("Spawn").clicked() {
                            ed.spawn(faction, world);
                        }
                    });
            }
        }
        Self::draw_top_buttons(ed, ctx, world);
        if !pd.hero_editor_data.eq(ed) {
            ed.save(world);
            mem::swap(&mut pd.hero_editor_data, ed);
            pd.save(world).unwrap();
        }
    }

    fn draw_top_buttons(ed: &mut HeroEditorData, ctx: &egui::Context, world: &mut World) {
        if ed.active.is_some() {
            return;
        }
        TopBottomPanel::top("battle btns").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Turn End").clicked() {
                    Event::TurnEnd.send(world);
                }
                if ui.button("Battle Start").clicked() {
                    Event::BattleStart.send(world);
                }
                if ui.button("Strike").clicked() {
                    if let Some((left, right)) = BattlePlugin::get_strikers(world) {
                        match BattlePlugin::run_strike(left, right, world) {
                            Ok(_) => {}
                            Err(e) => error!("{e}"),
                        }
                    }
                }
                ui.add_space(10.0);
                if ui.button_color("Save", yellow()).clicked() {
                    ed.saved_teams = ed.teams.clone();
                }
                if ui.button_color("Load", yellow()).clicked() {
                    ed.teams = ed.saved_teams.clone();
                    ed.load(world);
                }

                ui.add_space(10.0);
                if ui.button_red("Clear Statuses").clicked() {
                    for unit in ed.teams.0.iter_mut().chain(ed.teams.1.iter_mut()) {
                        unit.statuses.clear();
                    }
                    ed.load(world);
                }
                if ui.button_red("Reset").clicked() {
                    UnitPlugin::despawn_all_teams(world);
                    ed.load(world);
                }
                if ui.button_red("Clear").clicked() {
                    Self::clear(world);
                }
            });
        });
    }

    fn show_edit_panel(ed: &mut HeroEditorData, world: &mut World) {
        if let Some((entity, old_unit)) = ed.active.as_ref() {
            let ctx = &if let Some(context) = egui_context(world) {
                context
            } else {
                return;
            };
            let mut unit = old_unit.clone();
            let entity = *entity;

            SidePanel::left("edit panel")
                .frame(Frame {
                    stroke: Stroke {
                        width: 1.0,
                        color: white(),
                    },
                    outer_margin: Margin::same(4.0),
                    inner_margin: Margin::same(4.0),
                    fill: black(),
                    ..default()
                })
                .default_width(ctx.screen_rect().width() * 0.7)
                .show(ctx, |ui| {
                    ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                        if ui.button_red("CLOSE").clicked() {
                            ed.active = None;
                        }
                        if ui.button("PASTE").clicked() {
                            if let Some(s) = get_from_clipboard(world) {
                                match ron::from_str(&s) {
                                    Ok(u) => unit = u,
                                    Err(e) => AlertPlugin::add_error(
                                        Some("Paste Failed".to_owned()),
                                        e.to_string(),
                                        None,
                                    ),
                                }
                            }
                        }
                        if ui.button("COPY").clicked() {
                            let mut unit = unit.clone();
                            unit.state = default();
                            save_to_clipboard(
                                &to_string_pretty(&unit, PrettyConfig::new()).unwrap(),
                                world,
                            );
                        }
                        ui.add_space(100.0);
                        const SELECTED_STATUS_KEY: &str = "selected status";
                        let mut status = get_context_string(world, SELECTED_STATUS_KEY);
                        ComboBox::from_id_source("status select")
                            .selected_text(status.clone())
                            .show_ui(ui, |ui| {
                                for option in
                                    Pools::get(world).statuses.keys().cloned().collect_vec()
                                {
                                    let text = option.to_string();
                                    if ui
                                        .selectable_value(&mut status, option.to_owned(), text)
                                        .changed()
                                    {
                                        set_context_string(world, SELECTED_STATUS_KEY, option);
                                    }
                                }
                            });
                        ui.set_enabled(!status.is_empty());
                        if ui.button("Add Status").clicked() {
                            if let Some((i, _)) =
                                unit.statuses.iter().find_position(|(s, _)| status.eq(s))
                            {
                                unit.statuses[i].1 += 1;
                            } else {
                                unit.statuses.push((status, 1));
                            }
                        }
                    });
                    ScrollArea::new([true, true])
                        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                        .show(ui, |ui| {
                            let style = ui.style_mut();
                            style.override_text_style = Some(TextStyle::Small);
                            style.drag_value_text_style = TextStyle::Small;
                            style.visuals.widgets.inactive.bg_stroke = Stroke {
                                width: 1.0,
                                color: dark_gray(),
                            };
                            ui.horizontal(|ui| {
                                let name = &mut unit.name;
                                ui.label("name:");
                                TextEdit::singleline(name).desired_width(60.0).ui(ui);
                                let atk = &mut unit.atk;
                                ui.label("atk:");
                                DragValue::new(atk).clamp_range(0..=99).ui(ui);
                                let hp = &mut unit.hp;
                                ui.label("hp:");
                                DragValue::new(hp).clamp_range(0..=99).ui(ui);
                            });
                            ui.horizontal(|ui| {
                                let houses: HashMap<String, Color> = HashMap::from_iter(
                                    Pools::get(world)
                                        .houses
                                        .iter()
                                        .map(|(k, v)| (k.clone(), v.color.clone().into())),
                                );
                                ui.label("house:");
                                let house = &mut unit.houses;
                                ComboBox::from_id_source("house")
                                    .selected_text(house.clone())
                                    .width(140.0)
                                    .show_ui(ui, |ui| {
                                        for (h, _) in
                                            houses.into_iter().sorted_by_key(|(k, _)| k.clone())
                                        {
                                            ui.selectable_value(house, h.clone(), h.clone());
                                        }
                                    });
                            });
                            ui.horizontal(|ui| {
                                ui.label("desc:");
                                let description = &mut unit.description;
                                TextEdit::singleline(description)
                                    .desired_width(ui.available_width().min(200.0))
                                    .ui(ui);
                                if ui.button("reset").clicked() {
                                    *description = DEFAULT_UNIT_DESCRIPTION.to_owned();
                                }
                            });

                            let context = &Context::from_owner(entity, world);
                            ui.horizontal(|ui| {
                                let trigger = &mut unit.trigger;
                                match trigger {
                                    Trigger::Fire {
                                        trigger,
                                        target,
                                        effect,
                                        period,
                                    } => {
                                        CollapsingHeader::new("Trigger").default_open(true).show(
                                            ui,
                                            |ui| {
                                                ui.horizontal(|ui| {
                                                    ui.label("period:");
                                                    DragValue::new(period).ui(ui);
                                                });
                                                trigger.show_editor(entity, ui);
                                                match trigger {
                                                    FireTrigger::List(list) => {
                                                        ui.vertical(|ui| {
                                                            for (i, trigger) in
                                                                list.iter_mut().enumerate()
                                                            {
                                                                ComboBox::from_id_source(
                                                                    Id::new(entity).with(i),
                                                                )
                                                                .selected_text(trigger.to_string())
                                                                .show_ui(ui, |ui| {
                                                                    for option in
                                                                        FireTrigger::iter()
                                                                    {
                                                                        let text =
                                                                            option.to_string();
                                                                        ui.selectable_value(
                                                                            trigger.as_mut(),
                                                                            option,
                                                                            text,
                                                                        );
                                                                    }
                                                                });
                                                            }
                                                            if ui.button("+").clicked() {
                                                                list.push(default());
                                                            }
                                                        });
                                                    }
                                                    _ => {}
                                                }
                                            },
                                        );
                                        CollapsingHeader::new("Target").default_open(true).show(
                                            ui,
                                            |ui| {
                                                show_tree("", target, context, ui, world);
                                            },
                                        );

                                        CollapsingHeader::new("Effect").default_open(true).show(
                                            ui,
                                            |ui| {
                                                show_tree("", effect, context, ui, world);
                                            },
                                        );
                                    }
                                    Trigger::Change { .. } => todo!(),
                                    Trigger::List(_) => todo!(),
                                }
                            });

                            let rep = &mut unit.representation;
                            rep.show_editor(context, "root", ui, world);
                            ui.add_space(150.0);
                        });
                });

            if let Some((entity, old_unit)) = ed.active.as_ref() {
                let entity = *entity;
                if !unit.eq(old_unit) {
                    let slot =
                        VarState::get(entity, world).get_int(VarName::Slot).unwrap() as usize;
                    let parent = entity.get_parent(world).unwrap();
                    world.entity_mut(entity).despawn_recursive();
                    let entity = unit.clone().unpack(parent, Some(slot), world);
                    UnitPlugin::place_into_slot(entity, world).unwrap();
                    ed.active = Some((entity, unit));
                    ed.save(world);
                }
            }
        }
    }

    fn clear(world: &mut World) {
        let mut pd = PersistentData::load(world);
        let ed = &mut pd.hero_editor_data;
        UnitPlugin::despawn_all_teams(world);
        ed.clear();
        pd.save(world).unwrap();
        Self::on_enter(world);
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HeroEditorData {
    pub active: Option<(Entity, PackedUnit)>,

    pub teams: (Vec<PackedUnit>, Vec<PackedUnit>),
    pub camera_pos: Vec2,
    pub camera_need_pos: Vec2,
    pub camera_scale: f32,
    pub hovered_id: Option<String>,

    pub saved_teams: (Vec<PackedUnit>, Vec<PackedUnit>),
}

impl Default for HeroEditorData {
    fn default() -> Self {
        Self {
            camera_pos: default(),
            camera_need_pos: default(),
            camera_scale: 1.0,
            hovered_id: default(),
            active: default(),
            teams: default(),
            saved_teams: default(),
        }
    }
}

impl HeroEditorData {
    fn save(&mut self, world: &mut World) {
        debug!("Save hero editor data start");
        self.teams.0.clear();
        self.teams.1.clear();
        let mut units = UnitPlugin::collect_factions([Faction::Left, Faction::Right].into(), world);
        units.sort_by_key(|(e, _)| VarState::get(*e, world).get_int(VarName::Slot).unwrap());
        for (unit, faction) in units {
            let packed = PackedUnit::pack(unit, world);
            let units = match faction {
                Faction::Left => &mut self.teams.0,
                _ => &mut self.teams.1,
            };
            units.push(packed);
        }
    }

    fn load(&mut self, world: &mut World) {
        debug!("Load hero editor data start");
        UnitPlugin::despawn_all_teams(world);
        let left = PackedTeam::spawn(Faction::Left, world);
        let right = PackedTeam::spawn(Faction::Right, world);
        self.teams.0.iter().for_each(|u| {
            u.clone().unpack(left, None, world);
        });
        self.teams.1.iter().for_each(|u| {
            u.clone().unpack(right, None, world);
        });
        UnitPlugin::fill_gaps_and_translate(world);
    }

    fn clear(&mut self) {
        self.teams.0.clear();
        self.teams.1.clear();
        self.hovered_id = None;
    }

    fn apply_camera(&mut self, world: &mut World) {
        let dt = world.resource::<Time>().delta_seconds();
        if let Ok((mut transform, mut projection)) = world
            .query_filtered::<(&mut Transform, &mut OrthographicProjection), With<Camera>>()
            .get_single_mut(world)
        {
            let need_pos = if self.camera_need_pos.length() > 0.0 {
                self.camera_need_pos - vec2(projection.area.max.x - 1.2, 0.0)
            } else {
                default()
            };
            self.camera_pos += (need_pos - self.camera_pos) * dt * 10.0;
            let delta = self.camera_pos * self.camera_scale / projection.scale;
            self.camera_pos = delta;
            let z = transform.translation.z;
            transform.translation = self.camera_pos.extend(z);
            projection.scale = self.camera_scale;
        }
    }

    fn spawn(&mut self, faction: Faction, world: &mut World) {
        ron::from_str::<PackedUnit>("()").unwrap().unpack(
            PackedTeam::find_entity(faction, world).unwrap(),
            None,
            world,
        );
        UnitPlugin::fill_slot_gaps(faction, world);
        UnitPlugin::translate_to_slots(world);
        self.save(world);
    }
}

pub fn show_value(value: &Result<VarValue>, ui: &mut Ui) {
    match &value {
        Ok(v) => v.to_string().add_color(light_gray()),

        Err(e) => e.to_string().add_color(red()),
    }
    .set_style(ColoredStringStyle::Small)
    .as_label(ui)
    .truncate(true)
    .ui(ui);
}

pub fn show_tree(
    label: &str,
    root: &mut impl EditorNodeGenerator,
    context: &Context,
    ui: &mut Ui,
    world: &mut World,
) {
    show_trees([(label, root)].into(), context, ui, world);
}

pub fn show_trees(
    data: Vec<(&str, &mut impl EditorNodeGenerator)>,
    context: &Context,
    ui: &mut Ui,
    world: &mut World,
) {
    ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
        for (label, root) in data {
            ui.label(label);
            show_node(root, label.to_owned(), None, context, ui, world);
        }
    });
}

pub fn show_node(
    source: &mut impl EditorNodeGenerator,
    path: String,
    connect_pos: Option<Pos2>,
    context: &Context,
    ui: &mut Ui,
    world: &mut World,
) {
    let ctx = &if let Some(context) = egui_context(world) {
        context
    } else {
        return;
    };
    let path = format!("{path}/{source}");
    let InnerResponse {
        inner: name_resp,
        response: frame_resp,
    } = Frame::none()
        .stroke(Stroke::new(1.0, dark_gray()))
        .inner_margin(6.0)
        .outer_margin(6.0)
        .rounding(0.0)
        .fill(light_black())
        .show(ui, |ui| {
            let name = source
                .to_string()
                .add_color(source.node_color())
                .as_label(ui)
                .sense(Sense::click())
                .ui(ui);
            ui.allocate_ui_at_rect(
                name.rect.translate(egui::vec2(0.0, name.rect.height())),
                |ui| {
                    source.show_extra(&path, context, world, ui);
                },
            );
            name.on_hover_text(&path)
        });

    {
        let mut left_line = frame_resp.rect.translate(egui::vec2(3.0, 0.0));
        left_line.set_width(2.0);
        left_line = left_line.shrink2(egui::vec2(0.0, 14.0));
        let mut ui = ui.child_ui(left_line, Layout::left_to_right(egui::Align::Center));
        let response = ui.allocate_rect(left_line, Sense::click());
        let color = if response.hovered() {
            yellow()
        } else {
            dark_gray()
        };
        ui.painter_at(left_line)
            .rect_filled(left_line, Rounding::ZERO, color);
        if response.clicked() {
            source.wrap();
        }
    }

    const LOOKUP_KEY: &str = "lookup";
    if name_resp.clicked() {
        name_resp.request_focus();
        set_context_string(world, LOOKUP_KEY, default());
    }
    if name_resp.has_focus() || name_resp.lost_focus() {
        window("replace")
            .order(egui::Order::Foreground)
            .title_bar(false)
            .fixed_pos(frame_resp.rect.right_center().to_bvec2())
            .show(ctx, |ui| {
                Frame::none().inner_margin(8.0).show(ui, |ui| {
                    let mut lookup = get_context_string(world, LOOKUP_KEY);
                    let mut submit = false;
                    ctx.input(|i| {
                        for e in &i.events {
                            match e {
                                egui::Event::Text(t) => lookup += t,
                                egui::Event::Key { key, pressed, .. } => {
                                    if *pressed {
                                        if key.eq(&Key::Backspace) && !lookup.is_empty() {
                                            lookup.pop();
                                        } else if matches!(key, Key::Enter | Key::Tab) {
                                            submit = true;
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    });
                    ui.label(&lookup);
                    set_context_string(world, LOOKUP_KEY, lookup.clone());
                    ScrollArea::new([false, true])
                        .max_height(300.0)
                        .show(ui, |ui| {
                            let lookup = lookup.to_lowercase();
                            frame(ui, |ui| {
                                if source.show_replace_buttons(&lookup, submit, ui) {
                                    set_context_string(world, LOOKUP_KEY, default());
                                }
                            });
                        });
                });
            });
    }

    if let Some(pos) = connect_pos {
        let end = frame_resp.rect.left_center();
        let mut mid1 = pos;
        mid1.x += 5.0;
        let mut mid2 = end;
        mid2.x -= 5.0;
        let points = [pos, mid1, mid2, end];
        let curve = Shape::CubicBezier(egui::epaint::CubicBezierShape::from_points_stroke(
            points,
            false,
            Color32::TRANSPARENT,
            Stroke {
                width: 1.0,
                color: dark_gray(),
            },
        ));
        ui.painter().add(curve);
    }

    source.show_children(
        &path,
        Some(frame_resp.rect.right_center()),
        context,
        ui,
        world,
    );

    name_resp.context_menu(|ui| {
        if ui.button("COPY").clicked() {
            save_to_clipboard(
                &to_string_pretty(source, PrettyConfig::new()).unwrap(),
                world,
            );
            ui.close_menu();
        }
        if ui.button("PASTE").clicked() {
            let o = get_from_clipboard(world).unwrap();
            *source = ron::from_str(o.as_str()).unwrap();
            ui.close_menu();
        }
    });
}

pub trait EditorNodeGenerator: Display + Sized + Serialize + DeserializeOwned {
    fn node_color(&self) -> Color32;
    fn show_children(
        &mut self,
        path: &str,
        connect_pos: Option<Pos2>,
        context: &Context,
        ui: &mut Ui,
        world: &mut World,
    );
    fn show_extra(&mut self, path: &str, context: &Context, world: &mut World, ui: &mut Ui);
    fn show_replace_buttons(&mut self, lookup: &str, submit: bool, ui: &mut Ui) -> bool;
    fn wrap(&mut self);
}

impl EditorNodeGenerator for Effect {
    fn node_color(&self) -> Color32 {
        white()
    }

    fn show_extra(&mut self, path: &str, context: &Context, world: &mut World, ui: &mut Ui) {
        match self {
            Effect::AoeFaction(_, _)
            | Effect::WithTarget(_, _)
            | Effect::WithOwner(_, _)
            | Effect::Noop
            | Effect::Kill
            | Effect::FullCopy
            | Effect::RemoveLocalTrigger
            | Effect::Debug(_)
            | Effect::Text(_) => {}

            Effect::List(list) | Effect::ListSpread(list) => {
                if ui.button("CLEAR").clicked() {
                    list.clear()
                }
            }
            Effect::Damage(e) => {
                let mut v = e.is_some();
                if ui.checkbox(&mut v, "").changed() {
                    *e = match v {
                        true => Some(default()),
                        false => None,
                    };
                }
            }
            Effect::WithVar(x, e, _) => {
                ui.vertical(|ui| {
                    x.show_editor(path, ui);
                    let value = e.get_value(context, world);
                    show_value(&value, ui);
                });
            }
            Effect::If(e, ..) | Effect::Repeat(e, ..) => {
                ui.vertical(|ui| {
                    let value = e.get_value(context, world);
                    show_value(&value, ui);
                });
            }
            Effect::StateAddVar(x, target, value) => {
                ui.vertical(|ui| {
                    x.show_editor(path, ui);
                    ui.horizontal(|ui| {
                        ui.label("target:");
                        let target = target.get_value(context, world);
                        show_value(&target, ui);
                    });
                    ui.horizontal(|ui| {
                        ui.label("value:");
                        let value = value.get_value(context, world);
                        show_value(&value, ui);
                    });
                });
            }
            Effect::AbilityStateAddVar(name, x, value) => {
                ui.vertical(|ui| {
                    ComboBox::from_id_source(Id::new(path).with("ability"))
                        .selected_text(name.to_owned())
                        .show_ui(ui, |ui| {
                            let names = {
                                let pools = Pools::get(world);
                                pools
                                    .abilities
                                    .keys()
                                    .chain(pools.statuses.keys())
                                    .chain(pools.summons.keys())
                                    .unique()
                            };
                            for option in names {
                                let text = option.to_string();
                                ui.selectable_value(name, option.to_owned(), text);
                            }
                        });
                    x.show_editor(Id::new(path).with("var"), ui);
                    ui.horizontal(|ui| {
                        ui.label("value:");
                        let value = value.get_value(context, world);
                        show_value(&value, ui);
                    });
                });
            }
            Effect::UseAbility(name, mult) => {
                ui.vertical(|ui| {
                    ComboBox::from_id_source(&path)
                        .selected_text(name.to_owned())
                        .show_ui(ui, |ui| {
                            for option in Pools::get(world).abilities.keys() {
                                let text = option.to_string();
                                ui.selectable_value(name, option.to_owned(), text);
                            }
                        });
                    DragValue::new(mult).ui(ui);
                });
            }
            Effect::Summon(name) => {
                ui.vertical(|ui| {
                    ComboBox::from_id_source(&path)
                        .selected_text(name.to_owned())
                        .show_ui(ui, |ui| {
                            for option in Pools::get(world).summons.keys() {
                                let text = option.to_string();
                                ui.selectable_value(name, option.to_owned(), text);
                            }
                        });
                });
            }
            Effect::AddStatus(name) => {
                ui.vertical(|ui| {
                    ComboBox::from_id_source(&path)
                        .selected_text(name.to_owned())
                        .show_ui(ui, |ui| {
                            for option in Pools::get(world).statuses.keys() {
                                let text = option.to_string();
                                ui.selectable_value(name, option.to_owned(), text);
                            }
                        });
                });
            }
            Effect::Vfx(name) => {
                ui.vertical(|ui| {
                    ComboBox::from_id_source(&path)
                        .selected_text(name.to_owned())
                        .show_ui(ui, |ui| {
                            for option in Pools::get(world).vfx.keys() {
                                let text = option.to_string();
                                ui.selectable_value(name, option.to_owned(), text);
                            }
                        });
                });
            }
            Effect::SendEvent(name) => {
                ui.vertical(|ui| {
                    ComboBox::from_id_source(&path)
                        .selected_text(name.to_string())
                        .show_ui(ui, |ui| {
                            for option in [Event::BattleStart, Event::TurnStart, Event::TurnEnd] {
                                let text = option.to_string();
                                ui.selectable_value(name, option, text);
                            }
                        });
                });
            }
        }
    }

    fn show_replace_buttons(&mut self, lookup: &str, submit: bool, ui: &mut Ui) -> bool {
        for e in Effect::iter() {
            if e.to_string().to_lowercase().contains(lookup) {
                let btn = e.to_string().add_color(e.node_color()).rich_text(ui);
                let btn = ui.button(btn);
                if btn.clicked() || submit {
                    btn.request_focus();
                }
                if btn.gained_focus() {
                    *self = e;
                    return true;
                }
            }
        }
        false
    }

    fn show_children(
        &mut self,
        path: &str,
        connect_pos: Option<Pos2>,
        context: &Context,
        ui: &mut Ui,
        world: &mut World,
    ) {
        match self {
            Effect::Noop
            | Effect::Kill
            | Effect::FullCopy
            | Effect::UseAbility(..)
            | Effect::Summon(..)
            | Effect::AddStatus(..)
            | Effect::Vfx(..)
            | Effect::SendEvent(..)
            | Effect::RemoveLocalTrigger
            | Effect::Debug(..) => {}

            Effect::Text(e) | Effect::AbilityStateAddVar(_, _, e) => {
                show_node(e, format!("{path}:e"), connect_pos, context, ui, world)
            }
            Effect::Damage(e) => {
                if let Some(e) = e {
                    show_node(e, format!("{path}:e"), connect_pos, context, ui, world);
                }
            }
            Effect::AoeFaction(e, eff)
            | Effect::WithTarget(e, eff)
            | Effect::WithOwner(e, eff)
            | Effect::Repeat(e, eff) => {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        show_node(e, format!("{path}:e"), connect_pos, context, ui, world);
                    });
                    ui.horizontal(|ui| {
                        show_node(
                            eff.as_mut(),
                            format!("{path}:eff"),
                            connect_pos,
                            context,
                            ui,
                            world,
                        );
                    });
                });
            }
            Effect::List(list) => {
                ui.vertical(|ui| {
                    for (i, eff) in list.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            show_node(
                                eff.as_mut(),
                                format!("{path}:eff{i}"),
                                connect_pos,
                                context,
                                ui,
                                world,
                            );
                        });
                    }
                    if ui.button("+").clicked() {
                        list.push(default());
                    }
                });
            }
            Effect::ListSpread(_) => todo!(),
            Effect::WithVar(_, e, eff) => {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        show_node(e, format!("{path}:e"), connect_pos, context, ui, world);
                    });
                    ui.horizontal(|ui| {
                        show_node(
                            eff.as_mut(),
                            format!("{path}:eff"),
                            connect_pos,
                            context,
                            ui,
                            world,
                        );
                    });
                });
            }
            Effect::StateAddVar(_, target, value) => {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        show_node(
                            target,
                            format!("{path}:target"),
                            connect_pos,
                            context,
                            ui,
                            world,
                        );
                    });
                    ui.horizontal(|ui| {
                        show_node(
                            value,
                            format!("{path}:value"),
                            connect_pos,
                            context,
                            ui,
                            world,
                        );
                    });
                });
            }
            Effect::If(cond, th, el) => {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        show_node(
                            cond,
                            format!("{path}:cond"),
                            connect_pos,
                            context,
                            ui,
                            world,
                        );
                    });
                    ui.horizontal(|ui| {
                        show_node(
                            th.as_mut(),
                            format!("{path}:then"),
                            connect_pos,
                            context,
                            ui,
                            world,
                        );
                    });
                    ui.horizontal(|ui| {
                        show_node(
                            el.as_mut(),
                            format!("{path}:else"),
                            connect_pos,
                            context,
                            ui,
                            world,
                        );
                    });
                });
            }
        };
    }

    fn wrap(&mut self) {
        *self = Effect::List([Box::new(self.clone())].into());
    }
}
