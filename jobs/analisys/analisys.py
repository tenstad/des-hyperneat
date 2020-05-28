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
    evo_cfg = job['config']['evolution']

    with Pool(8) as p:
        p.map(analyse_job, args)
    results = []
    while not q.empty():
        results.append(q.get())

    styles = ParameterGrid({
        'color': ('#C0C0C0', '#808080', '#000000', '#FF0000', '#800000',
                  '#FFFF00', '#808000', '#00FF00', '#008000', '#00FFFF',
                  '#008080', '#0000FF', '#000080', '#FF00FF', '#800080',
                  ),
        'line': ('dotted', 'dashed', 'dashdot', 'solid'), })

    scoreboard = []
    for i, (job_name, fitnesses, _) in enumerate(results):
        max_fitness = fitnesses.max(axis=2).mean(axis=0)
        scoreboard.append((max_fitness[-1], i))
    scoreboard = sorted(scoreboard)[::-1]
    scoreboard_str = '\n'.join([f'{i}: {score}, {results[job_i][0]}' for (
        i, (score, job_i)) in enumerate(scoreboard)])
    with open(f'jobs/analisys/plots/batch_{batch}/scoreboard.txt', 'w') as f:
        f.write(scoreboard_str)

    with open(f'jobs/analisys/plots/batch_{batch}/scoreboard_top.txt', 'w') as f:
        f.write('')
    jobs_plotted = {}

    fig, ax = plt.subplots(figsize=(10, 10))
    style_i = -1
    NUM_PLOTS = 3
    for i, (score, job_i) in enumerate(scoreboard):
        job_name = results[job_i][0]
        job_details = results[job_i][2]
        key = job_details['METHOD'] + job_details['DATASET']
        if not key in jobs_plotted:
            jobs_plotted[key] = 0
        if jobs_plotted[key] < NUM_PLOTS:
            jobs_plotted[key] += 1
            style_i += 1
            line_style = styles[style_i]['line']
            line_color = styles[style_i]['color']
            fitnesses = results[job_i][1]
            max_fitness = fitnesses.max(axis=2).mean(axis=0)
            plt.plot(max_fitness, label=f'{line_style} {job_name}',
                     linestyle=line_style, linewidth=0.6, color=line_color)

            scoreboard_str = f'{i}: {score}, {results[job_i][0]}\n'
            with open(f'jobs/analisys/plots/batch_{batch}/scoreboard_top.txt', 'a') as f:
                f.write(scoreboard_str)

    plt.ylim([0, 1])
    plt.legend(prop={'size': 2})
    fig.canvas.draw()
    set_labels(ax, evo_cfg['iterations'], evo_cfg['seconds_limit'])

    path = f'jobs/analisys/plots/batch_{batch}/all.png'
    plt.savefig(path, dpi=1000)

    runtime = time.time() - start_time
    print(f'Analysis complete in {runtime} seconds')


def analyse_job(args):
    job, batch, q = args

    client, db = get_database()
    job = db.jobs.find(job)[0]
    client.close()

    if not f'batch_{batch}' in os.listdir('jobs/analisys/plots/'):
        os.mkdir(f'jobs/analisys/plots/batch_{batch}/')

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

    if evo_cfg['iterations'] != 0:
        for i, log in enumerate(db.logs.find({'job_id': job['_id']}, {'_id': 0, 'events.organisms.fitness': 1})):
            for j, event in enumerate(log['events']):
                fitnesses[i, j] = [organism['fitness']
                                   for organism in event['organisms']]
    else:
        for i, log in enumerate(db.logs.find({'job_id': job['_id']}, {'_id': 0, 'events.event_time': 1, 'events.organisms.fitness': 1})):
            event_times = [event['event_time'] for event in log['events']]
            start_time = event_times[0]
            event_times = [e - start_time for e in event_times]
            event_times = [e.seconds * 1000 +
                           e.microseconds / 1000 for e in event_times]

            for j in range(data_points):
                target_millis = evo_cfg['log_sec_interval'] * j * 1000
                for k, millis in enumerate(event_times):
                    if millis > target_millis:
                        fitnesses[i][j] = [organism['fitness']
                                           for organism in log['events'][k]['organisms']]
                        break

    q.put((job_name, fitnesses, job['parameters']))
    print(q.qsize())

    mean_fitness = fitnesses.mean(axis=(0, 2))
    max_fitness = fitnesses.max(axis=2).mean(axis=0)
    absolute_max_fitness = fitnesses.max(axis=(0, 2))
    fig, ax = plt.subplots(figsize=(8, 8))
    fig.suptitle(job_name)
    plt.plot(mean_fitness, label='mean fitness')
    plt.plot(max_fitness, label='max fitness')
    plt.plot(absolute_max_fitness, label='absolute max fitness')
    plt.ylim([0, 1])
    plt.legend()
    fig.canvas.draw()

    set_labels(ax, evo_cfg['iterations'], evo_cfg['seconds_limit'])

    fname = job['_id']
    path = f'jobs/analisys/plots/batch_{batch}/{fname}.png'
    plt.savefig(path, dpi=100)
    plt.close()


def set_labels(ax, iterations, seconds):
    value = iterations if iterations != 0 else seconds
    ticks = list(np.linspace(0, value, len(ax.get_xticklabels())-2))
    ax.set_xticklabels([''] + ticks + [''])
