
use crate::lib::db::models as db_models;
use crate::lib::db::utils as db_utils;


pub fn post_series() -> Vec<db_models::PostSeries> {
    let mut v = Vec::new();
    let mut conn = db_utils::get_connection().expect("Failure of database connection.");
    for row in conn.query("SELECT id, uid, author_id, title, joined_title, \
    abstract_md, abstract_html, date_added, last_edited, tags \
    FROM data.post_series;", &[]).unwrap() {
        let post_series = db_models::PostSeries {
            id: row.get(0),
            uid: row.get(1),
            author_id: row.get(2),
            title: row.get(3),
            joined_title: row.get(4),
            abstract_md: row.get(5),
            abstract_html: row.get(6),
            date_added: row.get(7),
            last_edited: row.get(8),
            tags: row.get(9),
        };
        v.push(post_series.clone());
    }
    return v;
}


pub fn posts() -> Vec<db_models::Post> {
    let mut v = Vec::new();
    let mut conn = db_utils::get_connection().expect("Failure of database connection.");
    for row in conn.query("SELECT \
        id, uid, title, joined_title, date_added, last_edited, year_added, month_added, \
        day_added, author_id, abstract_md, abstract_html, body_md, body_html, series_id, \
        position_in_series, references_md, references_html, tags \
        FROM data.posts;", &[]).unwrap() {
        let post = db_models::Post {
            id: row.get(0),
            uid: row.get(1),
            title: row.get(2),
            joined_title: row.get(3),
            date_added: row.get(4),
            last_edited: row.get(5),
            year_added: row.get(6),
            month_added: row.get(7),
            day_added: row.get(8),
            author_id: row.get(9),
            abstract_md: row.get(10),
            abstract_html: row.get(11),
            body_md: row.get(12),
            body_html: row.get(13),
            series_id: row.get(14),
            position_in_series: row.get(15),
            references_md: row.get(16),
            references_html: row.get(17),
            tags: row.get(18),
        };
        v.push(post.clone());
    }
    return v;
}
