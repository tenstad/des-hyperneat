from sklearn.model_selection import ParameterGrid
from src.scheduler import Scheduler
import json


def run():
    BATCH = 14
    REPEATS = 50
    sheduler = Scheduler()

    param_grid = {
        'METHOD': ['SiDES-HyperNEAT', 'CoDES-HyperNEAT', 'DES-HyperNEAT'],
        'DATASET': ['datasets/generated/iris',
                    'datasets/generated/wine',
                    'datasets/generated/retina'],
    }

    static_params = {
        'ITERATIONS': 600,
        'LOG_INTERVAL': 6,
        'VALIDATION_FRACTION': 0.2,
        'TEST_FRACTION': 0.0,
        'MAX_DISCOVERIES': 128,
        'MAX_OUTGOING': 16,
    }

    def run_grid(grid):
        for params in ParameterGrid(grid):
            params['OUTPUT_ACTIVATION'] = 'Tanh' if params['DATASET'] == 'datasets/generated/retina' else 'Softmax'

            if params['DATASET'].endswith('retina'):
                params['INPUT_CONFIG'] = "[[[-1.0, 0.5], [-0.33, 0.5], [-1.0, -0.5], [-0.33, -0.5], [0.33, 0.5], [1.0, 0.5], [0.33, -0.5], [1.0, -0.5]]]"

            name = json.dumps(params)
            params.update(static_params)
            sheduler.create_job(BATCH, name, REPEATS, params)

    run_grid(param_grid)
