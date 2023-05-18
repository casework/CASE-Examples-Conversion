public class GeoRecord
{
    public string? Latitude { get; set; }
    public string? Longitude { get; set; }
    public string? AddressType { get; set; }
    public string? Street { get; set; }
    public string? Locality { get; set; }
    public string? Region { get; set; }
    public string? PostalCode { get; set; }
    public string? Country { get; set; }
}

public class GeoJSON
{
    public class RootObject
    {
        public string? type { get; set; }
        public List<Feature> features = new();
    }

    public class Feature
    {
        public string? type { get; set; }
        public Properties? properties { get; set; }
        public Geometry? geometry { get; set; }
    }

    public class Properties
    {
        public string? addressType { get; set; }
        public string? street { get; set; }
        public string? locality { get; set; }
        public string? region { get; set; }
        public string? postalCode { get; set; }
        public string? country { get; set; }
    }

    public class Geometry
    {
        public string? type { get; set; }
        public List<double> coordinates { get; set; } = new();
    }
}