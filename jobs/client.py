import os
from pymongo import MongoClient


def get_client():
    return MongoClient(
        os.environ.get('DB_HOST', 'localhost'),
        username=os.environ.get('DB_USERNAME', 'admin'),
        password=os.environ.get('DB_PASSWORD', ''),
    )
