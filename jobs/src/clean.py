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

    if 'config' in job:
        clean_job_started(db, job)
    else:
        clean_job_not_started(db, job)

    client.close()


def clean_job_started(db, job):
    job_id = job['_id']
    job_name = job['name']

    evo_cfg = job['config']['evolution']
    if evo_cfg['iterations'] != 0:
        correct_length = int(evo_cfg['iterations'] /
                             evo_cfg['log_interval']) + 1
    else:
        correct_length = int(evo_cfg['seconds_limit'] /
                             evo_cfg['log_sec_interval']) + 1

    delete_result = db.logs.delete_many({
        'job_id': ObjectId(job_id),
        'events': {'$not': {'$size': correct_length}}
    })

    num_completed = db.logs.find({
        'job_id': ObjectId(job_id),
        'events': {'$size': correct_length}
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


def clean_job_not_started(db, job):
    job_id = job['_id']
    job_name = job['name']

    response = db.jobs.update_one(
        {'_id': ObjectId(job_id)},
        {'$set': {
            'started': 0,
            'completed': 0,
            'aborted': 0,
        }})

    if response.modified_count:
        print(f'Cleaned job {job_id}: {job_name}')
