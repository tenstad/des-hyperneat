import sys
from src.delete import delete_jobs

if __name__ == '__main__':
    if len(sys.argv) > 1:
        delete_jobs(sys.argv[1])
    else:
        print('Batch numper expected as argument')
