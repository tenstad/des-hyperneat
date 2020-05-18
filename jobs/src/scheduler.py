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

    def reset_job(self, job_id):
        db = getattr(self.client, os.environ.get(
            'DATABASE', 'deshyperneat'))

        job_query = {'_id': ObjectId(job_id)}
        job = db.jobs.find(job_query)

        try:
            job = job.next()
            db.jobs.update_one(job_query, {'$set': {
                'started': 0,
                'completed': 0,
                'aborted': 0,
            }})

            result = db.logs.delete_many({'job_id': ObjectId(job_id)})
            print(
                f'Cleared job and deleted {result.deleted_count} log entries')

        except StopIteration:
            print(f'Job does not exist: {job_id}')
