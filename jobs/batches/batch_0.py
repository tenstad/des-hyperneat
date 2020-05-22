from sklearn.model_selection import ParameterGrid
from src.scheduler import Scheduler
import json


def run():
    BATCH = 0
    REPEATS = 50
    sheduler = Scheduler()

    param_grid = {
        'METHOD': ['NEAT', 'CPPN', 'HyperNEAT'],
        'DATASET': ['datasets/generated/iris',
                    'datasets/generated/wine',
                    'datasets/generated/retina'],
        'SPECIES_TARGET': [5, 10, 20],
        'INITIAL_MUTATIONS': [100, 250, 500],
        'DROPOFF_AGE': [20, 100],
    }

    param_grid_non_adaptive = {
        'METHOD': ['NEAT', 'CPPN', 'HyperNEAT'],
        'DATASET': ['datasets/generated/iris',
                    'datasets/generated/wine',
                    'datasets/generated/retina'],
        'ADAPTIVE_SPECIATION_THRESHOLD': [False],
        'INITIAL_MUTATIONS': [100, 250, 500],
        'DROPOFF_AGE': [20, 100],
    }

    static_params = {
        'POPULATION_SIZE': 200,
        'ITERATIONS': 300,
        'LOG_INTERVAL': 15,
    }

    def run_grid(grid):
        for params in ParameterGrid(grid):
            if params['METHOD'] in ('NEAT', 'HyperNEAT'):
                params['OUTPUT_ACTIVATION'] = 'Softmax'
            elif params['METHOD'] == 'CPPN':
                params['OUTPUT_ACTIVATIONS'] = 'Softmax'
            if params['METHOD'] == 'NEAT':
                params['ADD_BIAS_INPUT'] = True
            if params['METHOD'] == 'NEAT' and params['DATASET'] == 'datasets/generated/retina':
                params['INPUT_CONFIG'] = "[[-1.0, -0.5], [-0.33, -0.5], [-1.0, -1.0], [-0.33, -1.0], [0.33, -0.5], [1.0, -0.5], [0.33, -1.0], [1.0, -1.0]]"
                params['HIDDEN_LAYERS'] = "[[[-1.0, 0.0], [-0.33, 0.0], [0.33, 0.0], [1.0, 0.0]], [[-1.0, 0.5], [-0.33, 0.5], [0.33, 0.5], [1.0, 0.5]]]"
            name = json.dumps(params)
            params.update(static_params)
            sheduler.create_job(BATCH, name, REPEATS, params)

    run_grid(param_grid)
    run_grid(param_grid_non_adaptive)
