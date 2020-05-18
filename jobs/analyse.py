from analisys.analisys import analyse
import sys

if __name__ == '__main__':
    if len(sys.argv) > 1:
        i = sys.argv[1]
        analyse(i)
    else:
        print('Batch numper expected as argument')
