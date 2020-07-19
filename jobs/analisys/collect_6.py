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
    for dataset in ('iris', 'wine', 'retina'):
        for j, depth in enumerate((-1, 0, 1, 2, 3, 4, 5)):
            depth = str(depth)
            d = 'datasets/generated/' + dataset
            vals = [
                *find_data('200', depth, d, 'max_validation_fitness'),
                *find_data('0', depth, d, 'max_validation_fitness'),
                *find_data('200', depth, d, 'max_validation_accuracy'),
                *find_data('0', depth, d, 'max_validation_accuracy')]
            vals = [('%.' + str(min(int(4-math.log10(val)), 3)) + 'f') % val
                    for val in vals]
            vals = ' & '.join(vals)
            line = f'& {depth} & {vals} \\\\\n'
            f.write(line)
    f.write('\n')

    avgs = np.zeros((3, 7, 8))
    for i, dataset in enumerate(('iris', 'wine', 'retina')):
        for j, depth in enumerate((-1, 0, 1, 2, 3, 4, 5)):
            depth = str(depth)
            d = 'datasets/generated/' + dataset
            vals = [
                *find_data('200', depth, d, 'num_nodes'),
                *find_data('0', depth, d, 'num_nodes'),
                *find_data('200', depth, d, 'num_edges'),
                *find_data('0', depth, d, 'num_edges')]
            for k, val in enumerate(vals):
                avgs[i, j, k] = val
            vals = [('%.' + str(min(int(4-math.log10(val)), 3)) + 'f') % val
                    for val in vals]
            vals = ' & '.join(vals)
            line = f'& {depth} & {vals} \\\\\n'
            f.write(line)
    f.write('\n')

    avgs = avgs.mean(axis=0)
    for j, depth in enumerate((-1, 0, 1, 2, 3, 4, 5)):
        vals = avgs[j]
        vals = [('%.' + str(min(int(4-math.log10(val)), 3)) + 'f') % val
                for val in vals]
        vals = ' & '.join(vals)
        line = f'& {depth} & {vals} \\\\\n'
        f.write(line)
    f.write('\n')

    avgs = np.zeros((3, 7, 4))
    for i, dataset in enumerate(('iris', 'wine', 'retina')):
        for j, depth in enumerate((-1, 0, 1, 2, 3, 4, 5)):
            depth = str(depth)
            d = 'datasets/generated/' + dataset
            vals = [
                *find_data('200', depth, d, 'iterations'),
                *find_data('0', depth, d, 'iterations')]
            for k, val in enumerate(vals):
                avgs[i, j, k] = val
            vals = [('%.' + str(min(int(4-math.log10(val)), 3)) + 'f') % val
                    for val in vals]
            vals = ' & '.join(vals)
            line = f'& {depth} & {vals} \\\\\n'
            f.write(line)
    f.write('\n')

    avgs = avgs.mean(axis=0)
    for j, depth in enumerate((-1, 0, 1, 2, 3, 4, 5)):
        depth = str(depth)
        vals = avgs[j]
        vals = [('%.' + str(min(int(4-math.log10(val)), 3)) + 'f') % val
                for val in vals]
        vals = ' & '.join(vals)
        line = f'& {depth} & {vals} \\\\\n'
        f.write(line)
    f.write('\n')
