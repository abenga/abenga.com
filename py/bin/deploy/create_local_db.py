import argparse
import logging
import os
import pathlib
import uuid

from sqlalchemy.orm import Session
from sqlalchemy.sql import text

from abenga_site.py.bin.deploy import fetch_db_to_local

from abenga_site.py.lib.models import base as base_db_config
from abenga_site.py.lib.models import core as core_models
from abenga_site.py.lib.models import data as data_models
from abenga_site.py.lib.utils import configuration
from abenga_site.py.lib.utils import database as db_utils


author_keys = ["id", "bio_md", "bio_html"]

person_keys = [
    "id",
    "uid",
    "email",
    "login_type",
    "oauth_provider",
    "first_name",
    "other_names",
    "contact_email",
    "postal_address",
    "active",
    "primary_phone_number",
    "username",
    "password",
    "oauth_token",
    "last_name",
    "date_added",
    "other_phone_numbers",
    "physical_address",
]

post_series_keys = [
    "uid",
    "author_id",
    "joined_title",
    "last_edited",
    "abstract_html",
    "tags",
    "title",
    "id",
    "date_added",
    "abstract_md",
    "cover_image_path",
]

post_keys = [
    "month_added",
    "position_in_series",
    "uid",
    "day_added",
    "references_md",
    "id",
    "author_id",
    "references_html",
    "title",
    "abstract_md",
    "cover_image_path",
    "joined_title",
    "abstract_html",
    "tags",
    "date_added",
    "body_md",
    "last_edited",
    "body_html",
    "year_added",
    "series_id",
]


def create_local_db(delete):
    engine = db_utils.get_database_engine("local")
    with engine.begin() as conn:
        for schema in base_db_config.schemas:
            if delete:
                conn.execute(text(f"DROP SCHEMA IF EXISTS {schema} CASCADE;"))
            conn.execute(text(f"CREATE SCHEMA IF NOT EXISTS {schema};"))

    with engine.begin() as conn:
        base_db_config.Base.metadata.create_all(conn.engine)
        conn.commit()


def main(args):
    if args.create_local:
        create_local_db(args.delete)

    people, authors, post_series, posts = fetch_db_to_local.fetch_remote_data()
    engine = db_utils.get_database_engine("local")
    with Session(engine) as session:
        for person in people:
            local_person = core_models.Person(
                **{k: getattr(person, k) for k in person_keys}
            )
            session.add(local_person)
            session.commit()

        for a in authors:
            local_author = data_models.Author(**{k: getattr(a, k) for k in author_keys})
            session.add(local_author)
            session.commit()

        for ps in post_series:
            local_ps = data_models.PostSeries(
                **{k: getattr(ps, k) for k in post_series_keys}
            )
            session.add(local_ps)
            session.commit()

        for p in posts:
            post = data_models.Post(**{k: getattr(p, k) for k in post_keys})
            session.add(post)
            session.commit()


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--delete", action="store_true", help="Create local db")
    parser.add_argument("--create_local", action="store_true", help="Create local db")
    _args = parser.parse_args()
    main(_args)
