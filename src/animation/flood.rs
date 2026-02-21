use crate::render::TerminalRenderer;
use crossterm::style::Color;
use std::io;

pub struct FloodSystem {
    level: f32,
    target_level: f32,
    phase: f32,
}

impl FloodSystem {
    pub fn new() -> Self {
        Self {
            level: 0.0,
            target_level: 0.0,
            phase: 0.0,
        }
    }

    pub fn update(&mut self, terminal_width: u16, terminal_height: u16) {
        let max_level = (terminal_height as f32 * 0.45).min(terminal_height as f32 - 3.0);
        if self.target_level != max_level {
            self.target_level = max_level;
        }

        if self.level < self.target_level {
            self.level += 0.08 + (terminal_width as f32 / 1000.0);
        }

        self.phase = (self.phase + 0.5) % 12.0;
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        terminal_width: u16,
        terminal_height: u16,
    ) -> io::Result<()> {
        if self.level < 1.0 {
            return Ok(());
        }

        let waterline_y = terminal_height.saturating_sub(self.level as u16);
        let surface_color = Color::Cyan;
        let deep_color = Color::Blue;

        for y in waterline_y..terminal_height {
            let depth = (y - waterline_y) as f32;
            let color = if depth < 2.0 { surface_color } else { deep_color };
            for x in 0..terminal_width {
                let ch = if y == waterline_y {
                    let idx = ((x as f32 + self.phase) as u16) % 6;
                    match idx {
                        0 | 3 => '~',
                        1 | 4 => '-',
                        _ => '=',
                    }
                } else {
                    if (x + y) % 7 == 0 { '~' } else { '=' }
                };
                renderer.render_char(x, y, ch, color)?;
            }
        }
        Ok(())
    }
}
