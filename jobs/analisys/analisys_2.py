from src.client import get_database
import os
import numpy as np
from matplotlib import pyplot as plt
from sklearn.model_selection import ParameterGrid
import json
from multiprocessing import Pool, Queue, Manager
from bson.objectid import ObjectId
import time


def analyse(batch):
    print(f'Analyzing batch {batch}')
    start_time = time.time()

    client, db = get_database()
    ids = list(db.jobs.find({'batch': int(batch)}, {'_id': 1}))

    num_jobs = len(ids)
    if num_jobs == 0:
        print(f'Batch does not exist: {batch}')
        return
    print(f'Found {num_jobs} jobs')

    q = Manager().Queue()
    args = [(_id, batch, q) for _id in ids]
    job = db.jobs.find(args[0][0])[0]
    client.close()

    with Pool(16) as p:
        p.map(analyse_job, args)

    param_a = 'DEPTH'
    param_b = 'DATASET'
    results = {}
    results_len = 0
    while not q.empty():
        d, li = q.get()
        results_len = len(li)
        results[(d[param_a], d[param_b])] = li

    values_a = set([a for (a, _) in results.keys()])
    values_b = set([b for (_, b) in results.keys()])
    data = np.zeros((len(values_a), len(values_b), results_len))
    for i, a in enumerate(values_a):
        for j, b in enumerate(values_b):
            data[i, j, :] = results[(a, b)]

    lines = [(param_a, *values_b)]
    for i, a in enumerate(values_a):
        for n in range(results_len):
            values = [data[i, j, n] for j in range(len(values_b))]
            lines.append((a, *values))

    with open('data_raw.txt', 'w') as f:
        f.write('\n'.join([' '.join(list(map(str, line))) for line in lines]))

    runtime = time.time() - start_time
    print(f'Analysis complete in {runtime} seconds')


def analyse_job(args):
    job, batch, q = args

    client, db = get_database()
    job = db.jobs.find(job)[0]
    client.close()

    try:
        os.mkdir(f'jobs/analisys/plots/batch_{batch}/')
    except FileExistsError:
        pass

    job_name = job['name']
    print(job_name)

    repeats = int(job['scheduled'])
    population_size = int(job['config']['population']['population_size'])

    evo_cfg = job['config']['evolution']
    if evo_cfg['iterations'] != 0:
        data_points = int(evo_cfg['iterations'] /
                          evo_cfg['log_interval']) + 1
    else:
        data_points = int(evo_cfg['seconds_limit'] /
                          evo_cfg['log_sec_interval']) + 1

    fitnesses = np.zeros((repeats, data_points, population_size),
                         dtype=np.float64)
    training_accuracy = np.zeros((repeats, data_points, population_size),
                                 dtype=np.float64)
    validation_accuracy = np.zeros((repeats, data_points, population_size),
                                   dtype=np.float64)
    validation_fitness = np.zeros((repeats, data_points, population_size),
                                  dtype=np.float64)

    if evo_cfg['iterations'] != 0:
        for i, log in enumerate(db.logs.find({'job_id': job['_id']}, {'_id': 0, 'events.organisms.fitness': 1, 'events.organisms.evaluation': 1})):
            for j, event in enumerate(log['events']):
                fitnesses[i, j] = [organism['fitness']
                                   for organism in event['organisms']]
                if 'validation_accuracy' in event['organisms'][0]['evaluation']:
                    # hidden_substrates[i, j] = [sum([1 if c > 0 else 0 for c in organism['phenotype']['hidden_substrate_node_counts']])
                    #                           for organism in event['organisms']]
                    validation_accuracy[i, j] = [organism['evaluation']['validation_accuracy']
                                                 for organism in event['organisms']]
                    validation_fitness[i, j] = [organism['evaluation']['validation_fitness']
                                                for organism in event['organisms']]
                    training_accuracy[i, j] = [organism['evaluation']['training_accuracy']
                                               for organism in event['organisms']]
                elif 'validation_accuracy' in event['organisms'][0]['evaluation'][0]:
                    # hidden_substrates[i, j] = [np.mean([sum([1 if c > 0 else 0 for c in p['hidden_substrate_node_counts']]) for p in organism['phenotype']])
                    #                           for organism in event['organisms']]
                    validation_accuracy[i, j] = [np.mean([e['validation_accuracy'] for e in organism['evaluation']])
                                                 for organism in event['organisms']]
                    validation_fitness[i, j] = [np.mean([e['validation_fitness'] for e in organism['evaluation']])
                                                for organism in event['organisms']]
                    training_accuracy[i, j] = [np.mean([e['training_accuracy'] for e in organism['evaluation']])
                                               for organism in event['organisms']]
    else:
        for i, log in enumerate(db.logs.find({'job_id': job['_id']})):
            event_times = [event['event_time'] for event in log['events']]
            start_time = event_times[0]
            event_times = [e - start_time for e in event_times]
            event_times = [e.seconds * 1000 +
                           e.microseconds / 1000 for e in event_times]

            for j in range(data_points):
                target_millis = evo_cfg['log_sec_interval'] * j * 1000
                for k, millis in list(enumerate(event_times))[:: -1]:
                    if millis <= target_millis:
                        fitnesses[i][j] = [organism['fitness']
                                           for organism in log['events'][k]['organisms']]
                        if 'validation_accuracy' in log['events'][k]['organisms'][0]['evaluation']:
                            # hidden_substrates[i, j] = [organism['phenotype']['hidden_substrates']
                            #                           for organism in log['events'][k]['organisms']]
                            validation_accuracy[i][j] = [organism['evaluation']['validation_accuracy']
                                                         for organism in log['events'][k]['organisms']]
                            validation_fitness[i][j] = [organism['evaluation']['validation_fitness']
                                                        for organism in log['events'][k]['organisms']]
                            training_accuracy[i][j] = [organism['evaluation']['training_accuracy']
                                                       for organism in log['events'][k]['organisms']]
                        elif 'validation_accuracy' in log['events'][k]['organisms'][0]['evaluation'][0]:
                            # hidden_substrates[i, j] = [np.mean([p['hidden_substrates'] for p in organism['phenotype']])
                            #                           for organism in log['events'][k]['organisms']]
                            validation_accuracy[i][j] = [np.mean([e['validation_accuracy'] for e in organism['evaluation']])
                                                         for organism in log['events'][k]['organisms']]
                            validation_fitness[i][j] = [np.mean([e['validation_fitness'] for e in organism['evaluation']])
                                                        for organism in log['events'][k]['organisms']]
                            training_accuracy[i][j] = [np.mean([e['training_accuracy'] for e in organism['evaluation']])
                                                       for organism in log['events'][k]['organisms']]
                        break

    job_params = job['parameters']

    max_i = validation_fitness.argmax(axis=2)
    max_validation_fitness = validation_fitness.max(axis=2)
    max_validation_accuracy = np.array([[a[b] for a, b in zip(c, d)] for c, d in zip(
        validation_accuracy, max_i)])

    fit = max_validation_fitness[:, -1]

    q.put(({
        'DATASET': job_params['DATASET'],
        'DEPTH': job_params['ENABLE_IDENTITY_MAPPING']
    }, fit))
