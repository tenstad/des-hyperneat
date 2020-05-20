import os
from src.client import get_client
from datetime import datetime
from bson.objectid import ObjectId


class Scheduler:
    def __init__(self):
        self.client = get_client()

    def create_job(self, batch, name, schedules, parameters):
        jobs = getattr(self.client, os.environ.get(
            'DATABASE', 'deshyperneat')).jobs

        job = jobs.insert_one(
            {
                'batch': batch,
                'name': name,
                'timestamp': datetime.now(),
                'scheduled': schedules,
                'started': 0,
                'completed': 0,
                'aborted': 0,
                'parameters': parameters,
            })

        if job.acknowledged:
            print(
                f'Successfully scheduled {schedules} instances of job "{name}"')
            print(f'with id {job.inserted_id} and parameters\n{parameters}')
        else:
            print('Unable to create job')

    def clean_jobs(self):
        db = getattr(self.client, os.environ.get(
            'DATABASE', 'deshyperneat'))

        for job in db.jobs.find():
            job_id = job['_id']
            job_name = job['name']

            if 'config' in job:
                ecfg = job['config']['evolution']
                correct_length = ecfg['iterations'] / ecfg['log_interval'] + 1

                delete_result = db.logs.delete_many({
                    'job_id': ObjectId(job_id),
                    'events': {'$not': {'$size': correct_length}}
                })

                num_completed = db.logs.find({
                    'job_id': ObjectId(job_id),
                    'events': {'$size': correct_length}
                }).count()

                db.jobs.update_one(
                    {'_id': job_id},
                    {'$set': {
                        'started': num_completed,
                        'completed': num_completed,
                        'aborted': 0,
                    }})

                print(
                    f'Cleaned job {job_name} and deleted {delete_result.deleted_count} log entries')
            else:
                db.jobs.update_one(
                    {'_id': ObjectId(job_id)},
                    {'$set': {
                        'started': 0,
                        'completed': 0,
                        'aborted': 0,
                    }})

                print(f'Cleaned job {job_name}')
