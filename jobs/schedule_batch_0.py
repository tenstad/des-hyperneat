from src.scheduler import Scheduler

sheduler = Scheduler()
BATCH = 0

for dataset in ('iris', 'wine'):
    for method in ('CPPN', 'HyperNEAT', 'CoDES-HyperNEAT', 'SiDES-HyperNEAT'):
        sheduler.create_job(BATCH, f'{method}_{dataset}', 50,
                            {
                                'METHOD': method,
                                'ITERATIONS': 1000,
                                'DATASET': f'datasets/generated/{dataset}'
                            })

for dataset in ('iris', 'wine'):
    for method in ('CPPN', 'HyperNEAT'):
        sheduler.create_job(BATCH, f'{method}_{dataset}', 50,
                            {
                                'METHOD': method,
                                'ITERATIONS': 500,
                                'DATASET': f'datasets/generated/{dataset}'
                            })

for dataset in ('iris', 'wine'):
    for method in ('ES-HyperNEAT', 'DES-HyperNEAT'):
        sheduler.create_job(BATCH, f'{method}_{dataset}_custom_var_limited', 50,
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
        sheduler.create_job(BATCH, f'{method}_{dataset}_default_limited', 50,
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

for dataset in ('iris', 'wine'):
    for method in ('ES-HyperNEAT', 'DES-HyperNEAT'):
        sheduler.create_job(BATCH,  f'{method}_{dataset}_custom_var_unlimited', 50,
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
                                'MAX_DISCOVERIES': 0,
                                'MAX_OUTGOING': 0,
                            })
        sheduler.create_job(BATCH, f'{method}_{dataset}_default_unlimited', 50,
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
                                'MAX_DISCOVERIES': 0,
                                'MAX_OUTGOING': 0,
                            })
