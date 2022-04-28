# Benchmarks of async pools for rust

[![View benchmark results](results-button.svg)](./RESULTS.md)

> ⚠ **Please read and run the benchmark code yourself.
> Don't blindly trust and base your decisions on what
> I've published here.**

> 💡 Keep in mind that speed isn't everything when it
> comes to software. Have a look at the feature list,
> overall code quality and then make an informed decision.

## How to run?

If you got `Make` installed simply call it:

```
make
```

This will run the benchmarks and render the figures included in
the [RESULTS.md](./RESULTS.md). Alternatively you can run the benchmarks with cargo...

```
cargo run --release
```

...and then create the SVG files in the figures directory by calling...

```
python3 render-figures.py
```
