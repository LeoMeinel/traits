/*
 * traits is a commandline application.
 * Copyright © 2022 Leopold Meinel & contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see https://github.com/TamrielNetwork/traits/blob/main/LICENSE
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
