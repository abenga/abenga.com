
use uuid;


#[derive(Clone, Debug, serde::Serialize)]
pub struct Person {
    id: i64,
    first_name: &'static str,
    last_name: &'static str,
    other_names: &'static str,
    email: &'static str,
}


#[derive(Clone, Debug, serde::Serialize)]
pub struct Author {
    id: i64,
    bio_md: &'static str,
    bio_html: &'static str,
}


#[derive(Clone, Debug, serde::Serialize)]
pub struct LoginSession {
    pub id: i64,
    pub person_id: i64,
    pub session_id: &'static str,
    pub time_started: Option<&'static str>,
    pub time_ended: Option<&'static str>,
    pub ended: bool
}


#[derive(Clone, Debug, serde::Serialize)]
pub struct PostSeries {
    pub id: i64,
    pub uid: &'static str,
    pub author_id: i64,
    pub title: &'static str,
    pub joined_title: &'static str,
    pub abstract_md: &'static str,
    pub abstract_html: &'static str,
    pub date_added: &'static str,
    pub last_edited: &'static str,
    pub tags: Vec<String>,
}


#[derive(Clone, Debug, serde::Serialize)]
pub struct Post {
    pub id: i64,
    pub uid: &'static str,
    pub title: &'static str,
    pub joined_title: &'static str,

    pub date_added: &'static str,
    pub last_edited: &'static str,

    pub year_added: i64,
    pub month_added: i64,
    pub day_added: i64,

    pub author_id: i64,

    pub abstract_md: &'static str,
    pub abstract_html: &'static str,

    pub body_md: &'static str,
    pub body_html: &'static str,

    pub series_id: Option<i64>,
    pub position_in_series: Option<i64>,


    pub references_md: Option<&'static str>,
    pub references_html: Option<&'static str>,

    pub tags: Vec<String>,
}
