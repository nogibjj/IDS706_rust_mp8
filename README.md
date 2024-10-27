# IDS706_rust_mp8
![CI](https://github.com/nogibjj/IDS706_rust_mp8/actions/workflows/CICD.yaml/badge.svg)

## Requirements
* Take an existing Python script for data processing
* Rewrite it in Rust
* Highlight improvements in speed and resource usage

## Brief Introduction

### Dataset
<img src="https://seeklogo.com/images/N/nba-logo-59F0731E03-seeklogo.com.png" alt="NBA logo" width="400" />

#### [`NBA_2021.csv`](data.csv)
This is the NBA 2021 global statistics table, which includes data for every player, such as PT (points), BL (blocks), and AT (assists).

### Python Script

In [`src/main.py`](src/main.py), This main.py script is used to read data from a CSV file and perform statistical analysis on each column.

In [`src/main.py`](src/main.py), function `compute_statistics()` will generate descriptive of the dataset, the output looks like:

- [Click here to find out what each column represents](https://www.nba.com/stats/help/glossary#pctfga)

```bash
Column: Age
  Mean: 25.870921985815603
  Median: 25.0
  Standard Deviation: 4.094975770808347

Column: G
  Mean: 37.36879432624114
  Median: 37.0
  Standard Deviation: 21.26917962911175

Column: GS
  Mean: 16.941843971631204
  Median: 5.0
  Standard Deviation: 21.603760444536935

Column: MP
  Mean: 19.435886524822696
  Median: 19.3
  Standard Deviation: 9.155005311840752

Column: FG
  Mean: 3.1660992907801417
  Median: 2.6
  Standard Deviation: 2.278288012639409
```

After processing the dataset, this script will give the tiem and memory usage:

```bash
Total Time Elapsed: 0.1987 seconds
Peak Memory Usage: 1.2929 MB
```

### Rust Script

In [`src/main.rs`](src/main.rs), This main.rs script is used to read data from a CSV file and perform statistical analysis on each column.

In [`src/main.rs`](src/main.rs), function `compute_statistics()` will generate descriptive of the dataset, the output looks like:

```bash
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/ids706_rust_mp8`
Column: MP
  Mean: 19.43588652482267
  Median: 19.3
  Standard Deviation: 9.155005311840746

Column: PTS
  Mean: 8.616595744680849
  Median: 7.2
  Standard Deviation: 6.272807910051313

Column: eFG%
  Mean: 0.508901849217638
  Median: 0.52
  Standard Deviation: 0.11304602992858886

Column: 3PA
  Mean: 2.714042553191487
  Median: 2.2
  Standard Deviation: 2.2276449354450367
```

Moreover, the `get_memory_usage_kb()` function will calculate the time and memory usage of the rust script:
```bash
Total Time Elapsed: 0.0243 seconds
Memory Used: 2868 KB
```

## Performance Comparision

|          | Rust (Time/Memory) | Python (Time/Memory) | Ratio (Python/Rust) |
|----------|---------------------|----------------------|----------------------|
| Time     | 0.0232 s              | 0.1987 s               | 8.56x                   |
| Memory   | 2732 KB               | 1.2929 MB               | 4.73x                   |

As you can see, rust is faster than python in terms of speed and memory usage. This is probably due to the fact that they both use only the most basic libraries for data processing.

## Format, Lint, Test

`make rust-install`: build

`make rust-format`: format rust scripts

`make rust-lint`: lint rust code

`make rust-test`: test rust code

`make rust-run`: run the rust code
