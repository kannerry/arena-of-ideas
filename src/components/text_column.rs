use bevy_egui::egui::Rect;

use super::*;

#[derive(Component)]
pub struct TextColumn {
    lines: Vec<(f32, ColoredString)>,
    entity: Entity,
}

impl TextColumn {
    pub fn new(entity: Entity) -> Self {
        Self {
            lines: default(),
            entity,
        }
    }
    pub fn render(&self, ctx: &egui::Context, world: &World) {
        const LIFETIME: f32 = 4.0;
        const EASE_IN: f32 = 0.4;
        const EASE_OUT: f32 = 0.5;

        let width = world.resource::<CameraData>().slot_pixel_spacing * 0.6;
        let pos = entity_screen_pos(self.entity, vec2(0.0, 1.5), world).to_pos2();
        let rect = Rect::from_two_pos(pos, pos2(pos.x, ctx.screen_rect().top()))
            .expand2(egui::vec2(width, 0.0));
        let t = GameTimer::get().play_head();

        Window::new("Text column")
            .id(Id::new(self.entity).with("text_column"))
            .fixed_rect(rect)
            .pivot(Align2::LEFT_TOP)
            .title_bar(false)
            .constrain(false)
            .interactable(false)
            .frame(Frame::none().fill(red()))
            .show(ctx, |ui| {
                let ui = &mut ui.child_ui(
                    ui.available_rect_before_wrap(),
                    Layout::bottom_up(egui::Align::Center),
                );
                for (ts, line) in self
                    .lines
                    .iter()
                    .rev()
                    .filter(|(ts, _)| *ts < t && *ts + LIFETIME > t)
                {
                    let t = t - *ts;
                    let t = smoothstep(0.0, EASE_IN, t)
                        .min(1.0 - smoothstep(LIFETIME - EASE_OUT, LIFETIME, t))
                        .clamp(0.0, 1.0);
                    line.label_alpha(t, ui);
                    ui.add_space(5.0);
                }
            });
    }
    pub fn add_colored(entity: Entity, text: ColoredString, world: &mut World) -> Result<()> {
        if SkipVisual::active(world) {
            return Ok(());
        }
        world
            .get_mut::<Self>(entity)
            .context("No TextColumn component")?
            .lines
            .push((GameTimer::get().insert_head(), text));
        Ok(())
    }
    pub fn add(entity: Entity, text: &str, color: Color32, world: &mut World) -> Result<()> {
        Self::add_colored(
            entity,
            text.add_color(color)
                .set_style(ColoredStringStyle::Bold)
                .take(),
            world,
        )
    }
}
