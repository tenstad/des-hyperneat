import numpy as np

combinations = [(a, b, c, d) for a in (0, 1)
                for b in (0, 1) for c in (0, 1) for d in (0, 1)]

left_patterns = [
    (0, 1, 1, 1),
    (1, 0, 1, 1),
    (0, 0, 1, 1),
    (0, 0, 0, 0),
    (0, 1, 0, 0),
    (1, 0, 0, 0),
    (0, 0, 0, 1),
    (0, 0, 1, 0),
]


right_patterns = [
    (0, 1, 0, 0),
    (1, 0, 0, 0),
    (0, 0, 0, 1),
    (0, 0, 1, 0),
    (1, 1, 0, 1),
    (1, 1, 1, 0),
    (1, 1, 0, 0),
    (0, 0, 0, 0),
]


left_not_patterns = [
    p for p in combinations if not p in left_patterns
]

right_not_patterns = [
    p for p in combinations if not p in right_patterns
]


def map_neg(li):
    return [1 if x else -1 for x in li]


def map_mul(li, n):
    return [n * x for x in li]


with open('generated/retina', 'w') as f:
    f.write('true\nfalse\n\n')

    inputs = []
    outputs = []
    for left in combinations:
        for right in combinations:
            is_pattern = [int(left in left_patterns),
                          int(right in right_patterns)]

            left_mapped = map_neg(left)
            right_mapped = map_neg(right)

            inputs.append([*left_mapped, *right_mapped])
            outputs.append(map_neg(is_pattern))

    for inp in inputs:
        f.write(', '.join(map(str, inp)))
        f.write('\n')

    f.write('\n')

    for output in outputs:
        f.write(', '.join(map(str, output)))
        f.write('\n')
