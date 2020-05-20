from sklearn.model_selection import ParameterGrid
from src.scheduler import Scheduler


def run():
    BATCH = 0
    REPEATS = 20
    sheduler = Scheduler()

    param_grid = {
        'METHOD': ['NEAT', 'CPPN'],
        'DATASET': ['datasets/generated/iris',
                    'datasets/generated/wine'],
        'SPECIES_TARGET': [10, 20, 40],
        'INITIAL_MUTATIONS': [100, 300],
        'DROPOFF_AGE': [20, 50],
    }

    static_params = {
        'POPULATION_SIZE': 200,
        'ITERATIONS': 300,
        'LOG_INTERVAL': 15,
    }

    for params in ParameterGrid(param_grid):
        params.update(static_params)
        sheduler.create_job(BATCH, str(params), REPEATS, params)
