/*
 * File: struct_pair.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

use std::fmt::Display;

pub(crate) struct Pair<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T> Pair<T> {
    pub(crate) fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
// Only allow cmp_display for traits: Display + PartialOrd

impl<T: Display + PartialOrd> Pair<T> {
    pub(crate) fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x)
        } else {
            println!("The largest member is y = {}", self.y)
        }
    }
}
