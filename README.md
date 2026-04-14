# QueryLens

QueryLens is a Rust CLI tool for analyzing SQL files, generating reports, scanning for patterns, calculating quality scores, and comparing SQL structures.

## Features

- `querylens-rs init` - Create a default configuration file
- `querylens-rs report` - Generate structured SQL insights
- `querylens-rs scan` - Deep scan for SQL issues and patterns
- `querylens-rs score` - Calculate SQL quality scores
- `querylens-rs diff` - Compare SQL files or versions

## Usage

```bash
cargo run -- report examples/example_sql/simple_select.sql --format markdown
cargo run -- scan example_sql --deep
cargo run -- score example_sql --min-score 75
cargo run -- diff example_sql/simple_select.sql example_sql/joins.sql --mode both
```

## Example SQL fixtures

The `example_sql/` folder contains sample SQL files for testing and exploration.

## Configuration

QueryLens supports YAML and JSON configuration files.
Create `.querylens.yaml` or `.querylens.json` in the project root.
