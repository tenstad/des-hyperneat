from src.scheduler import Scheduler


def run():
    BATCH = 2
    REPEATS = 20

    WINE_INPUT = '[[[-1.0, -1.0], [0.5, -0.5], [-0.5, 0.5], [1.0, 1.0], [0.0, 0.0]], [[-1.0, -1.0], [0.5, -0.5], [-0.5, 0.5], [1.0, 1.0]], [[-1.0, -1.0], [0.5, -0.5], [-0.5, 0.5], [1.0, 1.0]]]'
    IRIS_INPUT = '[[[-1.0, 0.0], [0.0, 0.0], [1.0, 0.0]]]'

    sheduler = Scheduler()

    for method in ('CPPN', 'HyperNEAT', 'ES-HyperNEAT', 'DES-HyperNEAT'):
        for (dataset, input_conf) in (('iris', IRIS_INPUT), ('wine', WINE_INPUT)):
            sheduler.create_job(BATCH, f'{method}_{dataset}_1000_150', REPEATS,
                                {
                                    'METHOD': method,
                                    'ITERATIONS': 1000,
                                    'POPULATION_SIZE': 150,
                                    'DATASET': f'datasets/generated/{dataset}',
                                    'VARIANCE_THRESHOLD': 0.4,
                                    'DIVISION_THRESHOLD': 0.4,
                                    'MAX_VARIANCE': True,
                                    'RELATIVE_VARIANCE': True,
                                    'MEDIAN_VARIANCE': True,
                                    'ONLY_LEAF_VARIANCE': False,
                                    'INPUT_CONFIG': input_conf
                                })
            sheduler.create_job(BATCH, f'{method}_{dataset}_250_600', REPEATS,
                                {
                                    'METHOD': method,
                                    'ITERATIONS': 250,
                                    'POPULATION_SIZE': 600,
                                    'DATASET': f'datasets/generated/{dataset}',
                                    'VARIANCE_THRESHOLD': 0.4,
                                    'DIVISION_THRESHOLD': 0.4,
                                    'MAX_VARIANCE': True,
                                    'RELATIVE_VARIANCE': True,
                                    'MEDIAN_VARIANCE': True,
                                    'ONLY_LEAF_VARIANCE': False,
                                    'INPUT_CONFIG': input_conf
                                })
