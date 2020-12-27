
use crate::lib::db::models as db_models;
use crate::lib::db::utils as db_utils;


pub fn post_series() -> Vec<db_models::PostSeries> {
    let mut v = Vec::new();
    let mut conn = db_utils::get_connection().expect("Failure of database connection.");
    for row in conn.query("SELECT id, title, joined_title, abstract_md, \
                              abstract_html, body_md, body_html, references_md, \
                              references_html, series_id, position_in_series FROM data.post_seriess;", &[]).unwrap() {
        let post_series = db_models::PostSeries {
            id: row.get(0),
            author_id: row.get(1),
            title: row.get(2),
            joined_title: row.get(3),
            abstract_md: row.get(4),
            abstract_html: row.get(5),
            date_added: row.get(6),
            last_edited: row.get(7),
            tags: row.get(8),
            cover_image_path: row.get(8),
        };
        v.push(post.clone());
    }
    return v;
}


pub fn posts() -> Vec<db_models::Post> {
    let mut v = Vec::new();
    let mut conn = db_utils::get_connection().expect("Failure of database connection.");
    for row in conn.query("SELECT id, title, joined_title, abstract_md, \
                              abstract_html, body_md, body_html, references_md, \
                              references_html, series_id, position_in_series FROM data.posts;", &[]).unwrap() {
        let post = db_models::Post {
            id: row.get(0),
            title: row.get(1),
            joined_title: row.get(2),
            abstract_md: row.get(3),
            abstract_html: row.get(4),
            body_md: row.get(5),
            body_html: row.get(6),
            references_md: row.get(7),
            references_html: row.get(8),
            series_id: row.get(9),
            position_in_series: row.get(10),
        };
        v.push(post.clone());
    }
    return v;
}
