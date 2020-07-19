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


def mlog(val):
    return math.log10(val) if val != 0 else 0


with open('data2.txt', 'w') as f:
    for dataset in ('iris', 'wine', 'retina'):
        for method in (('NEAT', 'HyperNEAT', 'ES-HyperNEAT', 'DES-HyperNEAT')):
            m = method
            d = 'datasets/generated/' + dataset
            vals = [
                *find_data('600', m, d, 'max_validation_fitness'),
                *find_data('0', m, d, 'max_validation_fitness'),
                *find_data('600', m, d, 'max_validation_accuracy'),
                *find_data('0', m, d, 'max_validation_accuracy')]
            vals = [('%.' + str(min(int(4-mlog(val)), 3)) + 'f') % val
                    for val in vals]
            vals = ' & '.join(vals)
            line = f'& {method} & {vals} \\\\\n'
            f.write(line)
    f.write('\n')

    avgs = np.zeros((3, 4, 4))
    for i, dataset in enumerate(('iris', 'wine', 'retina')):
        for j, method in enumerate(('NEAT', 'HyperNEAT', 'ES-HyperNEAT', 'DES-HyperNEAT')):
            m = method
            d = 'datasets/generated/' + dataset
            vals = [
                #*find_data('600', m, d, 'num_nodes'),
                *find_data('0', m, d, 'num_nodes'),
                # *find_data('600', m, d, 'num_edges'), ]
                *find_data('0', m, d, 'num_edges')]
            for k, val in enumerate(vals):
                avgs[i, j, k] = val
            vals = [('%.' + str(min(int(4-mlog(val)), 3)) + 'f') % val
                    for val in vals]
            vals = ' & '.join(vals)
            line = f'& {method} & {vals} \\\\\n'
            f.write(line)
    f.write('\n')

    avgs = avgs.mean(axis=0)
    for j, method in enumerate(('NEAT', 'HyperNEAT', 'ES-HyperNEAT', 'DES-HyperNEAT')):
        vals = avgs[j]
        vals = [('%.' + str(min(int(4-mlog(val)), 3)) + 'f') % val
                for val in vals]
        vals = ' & '.join(vals)
        line = f'& {method} & {vals} \\\\\n'
        f.write(line)
    f.write('\n')

    avgs = np.zeros((3, 4, 2))
    for i, dataset in enumerate(('iris', 'wine', 'retina')):
        for j, method in enumerate(('NEAT', 'HyperNEAT', 'ES-HyperNEAT', 'DES-HyperNEAT')):
            m = method
            d = 'datasets/generated/' + dataset
            vals = [
                #*find_data('600', m, d, 'iterations'),
                *find_data('0', m, d, 'iterations')]
            vals = [val for val in vals]
            #vals = vals[0:2]
            for k, val in enumerate(vals):
                avgs[i, j, k] = val
            vals = [('%.' + str(min(int(4-mlog(val)), 3)) + 'f') % val
                    for val in vals]
            vals = ' & '.join(vals)
            line = f'& {method} & {vals} \\\\\n'
            f.write(line)
    f.write('\n')

    avgs = avgs.mean(axis=0)
    for j, method in enumerate(('NEAT', 'HyperNEAT', 'ES-HyperNEAT', 'DES-HyperNEAT')):
        vals = avgs[j]
        vals = [('%.' + str(min(int(4-mlog(val)), 3)) + 'f') % val
                for val in vals]
        vals = ' & '.join(vals)
        line = f'& {method} & {vals} \\\\\n'
        f.write(line)
    f.write('\n')
