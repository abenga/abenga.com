#!../../env/bin/python

import argparse
import logging
import uuid

from pprint import pprint

from abenga_site.py.lib.models import base as base_db_config
from abenga_site.py.lib.models import core as core_models
from abenga_site.py.lib.models import data as data_models
from abenga_site.py.lib.utils import database as db_utils


def add_uids_to_remote():
    # NOTE: This was a one-time operation to add UUID columns in posts and post series
    with db_utils.get_database_connection('dummy') as remote_conn:
        query = "ALTER TABLE data.post_series ADD COLUMN IF NOT EXISTS uid UUID DEFAULT NULL UNIQUE;"
        remote_conn.execute(query)
        remote_conn.session.commit()

        post_series = remote_conn.session.query(data_models.PostSeries).all()
        for single_post_series in post_series:
            uid = uuid.uuid4()
            update_query = f"UPDATE data.post_series SET uid = '{uid}' " \
                           f"WHERE id = {single_post_series.id};"
            remote_conn.execute(update_query)
            remote_conn.session.commit()

        query = "ALTER TABLE data.posts ADD COLUMN IF NOT EXISTS uid UUID DEFAULT NULL UNIQUE;"
        remote_conn.execute(query)
        remote_conn.session.commit()

        posts = remote_conn.session.query(data_models.Post).all()
        for post in posts:
            uid = uuid.uuid4()
            update_query = f"UPDATE data.posts SET uid = '{uid}' " \
                           f"WHERE id = {post.id};"
            remote_conn.execute(update_query)
            remote_conn.session.commit()


def fetch_remote_data():
    with db_utils.get_database_connection('remote') as remote_conn:
        logging.debug("Fetching people")
        people = remote_conn.session.query(core_models.Person).all()

        logging.debug("Fetching authors")
        authors = remote_conn.session.query(data_models.Author).all()

        logging.debug("Fetching post series")
        post_series = remote_conn.session.query(data_models.PostSeries).all()

        logging.debug("Fetching posts")
        posts = remote_conn.session.query(data_models.Post).all()

        return people, authors, post_series, posts


def create_local_db(delete):
    with db_utils.get_database_connection('local') as conn:
        for schema in base_db_config.schemas:
            if delete:
                conn.execute('DROP SCHEMA IF EXISTS {} CASCADE;'.format(schema))
            conn.execute('CREATE SCHEMA IF NOT EXISTS {};'.format(schema))

        conn.session.commit()

        base_db_config.Base.metadata.create_all(conn.engine)
        conn.session.commit()


def add_local_db_data(people, authors, post_series, posts):
    with db_utils.get_database_connection('local') as local_conn:
        for person in people:
            local_conn.session.add(
                core_models.Person(**{
                    'id': person.id,
                    'uid': person.uid,
                    'username': person.username,
                    'email': person.email,
                    'primary_phone_number': person.primary_phone_number,
                    'login_type': person.login_type,
                    'password': person.password,
                    'oauth_provider': person.oauth_provider,
                    'oauth_token': person.oauth_token,
                    'first_name': person.first_name,
                    'last_name': person.last_name,
                    'other_names': person.other_names,
                    'date_added': person.date_added,
                    'contact_email': person.contact_email,
                    'other_phone_numbers': person.other_phone_numbers,
                    'postal_address': person.postal_address,
                    'physical_address': person.physical_address,
                    'active': person.active,
                })
            )
            local_conn.session.commit()

        for author in authors:
            local_conn.session.add(
                data_models.Author(**{
                    'id': author.id,
                    'bio_md': author.bio_md,
                    'bio_html': author.bio_html,
                })
            )
            local_conn.session.commit()

        for single_post_series in post_series:
            ps = data_models.PostSeries(**{
                'id': single_post_series.id,
                'uid': single_post_series.uid,
                'author_id': single_post_series.author_id,
                'title': single_post_series.title,
                'joined_title': single_post_series.joined_title,
                'date_added': single_post_series.date_added,
                'last_edited': single_post_series.last_edited,
                'abstract_md': single_post_series.abstract_md,
                'abstract_html': single_post_series.abstract_html,
                'cover_image_path': single_post_series.cover_image_path,
                'tags': single_post_series.tags,
            })
            local_conn.session.add(ps)
            local_conn.session.commit()

        for post in posts:
            p = data_models.Post(**{
                'id': post.id,
                'uid': post.uid,
                'title': post.title,
                'joined_title': post.joined_title,
                'date_added': post.date_added,
                'last_edited': post.last_edited,
                'year_added': post.year_added,
                'month_added': post.month_added,
                'day_added': post.day_added,
                'author_id': post.author_id,
                'abstract_md': post.abstract_md,
                'abstract_html': post.abstract_html,
                'body_md': post.body_md,
                'body_html': post.body_html,
                'series_id': post.series_id,
                'position_in_series': post.position_in_series,
                'references_md': post.references_md,
                'references_html': post.references_html,
                'cover_image_path': post.cover_image_path,
                'tags': post.tags,
            })
            local_conn.session.add(p)
            local_conn.session.commit()


def main(args):
    people, authors, post_series, posts = fetch_remote_data()
    if args.create_local:
        create_local_db(args.delete)
        add_local_db_data(people, authors, post_series, posts)


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument("--create_local", action="store_true", help="Create local db")
    parser.add_argument("--delete", action="store_true", help="Delete and recreate DB")
    args = parser.parse_args()
    main(args)
