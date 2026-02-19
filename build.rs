use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

const MAGIC: &[u8; 4] = b"EOP1";
const RECORD_SIZE: usize = 7 * 8; // 7 x f64

fn main() {
    let manifest = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    let c04_path = Path::new(&manifest).join("data/eopc04.1962-now");
    let finals_path = Path::new(&manifest).join("data/finals2000A.all");

    println!("cargo:rerun-if-changed=data/eopc04.1962-now");
    println!("cargo:rerun-if-changed=data/finals2000A.all");

    let c04_records = parse_c04(&c04_path);
    let finals_records = parse_finals(&finals_path);

    compress_and_write(&c04_records, &Path::new(&out_dir).join("eop_c04.bin.zst"));
    compress_and_write(&finals_records, &Path::new(&out_dir).join("eop_finals.bin.zst"));

    let timestamp = chrono_free_utc_date();
    fs::write(Path::new(&out_dir).join("eop_timestamp.txt"), &timestamp).unwrap();

    eprintln!(
        "eop-data: {} C04 records, {} finals records, timestamp={}",
        c04_records.len(),
        finals_records.len(),
        timestamp,
    );
}

fn chrono_free_utc_date() -> String {
    let output = std::process::Command::new("date")
        .args(["-u", "+%Y-%m-%d"])
        .output()
        .expect("failed to run `date`");
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

/// C04 format: fixed-width, 6 header lines, then data.
/// Cols (0-indexed): MJD=16..26, xp=27..39, yp=39..51, UT1-UTC=51..63, dX=63..75, dY=75..87
/// LOD=111..123
fn parse_c04(path: &Path) -> Vec<[f64; 7]> {
    let content = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Cannot read {}: {}", path.display(), e));

    content
        .lines()
        .filter(|line| !line.starts_with('#') && line.len() >= 120)
        .filter_map(parse_c04_line)
        .collect()
}

fn parse_c04_line(line: &str) -> Option<[f64; 7]> {
    let f = |start: usize, end: usize| -> Option<f64> {
        line.get(start..end)?.trim().parse::<f64>().ok()
    };
    let mjd = f(16, 26)?;
    let xp = f(26, 38)?;
    let yp = f(38, 50)?;
    let ut1_utc = f(50, 62)?;
    // C04 dX/dY are in arcseconds; convert to milliarcseconds
    let dx = f(62, 74)? * 1000.0;
    let dy = f(74, 86)? * 1000.0;
    // C04 LOD is in seconds
    let lod = f(110, 122)?;
    Some([mjd, xp, yp, ut1_utc, lod, dx, dy])
}

/// finals2000A format: fixed-width, 188 chars per line.
/// Column positions (0-indexed):
///   MJD: 7..15, xp: 18..27, yp: 37..46, UT1-UTC: 58..68, LOD: 79..86
///   dX: 97..106, dY: 116..125
fn parse_finals(path: &Path) -> Vec<[f64; 7]> {
    let content = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Cannot read {}: {}", path.display(), e));

    content
        .lines()
        .filter(|line| line.len() >= 79)
        .filter_map(parse_finals_line)
        .collect()
}

fn parse_finals_line(line: &str) -> Option<[f64; 7]> {
    let f = |start: usize, end: usize| -> Option<f64> {
        let s = line.get(start..end)?.trim();
        if s.is_empty() { return None; }
        s.parse::<f64>().ok()
    };
    let mjd = f(7, 15)?;
    let xp = f(18, 27)?;
    let yp = f(37, 46)?;
    let ut1_utc = f(58, 68)?;
    // Finals LOD is in milliseconds; convert to seconds
    let lod = f(79, 86).unwrap_or(0.0) * 0.001;
    // Finals dX/dY are already in milliarcseconds
    let dx = f(97, 106).unwrap_or(0.0);
    let dy = f(116, 125).unwrap_or(0.0);
    Some([mjd, xp, yp, ut1_utc, lod, dx, dy])
}

fn compress_and_write(records: &[[f64; 7]], out_path: &Path) {
    let mut raw = Vec::with_capacity(MAGIC.len() + records.len() * RECORD_SIZE);
    raw.extend_from_slice(MAGIC);
    for rec in records {
        for val in rec {
            raw.extend_from_slice(&val.to_le_bytes());
        }
    }
    let compressed = zstd::encode_all(&raw[..], 19).unwrap();
    let mut file = fs::File::create(out_path).unwrap();
    file.write_all(&compressed).unwrap();
}
