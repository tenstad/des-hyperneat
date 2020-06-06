from sklearn.model_selection import ParameterGrid
from src.scheduler import Scheduler
import json


def run():
    BATCH = 10
    REPEATS = 50
    sheduler = Scheduler()

    param_grid = {
        'METHOD': ['CPPN', 'HyperNEAT', 'ES-HyperNEAT'],
        'DATASET': ['datasets/generated/iris',
                    'datasets/generated/wine',
                    'datasets/generated/retina'],
        'HIDDEN_ACTIVATIONS': [
            'None',
            'Tanh Sigmoid',
            'None Linear Tanh OffsetGaussian Gaussian Sine',
            'None Tanh OffsetGaussian Gaussian Sine',
            'Linear Tanh OffsetGaussian Gaussian Sine',
            'Linear OffsetGaussian Gaussian Sine',
            'Tanh OffsetGaussian Gaussian Sine',
            'Tanh OffsetGaussian Gaussian Sine Sigmoid',
            'Tanh OffsetGaussian Gaussian Sine Step',
            'Tanh OffsetGaussian Gaussian Sine ReLU',
        ],
    }

    static_params = {
        'ITERATIONS': 0,
        'LOG_INTERVAL': 0,
        'SECONDS_LIMIT': 30,
        'LOG_SEC_INTERVAL': 2,
    }

    def run_grid(grid):
        for params in ParameterGrid(grid):
            params['OUTPUT_ACTIVATIONS'] = params['HIDDEN_ACTIVATIONS']

            if params['METHOD'] == 'CPPN':
                params['OUTPUT_ACTIVATIONS'] = 'Tanh' if params['DATASET'] == 'datasets/generated/retina' else 'Softmax'
            else:
                params['OUTPUT_ACTIVATION'] = 'Tanh' if params['DATASET'] == 'datasets/generated/retina' else 'Softmax'

            if params['METHOD'] == 'HyperNEAT' and params['DATASET'].endswith('retina'):
                params['INPUT_CONFIG'] = "[[-1.0, -0.5], [-0.33, -0.5], [-1.0, -1.0], [-0.33, -1.0], [0.33, -0.5], [1.0, -0.5], [0.33, -1.0], [1.0, -1.0]]"
                params['HIDDEN_LAYERS'] = "[[[-1.0, 0.0], [-0.33, 0.0], [0.33, 0.0], [1.0, 0.0]], [[-1.0, 0.5], [-0.33, 0.5], [0.33, 0.5], [1.0, 0.5]]]"
            if params['METHOD'] == 'ES-HyperNEAT':
                if params['DATASET'].endswith('retina'):
                    params['INPUT_CONFIG'] = "[[-1.0, -0.5], [-0.33, -0.5], [-1.0, -1.0], [-0.33, -1.0], [0.33, -0.5], [1.0, -0.5], [0.33, -1.0], [1.0, -1.0]]"
                params['VARIANCE_THRESHOLD'] = 0.03
                params['DIVISION_THRESHOLD'] = 0.03
                params['RELATIVE_VARIANCE'] = False
                params['MEDIAN_VARIANCE'] = False
                params['ONLY_LEAF_VARIANCE'] = True

            name = json.dumps(params)
            params.update(static_params)
            sheduler.create_job(BATCH, name, REPEATS, params)

    run_grid(param_grid)
