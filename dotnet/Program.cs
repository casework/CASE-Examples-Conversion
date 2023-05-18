using VDS.RDF;
using VDS.RDF.Nodes;
using VDS.RDF.Parsing;
using VDS.RDF.Query;
using VDS.RDF.Query.Datasets;

// Confirm the input and output paths were provided as command line arguments
if (args.Length != 2)
{
    Console.WriteLine("Usage: dotnet run <input file> <output file>");
    return;
}

// Get the input and output paths and save them validation as well as usage later in the script
string inputPath = args[0];
string outputPath = args[1];

try
{
    // Ensure the input path exists
    if (!File.Exists(inputPath))
    {
        Console.WriteLine("Input file does not exist: " + inputPath);
        return;
    }

    // Ensure the path for the output file exists
    string? outputDirectory = Path.GetDirectoryName(outputPath);
    if (!Directory.Exists(outputDirectory))
    {
        Console.WriteLine("Output directory does not exist: " + outputDirectory);
        return;
    }
}
catch (Exception e)
{
    // Catch any exception that occurs while processing the command line arguments and print the errors
    Console.WriteLine("Error processing command line arguments: " + e.Message);
    return;
}

// Define a query to get the location information from a CASE graph
const string query =
    @"SELECT ?lLatitude ?lLongitude ?lAddressType ?lCountry ?lLocality ?lPostalCode ?lRegion ?lStreet
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
                }";

// Load the file into a graph into an in-memory triple store
TripleStore tripleStore = new();
JsonLdParser jsonLdParser = new();
jsonLdParser.Load(tripleStore, inputPath);
ISparqlQueryProcessor queryProcessor = new LeviathanQueryProcessor(new InMemoryDataset(tripleStore));
SparqlQueryParser parser = new();

// Build a query object from the query string. This is a simple query that does not require any parameters.
// This adds the namespaces for the CASE Ontology to the query so that the query can be executed with proper naming
// resolution.
var queryString = new SparqlParameterizedString
{
    CommandText = query
};

// Add the default namespaces for the CASE Ontology
queryString.Namespaces.AddNamespace("case-investigation",
    new Uri("https://ontology.caseontology.org/case/investigation/"));
queryString.Namespaces.AddNamespace("case-vocabulary", new Uri("https://ontology.caseontology.org/case/vocabulary/"));
queryString.Namespaces.AddNamespace("uco-analysis", new Uri("https://ontology.unifiedcyberontology.org/uco/analysis/"));
queryString.Namespaces.AddNamespace("uco-configuration",
    new Uri("https://ontology.unifiedcyberontology.org/uco/configuration/"));
queryString.Namespaces.AddNamespace("uco-core", new Uri("https://ontology.unifiedcyberontology.org/uco/core/"));
queryString.Namespaces.AddNamespace("uco-identity", new Uri("https://ontology.unifiedcyberontology.org/uco/identity/"));
queryString.Namespaces.AddNamespace("uco-location", new Uri("https://ontology.unifiedcyberontology.org/uco/location/"));
queryString.Namespaces.AddNamespace("uco-marking", new Uri("https://ontology.unifiedcyberontology.org/uco/marking/"));
queryString.Namespaces.AddNamespace("uco-observable",
    new Uri("https://ontology.unifiedcyberontology.org/uco/observable/"));
queryString.Namespaces.AddNamespace("uco-pattern", new Uri("https://ontology.unifiedcyberontology.org/uco/pattern/"));
queryString.Namespaces.AddNamespace("uco-role", new Uri("https://ontology.unifiedcyberontology.org/uco/role/"));
queryString.Namespaces.AddNamespace("uco-time", new Uri("https://ontology.unifiedcyberontology.org/uco/time/"));
queryString.Namespaces.AddNamespace("uco-tool", new Uri("https://ontology.unifiedcyberontology.org/uco/tool/"));
queryString.Namespaces.AddNamespace("uco-types", new Uri("https://ontology.unifiedcyberontology.org/uco/types/"));
queryString.Namespaces.AddNamespace("uco-victim", new Uri("https://ontology.unifiedcyberontology.org/uco/victim/"));
queryString.Namespaces.AddNamespace("uco-vocabulary",
    new Uri("https://ontology.unifiedcyberontology.org/uco/vocabulary/"));

// Execute the query
var results = (SparqlResultSet)queryProcessor.ProcessQuery(parser.ParseFromString(queryString));

// Build the list of GeoRecords that will be used to build the GeoJSON object once they've been loaded from the SPARQL results
var records = new List<GeoRecord>();

// Define the list of fields in the query to parse back out of the results
string[] fields = { "Latitude", "Longitude", "AddressType", "Country", "Locality", "PostalCode", "Region", "Street" };

// Loop through the records and add them to the list of GeoRecords
foreach (var result in results)
{
    // Create the blank GeoRecord object
    var record = new GeoRecord();

    // Loop through the fields and set the values on the GeoRecord object if they are present in the results
    foreach (var field in fields)
    {
        if (result.TryGetValue("l" + field, out var node))
        {
            record.GetType().GetProperty(field)?.SetValue(record, node.AsValuedNode().AsString());
        }
    }

    // Add the record to the list if the latitude and longitude are present and not 0
    if (double.TryParse(record.Latitude, out var latitude) && double.TryParse(record.Longitude, out var longitude) &&
        latitude != 0 && longitude != 0)
    {
        records.Add(record);
    }
}

// Convert the list of records into a single GeoJSON object
var geoJSON = Helpers.ConvertToGeoJSON(records);

// Write the GeoJSON object to a file
Helpers.WriteGeoJSON(geoJSON, outputPath);
