/*
 * File: trait_balance.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

use crate::trait_balance::struct_pair::Pair;

mod struct_pair;

pub(crate) fn trait_balance() {
    compare_pair();
}

fn compare_pair() {
    let pair = Pair::new(1, 35);
    pair.cmp_display();
}
