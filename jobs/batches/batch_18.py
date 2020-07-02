from sklearn.model_selection import ParameterGrid
from src.scheduler import Scheduler
import json


def run():
    BATCH = 18
    REPEATS = 50
    sheduler = Scheduler()

    param_grid = {
        'METHOD': ['ES-HyperNEAT', 'DES-HyperNEAT'],
        'DATASET': ['datasets/generated/iris',
                    'datasets/generated/wine',
                    'datasets/generated/retina'],
        'VARIANCE_THRESHOLD': [1.2, 1.5, 2.0],
        'DIVISION_THRESHOLD': [1.2, 1.5, 2.0],
        'RELATIVE_VARIANCE': [False, True],
    }

    static_params = {
        'ITERATIONS': 0,
        'LOG_INTERVAL': 0,
        'SECONDS_LIMIT': 120,
        'LOG_SEC_INTERVAL': 12,
        'VALIDATION_FRACTION': 0.2,
        'TEST_FRACTION': 0.0,
        'MAX_DISCOVERIES': 256,
        'MAX_OUTGOING': 32,
        'MAX_VARIANCE': False,
        'ONLY_LEAF_VARIANCE': True,
        'MEDIAN_VARIANCE': False,
        'BAND_THRESHOLD': 0.0,
    }

    def run_grid(grid):
        for params in ParameterGrid(grid):
            params['OUTPUT_ACTIVATION'] = 'Tanh' if params['DATASET'] == 'datasets/generated/retina' else 'Softmax'

            if params['DATASET'].endswith('retina'):
                if params['METHOD'].startswith('ES-HyperNEAT'):
                    params['INPUT_CONFIG'] = "[[-1.0, -0.5], [-0.33, -0.5], [-1.0, -1.0], [-0.33, -1.0], [0.33, -0.5], [1.0, -0.5], [0.33, -1.0], [1.0, -1.0]]"
                else:
                    params['INPUT_CONFIG'] = "[[[-1.0, 0.5], [-0.33, 0.5], [-1.0, -0.5], [-0.33, -0.5], [0.33, 0.5], [1.0, 0.5], [0.33, -0.5], [1.0, -0.5]]]"

            name = json.dumps(params)
            params.update(static_params)
            sheduler.create_job(BATCH, name, REPEATS, params)

    run_grid(param_grid)
