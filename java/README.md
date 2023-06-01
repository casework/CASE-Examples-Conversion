# CASE Conversion Examples (Java)

This is an example script that uses the CASE graph to generate a GeoJSON output of locations. This highlights the value of utilizing the CASE Ontology to create a graph that can be used to query and generate data in a variety of formats, regardless of the source application's data model. This script is not production ready and does not properly handle errors. It is intended to be used as a starting point for a more robust solution.

## Usage

```bash
# Build the Maven Java project
mvn compile assembly:single

# Run the built project
java -jar ./target/case2geo-0.1.0.jar input.json output.geojson
```

## Example

```bash
java -jar ./target/case2geo-0.1.0.jar ../../data/locations.json ../../output/java.geojson
```

## Dependencies
This repository depends on several Java libraries, primarily Apache Jena. These dependencies are managed via Maven and can be viewed in the [`pom.xml`](./case2geo/pom.xml).
