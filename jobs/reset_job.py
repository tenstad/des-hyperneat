import sys
from src.scheduler import Scheduler

if __name__ == '__main__':
    if len(sys.argv) > 1:
        job_id = sys.argv[1]

        scheduler = Scheduler()
        scheduler.reset_job(job_id)
    else:
        print('Job id expected as argument')
