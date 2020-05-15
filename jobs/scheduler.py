import os
from jobs.client import get_client
from datetime import datetime


class Scheduler:
    def __init__(self):
        self.client = get_client()

    def create_job(self, name, schedules, parameters):
        jobs = getattr(self.client, os.environ['DATABASE']).jobs

        job = jobs.insert_one(
            {
                'name': name,
                'timestamp': datetime.now(),
                'scheduled': schedules,
                'started': 0,
                'completed': 0,
                'parameters': parameters,
            })

        if job.acknowledged:
            print(
                f'Successfully scheduled {schedules} instances of job "{name}"')
            print(f'with id {job.inserted_id} and parameters\n{parameters}')
        else:
            print('Unable to create job')
