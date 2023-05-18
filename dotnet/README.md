# CASE Conversion Examples (C#)

This is an example script that uses the CASE graph to generate a GeoJSON output of locations. This highlights the value
of utilizing the CASE Ontology to create a graph that can be used to query and generate data in a variety of formats,
regardless of the source application's data model. This script is not production ready and does not properly handle
errors. It is intended to be used as a starting point for a more robust solution.

## Usage

```bash
# Build the dotnet project
dotnet build

# Run the built project
dotnet run <input.ttl> <output.geojson>
```

## Example

```bash
dotnet run ../data/locations.json ../output/locations.geojson
```
