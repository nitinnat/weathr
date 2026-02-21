use crate::render::TerminalRenderer;
use crossterm::style::Color;
use rand::prelude::*;
use std::io;

struct Hailstone {
    x: f32,
    y: f32,
    vy: f32,
    vx: f32,
    bounced: bool,
    big: bool,
}

pub struct HailSystem {
    stones: Vec<Hailstone>,
    terminal_width: u16,
    terminal_height: u16,
    ground_y: u16,
}

impl HailSystem {
    pub fn new(terminal_width: u16, terminal_height: u16, ground_y: u16) -> Self {
        Self {
            stones: Vec::new(),
            terminal_width,
            terminal_height,
            ground_y,
        }
    }

    fn spawn_stone(&mut self, rng: &mut impl Rng) {
        let x = rng.random_range(0.0..self.terminal_width as f32);
        let speed = rng.random_range(0.7..1.4);
        let drift = rng.random_range(-0.2..0.2);
        self.stones.push(Hailstone {
            x,
            y: 0.0,
            vy: speed,
            vx: drift,
            bounced: false,
            big: rng.random::<f32>() > 0.65,
        });
    }

    pub fn update(
        &mut self,
        terminal_width: u16,
        terminal_height: u16,
        ground_y: u16,
        rng: &mut impl Rng,
    ) {
        self.terminal_width = terminal_width;
        self.terminal_height = terminal_height;
        self.ground_y = ground_y;

        let target_count = (terminal_width / 2).max(10) as usize;
        if self.stones.len() < target_count {
            for _ in 0..3 {
                self.spawn_stone(rng);
            }
        }

        let ground_y = self.ground_y.saturating_sub(1) as f32;
        self.stones.retain_mut(|stone| {
            stone.y += stone.vy;
            stone.x += stone.vx;

            if stone.y >= ground_y {
                if !stone.bounced {
                    stone.y = ground_y;
                    stone.vy = -stone.vy * 0.45;
                    stone.vx *= 0.5;
                    stone.bounced = true;
                    return true;
                }
                return false;
            }

            if stone.x < -5.0 || stone.x > (self.terminal_width as f32 + 5.0) {
                return false;
            }

            true
        });
    }

    pub fn render(&self, renderer: &mut TerminalRenderer) -> io::Result<()> {
        for stone in &self.stones {
            let x = stone.x as i16;
            let y = stone.y as i16;
            if x >= 0
                && x < self.terminal_width as i16
                && y >= 0
                && y < self.terminal_height as i16
            {
                let ch = if stone.big { 'O' } else { 'o' };
                let color = if stone.big { Color::Cyan } else { Color::White };
                renderer.render_char(x as u16, y as u16, ch, color)?;
            }
        }
        Ok(())
    }
}
