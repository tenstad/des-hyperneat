from sklearn.model_selection import ParameterGrid
from src.scheduler import Scheduler
import json


def run():
    BATCH = 36
    REPEATS = 50
    sheduler = Scheduler()

    param_grid = {
        'METHOD': ['ES-HyperNEAT', 'DES-HyperNEAT'],
        'DATASET': ['datasets/generated/iris',
                    'datasets/generated/wine',
                    'datasets/generated/retina'],
        'VARIANCE_THRESHOLD': [0.03, 0.1, 0.3, 0.5],
        'DIVISION_THRESHOLD': [0.03, 0.1, 0.3, 0.5],
    }

    static_params = {
        'ITERATIONS': 200,
        'LOG_INTERVAL': 10,
        'SECONDS_LIMIT': 0,
        'LOG_SEC_INTERVAL': 0,
        'VALIDATION_FRACTION': 0.2,
        'TEST_FRACTION': 0.0,
        'MAX_DISCOVERIES': 256,
        'MAX_OUTGOING': 32,
        'BAND_THRESHOLD': 0.3,
        'MEDIAN_VARIANCE': False,
        'MAX_VARIANCE': False,
        'RELATIVE_VARIANCE': False,
        'ONLY_LEAF_VARIANCE': True,
        'ENABLE_IDENTITY_MAPPING': False,
        'STATIC_SUBSTRATE_DEPTH': 0,
    }

    def run_grid(grid):
        for params in ParameterGrid(grid):
            params['OUTPUT_ACTIVATION'] = 'Tanh' if params['DATASET'] == 'datasets/generated/retina' else 'Softmax'

            if params['METHOD'] == 'DES-HyperNEAT':
                if params['DATASET'].endswith('iris'):
                    params['INPUT_CONFIG'] = "[[[-1.0, 1.0], [-1.0, -1.0], [1.0, 1.0], [1.0, -1.0]]]"
                if params['DATASET'].endswith('wine'):
                    params['INPUT_CONFIG'] = "[[[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]]]"
                if params['DATASET'].endswith('retina'):
                    params['INPUT_CONFIG'] = "line"
            if params['METHOD'] == 'ES-HyperNEAT':
                if params['DATASET'].endswith('retina'):
                    params['INPUT_CONFIG'] = "[[-1.0, -0.5], [-0.33, -0.5], [-1.0, -1.0], [-0.33, -1.0], [0.33, -0.5], [1.0, -0.5], [0.33, -1.0], [1.0, -1.0]]"

            name = json.dumps(params)
            params.update(static_params)
            sheduler.create_job(BATCH, name, REPEATS, params)

    run_grid(param_grid)
