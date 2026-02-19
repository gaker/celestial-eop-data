use std::sync::OnceLock;

use crate::EopEntry;

const MAGIC: &[u8; 4] = b"EOP1";
const RECORD_SIZE: usize = 7 * 8;

static C04: OnceLock<Vec<EopEntry>> = OnceLock::new();
static FINALS: OnceLock<Vec<EopEntry>> = OnceLock::new();

static C04_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/eop_c04.bin.zst"));
static FINALS_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/eop_finals.bin.zst"));

pub fn c04() -> &'static [EopEntry] {
    C04.get_or_init(|| decode(C04_BYTES))
}

pub fn finals() -> &'static [EopEntry] {
    FINALS.get_or_init(|| decode(FINALS_BYTES))
}

fn decode(compressed: &[u8]) -> Vec<EopEntry> {
    let raw = zstd::decode_all(compressed).expect("corrupt zstd data");
    assert!(raw.starts_with(MAGIC), "bad magic in EOP binary");
    let body = &raw[MAGIC.len()..];
    assert_eq!(body.len() % RECORD_SIZE, 0, "truncated EOP binary");
    body.chunks_exact(RECORD_SIZE).map(parse_record).collect()
}

fn parse_record(chunk: &[u8]) -> EopEntry {
    let f = |i: usize| f64::from_le_bytes(chunk[i * 8..(i + 1) * 8].try_into().unwrap());
    EopEntry {
        mjd: f(0),
        x_p: f(1),
        y_p: f(2),
        ut1_utc: f(3),
        lod: f(4),
        dx: f(5),
        dy: f(6),
    }
}
