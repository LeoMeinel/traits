/*
 * File: struct_news_article_tweet.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

use std::fmt::{Display, Formatter};

pub(crate) struct NewsArticleO {
    pub(crate) author: String,
    pub(crate) headline: String,
    pub(crate) content: String,
}

impl SummaryO for NewsArticleO {
    fn summarize_author(&self) -> String {
        format!("-> {}", self.author)
    }
}

impl Display for NewsArticleO {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.author, self.headline, self.content)
    }
}

#[allow(dead_code)]
pub(crate) struct TweetO {
    pub(crate) username: String,
    pub(crate) content: String,
    pub(crate) reply: bool,
    pub(crate) retweet: bool,
}

impl SummaryO for TweetO {
    fn summarize_author(&self) -> String {
        format!("-> {}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub(crate) trait SummaryO {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more from: {}", self.summarize_author())
    }
}
