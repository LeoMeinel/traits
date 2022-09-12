/*
 * File: main.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

use crate::text_content::text_content;
use crate::trait_balance::trait_balance;

mod blinket_implementations;
mod text_content;
mod trait_balance;

fn main() {
    text_content();
    trait_balance();
}
