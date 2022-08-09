use std::fmt::format;

use super::*;
use geng::{prelude::itertools::Itertools, Draw2d};

mod card;
mod layout;

pub use card::*;
pub use layout::*;

const TEXT_OCCUPY_SPACE: f32 = 0.6;
const BACKGROUND_COLOR: Color<f32> = Color::BLACK;
const TEXT_BACKGROUND_COLOR: Color<f32> = Color {
    r: 0.2,
    g: 0.2,
    b: 0.2,
    a: 1.0,
};
const BUTTON_COLOR: Color<f32> = Color {
    r: 0.0,
    g: 0.7,
    b: 1.0,
    a: 1.0,
};
const BUTTON_HOVER_COLOR: Color<f32> = Color {
    r: 0.0,
    g: 0.5,
    b: 0.8,
    a: 1.0,
};
const BUTTON_PRESS_COLOR: Color<f32> = Color {
    r: 0.0,
    g: 0.3,
    b: 0.6,
    a: 1.0,
};
const TEXT_COLOR: Color<f32> = Color::WHITE;
const CLAN_MAX_SIZE: f32 = 0.15;
const BAR_SIZE: f32 = 0.1;
/// Relative to the framebuffer size
const CLAN_INFO_SIZE: Vec2<f32> = vec2(0.3, 0.2);
/// Relative to the clan info size
const FONT_SIZE: f32 = 0.1;
const CLAN_BACKGROUND_COLOR: Color<f32> = Color {
    r: 0.3,
    g: 0.3,
    b: 0.3,
    a: 1.0,
};
const CLAN_INFO_BACKGROUND_COLOR: Color<f32> = Color {
    r: 0.5,
    g: 0.5,
    b: 0.5,
    a: 1.0,
};
const SHOP_FROZEN_COLOR: Color<f32> = Color {
    r: 0.1,
    g: 0.6,
    b: 0.7,
    a: 1.0,
};

pub struct RenderShop {
    pub layout: ShopLayout,
}

impl RenderShop {
    pub fn new(
        screen_size: Vec2<f32>,
        shop_cards: usize,
        party_cards: usize,
        inventory_cards: usize,
    ) -> Self {
        Self {
            layout: ShopLayout::new(screen_size, shop_cards, party_cards, inventory_cards),
        }
    }
}

pub struct Render {
    geng: Geng,
    pub camera: geng::Camera2d,
    pub framebuffer_size: Vec2<f32>,
    assets: Rc<Assets>,
    card_render: CardRender,
    unit_render: UnitRender,
}

