from typing import List, Optional


class GeoRecord:
    Latitude: str
    Longitude: str
    AddressType: str
    Street: str
    Locality: str
    Region: str
    PostalCode: str
    Country: str


class Properties:
    addressType: Optional[str]
    street: Optional[str]
    locality: Optional[str]
    region: Optional[str]
    postalCode: Optional[str]
    country: Optional[str]

    def reprJSON(self):
        """
        Provide a helper function to convert the properties into a dictionary so that the object can be JSON serialized.
        """
        return dict(
            addressType=self.addressType,
            street=self.street,
            locality=self.locality,
            region=self.region,
            postalCode=self.postalCode,
            country=self.country,
        )


class Geometry:
    type: Optional[str]
    coordinates: List[float] = []

    def reprJSON(self):
        """
        Provide a helper function to convert the properties into a dictionary so that the object can be JSON serialized.
        """
        return dict(type=self.type, coordinates=self.coordinates)


class Feature:
    type: Optional[str]
    properties: Optional[Properties]
    geometry: Optional[Geometry]

    def reprJSON(self):
        """
        Provide a helper function to convert the properties into a dictionary so that the object can be JSON serialized.
        """
        return dict(
            type=self.type,
            properties=(
                self.properties.reprJSON()
                if hasattr(self, "properties") and self.properties is not None
                else None
            ),
            geometry=(
                self.geometry.reprJSON()
                if hasattr(self, "geometry") and self.geometry is not None
                else None
            ),
        )


class GeoJSON:
    type: Optional[str]
    features: List[Feature] = []

    def reprJSON(self):
        """
        Provide a helper function to convert the properties into a dictionary so that the object can be JSON serialized.
        """
        return dict(
            type=self.type, features=[feature.reprJSON() for feature in self.features]
        )
