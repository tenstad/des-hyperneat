from analisys.analisys_2 import analyse
import sys

if __name__ == '__main__':
    if len(sys.argv) > 1:
        analyse(sys.argv[1])
    else:
        print('Batch numper expected as argument')
