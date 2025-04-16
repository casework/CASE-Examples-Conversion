# CASE Conversion Examples (Rust)

This is an example script that uses the CASE graph to generate a GeoJSON output of locations. This highlights the value of utilizing the CASE Ontology to create a graph that can be used to query and generate data in a variety of formats, regardless of the source application's data model. This script is not production ready and does not properly handle errors. It is intended to be used as a starting point for a more robust solution.

## Usage

```bash
# Build the Rust project
cd case2geojson
cargo build

# Run the built project
target/debug/case2geojson input.json output.geojson
```

## Example

```bash
target/debug/case2geojson ../../data/locations.json ../../output/rust.geojson
```

## Dependencies
This repository depends on several Cargo crates for functionality between parsing JSON-LD, running a SPARQL query against a populated RDF store, and instantiating and serializing GeoJSON objects. These dependencies are managed via Cargo and can be viewed in the [`Cargo.toml`](./case2geojson/Cargo.toml).
