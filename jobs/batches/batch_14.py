from sklearn.model_selection import ParameterGrid
from src.scheduler import Scheduler
import json


def run():
    BATCH = 14
    REPEATS = 50
    sheduler = Scheduler()

    param_grid = {
        'METHOD': ['ES-HyperNEAT', 'ES-HyperNEAT-default', 'SiDES-HyperNEAT', 'CoDES-HyperNEAT', 'DES-HyperNEAT'],
        'DATASET': ['datasets/generated/iris',
                    'datasets/generated/wine',
                    'datasets/generated/retina'],
    }

    static_params = {
        'ITERATIONS': 600,
        'LOG_INTERVAL': 2,
        'VALIDATION_FRACTION': 0.2,
        'TEST_FRACTION': 0.0,
    }

    def run_grid(grid):
        for params in ParameterGrid(grid):
            params['OUTPUT_ACTIVATION'] = 'Tanh' if params['DATASET'] == 'datasets/generated/retina' else 'Softmax'

            if params['DATASET'].endswith('retina'):
                if params['METHOD'].startswith('ES-HyperNEAT'):
                    params['INPUT_CONFIG'] = "[[-1.0, -0.5], [-0.33, -0.5], [-1.0, -1.0], [-0.33, -1.0], [0.33, -0.5], [1.0, -0.5], [0.33, -1.0], [1.0, -1.0]]"
                else:
                    params['INPUT_CONFIG'] = "[[[-1.0, 0.5], [-0.33, 0.5], [-1.0, -0.5], [-0.33, -0.5], [0.33, 0.5], [1.0, 0.5], [0.33, -0.5], [1.0, -0.5]]]"

            if params['METHOD'] == 'ES-HyperNEAT-default':
                params['METHOD'] = 'ES-HyperNEAT'
                params['VARIANCE_THRESHOLD'] = 0.03
                params['DIVISION_THRESHOLD'] = 0.03
                params['RELATIVE_VARIANCE'] = False
                params['MEDIAN_VARIANCE'] = False
                params['ONLY_LEAF_VARIANCE'] = True

            name = json.dumps(params)
            params.update(static_params)
            sheduler.create_job(BATCH, name, REPEATS, params)

    run_grid(param_grid)
