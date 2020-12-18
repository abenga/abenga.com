
import csv
import logging

from sqlalchemy import create_engine
from sqlalchemy.engine import reflection
from sqlalchemy.orm import sessionmaker

from sqlalchemy.schema import (
    MetaData,
    Table,
    DropTable,
    ForeignKeyConstraint,
    DropConstraint,
)


from abenga_site.py.lib.utils import configuration


def drop_all (db):
    # gather all data first before dropping anything.
    # some DBs lock after things have been dropped in
    # a transaction.

    # Postgres Data Types
    datatypes = []
    if db.engine.driver == 'psycopg2':
        r = db.conn.execute("""SELECT n.nspname as schema, t.typname as type 
            FROM pg_type t LEFT JOIN pg_catalog.pg_namespace n ON n.oid = t.typnamespace
            WHERE (
                t.typrelid = 0 OR (
                    SELECT c.relkind = 'c' FROM pg_catalog.pg_class c WHERE c.oid = t.typrelid
                )
            ) AND NOT EXISTS(
                SELECT 1 FROM pg_catalog.pg_type el WHERE el.oid = t.typelem AND el.typarray = t.oid
            ) AND n.nspname NOT IN ('pg_catalog', 'information_schema');""").fetchall()
        for dt in r:
            datatypes.append(dt[1])

    # Views
    views = []
    if db.engine.driver == 'psycopg2':
        r = db.conn.execute("SELECT table_name "
                            "FROM information_schema.tables "
                            "WHERE table_schema='public' AND table_type='VIEW';").fetchall()
        for v in r:
            views.append(v[0])
    elif db.engine.driver == 'mysqldb':
        r = db.conn.execute("SHOW FULL TABLES IN {} WHERE TABLE_TYPE LIKE 'VIEW';".format(db.database))
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
            if not fk['name']:
                continue
            fks.append(ForeignKeyConstraint((),(),name=fk['name']))
        t = Table(table_name,metadata,*fks)
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


class DatabaseConnection:
    engine = None
    session = None
    echo = None

    def __init__ (self, protocol, host, user, password, database, echo):
        self.engine = create_engine('{protocol}://{user}:{password}@{host}/{database}'
                                    ''.format(protocol=protocol,
                                              host=host,
                                              user=user,
                                              password=password,
                                              database=database),
                                    echo=echo)
        self.conn = self.engine.connect()
        self.database = database
        self.protocol = protocol
        self.echo = echo
        Session = sessionmaker(expire_on_commit=False)
        Session.configure(bind=self.engine)
        self.session = Session()

    def __enter__(self):
        pass

    def __exit__(self, type, value, traceback):
        # Exception handling here
        self.close()

    def select_db(self, db):
        pass

    def execute(self, query):
        self.conn.execute(query)

    def query(self, query):
        """
        :param query:
        :return: all rows matching query
        """
        result = self.conn.execute(query)
        rows = []
        for row in result:
            rows.append(row)
        return rows

    def fetch_data(self, schema_name,  table_name, conditions_dict):
        conditions = []
        for col, val in conditions_dict.items():
            if isinstance(val, list) or isinstance(val, tuple):
                conditions.append(
                    "{} IN ({})".format(col, ', '.join([str(el) for el in val]))
                )
            elif isinstance(val, int) or isinstance(val, float):
                conditions.append("{} = {}".format(col, val))
            else:
                conditions.append("{} = '{}'".format(col, val))
        conditions_str = ' AND '.join(conditions)
        where_str = "WHERE {}".format(conditions_str) if len(conditions) > 0 else ""
        query = "SELECT * FROM {schema}.{table} {where_str}" \
                "".format(schema=schema_name, table=table_name, where_str=where_str)
        return self.query(query)

    def insert(self, data):
        """
        :param data:
        :return: Number of rows inserted
        """
        if data.get('Model') and data.get('Data') is not None:
            error_occurred = False
            for link in data.get('Links', []):
                foreign_key_models = self.session.query(link['OriginModel']) \
                    .filter_by(**dict(link['OriginSelection'])) \
                    .all()
                if foreign_key_models and len(foreign_key_models) == 1:
                    data['Data'][link['SecondaryKey']] = getattr(foreign_key_models[0], link['OriginPrimaryKey'])
                else:
                    raise Exception('Error obtaining foreign key data.')
            if not error_occurred:
                el = data['Model'](**data['Data'])
                self.session.add(el)
                return el
            return -1
        elif data.get('Model') and data.get('DataFile'):
            n_rows = 0
            with open(data.get('DataFile'), 'r') as f:
                reader = csv.DictReader(f)
                for row in reader:
                    row = {k: (v or None) for k, v in row.items()}
                    for k, v in row.items():
                        if k in data.get('GeneratedColumns', {}):
                            func_arg_cols = data['GeneratedColumns'][k].get('args')
                            if func_arg_cols is None:
                                row[k] = data['GeneratedColumns'][k]['func']()
                            else:
                                row[k] = data['GeneratedColumns'][k]['func'](*[
                                    row.get(col) for col in func_arg_cols
                                ])

                    for link in data.get('Links', []):
                        selection_filter = {
                            db_col: row.get(file_col)
                            for db_col, file_col in link['OriginSelection'].items()
                        }
                        foreign_objects = self.session.query(link['OriginModel']). \
                            filter_by(**selection_filter).all()
                        if foreign_objects and len(foreign_objects) == 1:
                            row[link['SecondaryKey']] = getattr(foreign_objects[0],
                                                                link['OriginPrimaryKey'])
                        else:
                            raise Exception('Error obtaining foreign key data.')

                    data_entry = {k: row.get(k) or None for k in data['Columns']}
                    n_rows += 1 if self.insert({'Model': data['Model'],
                                                'Data': data_entry}) != -1 else 0
            if self.echo:
                print('number of rows inserted: {:,}'.format(n_rows))
            return 0


class DBConnMaker:
    def __init__(self, protocol, host, user, password, database, echo):
        self.db_conn = DatabaseConnection(protocol, host, user, password, database, echo)

    def __enter__(self):
        return self.db_conn

    def __exit__(self, exc_type, exc_val, exc_tb):
        logging.debug('Closing connection {}'.format(self.db_conn.conn))
        self.db_conn.session.close()
        self.db_conn.conn.close()


def get_database_connection(db_config_section):
    """

    :return:
    """
    config = configuration.get_config()

    protocol = config['databases'][db_config_section]['protocol']
    host = config['databases'][db_config_section]['host']
    database = config['databases'][db_config_section]['database']
    user = config['databases'][db_config_section]['user']
    password = config['databases'][db_config_section]['password']
    echo = config['databases'][db_config_section]['echo']

    return DBConnMaker(protocol, host, user, password, database, echo)
