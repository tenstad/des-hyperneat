import os
from pymongo.read_concern import ReadConcern
from pymongo import WriteConcern, ReadPreference
import time
from src.client import get_client
from datetime import datetime
import time
import json
import random


def work():
    start_time = time.time()
    minute_limit = int(os.environ.get("MINUTE_LIMIT", "0"))
    stop_if_idle = os.environ.get(
        "IDLE_TERMINATE", "false").lower() == 'true'

    def running():
        return minute_limit <= 0 or (time.time() - start_time) < minute_limit * 60

    time.sleep(random.random() * 10)

    while 1:
        if not running():
            print_now('out of time, terminating')
            break

        client = get_client()
        job = run_transaction(client, find_job_transaction)
        client.close()

        if job:
            try:
                do_job(job)
                time.sleep(0.5)
            except KeyboardInterrupt:
                print_now('terminating')
                client = get_client()
                run_transaction(
                    client, create_complete_job_transaction(job['_id'], abort=True))
                client.close()
                break

            client = get_client()
            run_transaction(
                client, create_complete_job_transaction(job['_id']))
            client.close()
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
    params = []
    for k, v in parameters.items():
        v = v if type(v) == str else json.dumps(v)
        params.append(f'{k}="{v}"')
    params = ' '.join(params)

    job_id = job.get('_id', -1)
    env_vars = f'RUST_BACKTRACE=1 DB_LOG=true JOB_ID={job_id} {params}'

    print_now('running job...')
    print(f'{env_vars} cargo run --release')
    os.system(f'{env_vars} cargo run --release')

    print_now('completed job')


def find_job_transaction(session):
    jobs = getattr(session.client, os.environ.get(
        'DATABASE', 'deshyperneat')).jobs

    return jobs.find_one_and_update(
        {'$expr': {
            '$lt': [{'$subtract': ['$started', '$aborted']}, '$scheduled']}},
        {'$inc': {'started': 1}},
        sort=[('timestamp', 1)], session=session)


def create_complete_job_transaction(id, abort=False):
    field = 'aborted' if abort else 'completed'

    def complete_job_transaction(session):
        jobs = getattr(session.client, os.environ.get(
            'DATABASE', 'deshyperneat')).jobs

        return jobs.find_one_and_update(
            {'_id': id},
            {'$inc': {field: 1}},
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
