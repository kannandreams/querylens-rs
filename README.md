# QueryLens

QueryLens is a Rust CLI tool for analyzing SQL files, generating reports, scanning for patterns, calculating quality scores, and comparing SQL structures.

## Features

- `querylens init` - Create a default configuration file
- `querylens report` - Generate structured SQL insights
- `querylens scan` - Deep scan for SQL issues and patterns
- `querylens score` - Calculate SQL quality scores
- `querylens diff` - Compare SQL files or versions

## Usage

```bash
querylens report example_sql/simple_select.sql --format markdown
querylens scan example_sql --deep
querylens score example_sql --min-score 75
querylens diff example_sql/simple_select.sql example_sql/joins.sql --mode both
```

## Example SQL fixtures

The `example_sql/` folder contains sample SQL files for testing and exploration.

## Configuration

QueryLens supports YAML and JSON configuration files.
Create `.querylens.yaml` or `.querylens.json` in the project root.
