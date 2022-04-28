all: benchmark render_figures

benchmark:
	cargo run --release

render_figures:
	poetry install
	poetry run python render-figures.py