impl Render {
    pub fn new(geng: &Geng, assets: &Rc<Assets>, config: &Config) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            camera: geng::Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: config.fov,
            },
            framebuffer_size: vec2(1.0, 1.0),
            card_render: CardRender::new(geng, assets),
            unit_render: UnitRender::new(geng, assets),
        }
    }

    pub fn draw(
        &mut self,
        shop: &Shop,
        render: &RenderShop,
        game_time: f64,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        self.framebuffer_size = framebuffer.size().map(|x| x as f32);
        ugli::clear(framebuffer, Some(BACKGROUND_COLOR), None);
        let camera = &geng::PixelPerfectCamera;
        let layout = &render.layout;

        let shop_back_color = if shop.frozen {
            SHOP_FROZEN_COLOR
        } else {
            TEXT_BACKGROUND_COLOR
        };
        draw_2d::Quad::new(layout.shop.position, shop_back_color).draw_2d(
            &self.geng,
            framebuffer,
            camera,
        );
        let mut selected_clan = None;
        for (index, card) in shop.cards.shop.iter().enumerate() {
            let layout = render
                .layout
                .shop_cards
                .get(index)
                .expect("Invalid shop layout");
            let hovered_clan =
                self.card_render
                    .draw(layout.position, card.as_ref(), game_time, framebuffer);
            selected_clan = selected_clan.or(hovered_clan);
        }

        for (index, card) in shop.cards.party.iter().enumerate() {
            if let Some(card) = card {
                // TODO: fix position
                let template = &self.assets.units[&card.unit.unit_type];
                let mut unit = card.unit.clone();
                unit.render_position = Position {
                    side: Faction::Player,
                    x: index.try_into().unwrap(),
                    height: Coord::ZERO,
                }
                .to_world();
                self.unit_render.draw_unit(
                    &unit,
                    &card.template,
                    None,
                    game_time,
                    &self.camera,
                    framebuffer,
                );
            }
        }

        // Draw slots
        let factions = [Faction::Player];
        let shader_program = &self.assets.renders_config.slot;
        for faction in factions {
            for i in 0..SIDE_SLOTS {
                let quad = shader_program.get_vertices(&self.geng);
                let framebuffer_size = framebuffer.size();
                let position = Position {
                    x: i as i64,
                    side: faction,
                    height: 0,
                }
                .to_world_f32();
                let empty = shop
                    .cards
                    .party
                    .get(i)
                    .and_then(|card| card.as_ref())
                    .is_some();

                ugli::draw(
                    framebuffer,
                    &shader_program.program,
                    ugli::DrawMode::TriangleStrip,
                    &quad,
                    (
                        ugli::uniforms! {
                            u_time: game_time,
                            u_unit_position: position,
                            u_parent_faction: match faction {
                                Faction::Player => 1.0,
                                Faction::Enemy => -1.0,
                            },
                            u_empty: if empty { 1.0 } else { 0.0 },
                        },
                        geng::camera2d_uniforms(&self.camera, framebuffer_size.map(|x| x as f32)),
                        &shader_program.parameters,
                    ),
                    ugli::DrawParameters {
                        blend_mode: Some(default()),
                        ..default()
                    },
                );
            }
        }

        draw_2d::Quad::new(layout.inventory.position, TEXT_BACKGROUND_COLOR).draw_2d(
            &self.geng,
            framebuffer,
            camera,
        );
        for (index, card) in shop.cards.inventory.iter().enumerate() {
            let layout = render
                .layout
                .inventory_cards
                .get(index)
                .expect("Invalid inventory layout");
            let hovered_clan =
                self.card_render
                    .draw(layout.position, card.as_ref(), game_time, framebuffer);
            selected_clan = selected_clan.or(hovered_clan);
        }

        let text = match tier_up_cost(shop.tier, shop.tier_rounds) {
            Some(cost) => format!("Tier Up ({})", cost),
            None => format!("Tier Up (?)"),
        };
        draw_rectangle(
            &text,
            layout.tier_up.position,
            button_color(&layout.tier_up),
            &self.geng,
            framebuffer,
        );

        draw_rectangle(
            &format!("Tier {}", shop.tier),
            layout.current_tier.position,
            TEXT_BACKGROUND_COLOR,
            &self.geng,
            framebuffer,
        );

        let text = if shop.money == 1 { "coin" } else { "coins" };
        draw_rectangle(
            &format!("{} {}", shop.money, text),
            layout.currency.position,
            TEXT_BACKGROUND_COLOR,
            &self.geng,
            framebuffer,
        );

        draw_rectangle(
            &format!("Reroll"),
            layout.reroll.position,
            button_color(&layout.reroll),
            &self.geng,
            framebuffer,
        );

        draw_rectangle(
            &format!("Freeze"),
            layout.freeze.position,
            button_color(&layout.freeze),
            &self.geng,
            framebuffer,
        );

        draw_rectangle(
            "",
            layout.clans.position,
            TEXT_BACKGROUND_COLOR,
            &self.geng,
            framebuffer,
        );

        let clans = calc_clan_members(
            shop.cards
                .party
                .iter()
                .filter_map(|card| card.as_ref())
                .map(|card| &card.unit),
        );
        let clans = clans.into_iter().sorted().collect::<Vec<_>>();
        if !clans.is_empty() {
            let height = layout.clans.position.height();
            let size = (CLAN_MAX_SIZE * height).min(height / clans.len() as f32);
            let clan_size = vec2(size, size);
            let mut position = layout.clans.position.top_left() + vec2(size, -size) / 2.0;
            for (clan, clan_count) in clans {
                let clan_color = self
                    .assets
                    .options
                    .clan_colors
                    .get(&clan)
                    .copied()
                    .unwrap_or(Color::WHITE);
                let text_color = Color::WHITE;
                let text = format!("{:?}", clan)
                    .chars()
                    .next()
                    .unwrap_or('?')
                    .to_uppercase()
                    .to_string();
                draw_2d::Ellipse::circle(position, size / 2.0, clan_color).draw_2d(
                    &self.geng,
                    framebuffer,
                    camera,
                );
                draw_2d::Text::unit(&**self.geng.default_font(), text, text_color)
                    .fit_into(AABB::point(position).extend_uniform(size / 2.0 / 2.0.sqrt()))
                    .draw_2d(&self.geng, framebuffer, camera);

                if let Some(config) = shop.config.render.clans.get(&clan) {
                    let bar_size = vec2(size * BAR_SIZE, size / config.rows as f32);
                    for x in 0..config.columns {
                        for y in 0..config.rows {
                            let position = position
                                + clan_size / 2.0
                                + bar_size * vec2(x as f32, -(y as f32) - 1.0);
                            let color = if x * config.rows + y < clan_count {
                                clan_color
                            } else {
                                CLAN_BACKGROUND_COLOR
                            };
                            draw_2d::Quad::new(
                                AABB::point(position).extend_positive(bar_size),
                                color,
                            )
                            .draw_2d(&self.geng, framebuffer, camera);
                        }
                    }

                    let mouse_pos = self.geng.window().mouse_pos().map(|x| x as f32);
                    if AABB::point(position)
                        .extend_uniform(size / 2.0)
                        .contains(mouse_pos)
                    {
                        selected_clan = selected_clan.or(Some(clan));
                    }
                }

                position.y -= size;
            }
        }

        draw_rectangle(
            &format!("Go"),
            layout.go.position,
            button_color(&layout.go),
            &self.geng,
            framebuffer,
        );

        if let Some(drag) = &shop.drag {
            match &drag.target {
                DragTarget::Card { card, .. } => {
                    let aabb =
                        AABB::point(drag.position).extend_symmetric(layout.drag_card_size / 2.0);
                    selected_clan = selected_clan.or(self.card_render.draw(
                        aabb,
                        Some(card),
                        game_time,
                        framebuffer,
                    ));
                }
            }
        }

        if let Some(clan) = selected_clan {
            if let Some(config) = shop.config.render.clans.get(&clan) {
                // Show clan info
                let mouse_pos = self.geng.window().mouse_pos().map(|x| x as f32);
                let description = format!("{:?}\n{}", clan, config.description);
                draw_clan_info(mouse_pos, &description, &self.geng, framebuffer, camera);
            }
        }
    }
}

