import sys
from src.clean import clean_jobs

if __name__ == '__main__':
    if len(sys.argv) > 1:
        clean_jobs(sys.argv[1])
    else:
        print('Batch numper expected as argument')
