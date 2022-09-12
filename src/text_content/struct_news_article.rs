/*
 * File: struct_news_article.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

#[allow(dead_code)]
pub(crate) struct NewsArticle {
    pub(crate) author: String,
    pub(crate) headline: String,
    pub(crate) content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

pub(crate) trait Summary {
    fn summarize(&self) -> String;
}
