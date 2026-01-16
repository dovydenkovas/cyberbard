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

use std::path::PathBuf;

use crate::storage::tag::Tag;

use super::source::Source;

pub enum StorageCredentials {
    Local { path: PathBuf },
}

/// Storage trait describe interface to audio sources manipulation.
#[typetag::serde(tag = "type")]
pub trait Storage: erased_serde::Serialize {
    fn get_caption(&self) -> String;
    fn set_caption(&mut self, new_caption: String);
    fn load_sources(&mut self);
    fn setup_storage(&mut self, cred: StorageCredentials);
    fn get(&self, index: usize) -> Option<Box<dyn Source>>;
    fn get_tags(&self, index: usize) -> Vec<&Tag>;
    fn all_tags(&self, index: usize) -> Vec<(Tag, bool)>;
    fn len(&self) -> usize;
    fn attach_tag(&mut self, index: usize, tag: String);
    fn unattach_tag(&mut self, index: usize, tag: String);
    fn rename_tag(&mut self, old_name: String, new_name: String);
    fn remove_tag(&mut self, name: String);
    fn add_tag(&mut self);
    fn set_tag_color(&mut self, tag: String, color: String);
    fn find(&self, substr: String) -> Vec<usize>;
}
