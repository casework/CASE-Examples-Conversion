import json
import sys
from os.path import exists, isdir, isfile

from geotypes import GeoRecord
from geoutilities import records_to_geojson, remove_nulls
from rdflib import Graph

# Parse the arguments from the CLI to get the input and output filenames
if len(sys.argv) != 3:
    print("Usage: python case2geojson.py <input-file> <output-file>")
    sys.exit(1)

input_filename: str = sys.argv[1]
output_filename: str = sys.argv[2]

# Ensure the input file exists
if not exists(input_filename) and not isfile(input_filename):
    print(f"File not found: {input_filename}")
    sys.exit(1)

# Ensure the output directory exists
output_directory: str = output_filename[: output_filename.rfind("/")]
if not exists(output_directory) and not isdir(output_directory):
    print(f"Directory not found: {output_directory}")
    sys.exit(1)

# Build the rdflib graph from the input file
graph: Graph = Graph()
graph.parse(input_filename)

# Write the SPARQL query to get the data from the graph
query: str = """
    SELECT ?lLatitude ?lLongitude ?lAddressType ?lCountry ?lLocality ?lPostalCode ?lRegion ?lStreet
                WHERE
                {
                    ?nLocation a uco-location:Location .
                    OPTIONAL
                    {
                        ?nLocation uco-core:hasFacet ?nLatLongFacet .
                        ?nLatLongFacet a uco-location:LatLongCoordinatesFacet .
                        OPTIONAL { ?nLatLongFacet uco-location:latitude ?lLatitude . }
                        OPTIONAL { ?nLatLongFacet uco-location:longitude ?lLongitude . }
                    }

                    OPTIONAL {
                        ?nLocation uco-core:hasFacet ?nSimpleAddressFacet .
                        ?nSimpleAddressFacet a uco-location:SimpleAddressFacet .
                        OPTIONAL { ?nSimpleAddressFacet uco-location:addressType ?lAddressType . }
                        OPTIONAL { ?nSimpleAddressFacet uco-location:country ?lCountry . }
                        OPTIONAL { ?nSimpleAddressFacet uco-location:locality ?lLocality . }
                        OPTIONAL { ?nSimpleAddressFacet uco-location:postalCode ?lPostalCode . }
                        OPTIONAL { ?nSimpleAddressFacet uco-location:region ?lRegion . }
                        OPTIONAL { ?nSimpleAddressFacet uco-location:street ?lStreet . }
                    }
                }
    """

results = graph.query(query)

# Define the list of GeoRecords
records: list[GeoRecord] = []

# Loop through the results and add them to the list of GeoRecords if the latitude and longitude are present
for row in results:
    geo_record: GeoRecord = GeoRecord()
    geo_record.Latitude = row.lLatitude
    geo_record.Longitude = row.lLongitude
    geo_record.AddressType = row.lAddressType
    geo_record.Country = row.lCountry
    geo_record.Locality = row.lLocality
    geo_record.PostalCode = row.lPostalCode
    geo_record.Region = row.lRegion
    geo_record.Street = row.lStreet
    records.append(geo_record)

# Convert the data to a GeoJSON structured object
geoJSON = records_to_geojson(records)

# Remove null values from the GeoJSON object
geoDict: dict = geoJSON.reprJSON()
geoDict = remove_nulls(geoDict)

# Write the GeoJSON object to the output file
with open(output_filename, "w") as output_file:
    output_file.write(json.dumps(geoDict, indent=4))
