from src.scheduler import Scheduler


def run():
    BATCH = 3
    REPEATS = 20

    WINE_INPUT = '[[[-1.0, -1.0], [0.5, -0.5], [-0.5, 0.5], [1.0, 1.0], [0.0, 0.0]], [[-1.0, -1.0], [0.5, -0.5], [-0.5, 0.5], [1.0, 1.0]], [[-1.0, -1.0], [0.5, -0.5], [-0.5, 0.5], [1.0, 1.0]]]'

    sheduler = Scheduler()

    dataset = 'wine'
    for method in ('CPPN', 'HyperNEAT', 'ES-HyperNEAT', 'DES-HyperNEAT'):
        sheduler.create_job(BATCH, f'{method}_{dataset}', REPEATS,
                            {
                                'METHOD': method,
                                'ITERATIONS': 0,
                                'SECONDS_LIMIT': 300,
                                'LOG_INTERVAL': 0,
                                'LOG_SEC_INTERVAL': 3,
                                'DATASET': f'datasets/generated/{dataset}',
                                'VARIANCE_THRESHOLD': 0.4,
                                'DIVISION_THRESHOLD': 0.4,
                                'MAX_VARIANCE': True,
                                'RELATIVE_VARIANCE': True,
                                'MEDIAN_VARIANCE': True,
                                'ONLY_LEAF_VARIANCE': False,
                                'INPUT_CONFIG': WINE_INPUT
                            })

    method = 'DES-HyperNEAT'
    sheduler.create_job(BATCH, f'{method}_{dataset}_limited', REPEATS,
                        {
                            'METHOD': method,
                            'ITERATIONS': 0,
                            'SECONDS_LIMIT': 300,
                            'LOG_INTERVAL': 0,
                            'LOG_SEC_INTERVAL': 3,
                            'DATASET': f'datasets/generated/{dataset}',
                            'VARIANCE_THRESHOLD': 0.4,
                            'DIVISION_THRESHOLD': 0.4,
                            'MAX_VARIANCE': True,
                            'RELATIVE_VARIANCE': True,
                            'MEDIAN_VARIANCE': True,
                            'ONLY_LEAF_VARIANCE': False,
                            'INPUT_CONFIG': WINE_INPUT,
                            'MAX_DISCOVERIES': 256,
                            'MAX_OUTGOING': 8,
                        })
