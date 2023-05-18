# CASE Conversion Examples (C#)

This is an example script that uses the CASE graph to generate a GeoJSON output of locations. This highlights the value
of utilizing the CASE Ontology to create a graph that can be used to query and generate data in a variety of formats,
regardless of the source application's data model. This script is not production ready and does not properly handle
errors. It is intended to be used as a starting point for a more robust solution.

## Usage

```bash
# Optionally create a venv for the project
python3 -m venv venv

# Install any dependencies required for the project
pip install rdflib

# Run the built project
python3 CASE2GeoJSON.py <input.json> <output.geojson>
```

## Example

```bash
python3 CASE2GeoJSON.py ../data/locations.json ../output/locations.geojson
```

## Dependencies
This repository depends on the rdflib library. This dependency is managed by PyPi and can be installed via `pip install rdflib`.
