mod decompress;
mod entry;

pub use entry::EopEntry;

pub fn c04_data() -> &'static [EopEntry] {
    decompress::c04()
}

pub fn finals_data() -> &'static [EopEntry] {
    decompress::finals()
}

pub fn combined_data() -> Vec<EopEntry> {
    let c04 = c04_data();
    let finals = finals_data();
    let c04_max_mjd = c04.last().map(|e| e.mjd).unwrap_or(0.0);
    let mut combined = c04.to_vec();
    combined.extend(finals.iter().filter(|e| e.mjd > c04_max_mjd));
    combined
}

pub fn data_time_span() -> (f64, f64) {
    let combined = combined_data();
    let first = combined.first().map(|e| e.mjd).unwrap_or(0.0);
    let last = combined.last().map(|e| e.mjd).unwrap_or(0.0);
    (first, last)
}

pub fn data_timestamp() -> &'static str {
    include_str!(concat!(env!("OUT_DIR"), "/eop_timestamp.txt"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c04_loads_enough_records() {
        assert!(c04_data().len() > 20_000);
    }

    #[test]
    fn finals_loads_enough_records() {
        assert!(finals_data().len() > 10_000);
    }

    #[test]
    fn combined_sorted_no_duplicates() {
        let data = combined_data();
        for pair in data.windows(2) {
            assert!(pair[1].mjd > pair[0].mjd, "not sorted at MJD {}", pair[0].mjd);
        }
    }

    #[test]
    fn time_span_covers_expected_range() {
        let (start, end) = data_time_span();
        assert!(start <= 37665.0, "start MJD {} too late", start);
        assert!(end >= 60000.0, "end MJD {} too early", end);
    }

    #[test]
    fn known_c04_value() {
        let data = c04_data();
        let entry = data.iter().find(|e| (e.mjd - 58849.0).abs() < 0.01);
        let entry = entry.expect("MJD 58849 not found in C04");
        assert!(
            (entry.x_p - 0.076614).abs() < 0.001,
            "xp={}, expected ~0.076614",
            entry.x_p,
        );
    }

    #[test]
    fn timestamp_is_valid_iso8601() {
        let ts = data_timestamp();
        assert_eq!(ts.len(), 10, "timestamp '{}' wrong length", ts);
        assert_eq!(&ts[4..5], "-");
        assert_eq!(&ts[7..8], "-");
        ts[0..4].parse::<u32>().expect("bad year");
        ts[5..7].parse::<u32>().expect("bad month");
        ts[8..10].parse::<u32>().expect("bad day");
    }
}
