from sklearn.model_selection import ParameterGrid
from src.scheduler import Scheduler
import json


def run():
    BATCH = 42
    REPEATS = 100
    sheduler = Scheduler()

    param_grid = {
        'METHOD': ['DES-HyperNEAT'],
        'DATASET': ['datasets/generated/iris',
                    'datasets/generated/wine',
                    'datasets/generated/retina'],
        'STATIC_SUBSTRATE_DEPTH': [-1, 0],
    }

    static_params = {
        'ITERATIONS': 0,
        'LOG_INTERVAL': 0,
        'SECONDS_LIMIT': 800,
        'LOG_SEC_INTERVAL': 12,
        'VALIDATION_FRACTION': 0.2,
        'TEST_FRACTION': 0.0,
        'MAX_DISCOVERIES': 256,
        'MAX_OUTGOING': 32,
        'BAND_THRESHOLD': 0.3,
        'MEDIAN_VARIANCE': False,
        'MAX_VARIANCE': False,
        'RELATIVE_VARIANCE': False,
        'ONLY_LEAF_VARIANCE': True,
        'VARIANCE_THRESHOLD': 0.5,
        'DIVISION_THRESHOLD': 0.03,
        'ENABLE_IDENTITY_MAPPING': False,
        'HIDDEN_ACTIVATIONS': 'None Linear Step ReLU Sigmoid Exp Sigmoid Tanh Gaussian OffsetGaussian Sine Square Abs',
        'OUTPUT_ACTIVATIONS': 'None Linear Step ReLU Sigmoid Exp Sigmoid Tanh Gaussian OffsetGaussian Sine Square Abs',
    }

    def run_grid(grid):
        for params in ParameterGrid(grid):
            params['OUTPUT_ACTIVATION'] = 'Tanh' if params['DATASET'] == 'datasets/generated/retina' else 'Softmax'

            if params['DATASET'].endswith('retina'):
                params['INPUT_CONFIG'] = "[[[-1.0, 0.0], [-0.33, 0.0], [0.33, 0.0], [1.0, 0.0]], [[-1.0, 0.0], [-0.33, 0.0], [0.33, 0.0], [1.0, 0.0]]]"
                params['OUTPUT_CONFIG'] = "[[[0.0, 0.0]], [[0.0, 0.0]]]"
            if params['DATASET'].endswith('iris'):
                params['INPUT_CONFIG'] = "[[[-1.0, 1.0], [-1.0, -1.0], [1.0, 1.0], [1.0, -1.0]]]"
            if params['DATASET'].endswith('wine'):
                params['INPUT_CONFIG'] = "[[[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]]]"

            name = json.dumps(params)
            params.update(static_params)
            sheduler.create_job(BATCH, name, REPEATS, params)

    run_grid(param_grid)