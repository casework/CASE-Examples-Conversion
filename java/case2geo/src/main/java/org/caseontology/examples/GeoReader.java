package org.caseontology.examples;

import org.apache.jena.query.QueryExecution;
import org.apache.jena.query.QueryExecutionFactory;
import org.apache.jena.query.QueryFactory;
import org.apache.jena.query.QuerySolution;
import org.apache.jena.rdf.model.Model;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStream;
import java.lang.reflect.Field;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Iterator;
import java.util.List;

import org.apache.jena.graph.Node;
import org.apache.jena.query.Query;
import org.apache.jena.query.ResultSet;
import org.apache.jena.rdf.model.ModelFactory;
import org.apache.jena.riot.Lang;
import org.apache.jena.riot.RDFDataMgr;
import org.caseontology.examples.geojson.Feature;
import org.caseontology.examples.geojson.GeoRecord;
import org.caseontology.examples.geojson.Geometry;
import org.caseontology.examples.geojson.Properties;
import org.caseontology.examples.geojson.Root;

import com.google.gson.Gson;
import com.google.gson.GsonBuilder;

public class GeoReader {

    private String InputPath;

    public GeoReader(String inputPath) {
        this.InputPath = inputPath;
    }

    /**
     * The primary entrypoint for the GeoReader class. This method will read the RDF graph from the input
     * file and convert it to GeoJSON string.
     *
     * @return the GeoJSON string generated from the input file
     */
    public String run() {

        // Define the SPARQL query to execute to retrieve the location information from
        // the input file
        String queryString = """
                            PREFIX uco-core: <https://ontology.unifiedcyberontology.org/uco/core/>
                            PREFIX uco-location: <https://ontology.unifiedcyberontology.org/uco/location/>

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
                """;

        try {
            // Load the input file into an RDF model
            Model model = getModel(this.InputPath);

            // Execute the SPARQL query on the RDF model
            ResultSet result = executeSparqlQuery(model, queryString);

            // Convert the SPARQL query result to a list of GeoRecord objects
            List<GeoRecord> records = resultToObjects(result);

            // Convert the list of GeoRecord objects to a GeoJSON string and return the string
            Gson gson = new GsonBuilder().setPrettyPrinting().create();
            Root geoJSON = recordsToGeoJSON(records);
            return gson.toJson(geoJSON);
        } catch (IOException e) {
            System.out.println("Error processing input: " + e.getMessage());
            e.printStackTrace();
            System.exit(1);
        }

        // This line will never be reached but the compiler complains if it is not explicitly returning a String value
        return "";
    }

    /**
     * Creates an RDF model from the provided input file.
     *
     * @param inputPath the path to the input file
     * @return Model the RDF graph built from the input file
     * @throws IOException if the input file does not exist or cannot be read
     */
    private static Model getModel(String inputPath) throws IOException {
        Model model = ModelFactory.createDefaultModel();

        InputStream inputStream = new FileInputStream(inputPath);
        RDFDataMgr.read(model, inputStream, Lang.JSONLD);

        return model;
    }

    /**
     * Executes a SPARQL query on the provided RDF model.
     *
     * @param model       the RDF model to query
     * @param sparqlQuery the SPARQL query to execute
     */
    private static ResultSet executeSparqlQuery(Model model, String sparqlQuery) {
        Query query = QueryFactory.create(sparqlQuery);
        QueryExecution queryExecution = QueryExecutionFactory.create(query, model);
        return queryExecution.execSelect();
    }

    /**
     * Converts the result of a SPARQL query to a list of GeoRecord objects.
     *
     * @param resultSet the result of a SPARQL query to be converted
     * @return List<GeoRecord> a list of GeoRecord objects
     */
    private static List<GeoRecord> resultToObjects(ResultSet resultSet) {
        // Initialize the list of geo Record objects
        List<GeoRecord> records = new ArrayList<GeoRecord>();

        while (resultSet.hasNext()) {
            QuerySolution soln = resultSet.nextSolution();

            // Initialize a new Record object
            GeoRecord record = new GeoRecord();
            // Get an iterator over the property names in the result set
            Iterator<String> names = soln.varNames();
            // For each property in the result set, strip the "l" prefix from the property
            // name and set the value of the corresponding property on the Record object
            while (names.hasNext()) {
                String property = names.next();
                String propertyName = property.substring(1);
                Node node = soln.get(property).asNode();
                if (node != null) {
                    String value = node.getLiteralLexicalForm();
                    try {
                        Field field = record.getClass().getDeclaredField(propertyName);
                        field.set(record, value);
                    } catch (NoSuchFieldException | IllegalAccessException e) {
                        System.out.println(
                                "Error setting property " + propertyName + " on Record object: " + e.getMessage());
                    }
                }
            }

            // Add the record to the list of records to be returned
            records.add(record);
        }

        return records;
    }

    /**
     * Converts a list of GeoRecord objects to a GeoJSON object.
     *
     * @param records the list of GeoRecord objects to be converted
     * @return Root the GeoJSON object
     */
    private static Root recordsToGeoJSON(List<GeoRecord> records) {
        Root root = new Root();
        root.type = "FeatureCollection";

        for (GeoRecord record : records) {
            Feature feature = new Feature();
            feature.type = "Feature";
            feature.properties = new Properties();
            feature.properties.addressType = record.AddressType;
            feature.properties.country = record.Country;
            feature.properties.locality = record.Locality;
            feature.properties.postalCode = record.PostalCode;
            feature.properties.region = record.Region;
            feature.properties.street = record.Street;
            // The GeoJSON format expects coordinates in the format [longitude, latitude]
            // Only add the coordinates if the latitude and longitude are not null
            if (record.Latitude != null && record.Longitude != null) {
                feature.geometry = new Geometry();
                feature.geometry.type = "Point";
                feature.geometry.coordinates = Arrays.asList(Double.parseDouble(record.Longitude),
                        Double.parseDouble(record.Latitude));
            }

            // Add the feature to the list of features in the GeoJSON object
            root.features.add(feature);
        }

        return root;
    }
}
