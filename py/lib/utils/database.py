import csv

from sqlalchemy import create_engine
from sqlalchemy.engine import reflection
from sqlalchemy.orm import Session


from sqlalchemy.schema import (
    MetaData,
    Table,
    DropTable,
    ForeignKeyConstraint,
    DropConstraint,
)

from abenga_site.py.lib.utils import configuration


def drop_all(db):
    # gather all data first before dropping anything.
    # some DBs lock after things have been dropped in
    # a transaction.

    # Postgres Data Types
    datatypes = []
    if db.engine.driver == "psycopg2":
        r = db.conn.execute(
            """SELECT n.nspname as schema, t.typname as type 
            FROM pg_type t LEFT JOIN pg_catalog.pg_namespace n ON n.oid = t.typnamespace
            WHERE (
                t.typrelid = 0 OR (
                    SELECT c.relkind = 'c' FROM pg_catalog.pg_class c WHERE c.oid = t.typrelid
                )
            ) AND NOT EXISTS(
                SELECT 1 FROM pg_catalog.pg_type el WHERE el.oid = t.typelem AND el.typarray = t.oid
            ) AND n.nspname NOT IN ('pg_catalog', 'information_schema');"""
        ).fetchall()
        for dt in r:
            datatypes.append(dt[1])

    # Views
    views = []
    if db.engine.driver == "psycopg2":
        r = db.conn.execute(
            "SELECT table_name "
            "FROM information_schema.tables "
            "WHERE table_schema='public' AND table_type='VIEW';"
        ).fetchall()
        for v in r:
            views.append(v[0])
    elif db.engine.driver == "mysqldb":
        r = db.conn.execute(
            "SHOW FULL TABLES IN {} WHERE TABLE_TYPE LIKE 'VIEW';".format(db.database)
        )
        for v in r:
            views.append(v[0])
    trans = db.conn.begin()

    inspector = reflection.Inspector.from_engine(db.engine)

    metadata = MetaData()

    tbs = []
    all_fks = []

    for table_name in inspector.get_table_names():
        fks = []
        for fk in inspector.get_foreign_keys(table_name):
            if not fk["name"]:
                continue
            fks.append(ForeignKeyConstraint((), (), name=fk["name"]))
        t = Table(table_name, metadata, *fks)
        tbs.append(t)
        all_fks.extend(fks)

    for fkc in all_fks:
        db.conn.execute(DropConstraint(fkc))

    for view in views:
        db.conn.execute("DROP VIEW IF EXISTS {} CASCADE;".format(view))

    for table in tbs:
        db.conn.execute(DropTable(table))

    for datatype in datatypes:
        db.conn.execute("DROP TYPE {} CASCADE;".format(datatype))

    trans.commit()


def get_database_engine(db_config_section):
    config = configuration.get_config()

    protocol = config["databases"][db_config_section]["protocol"]
    host = config["databases"][db_config_section]["host"]
    database = config["databases"][db_config_section]["database"]
    user = config["databases"][db_config_section]["user"]
    password = config["databases"][db_config_section]["password"]
    echo = config["databases"][db_config_section]["echo"]

    return create_engine(
        "{protocol}://{user}:{password}@{host}/{database}"
        "".format(
            protocol=protocol,
            host=host,
            user=user,
            password=password,
            database=database,
        ),
        echo=echo,
    )
