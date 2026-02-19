#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct EopEntry {
    pub mjd: f64,
    pub x_p: f64,
    pub y_p: f64,
    pub ut1_utc: f64,
    pub lod: f64,
    pub dx: f64,
    pub dy: f64,
}

const _: () = assert!(size_of::<EopEntry>() == 56);
