use crate::shop::render::CardRender;
use strfmt::strfmt;

use super::*;

impl Render {
    pub fn draw_unit_with_position(
        &self,
        unit: &Unit,
        game_time: f32,
        framebuffer: &mut ugli::Framebuffer,
        position: AABB<f32>,
    ) {
        // TODO: move this into to an earlier phase perhaps
        let shader_program = &self.assets.get_render(&unit.render.base_shader_config);
        let quad = shader_program.get_vertices(&self.geng);

        let clan_colors: Vec<Rgba<f32>> = unit
            .clans
            .iter()
            .map(|clan| self.assets.clans[clan].color)
            .collect();

        // Actual render
        let texture_position = position;
        let texture_size =
            (texture_position.height() * framebuffer.size().y as f32 / self.camera.fov * 2.0)
                .max(1.0) as usize;
        let texture_size = vec2(texture_size, texture_size);
        let texture_camera = geng::Camera2d {
            center: texture_position.center(),
            rotation: 0.0,
            fov: texture_position.height(),
        };
        let uniforms = (
            ugli::uniforms! {
                u_time: game_time,
                u_unit_position: position.center(),
                u_unit_radius: unit.render.radius.as_f32(),
                u_spawn: 1.0,
                u_random: unit.random_number.as_f32(),
                u_action_time: unit.render.last_action_time.as_f32(),
                u_injure_time: unit.render.last_injure_time.as_f32(),
                u_heal_time: unit.render.last_heal_time.as_f32(),
                u_parent_faction: match unit.faction {
                        Faction::Player => 1.0,
                        Faction::Enemy => -1.0,
                    },
                u_clan_color_1: clan_colors.get(0).copied().unwrap_or(Rgba::WHITE),
                u_clan_color_2: clan_colors.get(1).copied().unwrap_or(Rgba::WHITE),
                u_clan_color_3: clan_colors.get(2).copied().unwrap_or(Rgba::WHITE),
                u_clan_count: clan_colors.len(),
                u_ability_ready: 1.0,
                u_health: 1.0,
            },
            geng::camera2d_uniforms(&texture_camera, texture_size.map(|x| x as f32)),
            shader_program.parameters.clone(),
        );

        let mut texture = ugli::Texture::new_uninitialized(self.geng.ugli(), texture_size);
        {
            let mut framebuffer = ugli::Framebuffer::new_color(
                self.geng.ugli(),
                ugli::ColorAttachment::Texture(&mut texture),
            );
            let framebuffer = &mut framebuffer;
            ugli::clear(framebuffer, Some(Rgba::TRANSPARENT_WHITE), None, None);
            ugli::draw(
                framebuffer,
                &shader_program.program,
                ugli::DrawMode::TriangleStrip,
                &quad,
                &uniforms,
                ugli::DrawParameters {
                    // blend_mode: Some(default()),
                    ..default()
                },
            );
        }

        let clan_shader_programs = unit.render.clan_shader_configs
            [(unit.stats.get(UnitStat::Level) as usize) - 1]
            .iter()
            .map(|x| self.assets.get_render(&x))
            .collect_vec();

        for (ind, color) in clan_colors.iter().enumerate() {
            if clan_shader_programs.len() <= ind {
                continue;
            }
            let program = clan_shader_programs[ind].clone();
            let mut new_texture = ugli::Texture::new_uninitialized(self.geng.ugli(), texture_size);
            {
                let mut framebuffer = ugli::Framebuffer::new_color(
                    self.geng.ugli(),
                    ugli::ColorAttachment::Texture(&mut new_texture),
                );
                let framebuffer = &mut framebuffer;
                ugli::clear(framebuffer, Some(Rgba::TRANSPARENT_WHITE), None, None);
                ugli::draw(
                    framebuffer,
                    &program.program,
                    ugli::DrawMode::TriangleStrip,
                    &quad,
                    (
                        &uniforms,
                        ugli::uniforms! {
                            u_previous_texture: &texture,
                            u_time: game_time,
                            u_color: color,
                        },
                        program.parameters,
                    ),
                    ugli::DrawParameters {
                        // blend_mode: Some(default()),
                        ..default()
                    },
                );
            }
            texture = new_texture;
        }

        let mut statuses: Vec<_> = unit
            .all_statuses
            .iter()
            .filter_map(|status| {
                self.assets
                    .statuses
                    .get(&status.status.name)
                    .filter(|config| config.render.is_some())
                    // .and_then(|config| config.render.as_ref())
                    .map(|config| {
                        (
                            self.assets.get_render(config.render.as_ref().unwrap()),
                            status.time,
                            status.status.duration,
                            config.get_color(&self.assets.clans),
                        )
                    })
            })
            .collect();
        let status_count = statuses.len();
        for (status_index, (program, status_time, status_duration, status_color)) in
            statuses.into_iter().enumerate()
        {
            let mut new_texture = ugli::Texture::new_uninitialized(self.geng.ugli(), texture_size);
            {
                let mut framebuffer = ugli::Framebuffer::new_color(
                    self.geng.ugli(),
                    ugli::ColorAttachment::Texture(&mut new_texture),
                );
                let framebuffer = &mut framebuffer;
                let status_time = status_time.unwrap_or(0);
                let status_duration = status_duration.unwrap_or_else(|| 1.try_into().unwrap());
                ugli::clear(framebuffer, Some(Rgba::TRANSPARENT_WHITE), None, None);
                ugli::draw(
                    framebuffer,
                    &program.program,
                    ugli::DrawMode::TriangleStrip,
                    &quad,
                    (
                        &uniforms,
                        ugli::uniforms! {
                            u_previous_texture: &texture,
                            u_status_count: status_count,
                            u_status_index: status_index,
                            u_status_time: status_time as f32,
                            u_status_duration: u64::from(status_duration) as f32,
                            u_time: game_time,
                            u_color: status_color,
                        },
                        program.parameters,
                    ),
                    ugli::DrawParameters {
                        // blend_mode: Some(default()),
                        ..default()
                    },
                );
            }
            texture = new_texture;
        }
        self.geng.draw_2d(
            framebuffer,
            &self.camera,
            &draw_2d::TexturedQuad::new(texture_position, texture),
        );
    }

