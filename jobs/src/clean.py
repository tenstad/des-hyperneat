import os
from src.client import get_database
from datetime import datetime
from bson.objectid import ObjectId
from multiprocessing import Pool


def clean_jobs(batch_nr):
    client, db = get_database()
    ids = list(db.jobs.find({'batch': int(batch_nr)}, {'_id': 1}))
    client.close()

    with Pool(16) as p:
        p.map(clean_job, ids)


def clean_job(job_query):
    client, db = get_database()
    job = db.jobs.find(job_query)[0]

    job_id = job['_id']
    job_name = job['name']

    delete_result = db.logs.delete_many({
        'job_id': ObjectId(job_id),
        'completed': False,
    })

    num_completed = db.logs.find({
        'job_id': ObjectId(job_id),
        'completed': True,
    }).count()

    response = db.jobs.update_one(
        {'_id': job_id},
        {'$set': {
            'started': num_completed,
            'completed': num_completed,
            'aborted': 0,
        }})

    if response.modified_count or delete_result.deleted_count:
        msg = f'Cleaned job {job_id}: {job_name} and deleted {delete_result.deleted_count} log entries'
        print(msg)

    client.close()
