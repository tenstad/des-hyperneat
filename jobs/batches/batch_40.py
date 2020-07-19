from sklearn.model_selection import ParameterGrid
from src.scheduler import Scheduler
import json


def run():
    BATCH = 40
    REPEATS = 50
    sheduler = Scheduler()

    param_grid = {
        'METHOD': ['ES-HyperNEAT', 'DES-HyperNEAT'],
        'DATASET': ['datasets/generated/iris',
                    'datasets/generated/wine',
                    'datasets/generated/retina'],
        'ACT': ['Many', 'Few'],
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
        'BAND_THRESHOLD': 0.3,
        'MEDIAN_VARIANCE': False,
        'MAX_VARIANCE': False,
        'RELATIVE_VARIANCE': False,
        'ONLY_LEAF_VARIANCE': True,
        'VARIANCE_THRESHOLD': 0.03,
        'DIVISION_THRESHOLD': 0.03,
        'ENABLE_IDENTITY_MAPPING': False,
        'STATIC_SUBSTRATE_DEPTH': 0,
    }

    def run_grid(grid):
        for params in ParameterGrid(grid):
            if params['ACT'] == 'Many':
                params['HIDDEN_ACTIVATIONS'] = 'None Linear Step ReLU Sigmoid Exp Sigmoid Tanh Gaussian OffsetGaussian Sine Square Abs'
                params['OUTPUT_ACTIVATIONS'] = 'None Linear Step ReLU Sigmoid Exp Sigmoid Tanh Gaussian OffsetGaussian Sine Square Abs'
            else:
                params['HIDDEN_ACTIVATIONS'] = 'Tanh OffsetGaussian Gaussian Sine Sigmoid'
                params['OUTPUT_ACTIVATIONS'] = 'Tanh OffsetGaussian Gaussian Sine Sigmoid'

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
