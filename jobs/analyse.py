from analisys.analisys import analyse
from analisys.analisys_time import analyse_time
import sys

if __name__ == '__main__':
    if len(sys.argv) > 1:
        i = sys.argv[1]

        if len(sys.argv) > 2:
            if sys.argv[2] == 'time':
                analyse_time(i)
        else:
            analyse(i)
    else:
        print('Batch numper expected as argument')
