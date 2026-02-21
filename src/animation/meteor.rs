use crate::render::TerminalRenderer;
use crossterm::style::Color;
use rand::prelude::*;
use std::io;

#[derive(Clone, Copy)]
struct TrailParticle {
    x: f32,
    y: f32,
    life: u8,
}

#[derive(Clone, Copy)]
struct ImpactParticle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: u8,
    max_life: u8,
}

enum MeteorState {
    Flying,
    Impact,
    Aftermath,
}

pub struct MeteorSystem {
    state: MeteorState,
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    trail: Vec<TrailParticle>,
    impact: Vec<ImpactParticle>,
    timer: u16,
    flash_timer: u8,
}

impl MeteorSystem {
    pub fn new(terminal_width: u16) -> Self {
        Self {
            state: MeteorState::Flying,
            x: terminal_width as f32 - 2.0,
            y: 2.0,
            vx: -1.1,
            vy: 0.7,
            trail: Vec::new(),
            impact: Vec::new(),
            timer: 0,
            flash_timer: 0,
        }
    }

    pub fn update(
        &mut self,
        terminal_width: u16,
        horizon_y: u16,
        rng: &mut impl Rng,
    ) {
        match self.state {
            MeteorState::Flying => {
                self.x += self.vx;
                self.y += self.vy;
                self.trail.push(TrailParticle {
                    x: self.x + 1.0,
                    y: self.y - 0.5,
                    life: 6,
                });

                if self.y >= horizon_y as f32 - 2.0 {
                    self.state = MeteorState::Impact;
                    self.flash_timer = 4;
                    self.timer = 0;
                    self.impact.clear();
                    for _ in 0..40 {
                        let angle = rng.random_range(0.0..std::f32::consts::PI * 2.0);
                        let speed = rng.random_range(0.3..1.6);
                        self.impact.push(ImpactParticle {
                            x: self.x,
                            y: horizon_y as f32 - 1.0,
                            vx: angle.cos() * speed,
                            vy: angle.sin() * speed,
                            life: 0,
                            max_life: rng.random_range(6..16),
                        });
                    }
                }
            }
            MeteorState::Impact => {
                self.timer += 1;
                for particle in &mut self.impact {
                    particle.x += particle.vx;
                    particle.y += particle.vy;
                    particle.vy += 0.05;
                    particle.life += 1;
                }
                self.impact.retain(|p| p.life < p.max_life);
                if self.timer > 14 {
                    self.state = MeteorState::Aftermath;
                    self.timer = 0;
                }
            }
            MeteorState::Aftermath => {
                self.timer += 1;
                if self.timer > 40 {
                    self.reset(terminal_width);
                }
            }
        }

        self.trail.retain_mut(|p| {
            if p.life == 0 {
                false
            } else {
                p.life -= 1;
                true
            }
        });

        if self.flash_timer > 0 {
            self.flash_timer -= 1;
        }
    }

    pub fn is_flashing(&self) -> bool {
        self.flash_timer > 0
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        terminal_width: u16,
        terminal_height: u16,
        horizon_y: u16,
    ) -> io::Result<()> {
        for particle in &self.trail {
            let x = particle.x as i16;
            let y = particle.y as i16;
            if x >= 0 && x < terminal_width as i16 && y >= 0 && y < terminal_height as i16 {
                let ch = if particle.life > 3 { '*' } else { '.' };
                renderer.render_char(x as u16, y as u16, ch, Color::DarkYellow)?;
            }
        }

        match self.state {
            MeteorState::Flying => {
                let x = self.x as i16;
                let y = self.y as i16;
                if x >= 0 && x < terminal_width as i16 && y >= 0 {
                    renderer.render_char(x as u16, y as u16, '*', Color::Yellow)?;
                    if x + 1 < terminal_width as i16 && y + 1 < terminal_height as i16 {
                        renderer.render_char((x + 1) as u16, (y + 1) as u16, '/', Color::Yellow)?;
                    }
                }
            }
            MeteorState::Impact => {
                let impact_y = horizon_y.saturating_sub(1);
                for particle in &self.impact {
                    let x = particle.x as i16;
                    let y = particle.y as i16;
                    if x >= 0 && x < terminal_width as i16 && y >= 0 && y < terminal_height as i16
                    {
                        let ch = if particle.life % 2 == 0 { 'X' } else { '*' };
                        renderer.render_char(x as u16, y as u16, ch, Color::Red)?;
                    }
                }
                if self.timer < 6 {
                    for x in 0..terminal_width {
                        if x % 4 == 0 {
                            renderer.render_char(x, impact_y, '*', Color::Yellow)?;
                        }
                    }
                }
            }
            MeteorState::Aftermath => {
                let smoke_y = horizon_y.saturating_sub(3);
                for i in 0..6 {
                    let x = (self.x as i16 + i * 2 - 6).clamp(0, terminal_width as i16 - 1);
                    let y = smoke_y.saturating_sub((i % 3) as u16);
                    renderer.render_char(x as u16, y, '.', Color::DarkGrey)?;
                }
            }
        }

        Ok(())
    }

    fn reset(&mut self, terminal_width: u16) {
        self.state = MeteorState::Flying;
        self.x = terminal_width as f32 - 2.0;
        self.y = 2.0;
        self.vx = -1.1;
        self.vy = 0.7;
        self.timer = 0;
        self.trail.clear();
        self.impact.clear();
    }
}
