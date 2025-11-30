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

use super::source::Source;

pub enum StorageCredentials {
    Local { path: PathBuf },
}

/// Storage trait describe interface to audio sources manipulation.
pub trait Storage {
    fn get_caption(&self) -> String;
    fn set_caption(&mut self, new_caption: String);
    fn load_sources(&mut self);
    fn setup_storage(&mut self, cred: StorageCredentials);
    fn get(&self, index: usize) -> Option<Box<dyn Source>>;
    fn len(&self) -> usize;
    fn attach_tag(&mut self, index: usize, title: String);
    fn unattach_tag(&mut self, index: usize, title: String);
    fn find_by_tag(&self, substr: String) -> Vec<usize>;
    fn find_by_title(&self, substr: String) -> Vec<usize>;
}
