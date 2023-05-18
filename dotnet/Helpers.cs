using Newtonsoft.Json;

public static class Helpers
{
    public static GeoJSON.RootObject ConvertToGeoJSON(IEnumerable<GeoRecord> records)
    {
        // Convert the list of records into a single GeoJSON object
        var geoJSON = new GeoJSON.RootObject
        {
            type = "FeatureCollection"
        };

        foreach (var record in records)
        {
            var feature = new GeoJSON.Feature
            {
                type = "Feature",
                properties = new GeoJSON.Properties
                {
                    addressType = record.AddressType,
                    street = record.Street,
                    locality = record.Locality,
                    region = record.Region,
                    postalCode = record.PostalCode,
                    country = record.Country
                },
                geometry = new GeoJSON.Geometry
                {
                    type = "Point",
                    coordinates = new List<double>
                    {
                        double.Parse(record.Longitude ?? "0"),
                        double.Parse(record.Latitude ?? "0")
                    }
                }
            };

            geoJSON.features.Add(feature);
        }

        return geoJSON;
    }

    public static void WriteGeoJSON(GeoJSON.RootObject geoJSON, string outputPath)
    {
        // Write the GeoJSON object to a file
        var json = JsonConvert.SerializeObject(geoJSON, Formatting.Indented, new JsonSerializerSettings{
            NullValueHandling = NullValueHandling.Ignore
        });
        
        File.WriteAllText(outputPath, json);
    }
}