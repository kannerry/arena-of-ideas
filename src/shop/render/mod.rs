use super::*;

mod card;
mod layout;

pub use card::*;
use geng::Draw2d;
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
    camera: geng::Camera2d,
    assets: Rc<Assets>,
    card_render: CardRender,
}

impl Render {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            camera: geng::Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: 0.5,
            },
            card_render: CardRender::new(geng, assets),
        }
    }

    pub fn draw(
        &mut self,
        shop: &Shop,
        render: &RenderShop,
        game_time: f32,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        ugli::clear(framebuffer, Some(BACKGROUND_COLOR), None);
        let camera = &geng::PixelPerfectCamera;
        let layout = &render.layout;

        draw_2d::Quad::new(layout.shop.position, TEXT_BACKGROUND_COLOR).draw_2d(
            &self.geng,
            framebuffer,
            camera,
        );
        for (index, card) in shop.cards.shop.iter().enumerate() {
            let layout = render
                .layout
                .shop_cards
                .get(index)
                .expect("Invalid shop layout");
            self.card_render
                .draw(layout.position, card.as_ref(), game_time, framebuffer);
        }

        draw_2d::Quad::new(layout.party.position, TEXT_BACKGROUND_COLOR).draw_2d(
            &self.geng,
            framebuffer,
            camera,
        );
        for (index, card) in shop.cards.party.iter().enumerate() {
            let layout = render
                .layout
                .party_cards
                .get(index)
                .expect("Invalid party layout");
            self.card_render
                .draw(layout.position, card.as_ref(), game_time, framebuffer);
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
            self.card_render
                .draw(layout.position, card.as_ref(), game_time, framebuffer);
        }

        let text = match tier_up_cost(shop.tier) {
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
            &format!("TODO: Alliances"),
            layout.alliances.position,
            TEXT_BACKGROUND_COLOR,
            &self.geng,
            framebuffer,
        );

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
                    self.card_render
                        .draw(aabb, Some(card), game_time, framebuffer);
                }
            }
        }
    }
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
