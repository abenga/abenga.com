

#[derive(serde::Serialize)]
pub struct Person {
    id: i64,
    first_name: &'static str,
    last_name: &'static str,
    other_names: &'static str,
    email: &'static str,
}


#[derive(serde::Serialize)]
pub struct Author {
    id: i64,
    bio_md: &'static str,
    bio_html: &'static str,
}


#[derive(serde::Serialize)]
pub struct LoginSession {
    id: i64,
    person_id: i64,
    session_id: &'static str,
    time_started: Option<&'static str>,
    time_ended: Option<&'static str>,
    ended: bool
}


#[derive(serde::Serialize)]
pub struct Post {
    id: i64,
    title: &'static str,
    joined_title: &'static str,
    abstract_md: &'static str,
    abstract_html: &'static str,
    body_md: &'static str,
    body_html: &'static str,
    references_md: Option<&'static str>,
    references_html: Option<&'static str>,
    series_id: Option<i64>,
    position_in_series: Option<u8>,
}


#[derive(serde::Serialize)]
pub struct PostSeries {
    id: i64,
    author_id: i64,
    title: &'static str,
    joined_title: &'static str,
    abstract_md: &'static str,
    abstract_html: &'static str,
    date_added: &'static str,
    last_edited: &'static str,
    tags: Vec<i64>,
    cover_image_path: &'static str,
    // n_posts: u8,
    // posts: HashMap<i64, Post>,
}