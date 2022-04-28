#!/usr/bin/env -S poetry run python

import json
import matplotlib.pyplot as plt
import matplotlib as mpl


def parse_duration(duration):
    if duration:
        return duration['secs'] + duration['nanos'] / 1e9
    else:
        return 0.0


def format_duration(secs):
    if secs:
        return f'{int(secs * 1e3):,}ms'
    else:
        return "Timeout"


if __name__ == "__main__":
    # The following matplotlib configuration was copied from
    # https://github.com/Astro36/rust-pool-benchmark/blob/main/visualizer.py
    mpl.rcParams['axes.edgecolor'] = '#676466'
    mpl.rcParams['axes.facecolor'] = '#f5f4f3'
    mpl.rcParams['axes.prop_cycle'] = "cycler('color', ['#6768ab'])"
    mpl.rcParams['figure.autolayout'] = True
    mpl.rcParams['figure.titlesize'] = 16
    mpl.rcParams['font.family'] = 'monospace'
    mpl.rcParams['font.size'] = 9
    mpl.rcParams['text.color'] = '#2d282e'
    mpl.rcParams['ytick.labelcolor'] = '#2d282e'
    mpl.rcParams['ytick.labelsize'] = 10
    mpl.rcParams['ytick.color'] = '#676466'

    with open('results.json', 'r') as fh:
        benchmarks = json.load(fh)
        for benchmark in benchmarks:
            config = benchmark['config']
            results = benchmark['results']
            results.reverse()
            pool_size = config['pool_size']
            workers = config['workers']
            
            labels = [f'{result["name"]} {result["version"]}' for result in results]
            values = [parse_duration(result['duration']) for result in results]
            formatted_values = [format_duration(value) for value in values]

            plt.suptitle(f'Benchmark (pool_size={pool_size}, workers={workers})')
            bar = plt.barh(labels, values)
            plt.bar_label(bar, formatted_values)
            plt.gca().get_xaxis().set_visible(False)
            plt.gca().set_xlim(0, max(values) * 1.1)
            plt.savefig(f'figures/result_s{pool_size}_w{workers}.svg')
            plt.close()
