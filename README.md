# Benchmarks of async pools for rust

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

## Results?

<a href="./RESULTS.md"><img src="./results-button.svg"></a>
