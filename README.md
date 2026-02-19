# celestial-eop-data

Bundled [IERS](https://www.iers.org/) Earth Orientation Parameter data for Rust. Ships real EOP data as compressed binary, embedded at compile time — no network access needed at runtime.

Used by [`celestial`](https://github.com/gaker/celestial) for coordinate transformations, precession-nutation corrections, and UT1-UTC lookups.

## Data sources

- **C04 series** — IERS EOP 20 C04 (observed values, 1962–present)
- **finals2000A** — IERS finals (observed + ~1 year predictions)

Raw data files live in `data/` and are updated weekly by a GitHub Action.

## Usage

```rust
use celestial_eop_data::{c04_data, finals_data, combined_data, data_time_span};

// Lazy-loaded on first access, cached for the process lifetime
let c04 = c04_data();
let finals = finals_data();

// C04 preferred, finals appended for dates beyond C04 coverage
let all = combined_data();

let (start_mjd, end_mjd) = data_time_span();
```

Each `EopEntry` contains:

| Field     | Unit            |
|-----------|-----------------|
| `mjd`     | days            |
| `x_p`     | arcseconds      |
| `y_p`     | arcseconds      |
| `ut1_utc` | seconds         |
| `lod`     | seconds         |
| `dx`      | milliarcseconds |
| `dy`      | milliarcseconds |

## How it works

`build.rs` parses the raw IERS text files from `data/`, packs them into a binary format (7 × f64 per record), and compresses with zstd. The compressed blobs are embedded via `include_bytes!` and decompressed lazily on first access using `OnceLock`.

## Data updates

A weekly GitHub Action downloads fresh IERS files, runs the test suite, and if the data changed, bumps the patch version and publishes to crates.io. Downstream crates pick up new data with `cargo update -p celestial-eop-data`.

## License

MIT OR Apache-2.0
