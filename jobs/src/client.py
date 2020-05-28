import os
from pymongo import MongoClient


def get_database(database=None):
    client = get_client()
    if database is None:
        database = os.environ.get('DATABASE', 'deshyperneat')
    return client, getattr(client, database)


def get_client():
    return MongoClient(
        os.environ.get('DB_HOST', 'localhost'),
        username=os.environ.get('DB_USERNAME', 'admin'),
        password=os.environ.get('DB_PASSWORD', ''),
    )
