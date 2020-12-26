
use crate::lib::db::models as db_models;
use crate::lib::db::utils as db_utils;


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
