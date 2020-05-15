import os
from pymongo.read_concern import ReadConcern
from pymongo import WriteConcern, ReadPreference
import time
from src.client import get_client
from datetime import datetime
import time


def work():
    start_time = time.time()
    minute_limit = int(os.environ.get("MINUTE_LIMIT", "0"))
    stop_if_idle = os.environ.get(
        "IDLE_TERMINATE", "false").lower() == 'true'

    def running():
        return minute_limit <= 0 or (time.time() - start_time) < minute_limit * 60

    client = get_client()

    while 1:
        if not running():
            print_now('out of time, terminating')
            break

        job = run_transaction(client, find_job_transaction)

        if job:
            do_job(job)
            run_transaction(
                client, create_complete_job_transaction(job['_id']))
        elif stop_if_idle:
            print_now('no more jobs, terminating')
            break
        else:
            print_now('waiting for job...')
            time.sleep(1)


def do_job(job):
    name, job_id = job.get('name', "unnamed"), job.get('_id', -1)
    print_now('found job:', name, job_id)

    parameters = job.get('parameters', {})
    for k, v in parameters.items():
        os.putenv(k, str(v))
    os.putenv("DEBUG", "false")
    os.putenv("JOB_ID", str(job.get('_id', -1)))

    print_now('running job...')
    os.system('cargo run --release')

    print_now('completed job')


def find_job_transaction(session):
    jobs = getattr(session.client, os.environ.get(
        'DATABASE', 'deshyperneat')).jobs

    return jobs.find_one_and_update(
        {'$expr': {'$lt': ['$started', '$scheduled']}},
        {'$inc': {'started': 1}},
        sort=[('timestamp', 1)], session=session)


def create_complete_job_transaction(id):
    def complete_job_transaction(session):
        jobs = getattr(session.client, os.environ.get(
            'DATABASE', 'deshyperneat')).jobs

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


def print_now(*msg):
    msg = ' '.join(map(str, msg))
    print(f'[{now()}] {msg}')


def now():
    return datetime.now().strftime("%b %d %H:%M:%S")
