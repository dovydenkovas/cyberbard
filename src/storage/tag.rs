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
//   along with this program.  If not, see <https://www.gnu.org/licenses/>

use rand::TryRngCore;
use serde::{Deserialize, Serialize};

/// Tag structures. Used to Sources in Storage classification.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tag {
    text: String,
    color: String,
}

fn rand_color() -> String {
    // TODO: Generate beautiful random colors
    let mut bytes = [0u8; 3];
    let _ = rand::rngs::OsRng.try_fill_bytes(&mut bytes);
    format!(
        "#{:02x}{:02x}{:02x}",
        bytes[0] / 4,
        bytes[1] / 4,
        bytes[2] / 4
    )
}

impl Tag {
    pub fn new(text: String) -> Tag {
        let mut tag = Tag {
            text: String::new(),
            color: String::new(),
        };
        tag.set_text(text);
        tag.set_color(rand_color());
        tag
    }

    pub fn random() -> Tag {
        let txt = rand_color();
        Tag {
            text: txt.clone(),
            color: txt.clone(),
        }
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    /// unicode string 30 chars max.
    pub fn set_text(&mut self, text: String) {
        self.text = text.chars().take(30).collect()
    }

    pub fn get_color(&self) -> String {
        self.color.clone()
    }

    /// Color has format #rrbbgg.
    pub fn set_color(&mut self, color: String) {
        if self.is_correct_color(&color) {
            self.color = color;
        } else if self.is_correct_color(&self.color) {
            // Skip is correct color
        } else {
            self.color = "#092032".to_string()
        }
    }

    fn is_correct_color(&self, color: &String) -> bool {
        if color.len() != 7 || color.chars().next().unwrap() != '#' {
            return false;
        }

        for ch in color.chars().skip(1) {
            if !ch.is_ascii_hexdigit() {
                return false;
            }
        }
        return true;
    }
}

pub struct SourceTagLink {
    source: String,
    tag: String,
}

impl SourceTagLink {
    pub fn new(source: String, tag: String) -> SourceTagLink {
        SourceTagLink { source, tag }
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone()
    }

    pub fn get_source(&self) -> String {
        self.source.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_text() {
        let mut tag = Tag::new("some text".to_string());
        assert_eq!("some text", tag.get_text());

        // 30 chars max
        tag.set_text("1234567890123456789012345678901234567890".to_string());
        assert_eq!("123456789012345678901234567890", tag.get_text());

        // utf8 30 chars max
        tag.set_text("абвгд ёЁ 123 __ 0 jlkdsjg".to_string());
        assert_eq!("абвгд ёЁ 123 __ 0 jlkdsjg", tag.get_text());
    }

    #[test]
    fn tag_color() {
        // default color
        let mut tag = Tag::new("some text".to_string());

        // correct color
        tag.set_color("#fa0055".to_string());
        assert_eq!("#fa0055", tag.get_color());

        // wrong color
        tag.set_color("абвгд ёЁ 123 __ 0 jlkdsjg".to_string());
        assert_eq!("#fa0055", tag.get_color());

        // wrong color
        tag.set_color("aa0035".to_string());
        assert_eq!("#fa0055", tag.get_color());

        // wrong color
        tag.set_color("#gg0035".to_string());
        assert_eq!("#fa0055", tag.get_color());

        // correct color
        tag.set_color("#abcdef".to_string());
        assert_eq!("#abcdef", tag.get_color());
    }
}
