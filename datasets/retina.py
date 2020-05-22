import numpy as np

N = 2000

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

    outputs = []
    for _ in range(N):
        is_pattern = (np.random.random((2, )) < 0.5).astype(np.int)
        pattern_index = (8 * np.random.random((2, ))).astype(np.int)

        left = left_patterns[pattern_index[0]] \
            if is_pattern[0] \
            else left_not_patterns[pattern_index[0]]

        right = right_patterns[pattern_index[1]] \
            if is_pattern[1] \
            else right_not_patterns[pattern_index[1]]

        left = map_neg(left)
        right = map_neg(right)

        outputs.append(map_neg(is_pattern))
        f.write(', '.join(map(str, [*left, *right])))
        f.write('\n')

    f.write('\n')

    for output in outputs:
        f.write(', '.join(map(str, output)))
        f.write('\n')
