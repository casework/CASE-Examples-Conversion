from typing import Any, List, Optional


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

    def reprJSON(self, *args: Any, remove_nulls: bool = False, **kwargs: Any) -> dict:
        """
        Provide a helper function to convert the properties into a dictionary so that the object can be JSON serialized.
        """
        if remove_nulls:
            retdict = dict()
            if self.addressType is not None:
                retdict["addressType"] = self.addressType
            if self.street is not None:
                retdict["street"] = self.street
            if self.locality is not None:
                retdict["locality"] = self.locality
            if self.region is not None:
                retdict["region"] = self.region
            if self.postalCode is not None:
                retdict["postalCode"] = self.postalCode
            if self.country is not None:
                retdict["country"] = self.country
            return retdict
        else:
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

    def reprJSON(self, *args: Any, **kwargs: Any) -> dict:
        """
        Provide a helper function to convert the properties into a dictionary so that the object can be JSON serialized.
        """
        # NOTE: remove_nulls is not applicable in this method.
        return dict(type=self.type, coordinates=self.coordinates)


class Feature:
    type: Optional[str]
    properties: Optional[Properties]
    geometry: Optional[Geometry]

    def reprJSON(self, *args: Any, remove_nulls: bool = False, **kwargs: Any) -> dict:
        """
        Provide a helper function to convert the properties into a dictionary so that the object can be JSON serialized.
        """
        return dict(
            type=self.type,
            properties=(
                self.properties.reprJSON(remove_nulls=remove_nulls)
                if hasattr(self, "properties") and self.properties is not None
                else None
            ),
            geometry=(
                self.geometry.reprJSON(remove_nulls=remove_nulls)
                if hasattr(self, "geometry") and self.geometry is not None
                else None
            ),
        )


class GeoJSON:
    type: Optional[str]
    features: List[Feature] = []

    def reprJSON(self, *args: Any, remove_nulls: bool = False, **kwargs: Any) -> dict:
        """
        Provide a helper function to convert the properties into a dictionary so that the object can be JSON serialized.
        """
        return dict(
            type=self.type, features=[feature.reprJSON(remove_nulls=remove_nulls) for feature in self.features]
        )
