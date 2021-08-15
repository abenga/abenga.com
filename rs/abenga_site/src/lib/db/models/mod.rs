
#[derive(Clone, Debug, serde::Serialize)]
pub struct Person {
    id: i32,
    first_name: String,
    last_name: String,
    other_names: String,
    email: String,
}


#[derive(Clone, Debug, serde::Serialize)]
pub struct Author {
    id: i32,
    bio_md: String,
    bio_html: String,
}


#[derive(Clone, Debug, serde::Serialize)]
pub struct LoginSession {
    pub id: i32,
    pub person_id: i32,
    pub session_id: String,
    pub time_started: Option<String>,
    pub time_ended: Option<String>,
    pub ended: bool
}


#[derive(Clone, Debug, serde::Serialize)]
pub struct PostSeries {
    pub id: i32,
    pub uid: String,
    pub author_id: i32,
    pub title: String,
    pub joined_title: String,
    pub abstract_md: String,
    pub abstract_html: String,
    pub date_added: String,
    pub last_edited: String,
    pub tags: Vec<String>,
}


#[derive(Clone, Debug, serde::Serialize)]
pub struct Post {
    pub id: i32,
    pub uid: String,
    pub title: String,
    pub joined_title: String,

    pub date_added: String,
    pub last_edited: String,

    pub year_added: i32,
    pub month_added: i32,
    pub day_added: i32,

    pub author_id: i32,

    pub abstract_md: String,
    pub abstract_html: String,

    pub body_md: String,
    pub body_html: String,

    pub series_id: Option<i32>,
    pub position_in_series: Option<i32>,


    pub references_md: Option<String>,
    pub references_html: Option<String>,

    pub tags: Vec<String>,
}
