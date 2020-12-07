
use chrono::{DateTime, Utc};
// use std::collections::HashMap;


// #[derive(serde::Serialize)]
pub struct Person {
    id: i64,
    first_name: &'static str,
    last_name: &'static str,
    other_names: &'static str,
    email: &'static str,
}


// #[derive(serde::Serialize)]
pub struct Author {
    id: i64,
    bio_md: &'static str,
    bio_html: &'static str,
}


// #[derive(serde::Serialize)]
pub struct LoginSession {
    id: i64,
    person_id: i64,
    session_id: &'static str,
    time_started: Utc,
    time_ended: Some(Utc),
    ended: bool
}


// #[derive(serde::Serialize)]
pub struct Post {
    id: i64,
    title: &'static str,
    joined_title: &'static str,
    abstract_md: &'static str,
    abstract_html: &'static str,
    body_md: &'static str,
    body_html: &'static str,
    references_md: Some(&'static str),
    references_html: Some(&'static str),
    series_id: Some(i64),
    position_in_series: Some(u8),
}


// #[derive(serde::Serialize)]
pub struct PostSeries {
    id: i64,
    author_id: i64,
    title: &'static str,
    joined_title: &'static str,
    abstract_md: &'static str,
    abstract_html: &'static str,
    date_added: Utc,
    last_edited: Utc,
    tags: Vec<i64>,
    cover_image_path: &'static str,
    // n_posts: u8,
    // posts: HashMap<i64, Post>,
}