import os
from src.client import get_database
from datetime import datetime
from bson.objectid import ObjectId
from multiprocessing import Pool


def delete_jobs(batch_nr):
    client, db = get_database()
    ids = list(db.jobs.find({'batch': int(batch_nr)}, {'_id': 1}))
    client.close()

    ans = input(
        f'Are you sure you want to delete all the {len(ids)} jobs in batch {batch_nr}? ("YES {batch_nr}")')

    if ans == f'YES {batch_nr}':
        with Pool(16) as p:
            p.map(delete_job, ids)
    else:
        print('Aborted')


def delete_job(job_query):
    client, db = get_database()
    job = db.jobs.find(job_query)[0]

    job_id = job['_id']
    job_name = job['name']

    delete_result = db.logs.delete_many({
        'job_id': ObjectId(job_id),
    })

    response = db.jobs.delete_one(
        {'_id': job_id},
    )

    if response.deleted_count or delete_result.deleted_count:
        msg = f'Deleted job {job_id}: {job_name} and deleted {delete_result.deleted_count} log entries'
        print(msg)

    client.close()