fn draw_clan_info(
    position: Vec2<f32>,
    description: &str,
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
    camera: &impl geng::AbstractCamera2d,
) {
    let framebuffer_size = framebuffer.size().map(|x| x as f32);
    let info_aabb = AABB::point(position).extend_positive(CLAN_INFO_SIZE * framebuffer_size);
    let dx = (info_aabb.x_max - framebuffer_size.x).max(0.0);
    let dy = (info_aabb.y_max - framebuffer_size.y).max(0.0);
    let info_aabb = info_aabb.translate(-vec2(dx, dy));
    draw_2d::Quad::new(info_aabb, CLAN_INFO_BACKGROUND_COLOR).draw_2d(geng, framebuffer, camera);
    crate::render::draw_text_wrapped(
        &**geng.default_font(),
        description,
        FONT_SIZE * info_aabb.height(),
        info_aabb,
        TEXT_COLOR,
        framebuffer,
        camera,
    );
}

fn button_color(widget: &LayoutWidget) -> Color<f32> {
    if widget.pressed {
        BUTTON_PRESS_COLOR
    } else if widget.hovered {
        BUTTON_HOVER_COLOR
    } else {
        BUTTON_COLOR
    }
}

fn draw_rectangle(
    text: impl AsRef<str>,
    aabb: AABB<f32>,
    color: Color<f32>,
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
) {
    let camera = &geng::PixelPerfectCamera;
    draw_2d::Quad::new(aabb, color).draw_2d(geng, framebuffer, camera);
    draw_2d::Text::unit(&**geng.default_font(), text, TEXT_COLOR)
        .fit_into(
            AABB::point(aabb.center()).extend_symmetric(aabb.size() * TEXT_OCCUPY_SPACE / 2.0),
        )
        .draw_2d(geng, framebuffer, camera);
}
