from sklearn.model_selection import ParameterGrid
from src.scheduler import Scheduler


def run():
    BATCH = 0
    REPEATS = 50
    sheduler = Scheduler()

    param_grid = {
        'METHOD': ['NEAT', 'CPPN', 'HyperNEAT'],
        'DATASET': ['datasets/generated/iris',
                    'datasets/generated/wine'],
        'SPECIES_TARGET': [5, 10, 20],
        'INITIAL_MUTATIONS': [100, 250, 500],
        'DROPOFF_AGE': [20, 100],
    }

    static_params = {
        'POPULATION_SIZE': 200,
        'ITERATIONS': 300,
        'LOG_INTERVAL': 15,
    }

    for params in ParameterGrid(param_grid):
        if params['METHOD'] in ('NEAT', 'HyperNEAT'):
            params['OUTPUT_ACTIVATION'] = 'Softmax'
        elif params['METHOD'] == 'CPPN':
            params['OUTPUT_ACTIVATIONS'] = 'Softmax'

        name = str(params)
        params.update(static_params)
        sheduler.create_job(BATCH, name, REPEATS, params)