    pub fn draw_unit(&self, unit: &Unit, game_time: f32, framebuffer: &mut ugli::Framebuffer) {
        let position = AABB::point(unit.render.render_position.map(|x| x.as_f32()))
            .extend_uniform(unit.render.radius.as_f32() * 2.0); // TODO: configuring?
        self.draw_unit_with_position(unit, game_time, framebuffer, position)
    }

    pub fn draw_unit_stats(&self, unit: &Unit, framebuffer: &mut ugli::Framebuffer) {
        let radius = unit.render.radius.as_f32();

        // Draw damage and health
        let unit_aabb =
            AABB::point(unit.render.render_position.map(|x| x.as_f32())).extend_uniform(radius);
        let size = radius * 0.5;
        let damage = AABB::point(unit_aabb.bottom_left())
            .extend_right(size)
            .extend_up(size)
            .translate(vec2(0.0, -0.1));
        let health = AABB::point(unit_aabb.bottom_right())
            .extend_left(size)
            .extend_up(size)
            .translate(vec2(0.0, -0.1));
        let lvl = AABB::point(unit_aabb.bottom_right())
            .extend_left(size * 2.0)
            .extend_up(size)
            .translate(vec2(-0.1, -0.85));
        let next_lvl = AABB::point(unit_aabb.bottom_right())
            .extend_left(size * 2.0)
            .extend_up(size * 0.5)
            .translate(vec2(-0.1, -1.0));

        draw_2d::Quad::new(
            damage.extend_uniform(0.03),
            Rgba::try_from("#d0a632").unwrap(),
        )
        .draw_2d(&self.geng, framebuffer, &self.camera);
        draw_2d::Quad::new(
            health.extend_uniform(0.03),
            Rgba::try_from("#e13d2f").unwrap(),
        )
        .draw_2d(&self.geng, framebuffer, &self.camera);
        let text_color = Rgba::try_from("#ffffff").unwrap();
        draw_2d::Text::unit(
            self.geng.default_font().clone(),
            format!("{:.0}", unit.stats.attack),
            text_color,
        )
        .fit_into(damage)
        .draw_2d(&self.geng, framebuffer, &self.camera);
        draw_2d::Text::unit(
            self.geng.default_font().clone(),
            format!("{:.0}", unit.stats.health),
            text_color,
        )
        .fit_into(health)
        .draw_2d(&self.geng, framebuffer, &self.camera);
        if unit.faction == Faction::Player {
            draw_2d::Quad::new(lvl.extend_uniform(0.03), Rgba::try_from("#004ac2").unwrap())
                .draw_2d(&self.geng, framebuffer, &self.camera);
            draw_2d::Text::unit(
                self.geng.default_font().clone(),
                format!("LVL:{}", unit.stats.level().to_string()),
                text_color,
            )
            .fit_into(lvl)
            .draw_2d(&self.geng, framebuffer, &self.camera);
            if unit.stats.level() < MAX_LEVEL {
                draw_2d::Quad::new(
                    next_lvl.extend_uniform(0.03),
                    Rgba::try_from("#00e2de").unwrap(),
                )
                .draw_2d(&self.geng, framebuffer, &self.camera);
                draw_2d::Text::unit(
                    self.geng.default_font().clone(),
                    format!("NEXT:{}", unit.stats.stacks_left_to_level().to_string()),
                    text_color,
                )
                .fit_into(next_lvl)
                .draw_2d(&self.geng, framebuffer, &self.camera);
            }
        }

        // Draw name
        let name_aabb = AABB::point(unit_aabb.bottom_left())
            .translate(vec2(0.0, -0.3))
            .extend_right(radius * 2.0)
            .extend_down(radius * 0.7);
        let text_color = Rgba::try_from("#838383").unwrap();

        draw_2d::Text::unit(
            self.geng.default_font().clone(),
            unit.unit_type.to_string(),
            text_color,
        )
        .fit_into(name_aabb)
        .draw_2d(&self.geng, framebuffer, &self.camera);
    }

    pub fn draw_hover(
        &self,
        model: &Model,
        unit: &Unit,
        camera: &geng::Camera2d,
        framebuffer: &mut ugli::Framebuffer,
        vars: HashMap<VarName, i32>,
    ) {
        if unit.clans.is_empty() {
            return;
        }
        let aabb = AABB::point(unit.render.render_position.map(|x| x.as_f32()))
            .extend_uniform(0.5)
            .translate(vec2(0.0, -1.5));
        let clan_color = self.assets.clans[&unit.clans[0]]
            .color
            .clone();
        draw_2d::Quad::new(aabb, clan_color).draw_2d(&self.geng, framebuffer, camera);
        draw_2d::Quad::new(aabb.extend_uniform(-0.05), Rgba::BLACK).draw_2d(
            &self.geng,
            framebuffer,
            camera,
        );
        let font_size = 0.17;
        let template = model.unit_templates.get(&unit.unit_type).unwrap();
        let text = strfmt(&template.description, &vars).unwrap_or(template.description.clone());
        crate::render::draw_text_wrapped(
            &**self.geng.default_font(),
            &text,
            font_size,
            aabb.extend_uniform(-0.07),
            Rgba::WHITE,
            framebuffer,
            camera,
        );
    }
}
