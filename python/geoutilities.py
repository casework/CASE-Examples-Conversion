from typing import List

from geotypes import Feature, GeoJSON, Geometry, GeoRecord, Properties


def records_to_geojson(records: List[GeoRecord]) -> GeoJSON:
    """
    Converts a list of GeoRecords to a GeoJSON object.

    :param records: The list of GeoRecords to convert.
    :return: The GeoJSON object.
    """
    geojson: GeoJSON = GeoJSON()
    geojson.type = "FeatureCollection"

    for record in records:
        feature = Feature()
        feature.type = "Feature"
        feature.properties = Properties()
        feature.properties.addressType = record.AddressType
        feature.properties.street = record.Street
        feature.properties.locality = record.Locality
        feature.properties.region = record.Region
        feature.properties.postalCode = record.PostalCode
        feature.properties.country = record.Country

        # If the coordinates aren't 0,0, then add them to the GeoJSON object
        if record.Latitude is not None and record.Longitude is not None:
            feature.geometry = Geometry()
            feature.geometry.type = "Point"
            feature.geometry.coordinates = [
                float(record.Longitude),
                float(record.Latitude),
            ]

        geojson.features.append(feature)

    return geojson
