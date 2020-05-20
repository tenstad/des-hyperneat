from src.client import get_client
import os
import numpy as np
from matplotlib import pyplot as plt
from sklearn.model_selection import ParameterGrid


def analyse(batch):
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
        iterations = int(job['config']['evolution']['iterations'] /
                         job['config']['evolution']['log_interval']) + 1
        population_size = int(job['config']['population']['population_size'])

        fitnesses = np.zeros(
            (repeats, iterations, population_size), dtype=np.float64)

        for i, log in enumerate(db.logs.find({'job_id': job['_id']})):
            for j, event in enumerate(log['events']):
                fitnesses[i, j] = [organism['fitness']
                                   for organism in event['organisms']]

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

        fname = job_name.replace("'", '').replace(
            '{', '').replace('}', '').replace('/', '_')
        plt.savefig(
            f'jobs/analisys/plots/batch_{batch}/{fname}.png', dpi=300)

    styles = ParameterGrid({
        'color': ('#C0C0C0', '#808080', '#000000', '#FF0000', '#800000',
                  '#FFFF00', '#808000', '#00FF00', '#008000', '#00FFFF',
                  '#008080', '#0000FF', '#000080', '#FF00FF', '#800080',
                  ),
        'line': ('dotted', 'dashed', 'dashdot', 'solid'), })
    scoreboard = []

    fig = plt.figure(figsize=(10, 10))
    for i, (job_name, fitnesses) in enumerate(results):
        line_style = styles[i]['line']
        line_color = styles[i]['color']
        max_fitness = fitnesses.max(axis=2).mean(axis=0)
        scoreboard.append((max_fitness[-1], job_name))
        plt.plot(max_fitness, label=f'{line_style} {job_name}',
                 linestyle=line_style, linewidth=0.6, color=line_color)
    plt.ylim([0, 1])
    plt.legend(prop={'size': 2})
    plt.savefig(
        f'jobs/analisys/plots/batch_{batch}/all.png', dpi=1000)

    scoreboard = sorted(scoreboard)[::-1]
    print('\n'.join([f'{i}: {score}, {job_name}' for (
        i, (score, job_name)) in enumerate(scoreboard)]))

    if first:
        print(f'Batch does not exist: {batch}')
    else:
        print('Analysis complete')
