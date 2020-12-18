#!../../env/bin/python

import argparse

from pprint import pprint

from abenga_site.py.lib.models import core as core_models
from abenga_site.py.lib.models import data as data_models
from abenga_site.py.lib.utils import database as db_utils


def create_local_db():
    pass


def fetch_remote_data():
    with db_utils.get_database_connection('remote') as remote_conn:
        print("Fetching people")
        people = [p.__dict__ for p in remote_conn.session.query(core_models.Person).all()]

        print("Fetching authors")
        authors = [a.__dict__ for a in remote_conn.session.query(data_models.Author).all()]

        print("Fetching post series")
        post_series = [s.__dict__ for s in remote_conn.session.query(data_models.PostSeries).all()]

        print("Fetching posts")
        posts = [p.__dict__ for p in remote_conn.session.query(data_models.Post).all()]

        return people, authors, post_series, posts


def main(args):
    people, authors, post_series, posts = fetch_remote_data()
    print(people)


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument("--delete", action="store_true", help="Delete and recreate DB")
    args = parser.parse_args()
    main(args)
