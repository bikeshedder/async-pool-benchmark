#!/usr/bin/env -S poetry run python

import json
import statistics

import matplotlib.pyplot as plt
import matplotlib as mpl


if __name__ == "__main__":
    mpl.rcParams["axes.edgecolor"] = "#6699aa"
    mpl.rcParams["axes.facecolor"] = "#f7faff"
    mpl.rcParams["axes.prop_cycle"] = "cycler('color', ['#336699'])"
    mpl.rcParams["figure.autolayout"] = True
    mpl.rcParams["font.size"] = 9
    mpl.rcParams["text.color"] = "#333333"

    with open("results.json", "r") as fh:
        benchmarks = json.load(fh)
        for benchmark in benchmarks:
            config = benchmark["config"]
            results = benchmark["results"]
            results.reverse()
            pool_size = config["pool_size"]
            workers = config["workers"]

            # Filter out async-object-pool as it doesn't really compare
            # to the other pool implementations. It is very fast but
            # neither stores the create function nor has any means of
            # recycling values.
            results = [
                result for result in results if result["name"] != "async-object-pool"
            ]

            labels = [f'{result["name"]} {result["version"]}' for result in results]
            ops = [result["ops"] for result in results]
            max_kops = max(max(x) for x in ops)
            mean = [statistics.mean(x) for x in ops]
            median = [statistics.median(x) for x in ops]
            stdev = [statistics.stdev(x) for x in ops]

            formatted = [f" {x:,.0f}" for x in median]

            fig, ax = plt.subplots()
            plt.suptitle(f"pool_size={pool_size}, workers={workers}")
            bar = plt.barh(labels, median, xerr=stdev, capsize=5, height=0.5)
            plt.bar_label(bar, formatted)
            plt.gca().set_xticks([])
            plt.gca().set_xlabel("operations per second")
            plt.gca().set_xlim(0, max_kops * 1.3)
            plt.savefig(f"figures/result_s{pool_size}_w{workers}.svg")
            plt.close()
