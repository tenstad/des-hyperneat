import math
import numpy as np
import json


with open('jobs/analisys/plots/batch_25/scoreboard.txt', 'r') as f:
    lines = [l.split() for l in f.readlines()]
    lines = [(float(l[1]), float(l[2]), json.loads(' '.join(l[3:])))
             for l in lines]
    r = {}
    for score, std, d in lines:
        k = (
            d['DATASET'],
            d['METHOD'],
            d['VARIANCE_THRESHOLD'],
            d['DIVISION_THRESHOLD'],
            d['MAX_VARIANCE'],
            d['ONLY_LEAF_VARIANCE'],
            d['RELATIVE_VARIANCE'],
        )
        r[k] = (score, std)


def find_data(*params):
    try:
        return r[tuple(params)]
    except:
        raise Exception('Could not find line ' + str(params))


r2 = {}
for method in ('DES-HyperNEAT', 'ES-HyperNEAT'):
    with open('data_tmp.txt', 'a') as f:
        f.write(f'\n,')
    for m_v in (True, False):
        for l_v in (True, False):
            for r_v in (True, False):
                total_score = 0
                best_score = 0
                best_v = None
                best_d = None
                best_l = None
                best_std = None
                for v_t in [0.05, 0.15, 0.3]:
                    for d_t in [0.05, 0.15, 0.3]:
                        scores = 0
                        l = []
                        stds = []
                        for dataset in ('iris', 'wine', 'retina'):
                            d = 'datasets/generated/' + dataset
                            s, std = find_data(
                                d, method, v_t, d_t, m_v, l_v, r_v)
                            l.append(s)
                            stds.append(std)
                            scores += s
                            total_score += s
                        score = scores / 3
                        if score > best_score:
                            best_v = v_t
                            best_d = d_t
                            best_score = score
                            best_std = stds
                            best_l = l
                print(method, m_v, l_v, r_v, best_score,
                      best_l, best_std, best_v, best_d)
                with open('data_tmp.txt', 'a') as f:
                    f.write(f'({m_v}, {l_v}, {r_v}): ({best_v}, {best_d}),')
                for dataset, s, std in zip(('iris', 'wine', 'retina'), best_l, best_std):
                    r2[(method, m_v, l_v, r_v, dataset)] = (s, std)
                r2[(method, m_v, l_v, r_v, 'mean')] = (
                    best_score, sum(stds) / 3)

with open('data2.txt', 'w') as f:
    for method in ('ES-HyperNEAT', 'DES-HyperNEAT'):
        for relative in (True, False):
            for d in ('iris', 'wine', 'retina', 'mean'):
                vals = [
                    *r2[(method, True, False, relative, d)],
                    *r2[(method, True, True, relative, d)],
                    *r2[(method, False, False, relative, d)],
                    *r2[(method, False, True, relative, d)]
                ]

                vals = [('%.' + str(min(int(4-math.log10(val)), 3)) + 'f') % val
                        for val in vals]
                vals = ' & '.join(vals)
                d = d.title()
                line = f'& \\textbf{{{d}}} & {vals} \\\\\n'
                f.write(line)
        f.write('\n')
