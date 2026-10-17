# widom

A Monte Carlo adsorption simulator in Rust, for gases such as CO2 in porous crystals (MOFs and zeolites).

Status: early. Nothing runs yet.

## Goal

Rebuild the core of [RASPA3](https://github.com/iRASPA/RASPA3) (MIT licence) in Rust, one slice at a time, and check every slice against RASPA3's own results:

1. Widom insertion: Henry coefficients and the heat of adsorption at zero loading, for rigid molecules in a rigid framework.
2. Grand canonical Monte Carlo: adsorption isotherms, starting with CO2 in Cu-BTC (HKUST-1).
3. Later, if asked for: water, flexible molecules, mixtures.

A slice is done only when it matches RASPA3 on the same inputs within error bars.

## Why

Carbon capture research screens thousands of materials by computer before anyone makes them in a lab. The simulation tools are mostly C++ or Python. This project is a readable, tested Rust version of the parts that matter for capturing CO2 from air.

## Requests and contributions

- Want a feature, or found a bug? Open an [issue](../../issues/new/choose).
- Ideas and questions: use Discussions.
- Code: see [CONTRIBUTING.md](CONTRIBUTING.md).

## Licence

MIT. See [LICENSE](LICENSE) and [NOTICE](NOTICE) for the RASPA3 credit.
