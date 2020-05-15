import os
from pymongo.read_concern import ReadConcern
from pymongo import WriteConcern, ReadPreference
import time
from jobs.client import get_client


def find_job_transaction(session):
    jobs = getattr(session.client, os.environ['DATABASE']).jobs

    return jobs.find_one_and_update(
        {'$expr': {'$lt': ['$started', '$scheduled']}},
        {'$inc': {'started': 1}},
        sort=[('started', -1)], session=session)


def create_complete_job_transaction(id):
    def complete_job_transaction(session):
        jobs = getattr(session.client, os.environ['DATABASE']).jobs

        return jobs.find_one_and_update(
            {'_id': id},
            {'$inc': {'completed': 1}},
            session=session)

    return complete_job_transaction


def run_transaction(client, transaction):
    with client.start_session() as session:
        return session.with_transaction(
            transaction, read_concern=ReadConcern('majority'),
            write_concern=WriteConcern('majority', wtimeout=1000),
            read_preference=ReadPreference.PRIMARY)


def main():
    client = get_client()

    while 1:
        job = run_transaction(client, find_job_transaction)

        if job is not None:
            print('found job: ', job)

            parameters = job.get('parameters', {})
            for k, v in parameters.items():
                os.putenv(k, str(v))
            os.putenv("DEBUG", "false")
            os.putenv("JOB_ID", str(job.get('_id', -1)))

            print('running job...')
            os.system('cargo run --release')

            print('job complete')
            run_transaction(
                client, create_complete_job_transaction(job['_id']))
        else:
            print('waiting for job...')

            time.sleep(1)


if __name__ == '__main__':
    main()
