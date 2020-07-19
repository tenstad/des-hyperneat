from sklearn.model_selection import ParameterGrid
from src.scheduler import Scheduler
import json

iris_grids = {
    'line': "line",
    'grid': "[[[-1.0, 1.0], [-1.0, -1.0], [1.0, 1.0], [1.0, -1.0]]]",
    'rotated_grid': "[[[-0.92, -0.38], [-0.38, 0.92], [0.38, -0.92], [0.92, 0.38]]]",
    'two': "[[[-1.0, 0.0], [1.0, 0.0]], [[-1.0, 0.0], [1.0, 0.0]]]",
    'individual': "[[[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]]]",
}

wine_grids = {
    'line': "line",
    'single': "[[[0.0, 0.0], [-1.0, -1.0], [-0.33, -0.8], [0.33, -0.8], [1.0, -1.0], [-0.8, -0.33], [-0.8, 0.33], [0.8, -0.33], [0.8, 0.33], [-1.0, 1.0], [-0.33, 0.8], [0.33, 0.8], [1.0, 1.0]]]",
    'grids': "[[[0.0, 0.0], [-1.0, 1.0], [1.0, 1.0], [-1.0, -1.0], [1.0, -1.0]], [[-1.0, 1.0], [1.0, 1.0], [-1.0, -1.0], [1.0, -1.0]], [[-1.0, 1.0], [1.0, 1.0], [-1.0, -1.0], [1.0, -1.0]]]",
    'rotated_grids': "[[[0.0, 0.0], [-0.92, -0.38], [-0.38, 0.92], [0.38, -0.92], [0.92, 0.38]], [[-0.92, -0.38], [-0.38, 0.92], [0.38, -0.92], [0.92, 0.38]], [[-0.92, -0.38], [-0.38, 0.92], [0.38, -0.92], [0.92, 0.38]]]",
    'individual': "[[[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]]]",
}

retina_grids = {
    'line': 'line',
    'single': "[[[-1.0, 0.5], [-0.33, 0.5], [-1.0, -0.5], [-0.33, -0.5], [0.33, 0.5], [1.0, 0.5], [0.33, -0.5], [1.0, -0.5]]]",
    'grids': "[[[-1.0, 1.0], [1.0, 1.0], [-1.0, -1.0], [1.0, -1.0]], [[-1.0, 1.0], [1.0, 1.0], [-1.0, -1.0], [1.0, -1.0]]]",
    'lines': "[[[-1.0, 0.0], [-0.33, 0.0], [0.33, 0.0], [1.0, 0.0]], [[-1.0, 0.0], [-0.33, 0.0], [0.33, 0.0], [1.0, 0.0]]]",
    'rotated_grids': "[[[-0.92, -0.38], [-0.38, 0.92], [0.38, -0.92], [0.92, 0.38]], [[-0.92, -0.38], [-0.38, 0.92], [0.38, -0.92], [0.92, 0.38]]]",
    'individual': "[[[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]], [[0.0, 0.0]]]",
}


def run():
    BATCH = 44
    REPEATS = 100
    sheduler = Scheduler()

    param_grid_1 = {
        'METHOD': ['DES-HyperNEAT'],
        'DATASET': ['datasets/generated/iris'],
        'I_CONF': iris_grids.keys(),
    }

    param_grid_2 = {
        'METHOD': ['DES-HyperNEAT'],
        'DATASET': ['datasets/generated/wine'],
        'I_CONF': wine_grids.keys(),
    }

    param_grid_3 = {
        'METHOD': ['DES-HyperNEAT'],
        'DATASET': ['datasets/generated/retina'],
        'I_CONF': retina_grids.keys(),
    }

    static_params = {
        'ITERATIONS': 0,
        'LOG_INTERVAL': 0,
        'SECONDS_LIMIT': 300,
        'LOG_SEC_INTERVAL': 60,
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
            params['OUTPUT_ACTIVATION'] = 'Tanh' if params['DATASET'] == 'datasets/generated/retina' else 'Softmax'

            if params['DATASET'].endswith('iris'):
                params['INPUT_CONFIG'] = iris_grids[params['I_CONF']]
            if params['DATASET'].endswith('wine'):
                params['INPUT_CONFIG'] = wine_grids[params['I_CONF']]
            if params['DATASET'].endswith('retina'):
                params['INPUT_CONFIG'] = retina_grids[params['I_CONF']]

            name = json.dumps(params)
            params.update(static_params)
            sheduler.create_job(BATCH, name, REPEATS, params)

    run_grid(param_grid_1)
    run_grid(param_grid_2)
    run_grid(param_grid_3)
