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
 * along with this program. If not, see https://github.com/LeoMeinel/traits/blob/main/LICENSE
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
