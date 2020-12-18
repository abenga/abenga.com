#!../../env/bin/python

from abenga_site.py.lib import models
from abenga_site.py.lib.utils import database as db_utils


def main():
    with db_utils.get_database_connection('local') as conn:
        print(conn)


if __name__ == '__main__':
    main()
