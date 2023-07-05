import argparse
import logging
import os
import pathlib
import uuid

from abenga_site.py.lib.models import base as base_db_config
from abenga_site.py.lib.models import core as core_models
from abenga_site.py.lib.models import data as data_models
from abenga_site.py.lib.utils import configuration
from abenga_site.py.lib.utils import database as db_utils


SERIES_PAGE = """{{% extends "base.html" %}}

{{% block contents %}}
<div class="row">
    <div class="col-md-2 d-md-block d-sm-none"></div>
    <div class="col-md-8">
        <h1>{title}</h1>
        {abstract}
        <hr>
        {posts}
    </div>
    <div class="col-md-2 d-md-block d-sm-none"></div>
</div>
{{% endblock %}}
"""

POST_SERIES_ENTRY = """<div class="card">
    <div class="card-body">
        <h5 class="card-title">{post_title}</h5>
        {post_abstract}
        <a href="/post/{post_uid}" class="btn btn-sm">view</a>
    </div>
</div>
"""

POST_PAGE = """{{% extends "base.html" %}}
{{% block contents %}}
<div class="row">
<div class="col-md-1 d-md-block d-sm-none"></div>
<div class="col-md-10">

<h1>{post_title}</h1>

<p class="text-muted text-right"><em>{date_published}</em></p>

<blockquote>{post_abstract}</blockquote>

{post_body}

<hr>

{post_references}

</div>
<div class="col-md-1 d-md-block d-sm-none"></div>
</div>
{{% endblock %}}
"""


def write_post_series_page(series_page_path, series_title, abstract_html, posts):
    with open(series_page_path, "wt") as f:
        posts_list_html = ""
        for post in posts:
            posts_list_html += POST_SERIES_ENTRY.format(
                post_title=post.title,
                post_abstract=post.abstract_html,
                post_uid=post.uid,
            )

        f.write(
            SERIES_PAGE.format(
                title=series_title, abstract=abstract_html, posts=posts_list_html
            )
        )


def write_post_page(post, post_page_path, post_md_dir):
    with open(post_page_path, "wt") as f:
        f.write(
            POST_PAGE.format(
                post_title=post.title,
                date_published=post.date_added.strftime("%-d %B %Y"),
                post_abstract=post.abstract_html,
                post_body=post.body_html,
                post_references=post.references_html,
            )
        )

    md_attributes = ["abstract", "body", "references"]
    for md_attribute in md_attributes:
        with open(post_md_dir / f"{md_attribute}.md", "wt") as f:
            f.write(getattr(post, f"{md_attribute}_md"))


def fetch_remote_data():
    with db_utils.get_database_connection("remote") as remote_conn:
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
    with db_utils.get_database_connection("local") as conn:
        for schema in base_db_config.schemas:
            if delete:
                conn.execute("DROP SCHEMA IF EXISTS {} CASCADE;".format(schema))
            conn.execute("CREATE SCHEMA IF NOT EXISTS {};".format(schema))

        conn.session.commit()

        base_db_config.Base.metadata.create_all(conn.engine)
        conn.session.commit()


def save_remote_posts_to_local_file_system(people, authors, all_post_series, posts):
    config = configuration.get_config()
    data_config_key = "local" if os.uname()[1] == "horace-monster" else "remote"
    base_dir = pathlib.Path(config["directories"][data_config_key]["base_dir"])
    writing_output_dir = (
        base_dir / "rs" / "abenga_site" / "templates" / "pages" / "writing"
    )

    post_series_uid = None
    for post_series in all_post_series:
        post_series_uid = str(post_series.uid)

        post_series_dir = writing_output_dir / "post_series" / post_series_uid
        post_series_dir.mkdir(parents=True, exist_ok=True)

        abstract_md_file_path = post_series_dir / "md" / "abstract.md"
        (post_series_dir / "md").mkdir(parents=True, exist_ok=True)
        with open(abstract_md_file_path, "wt") as f:
            f.write(post_series.abstract_md)

        series_html_file_path = os.path.join(post_series_dir, "series_page.html")
        write_post_series_page(
            series_html_file_path, post_series.title, post_series.abstract_html, posts
        )

        for i, post in enumerate(posts):
            post_dir = writing_output_dir / "posts" / str(post.uid)
            post_dir.mkdir(exist_ok=True, parents=True)

            post_md_dir = post_dir / "md"
            post_md_dir.mkdir(exist_ok=True, parents=True)

            post_page_path = post_dir / "post_page.html"
            write_post_page(post, post_page_path, post_md_dir)


def main(args):
    people, authors, post_series, posts = fetch_remote_data()
    print(people)
    save_remote_posts_to_local_file_system(people, authors, post_series, posts)
    # if args.create_local:
    #     create_local_db(args.delete)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--delete", action="store_true", help="Delete and recreate DB")
    parser.add_argument("--create_local", action="store_true", help="Create local db")
    _args = parser.parse_args()
    main(_args)
