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

    with Pool(16) as p:
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
    x = set_labels(ax, evo_cfg['iterations'], evo_cfg['seconds_limit'],
                   evo_cfg['log_interval'], evo_cfg['log_sec_interval'])

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
            plt.plot(x, max_fitness, label=f'{line_style} {job_name}',
                     linestyle=line_style, linewidth=0.6, color=line_color)

            scoreboard_str = f'{i}: {score}, {results[job_i][0]}\n'
            with open(f'jobs/analisys/plots/batch_{batch}/scoreboard_top.txt', 'a') as f:
                f.write(scoreboard_str)

    plt.ylim([0, 1])
    plt.legend(prop={'size': 2})
    fig.canvas.draw()

    path = f'jobs/analisys/plots/batch_{batch}/all.png'
    plt.savefig(path, dpi=1000)

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
    nodes = np.zeros((repeats, data_points, population_size),
                     dtype=np.float64)
    edges = np.zeros((repeats, data_points, population_size),
                     dtype=np.float64)
    hidden_substrates = np.zeros((repeats, data_points, population_size),
                                 dtype=np.float64)
    num_iters = np.zeros(repeats, dtype=np.float64)

    if evo_cfg['iterations'] != 0:
        for i, log in enumerate(db.logs.find({'job_id': job['_id']})):
            start_time = log['events'][0]['event_time']
            delta_t = (log['events'][-1]['event_time'] - start_time)
            num_iters[i] = (delta_t.seconds + delta_t.microseconds /
                            1000000) / evo_cfg['iterations']
            for j, event in enumerate(log['events']):
                fitnesses[i, j] = [organism['fitness']
                                   for organism in event['organisms']]
                if 'validation_accuracy' in event['organisms'][0]['evaluation']:
                    nodes[i, j] = [organism['phenotype']['network_stats']['nodes']
                                   for organism in event['organisms']]
                    edges[i, j] = [organism['phenotype']['network_stats']['edges']
                                   for organism in event['organisms']]
                    hidden_substrates[i, j] = [organism['phenotype']['hidden_substrates']
                                               for organism in event['organisms']]

                    validation_accuracy[i, j] = [organism['evaluation']['validation_accuracy']
                                                 for organism in event['organisms']]
                    validation_fitness[i, j] = [organism['evaluation']['validation_fitness']
                                                for organism in event['organisms']]
                    training_accuracy[i, j] = [organism['evaluation']['training_accuracy']
                                               for organism in event['organisms']]
                elif 'validation_accuracy' in event['organisms'][0]['evaluation'][0]:
                    nodes[i, j] = [np.mean([p['network_stats']['nodes'] for p in organism['phenotype']])
                                   for organism in event['organisms']]
                    edges[i, j] = [np.mean([p['network_stats']['edges'] for p in organism['phenotype']])
                                   for organism in event['organisms']]
                    hidden_substrates[i, j] = [np.mean([p['hidden_substrates'] for p in organism['phenotype']])
                                               for organism in event['organisms']]

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
            num_iters[i] = log['events'][-1]['iteration'] / \
                evo_cfg['seconds_limit']

            for j in range(data_points):
                target_millis = evo_cfg['log_sec_interval'] * j * 1000
                for k, millis in list(enumerate(event_times))[:: -1]:
                    if millis <= target_millis:
                        fitnesses[i][j] = [organism['fitness']
                                           for organism in log['events'][k]['organisms']]
                        if 'validation_accuracy' in log['events'][k]['organisms'][0]['evaluation']:
                            nodes[i, j] = [organism['phenotype']['network_stats']['nodes']
                                           for organism in log['events'][k]['organisms']]
                            edges[i, j] = [organism['phenotype']['network_stats']['edges']
                                           for organism in log['events'][k]['organisms']]
                            hidden_substrates[i, j] = [organism['phenotype']['hidden_substrates']
                                                       for organism in log['events'][k]['organisms']]

                            validation_accuracy[i][j] = [organism['evaluation']['validation_accuracy']
                                                         for organism in log['events'][k]['organisms']]
                            validation_fitness[i][j] = [organism['evaluation']['validation_fitness']
                                                        for organism in log['events'][k]['organisms']]
                            training_accuracy[i][j] = [organism['evaluation']['training_accuracy']
                                                       for organism in log['events'][k]['organisms']]
                        elif 'validation_accuracy' in log['events'][k]['organisms'][0]['evaluation'][0]:
                            nodes[i, j] = [np.mean([p['network_stats']['nodes'] for p in organism['phenotype']])
                                           for organism in log['events'][k]['organisms']]
                            edges[i, j] = [np.mean([p['network_stats']['edges'] for p in organism['phenotype']])
                                           for organism in log['events'][k]['organisms']]
                            hidden_substrates[i, j] = [np.mean([p['hidden_substrates'] for p in organism['phenotype']])
                                                       for organism in log['events'][k]['organisms']]

                            validation_accuracy[i][j] = [np.mean([e['validation_accuracy'] for e in organism['evaluation']])
                                                         for organism in log['events'][k]['organisms']]
                            validation_fitness[i][j] = [np.mean([e['validation_fitness'] for e in organism['evaluation']])
                                                        for organism in log['events'][k]['organisms']]
                            training_accuracy[i][j] = [np.mean([e['training_accuracy'] for e in organism['evaluation']])
                                                       for organism in log['events'][k]['organisms']]
                        break

    q.put((job_name, fitnesses, job['parameters']))
    print(q.qsize())

    job_params = job['parameters']
    job_params_str = json.dumps(job_params)
    job_method = job_params['METHOD']
    job_dataset = job_params['DATASET']
    job_iterations = job_params['ITERATIONS']

    def _log(name, x_labels, data):
        final_iter = data[:, -1]
        st = np.std(final_iter)
        mi = np.min(final_iter)
        ma = np.max(final_iter)

        mean_data = data.mean(axis=0)
        li = ''.join(f'({a},{b})' for a, b in zip(x_labels, mean_data))
        final_mean = mean_data[-1]

        with open('data.txt', 'a') as f:
            f.write(
                f'{job_iterations} {job_method} {job_dataset} {name} {final_mean} {st}\n')

        return f'{name} final: {final_mean} std: {st} \nmi: {mi} ma: {ma} all: {li}\n\n'

    max_i = validation_fitness.argmax(axis=2)
    max_validation_fitness = validation_fitness.max(axis=2)

    # mean_fitness = fitnesses.mean(axis=2)
    # max_fitness = fitnesses.max(axis=2)
    # max_training_accuracy = training_accuracy.max(axis=2)

    num_nodes = np.array([[a[b] for a, b in zip(c, d)] for c, d in zip(
        nodes, max_i)])
    num_edges = np.array([[a[b] for a, b in zip(c, d)] for c, d in zip(
        edges, max_i)])
    max_validation_accuracy = np.array([[a[b] for a, b in zip(c, d)] for c, d in zip(
        validation_accuracy, max_i)])

    fig, ax = plt.subplots(figsize=(8, 8))
    fig.suptitle(job_name)
    x = set_labels(ax, evo_cfg['iterations'], evo_cfg['seconds_limit'],
                   evo_cfg['log_interval'], evo_cfg['log_sec_interval'])
    # plt.plot(x, mean_fitness.mean(axis=0), label='mean fitness')
    max_nodes = int(np.max(num_nodes.mean(axis=0)))
    max_edges = int(np.max(num_edges.mean(axis=0)))
    plt.plot(x, max_validation_fitness.mean(axis=0), label='max fitness')
    plt.plot(x, max_validation_accuracy.mean(axis=0), label='max val acc')
    plt.plot(x, num_nodes.mean(axis=0) /
             max_nodes, label=f'nodes {max_nodes}')
    plt.plot(x, num_edges.mean(axis=0) /
             max_edges, label=f'edges {max_edges}')
    # plt.plot(x, fitnesses.max(axis=(0, 2)), label='absolute max fitness')
    plt.ylim([0, 1])
    plt.legend()
    fig.canvas.draw()

    fname = job['_id']
    path = f'jobs/analisys/plots/batch_{batch}/{fname}'
    plt.savefig(f'{path}.png', dpi=100)
    plt.close()

    with open(f'{path}.txt', 'w') as f:
        f.write(job_params_str)
        f.write(2*'\n')
        # f.write('max_fitness' + log(x, max_fitness))
        # f.write('max_training_accuracy' +
        #        log(x, max_training_accuracy))
        f.write(_log('max_validation_accuracy', x, max_validation_accuracy))
        f.write(_log('max_validation_fitness', x, max_validation_fitness))
        f.write(_log('num_nodes', x, num_nodes))
        f.write(_log('num_edges', x, num_edges))
        st = np.std(num_iters)
        mi = np.min(num_iters)
        ma = np.max(num_iters)
        iters = num_iters.mean()
        f.write(f'iterations: {iters}, st: {st}, mi: {mi}, ma: {ma}\n')
        with open('data.txt', 'a') as f:
            f.write(
                f'{job_iterations} {job_method} {job_dataset} iterations {iters} {st}\n')


def set_labels(ax, iterations, seconds, iter_interval, sec_interval):
    end_point = iterations if iterations != 0 else seconds
    interval = iter_interval if iterations != 0 else sec_interval
    num_ticks = int(end_point / interval) + 1
    ticks = list(map(int, np.linspace(0, end_point, num_ticks)))
    ax.set_xticks(ticks)
    return np.linspace(0, end_point, num_ticks)
