from sqlalchemy.ext.declarative import declarative_base


Base = declarative_base()

schemas = [
    "core",
    "data",
]

db_types = []
