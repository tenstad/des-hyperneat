from src.scheduler import Scheduler


def run():
    BATCH = 1
    REPEATS = 30
    sheduler = Scheduler()

    for dataset in ('iris', 'wine'):
        for method in ('CPPN', 'HyperNEAT', 'SiDES-HyperNEAT'):
            sheduler.create_job(BATCH, f'{method}_{dataset}', REPEATS,
                                {
                                    'METHOD': method,
                                    'ITERATIONS': 500,
                                    'DATASET': f'datasets/generated/{dataset}'
                                })

    for dataset in ('iris', 'wine'):
        method = 'DES-HyperNEAT'
        sheduler.create_job(BATCH, f'{method}_{dataset}_custom_var', REPEATS,
                            {
                                'METHOD': method,
                                'ITERATIONS': 500,
                                'DATASET': f'datasets/generated/{dataset}',
                                'VARIANCE_THRESHOLD': 0.2,
                                'DIVISION_THRESHOLD': 0.2,
                                'MAX_VARIANCE': True,
                                'RELATIVE_VARIANCE': True,
                                'MEDIAN_VARIANCE': True,
                                'ONLY_LEAF_VARIANCE': False,
                            })
        sheduler.create_job(BATCH, f'{method}_{dataset}_custom_var_04', REPEATS,
                            {
                                'METHOD': method,
                                'ITERATIONS': 500,
                                'DATASET': f'datasets/generated/{dataset}',
                                'VARIANCE_THRESHOLD': 0.4,
                                'DIVISION_THRESHOLD': 0.4,
                                'MAX_VARIANCE': True,
                                'RELATIVE_VARIANCE': True,
                                'MEDIAN_VARIANCE': True,
                                'ONLY_LEAF_VARIANCE': False,
                            })
        sheduler.create_job(BATCH, f'{method}_{dataset}_custom_var_mutate_all', REPEATS,
                            {
                                'METHOD': method,
                                'ITERATIONS': 500,
                                'DATASET': f'datasets/generated/{dataset}',
                                'VARIANCE_THRESHOLD': 0.2,
                                'DIVISION_THRESHOLD': 0.2,
                                'MAX_VARIANCE': True,
                                'RELATIVE_VARIANCE': True,
                                'MEDIAN_VARIANCE': True,
                                'ONLY_LEAF_VARIANCE': False,
                                'MUTATE_ALL_COMPONENTS': True,
                            })

        method = 'ES-HyperNEAT'
        sheduler.create_job(BATCH, f'{method}_{dataset}_custom_var', REPEATS,
                            {
                                'METHOD': method,
                                'ITERATIONS': 500,
                                'DATASET': f'datasets/generated/{dataset}',
                                'VARIANCE_THRESHOLD': 0.2,
                                'DIVISION_THRESHOLD': 0.2,
                                'MAX_VARIANCE': True,
                                'RELATIVE_VARIANCE': True,
                                'MEDIAN_VARIANCE': True,
                                'ONLY_LEAF_VARIANCE': False,
                            })
        sheduler.create_job(BATCH, f'{method}_{dataset}_custom_var_04', REPEATS,
                            {
                                'METHOD': method,
                                'ITERATIONS': 500,
                                'DATASET': f'datasets/generated/{dataset}',
                                'VARIANCE_THRESHOLD': 0.4,
                                'DIVISION_THRESHOLD': 0.4,
                                'MAX_VARIANCE': True,
                                'RELATIVE_VARIANCE': True,
                                'MEDIAN_VARIANCE': True,
                                'ONLY_LEAF_VARIANCE': False,
                            })
        sheduler.create_job(BATCH, f'{method}_{dataset}_default', REPEATS,
                            {
                                'METHOD': method,
                                'ITERATIONS': 500,
                                'DATASET': f'datasets/generated/{dataset}',
                                'VARIANCE_THRESHOLD': 0.03,
                                'DIVISION_THRESHOLD': 0.03,
                                'MAX_VARIANCE': False,
                                'RELATIVE_VARIANCE': False,
                                'MEDIAN_VARIANCE': False,
                                'ONLY_LEAF_VARIANCE': True,
                            })

    method = 'DES-HyperNEAT'
    dataset = 'wine'
    sheduler.create_job(BATCH, f'{method}_{dataset}_custom_var_04_io', REPEATS,
                        {
                            'METHOD': method,
                            'ITERATIONS': 500,
                            'DATASET': f'datasets/generated/{dataset}',
                            'VARIANCE_THRESHOLD': 0.4,
                            'DIVISION_THRESHOLD': 0.4,
                            'MAX_VARIANCE': True,
                            'RELATIVE_VARIANCE': True,
                            'MEDIAN_VARIANCE': True,
                            'ONLY_LEAF_VARIANCE': False,
                            'INPUT_CONFIG': '[[(-1.0, -1.0), (0.5, -0.5), (-0.5, 0.5), (1.0, 1.0), (0.0, 0.0)], [(-1.0, -1.0), (0.5, -0.5), (-0.5, 0.5), (1.0, 1.0)], [(-1.0, -1.0), (0.5, -0.5), (-0.5, 0.5), (1.0, 1.0)]'
                        })

    dataset = 'iris'
    sheduler.create_job(BATCH, f'{method}_{dataset}_custom_var_04_io', REPEATS,
                        {
                            'METHOD': method,
                            'ITERATIONS': 500,
                            'DATASET': f'datasets/generated/{dataset}',
                            'VARIANCE_THRESHOLD': 0.4,
                            'DIVISION_THRESHOLD': 0.4,
                            'MAX_VARIANCE': True,
                            'RELATIVE_VARIANCE': True,
                            'MEDIAN_VARIANCE': True,
                            'ONLY_LEAF_VARIANCE': False,
                            'INPUT_CONFIG': '[[(-1.0, -1.0), (-0.25, 1.0), (1.0, -0.25)]]'
                        })
