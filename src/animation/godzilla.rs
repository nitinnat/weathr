use crate::render::TerminalRenderer;
use crossterm::style::Color;
use rand::prelude::*;
use std::io;

const GODZILLA_FRAMES: [&[&str]; 3] = [
    &[
        r"        __",
        r"   _.-'`  `-._",
        r"  /  o  o     \",
        r" /     ^      \",
        r"<__/\_____/\__>",
        r"   /   ||   \",
        r"  /_/\_||_/\_\",
        r"     /_/  \",
    ],
    &[
        r"        __",
        r"   _.-'`  `-._",
        r"  /  o  o     \",
        r" /     ^      \",
        r"<__/\_____/\__>",
        r"   /   ||   \",
        r"  /_/\_||_/\_\",
        r"     /_\  /_\",
    ],
    &[
        r"        __",
        r"   _.-'`  `-._",
        r"  /  o  o     \",
        r" /     ^      \",
        r"<__/\_____/\__>",
        r"   /   ||   \",
        r"  /_/\_||_/\_\",
        r"    /_/    \",
    ],
];

const SKYLINE: [&str; 7] = [
    r"   /\      | |     /\   ",
    r"  /  \   /\ | |    /  \ ",
    r" /____\ /__\| |   /____\",
    r"   ||     || | |    ||  ",
    r"  /||\   /||\| |   /||\ ",
    r"________________________",
    r"|TOKYO|[NEON]|SKY|______",
];

pub struct GodzillaSystem {
    x: i16,
    frame: usize,
    timer: u8,
    breath_timer: u8,
    breath_cooldown: u8,
    stomp_timer: u8,
}

impl GodzillaSystem {
    pub fn new() -> Self {
        Self {
            x: -20,
            frame: 0,
            timer: 0,
            breath_timer: 0,
            breath_cooldown: 30,
            stomp_timer: 0,
        }
    }

    pub fn update(&mut self, terminal_width: u16, rng: &mut impl Rng) {
        let sprite_width = GODZILLA_FRAMES[0][0].len() as i16;
        self.timer = self.timer.wrapping_add(1);
        if self.timer % 6 == 0 {
            self.frame = (self.frame + 1) % GODZILLA_FRAMES.len();
        }

        self.x += 1;
        if self.timer % 8 == 0 {
            self.stomp_timer = 3;
        }
        if self.stomp_timer > 0 {
            self.stomp_timer -= 1;
        }

        if self.breath_timer > 0 {
            self.breath_timer -= 1;
        } else if self.breath_cooldown > 0 {
            self.breath_cooldown -= 1;
        } else if rng.random::<f32>() > 0.75 {
            self.breath_timer = 10 + (rng.random::<u8>() % 8);
            self.breath_cooldown = 40 + (rng.random::<u8>() % 30);
        }

        if self.x > terminal_width as i16 + sprite_width {
            self.x = -sprite_width;
        }
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        terminal_width: u16,
        horizon_y: u16,
    ) -> io::Result<()> {
        let sprite = GODZILLA_FRAMES[self.frame];
        let sprite_height = sprite.len() as u16;
        let y_start = horizon_y.saturating_sub(sprite_height + 1);

        let skyline_width = SKYLINE[0].len() as u16;
        let skyline_x = terminal_width.saturating_sub(skyline_width + 2);
        let skyline_y = horizon_y.saturating_sub(SKYLINE.len() as u16 - 1);
        for (row, line) in SKYLINE.iter().enumerate() {
            let y = skyline_y + row as u16;
            for (col, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    let x = skyline_x + col as u16;
                    if x < terminal_width {
                        let color = if row == 1 || row == 2 {
                            Color::DarkGrey
                        } else {
                            Color::Grey
                        };
                        renderer.render_char(x, y, ch, color)?;
                    }
                }
            }
        }

        for (row, line) in sprite.iter().enumerate() {
            let y = y_start + row as u16;
            for (col, ch) in line.chars().enumerate() {
                let x = self.x + col as i16;
                if x >= 0 && x < terminal_width as i16 && ch != ' ' {
                    renderer.render_char(x as u16, y, ch, Color::DarkGreen)?;
                }
            }
        }

        if self.breath_timer > 0 {
            let mouth_x = self.x + 4;
            let mouth_y = y_start + 3;
            for i in 0..12 {
                let x = mouth_x + i;
                if x >= 0 && x < terminal_width as i16 {
                    let ch = if i % 3 == 0 { '~' } else { '-' };
                    renderer.render_char(x as u16, mouth_y, ch, Color::Cyan)?;
                    if i % 4 == 0 && mouth_y > 0 {
                        renderer.render_char(x as u16, mouth_y - 1, '*', Color::White)?;
                    }
                }
            }
        }

        if self.stomp_timer > 0 {
            let stomp_x = (self.x + 6).clamp(0, terminal_width as i16 - 1) as u16;
            let y = horizon_y.saturating_sub(1);
            if y < horizon_y {
                renderer.render_char(stomp_x, y, '#', Color::DarkYellow)?;
                if stomp_x + 1 < terminal_width {
                    renderer.render_char(stomp_x + 1, y, '#', Color::DarkYellow)?;
                }
            }
        }
        Ok(())
    }
}
