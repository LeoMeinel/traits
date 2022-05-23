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

use std::fmt::{Debug, Display};

use crate::text_content::struct_news_article::{NewsArticle, Summary};
use crate::text_content::struct_news_article_tweet::{NewsArticleO, SummaryO, TweetO};
use crate::text_content::struct_tweet::{Summary as OtherSummary, Tweet};

mod struct_news_article;
mod struct_news_article_tweet;
mod struct_tweet;

pub(crate) fn text_content() {
    summarize_simple();
    summarize_default();
    call_returns_any_summarizable();
}
// `if` and `else` have incompatible types
// return impl can only return one type!
/*
fn returns_any_summarizable_(switch: bool) -> impl SummaryO {
    if switch {
        NewsArticleO {
            headline: String::from(""),
            author: String::from(""),
            content: String::from(""),
        }
    } else {
        TweetO {
        username: String::from("horse_ebooks"),
        content: String::from("Of course, as you probably already know, people"),
        reply: false,
        retweet: false,
        }
    }
}
 */

fn call_returns_any_summarizable() {
    println!("{:#?}", returns_any_summarizable().summarize());
}
// Return any type that impl SummaryO

fn returns_any_summarizable() -> impl SummaryO {
    TweetO {
        username: String::from("horse_ebooks"),
        content: String::from("Of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// Same as below but more readable
#[allow(dead_code)]
fn some_function_2<T, U>(_t: &T, _u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
{
    2
}

#[allow(dead_code)]
fn some_function_1<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) -> i32 {
    1
}
// Like this you can make sure type is the same
// You can also make sure that multiple traits are respected

fn notify_o2<T: SummaryO + Display>(item1: &T, item2: &T) {
    println!("Breaking News!\n{}", item1.summarize());
    println!("Breaking News!\n{}", item2.summarize());
}
// Not like this

fn notify_o1(item1: &impl SummaryO, item2: &impl SummaryO) {
    println!("Breaking News!\n{}", item1.summarize());
    println!("Breaking News!\n{}", item2.summarize());
}
// Same, but for more complicated cases

fn notify_o<T: SummaryO>(item: &T) {
    println!("Breaking News!\n{}", item.summarize());
}
// As this (only for simple cases)

fn notify(item: &impl SummaryO) {
    println!("Breaking News!\n{}", item.summarize());
}

fn summarize_default() {
    let news_article = NewsArticleO {
        author: String::from("John Doe"),
        headline: String::from("The Sky is falling"),
        content: String::from("The Sky is not actually falling"),
    };
    let news_article_1 = NewsArticleO {
        author: String::from("Peter Right"),
        headline: String::from("Peter is always right"),
        content: String::from("Peter isn't actually right at all"),
    };
    let tweet = TweetO {
        username: String::from("@john"),
        content: String::from("Hello World!"),
        reply: false,
        retweet: false,
    };
    notify(&news_article);
    notify_o(&tweet);
    notify_o1(&news_article, &tweet);
    // ERROR: expected `&NewsArticleO`, found `&TweetO`
    //notify_o2(&news_article, &tweet);
    notify_o2(&news_article, &news_article_1);
}

fn summarize_simple() {
    let news_article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("The Sky is falling"),
        content: String::from("The Sky is not actually falling"),
    };
    let tweet = Tweet {
        username: String::from("@john"),
        content: String::from("Hello World!"),
        reply: false,
        retweet: false,
    };
    println!("Article summary: {}", news_article.summarize());
    println!("Tweet summary: {}", tweet.summarize());
}
