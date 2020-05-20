from src.client import get_client
import os
import numpy as np
from matplotlib import pyplot as plt
from datetime import datetime


def analyse_time(batch):
    client = get_client()

    db = getattr(client, os.environ.get(
        'DATABASE', 'deshyperneat'))
    jobs = db.jobs.find({'batch': int(batch)})

    first = True
    results = []

    for job in jobs:
        if first:
            first = False
            print(f'Analyzing batch {batch}')
            if not f'batch_{batch}' in os.listdir('jobs/analisys/plots/'):
                os.mkdir(f'jobs/analisys/plots/batch_{batch}/')

        job_name = job['name']
        print(job_name)

        repeats = int(job['scheduled'])
        runtime = job['config']['evolution']['seconds_limit']
        second_interval = job['config']['evolution']['log_sec_interval']
        data_points = int(runtime / second_interval)
        population_size = int(job['config']['population']['population_size'])

        fitnesses = np.zeros(
            (repeats, data_points, population_size), dtype=np.float64)

        for i, log in enumerate(db.logs.find({'job_id': job['_id']})):
            event_times = [event['event_time'] for event in log['events']]
            start_time = event_times[0]
            event_times = [e - start_time for e in event_times]
            event_times = [e.seconds * 1000 +
                           e.microseconds / 1000 for e in event_times]

            for j in range(data_points):
                target_millis = second_interval * j * 1000
                for k, millis in enumerate(event_times):
                    if millis > target_millis:
                        fitnesses[i][j] = [organism['fitness']
                                           for organism in log['events'][k]['organisms']]
                        break

        results.append((job_name, fitnesses))

        mean_fitness = fitnesses.mean(axis=(0, 2))
        max_fitness = fitnesses.max(axis=2).mean(axis=0)
        absolute_max_fitness = fitnesses.max(axis=(0, 2))
        fig = plt.figure(figsize=(10, 10))
        fig.suptitle(job_name)
        plt.plot(mean_fitness, label='mean fitness')
        plt.plot(max_fitness, label='max fitness')
        plt.plot(absolute_max_fitness, label='absolute max fitness')
        plt.ylim([0, 1])
        plt.legend()

        plt.savefig(
            f'jobs/analisys/plots/batch_{batch}/{job_name}.png', dpi=300)

    fig = plt.figure(figsize=(10, 10))
    for job_name, fitnesses in results:
        max_fitness = fitnesses.max(axis=2).mean(axis=0)
        plt.plot(max_fitness, label=job_name)
    plt.ylim([0, 1])
    plt.legend()
    plt.savefig(
        f'jobs/analisys/plots/batch_{batch}/all.png', dpi=300)

    if first:
        print(f'Batch does not exist: {batch}')
    else:
        print('Analysis complete')
