
use rocket_contrib::uuid::Uuid;

use crate::lib::db::models as db_models;
use crate::lib::db::utils as db_utils;


pub fn post_series<'a>() -> Vec<db_models::PostSeries> {
    let mut v = Vec::new();
    let conn = db_utils::get_db_connection().expect("Could not connect to database!");
    for row in &conn.query("SELECT id, uid, author_id, title, joined_title, \
    abstract_md, abstract_html, date_added::text, last_edited::text, tags \
    FROM data.post_series;", &[]).unwrap() {
        let _post_series = db_models::PostSeries {
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
        v.push(_post_series.clone());
    }
    return v;
}


pub fn posts() -> Vec<db_models::Post> {
    let mut v = Vec::new();
    let conn = db_utils::get_db_connection().expect("Could not connect to database!");
    for row in &conn.query("SELECT \
        id, uid, title, joined_title, ltrim(to_char(date_added, 'DDth Month YYYY'), '0'), \
        ltrim(to_char(last_edited, 'DDth Month YYYY'), '0'), year_added, month_added, \
        day_added, author_id, abstract_md, abstract_html, body_md, body_html,  series_id, \
        position_in_series, references_md, references_html, tags \
        FROM data.posts \
        ORDER BY date_added DESC;", &[]).unwrap() {
        v.push(db_models::Post {
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
        });
    }
    return v;
}


pub fn get_post(uid: Uuid) -> Option<db_models::Post> {
    let mut v = Vec::new();
    let conn = db_utils::get_db_connection().expect("Could not connect to database!");
    let uid_str = format!("{}", uid);
    for row in &conn.query("SELECT \
        id, uid, title, joined_title, ltrim(to_char(date_added, 'DDth Month YYYY'), '0'), \
        ltrim(to_char(last_edited, 'DDth Month YYYY'), '0'), year_added, month_added, \
        day_added, author_id, abstract_md, abstract_html, body_md, body_html, \
        series_id, position_in_series, references_md, references_html, tags \
        FROM data.posts \
        WHERE uid = $1;", &[&uid_str]).unwrap() {
        v.push(db_models::Post {
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
        });
    }
    if v.len() != 1 {
        None
    } else {
        let post = v.pop();
        post
    }
}


pub fn get_uid_from_ymd_and_title(year_added: i32, month_added: i32, day_added: i32, joined_title: String) -> Option<String> {
    let mut v = Vec::new();
    let conn = db_utils::get_db_connection().expect("Could not connect to database!");
    for row in &conn.query("SELECT \
        uid FROM data.posts \
        WHERE year_added = $1 AND month_added = $2 AND \
              day_added = $3 AND joined_title = $4;",
                           &[&year_added, &month_added, &day_added, &joined_title]).unwrap() {
        v.push(row.get(0))
    }
    if v.len() != 1 {
        None
    } else {
        let uuid_str = v.pop();
        uuid_str
    }
}
