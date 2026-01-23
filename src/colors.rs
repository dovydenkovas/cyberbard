//   Cyberbard music player for board role-playing games.
//   Copyright (C) 2025  Aleksandr Dovydenkov <asd@altlinux.org>
//
//   This program is free software: you can redistribute it and/or modify
//   it under the terms of the GNU General Public License as published by
//   the Free Software Foundation, either version 3 of the License, or
//   (at your option) any later version.
//
//   This program is distributed in the hope that it will be useful,
//   but WITHOUT ANY WARRANTY; without even the implied warranty of
//   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//   GNU General Public License for more details.
//
//   You should have received a copy of the GNU General Public License
//   along with this program.  If not, see <https://www.gnu.org/licenses/>.

use egui::ecolor::rgb_from_hsv;
use rand::Rng;

static mut IS_DARK: bool = false;
static mut N_PREV_GENERATINS: u32 = 1;
static mut PREV_COLOR: f32 = 0.0;

pub fn set_light() {
    unsafe {
        IS_DARK = false;
    }
}

pub fn set_dark() {
    unsafe {
        IS_DARK = true;
    }
}

/// Create next preudorandom color.
fn next() -> f32 {
    // First color is true random
    unsafe {
        if PREV_COLOR == 0.0 {
            PREV_COLOR = rand::rng().random();
            N_PREV_GENERATINS = 1;
            return PREV_COLOR;
        }
    }

    // Double step to skip existing colors
    let n = (unsafe { N_PREV_GENERATINS + 1 } as f32).log2();
    let k = if n - (n as i32 as f32) < 4.0 * f32::EPSILON {
        0.5
    } else {
        1.0
    };
    let n = n as u32;

    // Next color = prev + step
    // where step is half of range between two existed colors.
    let mut color = unsafe { PREV_COLOR } + k / 2_u32.pow(n) as f32;

    // Round colors in [0; 1] range.
    if color >= 1.0 {
        color -= 1.0;
    }
    unsafe {
        PREV_COLOR = color;
        N_PREV_GENERATINS += 1;
    }
    color
}

pub fn rand_dark_background() -> String {
    let h: f32 = next();
    let s = 0.7;
    let v = 0.3;
    let bytes = rgb_from_hsv((h, s, v));
    format!(
        "#{:02x}{:02x}{:02x}",
        (255.0 * bytes[0]) as u8,
        (255.0 * bytes[1]) as u8,
        (255.0 * bytes[2]) as u8,
    )
}

pub fn rand_light_background() -> String {
    let h: f32 = next();
    let s = 0.3;
    let v = 0.9;
    let bytes = rgb_from_hsv((h, s, v));
    format!(
        "#{:02x}{:02x}{:02x}",
        (255.0 * bytes[0]) as u8,
        (255.0 * bytes[1]) as u8,
        (255.0 * bytes[2]) as u8,
    )
}

pub fn rand_color() -> String {
    if unsafe { IS_DARK } {
        rand_dark_background()
    } else {
        rand_light_background()
    }
}
