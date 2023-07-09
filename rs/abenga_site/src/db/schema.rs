// @generated automatically by Diesel CLI.

pub mod data {
    diesel::table! {
        data.authors (id) {
            id -> Int4,
            bio_md -> Nullable<Varchar>,
            bio_html -> Nullable<Varchar>,
        }
    }

    diesel::table! {
        data.post_series (id) {
            id -> Int4,
            uid -> Varchar,
            author_id -> Nullable<Int4>,
            title -> Varchar,
            joined_title -> Varchar,
            date_added -> Timestamp,
            last_edited -> Timestamp,
            abstract_md -> Varchar,
            abstract_html -> Varchar,
            cover_image_path -> Nullable<Varchar>,
            tags -> Nullable<Array<Nullable<Text>>>,
        }
    }

    diesel::table! {
        data.posts (id) {
            id -> Int4,
            uid -> Varchar,
            title -> Varchar,
            joined_title -> Varchar,
            date_added -> Timestamp,
            last_edited -> Timestamp,
            year_added -> Int4,
            month_added -> Int4,
            day_added -> Int4,
            author_id -> Nullable<Int4>,
            abstract_md -> Varchar,
            abstract_html -> Varchar,
            body_md -> Varchar,
            body_html -> Varchar,
            series_id -> Nullable<Int4>,
            position_in_series -> Nullable<Int4>,
            references_md -> Varchar,
            references_html -> Varchar,
            cover_image_path -> Nullable<Varchar>,
            tags -> Nullable<Array<Nullable<Text>>>,
        }
    }

    diesel::joinable!(post_series -> authors (author_id));
    diesel::joinable!(posts -> authors (author_id));
    diesel::joinable!(posts -> post_series (series_id));

    diesel::allow_tables_to_appear_in_same_query!(
        authors,
        post_series,
        posts,
    );
}
