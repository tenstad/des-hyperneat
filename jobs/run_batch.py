import sys
import batches

if __name__ == '__main__':
    if len(sys.argv) > 1:
        i = sys.argv[1]
        try:
            exec(f'from batches.batch_{i} import run')
            run()
        except ModuleNotFoundError:
            print(f'Invalid batch: {i}')
    else:
        print('Batch numper expected as argument')
