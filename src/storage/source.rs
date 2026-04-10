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

use crate::{storage::localstorage::LocalOpener, stream::Stream};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Source {
    filename: String,
    title: String,
    tags: Vec<usize>,
}

impl Source {
    pub fn new(filename: String, title: String) -> Source {
        Source {
            filename,
            title,
            tags: Vec::new(),
        }
    }

    pub fn get_stream(&self) -> Result<Stream, Box<dyn std::error::Error>> {
        Stream::from_source(Box::new(LocalOpener::new(self.filename.clone())), 100.0)
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn attach_tag(&mut self, tag_index: usize) {
        if let Err(pos) = self.tags.binary_search(&tag_index) {
            self.tags.insert(pos, tag_index);
        }
    }

    pub fn unattach_tag(&mut self, tag_index: usize) {
        if let Ok(pos) = self.tags.binary_search(&tag_index) {
            self.tags.remove(pos);
        };
    }

    /// Remove tag from tag list and decrement all tags greater than removed.
    pub fn remove_tag_and_shift_indexes(&mut self, tag_index: usize) {
        self.unattach_tag(tag_index);
        for tag in self.tags.iter_mut() {
            if *tag > tag_index {
                *tag -= 1;
            }
        }
    }

    pub fn tags(&self) -> Vec<usize> {
        self.tags.clone()
    }

    pub fn has_tag(&self, tag_index: usize) -> bool {
        self.tags.binary_search(&tag_index).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_empty_stream() {
        let title = "Test track";
        let source = Source::new("/tmp/test.mp3".into(), title.into());

        assert_eq!(title, source.get_title());
        assert!(source.get_stream().is_err());
    }

    #[test]
    fn source_create() {
        let title = "Test track";
        let mut source = Source::new("/tmp/test.mp3".into(), title.into());

        for i in 0..11 {
            source.attach_tag(i);
        }

        assert!(source.has_tag(0));
        assert!(source.has_tag(1));
        assert!(source.has_tag(6));
        assert!(source.has_tag(3));
        assert!(source.has_tag(10));
        assert!(!source.has_tag(11));

        assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], source.tags());
        source.unattach_tag(0);
        assert!(!source.has_tag(0));
        source.unattach_tag(1);
        assert!(!source.has_tag(1));
        source.unattach_tag(6);
        assert!(!source.has_tag(6));
        source.unattach_tag(13);
        assert!(!source.has_tag(13));
        source.unattach_tag(8);
        assert!(!source.has_tag(8));
        assert_eq!(vec![2, 3, 4, 5, 7, 9, 10], source.tags());

        source.remove_tag_and_shift_indexes(5);
        assert_eq!(vec![2, 3, 4, 6, 8, 9], source.tags());
    }
}
