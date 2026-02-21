use crate::render::TerminalRenderer;
use crossterm::style::Color;
use rand::prelude::*;
use std::io;

struct AshParticle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    age: u16,
    max_age: u16,
}

pub struct VolcanoSystem {
    ash: Vec<AshParticle>,
    timer: u16,
}

impl VolcanoSystem {
    pub fn new() -> Self {
        Self {
            ash: Vec::new(),
            timer: 0,
        }
    }

    pub fn update(
        &mut self,
        terminal_width: u16,
        horizon_y: u16,
        rng: &mut impl Rng,
    ) {
        let (volcano_x, volcano_width, volcano_height) = Self::layout(terminal_width);
        let crater_x = volcano_x + volcano_width / 2;
        let crater_y = horizon_y.saturating_sub(volcano_height + 1);

        let spawn_rate = if self.timer % 3 == 0 { 3 } else { 1 };
        for _ in 0..spawn_rate {
            let jitter = rng.random_range(-1.5..1.5);
            self.ash.push(AshParticle {
                x: crater_x as f32 + jitter,
                y: crater_y as f32,
                vx: rng.random_range(-0.2..0.2),
                vy: rng.random_range(-0.8..-0.3),
                age: 0,
                max_age: rng.random_range(30..80),
            });
        }

        self.ash.retain_mut(|particle| {
            particle.x += particle.vx;
            particle.y += particle.vy;
            particle.vy -= 0.01;
            particle.age += 1;
            particle.age < particle.max_age && particle.y > 0.0
        });

        self.timer = self.timer.wrapping_add(1);
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        terminal_width: u16,
        horizon_y: u16,
    ) -> io::Result<()> {
        let (volcano_x, _, _) = Self::layout(terminal_width);
        let base_y = horizon_y.saturating_sub(5);

        let volcano = [
            "    /\\",
            "   /  \\",
            "  /^^^^\\",
            " /      \\",
            "/________\\",
        ];

        for (idx, line) in volcano.iter().enumerate() {
            let y = base_y + idx as u16;
            for (offset, ch) in line.chars().enumerate() {
                renderer.render_char(
                    volcano_x + offset as u16,
                    y,
                    ch,
                    Color::DarkRed,
                )?;
            }
        }

        for particle in &self.ash {
            let x = particle.x as i16;
            let y = particle.y as i16;
            if x >= 0 && x < terminal_width as i16 && y >= 0 {
                let ch = if particle.age % 3 == 0 { '.' } else { '*' };
                renderer.render_char(x as u16, y as u16, ch, Color::DarkGrey)?;
            }
        }

        Ok(())
    }

    fn layout(terminal_width: u16) -> (u16, u16, u16) {
        let volcano_width = 10;
        let volcano_height = 5;
        let volcano_x = (terminal_width / 6).saturating_sub(volcano_width / 2);
        (volcano_x, volcano_width, volcano_height)
    }
}
