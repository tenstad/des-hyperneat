import math
import numpy as np

with open('data.txt', 'r') as f:
    lines = [l.split() for l in f.readlines()]
    lines = dict([(tuple(l[:-2]), tuple(map(float, l[-2:]))) for l in lines])


def find_data(*params):
    try:
        return lines[tuple(params)]
    except:
        raise Exception('Could not find line ' + str(params))


with open('data2.txt', 'w') as f:
    avgs = np.zeros((3, 3, 4))
    for i, io in enumerate(('0', '5')):
        for j, dataset in enumerate(('iris', 'wine', 'retina')):
            d = 'datasets/generated/' + dataset
            vals = [
                *find_data('0', io, d, 'max_validation_fitness'),
                *find_data('5', io, d, 'max_validation_fitness'),
                #*find_data('0', io, d, 'max_validation_accuracy'),
                # *find_data('5', io, d, 'max_validation_accuracy')
            ]
            vals = [('%.' + str(min(int(4-math.log10(val)), 3)) + 'f') % val
                    for val in vals]
            for k, val in enumerate(vals):
                avgs[i, j, k] = val
            vals = ' & '.join(vals)
            line = f'& {d} & {vals} \\\\\n'
            f.write(line)
    f.write('\n')

    avgs = avgs.mean(axis=1)
    for j, io in enumerate(('0', '5')):
        vals = avgs[j]
        vals = [('%.' + str(min(int(4-math.log10(val)), 3)) + 'f') % val
                for val in vals]
        vals = ' & '.join(vals)
        line = f'& mean & {vals} \\\\\n'
        f.write(line)
    f.write('\n')

    avgs = np.zeros((3, 3, 12))
    for i, io in enumerate(('0', '5')):
        for j, dataset in enumerate(('iris', 'wine', 'retina')):
            d = 'datasets/generated/' + dataset
            vals = [
                *find_data('0', io, d, 'num_nodes'),
                *find_data('5', io, d, 'num_nodes'),
                *find_data('0', io, d, 'num_edges'),
                *find_data('5', io, d, 'num_edges'),
            ]
            vals = [('%.' + str(min(int(4-math.log10(val)), 3)) + 'f') % val
                    for val in vals]
            for k, val in enumerate(vals):
                avgs[i, j, k] = val
            vals = ' & '.join(vals)
            line = f'& {d} & {vals} \\\\\n'
            f.write(line)
    f.write('\n')

    avgs = avgs.mean(axis=1)
    for j, io in enumerate(('0', '5')):
        vals = avgs[j]
        vals = [('%.' + str(min(int(4-math.log10(val)), 3)) + 'f') % val
                for val in vals]
        vals = ' & '.join(vals)
        line = f'& mean & {vals} \\\\\n'
        f.write(line)
    f.write('\n')
