import math
import numpy as np
import json

with open('jobs/analisys/plots/batch_22/scoreboard.txt', 'r') as f:
    lines = [l.split() for l in f.readlines()]
    lines = [(float(l[1][:-1]), json.loads(' '.join(l[2:]))) for l in lines]
    r = {}
    for score, d in lines:
        if d['BAND_THRESHOLD'] == 0 and d['DIVISION_THRESHOLD'] == 0.2 and d['VARIANCE_THRESHOLD'] == 0.2:
            k = (
                d['DATASET'],
                d['METHOD'],
                d['MAX_VARIANCE'],
                d['ONLY_LEAF_VARIANCE'],
                d['RELATIVE_VARIANCE'],
            )
            assert not k in r
            r[k] = score

print(r)


def find_data(*params):
    try:
        return r[tuple(params)]
    except:
        raise Exception('Could not find line ' + str(params))


with open('data2.txt', 'w') as f:
    avgs = np.zeros((2, 3, 4))
    for i, relative in enumerate((True, False)):
        for j, dataset in enumerate(('iris', 'wine', 'retina')):
            d = 'datasets/generated/' + dataset
            vals = [
                find_data(d, 'DES-HyperNEAT', True, True, relative),
                find_data(d, 'DES-HyperNEAT', True, False, relative),
                find_data(d, 'DES-HyperNEAT', False, True, relative),
                find_data(d, 'DES-HyperNEAT', False, False, relative)]
            for k, val in enumerate(vals):
                avgs[i, j, k] = val
            vals = [('%.' + str(min(int(4-math.log10(val)), 3)) + 'f') % val
                    for val in vals]
            vals = ' & '.join(vals)
            line = f'& {dataset} & {vals} \\\\\n'
            f.write(line)
    f.write('\n')

    avgs = avgs.mean(axis=1)
    for vals in avgs:
        vals = [('%.' + str(min(int(4-math.log10(val)), 3)) + 'f') % val
                for val in vals]
        vals = ' & '.join(vals)
        line = f'& mean & {vals} \\\\\n'
        f.write(line)
        f.write('\n')
