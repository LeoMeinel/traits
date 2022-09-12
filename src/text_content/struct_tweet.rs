/*
 * File: struct_tweet.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

#[allow(dead_code)]
pub(crate) struct Tweet {
    pub(crate) username: String,
    pub(crate) content: String,
    pub(crate) reply: bool,
    pub(crate) retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub(crate) trait Summary {
    fn summarize(&self) -> String;
}
