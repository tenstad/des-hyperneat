from sklearn.model_selection import ParameterGrid
from src.scheduler import Scheduler
import json


def run():
    BATCH = 71
    REPEATS = 100
    sheduler = Scheduler()

    param_grid = {
        'METHOD': ['ES-HyperNEAT'],
        'DATASET': ['datasets/generated/iris',
                    'datasets/generated/wine',
                    'datasets/generated/retina'],
        'DUR': [False, True],
    }

    static_params = {
        'VALIDATION_FRACTION': 0.2,
        'TEST_FRACTION': 0.0,
        'MAX_DISCOVERIES': 256,
        'MAX_OUTGOING': 32,
        'MEDIAN_VARIANCE': False,
        'MAX_VARIANCE': False,
        'RELATIVE_VARIANCE': False,
        'ONLY_LEAF_VARIANCE': True,
        'HIDDEN_ACTIVATIONS': 'None Linear Step ReLU Sigmoid Sigmoid Tanh Gaussian OffsetGaussian Sine Square',
        'OUTPUT_ACTIVATIONS': 'None Linear Step ReLU Sigmoid Sigmoid Tanh Gaussian OffsetGaussian Sine Square',
    }

    des_params = {
        'VARIANCE_THRESHOLD': 0.5,
        'DIVISION_THRESHOLD': 0.05,
        'BAND_THRESHOLD': 0.0,
        'ENABLE_IDENTITY_MAPPING': False,
        'STATIC_SUBSTRATE_DEPTH': 0,
        'LAYOUT_ADD_NODE_PROBABILITY': 0.025,
        'LAYOUT_ADD_LINK_PROBABILITY': 0.4,
        'ADD_NODE_PROBABILITY': 0.025,
        'ADD_LINK_PROBABILITY': 0.4,
        'LAYOUT_REMOVE_NODE_PROBABILITY': 0.001,
        'LAYOUT_REMOVE_LINK_PROBABILITY': 0.05,
    }

    es_params = {
        'VARIANCE_THRESHOLD': 0.03,
        'DIVISION_THRESHOLD': 0.03,
        'BAND_THRESHOLD': 0.3,
        'ADD_LINK_PROBABILITY': 0.4,
        'ADD_NODE_PROBABILITY': 0.03,
    }

    def run_grid(grid):
        for params in ParameterGrid(grid):
            if params['DUR']:
                params['ITERATIONS'] = 600
                params['LOG_INTERVAL'] = 6
                params['SECONDS_LIMIT'] = 0
                params['LOG_SEC_INTERVAL'] = 0
            else:
                params['ITERATIONS'] = 0
                params['LOG_INTERVAL'] = 0
                params['SECONDS_LIMIT'] = 1200
                params['LOG_SEC_INTERVAL'] = 12
            name = json.dumps(params)
            params.update(static_params)

            params['OUTPUT_ACTIVATION'] = 'Tanh' if params['DATASET'] == 'datasets/generated/retina' else 'Softmax'

            if params['METHOD'] == 'DES-HyperNEAT':
                params.update(des_params)
                if params['DATASET'].endswith('iris'):
                    params['INPUT_CONFIG'] = "[[[-1.0, 1.0], [-1.0, -1.0], [1.0, 1.0], [1.0, -1.0]]]"
                if params['DATASET'].endswith('wine'):
                    params['INPUT_CONFIG'] = "[[[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]]]"
                if params['DATASET'].endswith('retina'):
                    params['INPUT_CONFIG'] = "[[[-1.0, 1.0], [1.0, 1.0], [-1.0, -1.0], [1.0, -1.0]], [[-1.0, 1.0], [1.0, 1.0], [-1.0, -1.0], [1.0, -1.0]]]"
                    params['OUTPUT_CONFIG'] = "[[[0.0, 0.0]], [[0.0, 0.0]]]"
            if params['METHOD'] == 'ES-HyperNEAT':
                params.update(es_params)
                if params['DATASET'].endswith('retina'):
                    params['INPUT_CONFIG'] = "[[-1.0, -0.5], [-0.33, -0.5], [-1.0, -1.0], [-0.33, -1.0], [0.33, -0.5], [1.0, -0.5], [0.33, -1.0], [1.0, -1.0]]"

            sheduler.create_job(BATCH, name, REPEATS, params)

    run_grid(param_grid)
