use diesel::prelude::*;
use chrono::NaiveDateTime;


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::data::authors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Author {
    pub id: i32,
    pub bio_md: Option<String>,
    pub bio_html: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::data::post_series)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostSeries {
    pub id: i32,
    pub uid: String,
    pub author_id: Option<i32>,
    pub title: String,
    pub joined_title: String,
    pub date_added: NaiveDateTime,
    pub last_edited: NaiveDateTime,
    pub abstract_md: String,
    pub abstract_html: String,
    pub cover_image_path: Option<String>,
    pub tags: Option<Vec<Option<String>>>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::data::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub uid: String,
    pub title: String,
    pub joined_title: String,
    pub date_added: NaiveDateTime,
    pub last_edited: NaiveDateTime,
    pub year_added: i32,
    pub month_added: i32,
    pub day_added: i32,
    pub author_id: Option<i32>,
    pub abstract_md: String,
    pub abstract_html: String,
    pub body_md: String,
    pub body_html: String,
    pub series_id: Option<i32>,
    pub position_in_series: Option<i32>,
    pub references_md: String,
    pub references_html: String,
    pub cover_image_path: Option<String>,
    pub tags: Option<Vec<Option<String>>>,
}
