use crate::render::TerminalRenderer;
use crossterm::style::Color;
use rand::prelude::*;
use std::io;

struct Debris {
    x: f32,
    y: f32,
    vx: f32,
    life: u8,
}

pub struct TsunamiSystem {
    phase: f32,
    wave_x: f32,
    speed: f32,
    debris: Vec<Debris>,
    spray_timer: u8,
}

impl TsunamiSystem {
    pub fn new() -> Self {
        Self {
            phase: 0.0,
            wave_x: 0.0,
            speed: 0.9,
            debris: Vec::new(),
            spray_timer: 0,
        }
    }

    pub fn update(&mut self, terminal_width: u16, horizon_y: u16, rng: &mut impl Rng) {
        let wave_width = (terminal_width as f32 * 0.6).max(20.0);
        self.wave_x += self.speed;
        if self.wave_x > terminal_width as f32 + wave_width {
            self.wave_x = -wave_width;
        }
        self.phase = (self.phase + 0.3) % 12.0;
        self.spray_timer = self.spray_timer.wrapping_add(1);

        if rng.random::<f32>() > 0.8 {
            let y = horizon_y.saturating_sub(1) as f32;
            self.debris.push(Debris {
                x: rng.random_range(0.0..terminal_width as f32),
                y,
                vx: rng.random_range(-0.2..0.6),
                life: 30 + (rng.random::<u8>() % 40),
            });
        }

        self.debris.retain_mut(|d| {
            d.x += d.vx;
            d.life = d.life.saturating_sub(1);
            d.life > 0 && d.x > -5.0 && d.x < terminal_width as f32 + 5.0
        });
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        terminal_width: u16,
        terminal_height: u16,
        horizon_y: u16,
    ) -> io::Result<()> {
        let wave_width = (terminal_width as f32 * 0.6).max(20.0);
        let amplitude = (terminal_height as f32 * 0.25).min(10.0).max(5.0);
        let base = 2.0;
        let surface_color = Color::Cyan;
        let deep_color = Color::Blue;

        for y in horizon_y..terminal_height {
            for x in 0..terminal_width {
                let ch = if (x + y) % 8 == 0 { '~' } else { '=' };
                renderer.render_char(x, y, ch, deep_color)?;
            }
        }

        for x in 0..terminal_width {
            let dx = (x as f32 - self.wave_x).abs();
            if dx < wave_width / 2.0 {
                let t = 1.0 - (dx / (wave_width / 2.0));
                let crest_height = amplitude * t * t;
                let y = (horizon_y as f32 - base - crest_height).max(1.0);
                let y = y as u16;
                renderer.render_char(x, y, '~', surface_color)?;
                if y > 0 && (x as f32 + self.phase) % 5.0 < 1.0 {
                    renderer.render_char(x, y - 1, '\'', Color::White)?;
                }
                if y + 1 < terminal_height {
                    renderer.render_char(x, y + 1, '=', surface_color)?;
                }
                if self.spray_timer % 4 == 0 && y > 1 {
                    renderer.render_char(x, y - 2, '.', Color::White)?;
                }
            }
        }

        for d in &self.debris {
            let x = d.x as i16;
            let y = d.y as i16;
            if x >= 0 && x < terminal_width as i16 && y >= 0 {
                let ch = if d.life % 3 == 0 { '#' } else { '=' };
                renderer.render_char(x as u16, y as u16, ch, Color::DarkYellow)?;
            }
        }
        Ok(())
    }
}
