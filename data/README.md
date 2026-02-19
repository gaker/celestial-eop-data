# Raw IERS Earth Orientation Parameter files used by build.rs

- `eopc04.1962-now` — IERS C04 series (observed values, 1962-present)
- `finals2000A.all` — IERS finals (observed + ~1yr predictions)

Updated weekly by the GitHub Action in `.github/workflows/update.yml`.

## Sources & Attribution

This data is provided by the [International Earth Rotation and Reference Systems
Service (IERS)](https://www.iers.org).

### IERS C04 Series

Produced by the IERS Earth Orientation Centre, Paris Observatory (SYRTE).

- URL: <https://hpiers.obspm.fr/iers/eop/eopc04/eopc04.1962-now>
- Citation: Bizouard, C., Lambert, S., Gattano, C., Becker, O., & Richard, J.-Y.
  (2019). The IERS EOP 14C04 solution for Earth orientation parameters consistent
  with ITRF 2014. *Journal of Geodesy*, 93, 621–633.
  [doi:10.1007/s00190-018-1186-3](https://doi.org/10.1007/s00190-018-1186-3)

### IERS finals2000A

Produced by the IERS Rapid Service/Prediction Centre, U.S. Naval Observatory.

- URL: <https://datacenter.iers.org/data/9/finals2000A.all>
- Distribution Statement A: Approved for public release; distribution unlimited.
- Citation: Luzum, B., Ray, J., Carter, M. et al. (2001). Recent Improvements to
  IERS Bulletin A Combination and Prediction. *GPS Solutions*, 4, 34–40.
  [doi:10.1007/PL00012853](https://doi.org/10.1007/PL00012853)

### General Reference

Petit, G. & Luzum, B. (2010). IERS Conventions (2010). *IERS Technical Note
No. 36*, Verlag des Bundesamts für Kartographie und Geodäsie, Frankfurt am Main.
ISBN 3-89888-989-6.
