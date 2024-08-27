using Newtonsoft.Json;

public static class Helpers
{
    ///
    /// <summary>Convert the list of records into a single GeoJSON object</summary>
    ///
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

            };

            // If the coordinates are 0,0 then don't add the record to the GeoJSON object
            // Add the record to the list if the latitude and longitude are present and not 0
            if (double.TryParse(record.Latitude, out var latitude) && double.TryParse(record.Longitude, out var longitude) &&
                latitude != 0 && longitude != 0)
            {
                feature.geometry = new GeoJSON.Geometry
                {
                    type = "Point",
                    coordinates = new List<double> { longitude, latitude }
                };
            }

            geoJSON.features.Add(feature);
        }

        return geoJSON;
    }

    ///
    /// <summary>Write the GeoJSON object to a file</summary>
    ///
    public static void WriteGeoJSON(GeoJSON.RootObject geoJSON, string outputPath)
    {
        // Write the GeoJSON object to a file
        var json = JsonConvert.SerializeObject(geoJSON, Formatting.Indented, new JsonSerializerSettings{
            NullValueHandling = NullValueHandling.Include
        });

        File.WriteAllText(outputPath, json);
    }
}
