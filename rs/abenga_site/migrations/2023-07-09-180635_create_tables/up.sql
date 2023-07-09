CREATE SCHEMA IF NOT EXISTS core;
CREATE SCHEMA IF NOT EXISTS data;

CREATE TABLE core.people (
    id SERIAL NOT NULL,
    uid UUID NOT NULL,
    username VARCHAR,
    email VARCHAR NOT NULL,
    primary_phone_number VARCHAR(32),
    login_type VARCHAR(255),
    password VARCHAR,
    oauth_provider VARCHAR,
    oauth_token VARCHAR,
    first_name VARCHAR,
    last_name VARCHAR,
    other_names VARCHAR,
    date_added TIMESTAMP WITHOUT TIME ZONE,
    contact_email VARCHAR,
    other_phone_numbers JSONB,
    postal_address JSONB,
    physical_address JSONB,
    active BOOLEAN,
    PRIMARY KEY (id),
    UNIQUE (uid),
    UNIQUE (username),
    UNIQUE (email)
);

CREATE TABLE core.login_sessions (
    id SERIAL NOT NULL,
    person_id INTEGER,
    session_id VARCHAR(128) NOT NULL,
    time_started TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    last_action_time TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    ended BOOLEAN DEFAULT 'f',
    time_ended TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY(person_id) REFERENCES core.people (id),
    UNIQUE (session_id)
);

CREATE TABLE data.authors (
    id INTEGER NOT NULL,
    bio_md VARCHAR,
    bio_html VARCHAR,
    PRIMARY KEY (id),
    FOREIGN KEY(id) REFERENCES core.people (id)
);

CREATE TABLE data.post_series (
    id SERIAL NOT NULL,
    uid VARCHAR NOT NULL,
    author_id INTEGER,
    title VARCHAR NOT NULL,
    joined_title VARCHAR NOT NULL,
    date_added TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    last_edited TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    abstract_md VARCHAR NOT NULL,
    abstract_html VARCHAR NOT NULL,
    cover_image_path VARCHAR,
    tags TEXT[],
    PRIMARY KEY (id),
    UNIQUE (uid),
    FOREIGN KEY(author_id) REFERENCES data.authors (id),
    UNIQUE (joined_title)
);

CREATE TABLE data.posts (
    id SERIAL NOT NULL,
    uid VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    joined_title VARCHAR NOT NULL,
    date_added TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    last_edited TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    year_added INTEGER NOT NULL,
    month_added INTEGER NOT NULL,
    day_added INTEGER NOT NULL,
    author_id INTEGER,
    abstract_md VARCHAR NOT NULL,
    abstract_html VARCHAR NOT NULL,
    body_md VARCHAR NOT NULL,
    body_html VARCHAR NOT NULL,
    series_id INTEGER,
    position_in_series INTEGER,
    references_md VARCHAR NOT NULL,
    references_html VARCHAR NOT NULL,
    cover_image_path VARCHAR,
    tags TEXT[],
    PRIMARY KEY (id),
    UNIQUE (uid),
    UNIQUE (joined_title),
    FOREIGN KEY(author_id) REFERENCES data.authors (id),
    FOREIGN KEY(series_id) REFERENCES data.post_series (id)
);
