// Portions of this file contributed by NIST are governed by the
// following statement:
//
// This software was developed at the National Institute of Standards
// and Technology by employees of the Federal Government in the course
// of their official duties. Pursuant to Title 17 Section 105 of the
// United States Code, this software is not subject to copyright
// protection within the United States. NIST assumes no responsibility
// whatsoever for its use by other parties, and makes no guarantees,
// expressed or implied, about its quality, reliability, or any other
// characteristic.
//
// We would appreciate acknowledgement if the software is used.

//! This program takes several steps to convert a CASE JSON-LD file to GeoJSON.
//!
//! 1. A JSON-LD file is parsed and expanded.  This is tested in `jsonld_expand_adapted_example`.
//! 1. The expanded JSON-LD object is converted into RDF quads, populating an Oxigraph store.  Store population is tested with individual hard-coded quads in `store_insert_adapted_example`.
//! 1. A SPARQL query is run.  This is tested in `store_query_adapted_example`.
//! 1. GeoJSON objects are constructed and serialized.

use geojson;
use json_ld::{
    rdf_types::{Quad, Term},
    syntax::{Parse, Value},
    JsonLdProcessor, RemoteDocument,
};
use oxigraph::model::{GraphNameRef, LiteralRef, NamedNodeRef, QuadRef};
use oxigraph::sparql::QueryResults;
use oxigraph::store::Store;
use static_iref::iri;
use std::{env, fs};

fn unquote_string(mut s: String) -> String {
    if s.len() > 0 {
        if s[0..1] == *"\"" {
            s.pop();
            if s.len() > 0 {
                s.remove(0);
            }
        }
    }
    s
}

#[tokio::main]
async fn main() {
    // Drawn from:
    // https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
    let args: Vec<String> = env::args().collect();

    let input_case_json_ld_path = &args[1];

    // Drawn from:
    // https://doc.rust-lang.org/book/ch12-02-reading-a-file.html
    let file_contents: String = fs::read_to_string(input_case_json_ld_path)
        .expect("Should have been able to read the file");

    // Parse the file.
    let value = Value::parse_str(&file_contents)
        .expect("unable to parse file")
        .0;

    // JSON-LD parsing and expansion code adapted from:
    // https://docs.rs/json-ld/0.21.1/json_ld/#example
    // The runtime selection followd this documentation:
    // https://rust-lang.github.io/async-book/part-guide/async-await.html#the-runtime

    // Create a "remote" document by parsing a file manually.
    let input = RemoteDocument::new(
        // We use `IriBuf` as IRI type.
        Some(iri!("https://example.com/sample.jsonld").to_owned()),
        // Optional content type.
        Some("application/ld+json".parse().unwrap()),
        value,
    );

    // Use `NoLoader` as we won't need to load any remote document.
    let loader = json_ld::NoLoader;

    let mut generator = json_ld::rdf_types::generator::Blank::new();

    // Drawn from https://docs.rs/json-ld/latest/json_ld/trait.JsonLdProcessor.html#method.to_rdf
    let mut rdf = input
        .to_rdf(&mut generator, &loader)
        .await
        .expect("flattening failed");

    // Store-querying code adapted from:
    // https://docs.rs/oxigraph/0.4.1/oxigraph/store/struct.Store.html#method.query
    // Store-populating code adapted from:
    // https://docs.rs/oxigraph/0.4.3/oxigraph/store/struct.Store.html#method.insert

    let store = Store::new().unwrap();

    for quad in rdf.quads() {
        // dbg!(&quad);
        let Quad(s, p, o, _g) = quad;
        let n_subject = NamedNodeRef::new(s.as_iri().unwrap()).unwrap();
        let n_predicate = NamedNodeRef::new(p.as_iri().unwrap()).unwrap();

        if let Term::Id(_id) = o.to_owned() {
            store
                .insert(QuadRef::new(
                    n_subject,
                    n_predicate,
                    NamedNodeRef::new(_id.as_iri().unwrap()).unwrap(),
                    GraphNameRef::DefaultGraph,
                ))
                .unwrap();
        };

        if let Term::Literal(_literal) = o.to_owned() {
            store
                .insert(QuadRef::new(
                    n_subject,
                    n_predicate,
                    LiteralRef::new_simple_literal(&_literal.value),
                    GraphNameRef::DefaultGraph,
                ))
                .unwrap();
        };
    }

    let mut gj_features = vec![];

    let query = r#"
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
"#;

    // SPARQL query
    if let QueryResults::Solutions(solutions) = store.query(query).unwrap() {
        for option_solution in solutions {
            let solution = option_solution.unwrap();

            let l_latitude = &solution.get("lLatitude");
            let l_longitude = &solution.get("lLongitude");
            let _l_address_type = &solution.get("lAddressType");
            let l_country = &solution.get("lCountry");
            let l_locality = &solution.get("lLocality");
            let l_postal_code = &solution.get("lPostalCode");
            let l_region = &solution.get("lRegion");
            let l_street = &solution.get("lStreet");

            // Note - this property was not demonstrated in the example data.
            // dbg!(_l_address_type);

            // dbg!(l_latitude);
            // dbg!(l_longitude);
            let mut gj_point: Option<geojson::Geometry> = None;
            if let Some(x) = l_longitude {
                if let Some(y) = l_latitude {
                    // Remove 1st and last chars due to extra quote marks.
                    // https://stackoverflow.com/a/70598494
                    let s_latitude: String = unquote_string(y.to_string());
                    let s_longitude: String = unquote_string(x.to_string());

                    let f_latitude = (s_latitude).parse::<f64>().unwrap();
                    let f_longitude = (s_longitude).parse::<f64>().unwrap();
                    gj_point = Some(geojson::Geometry::new(geojson::Value::Point(vec![
                        f_longitude,
                        f_latitude,
                    ])));
                };
            };

            // dbg!(l_street);
            let mut gj_properties: geojson::JsonObject = serde_json::Map::new();
            if let Some(x) = l_street {
                gj_properties.insert(
                    String::from("street"),
                    geojson::JsonValue::String(unquote_string(x.to_string())),
                );
            }
            // dbg!(l_locality);
            if let Some(x) = l_locality {
                gj_properties.insert(
                    String::from("locality"),
                    geojson::JsonValue::String(unquote_string(x.to_string())),
                );
            }
            // dbg!(l_region);
            if let Some(x) = l_region {
                gj_properties.insert(
                    String::from("region"),
                    geojson::JsonValue::String(unquote_string(x.to_string())),
                );
            }
            // dbg!(l_postal_code);
            if let Some(x) = l_postal_code {
                gj_properties.insert(
                    String::from("postalCode"),
                    geojson::JsonValue::String(unquote_string(x.to_string())),
                );
            }
            // dbg!(l_country);
            if let Some(x) = l_country {
                gj_properties.insert(
                    String::from("country"),
                    geojson::JsonValue::String(unquote_string(x.to_string())),
                );
            }

            let gj_feature = geojson::Feature {
                bbox: None,
                geometry: gj_point,
                id: None,
                properties: Some(gj_properties),
                foreign_members: None,
            };
            // dbg!(gj_feature);
            gj_features.push(gj_feature)
        }
    }
    let gj_feature_collection = geojson::FeatureCollection {
        bbox: None,
        features: gj_features,
        foreign_members: None,
    };
    println!(
        "{}",
        geojson::GeoJson::from(gj_feature_collection).to_string()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[tokio::test]
    /// Adapted from [example source](https://docs.rs/json-ld/0.21.1/json_ld/#example)
    async fn jsonld_expand_adapted_example() {
        let file_contents = r#"
{
    "@context": {
        "@vocab": "http://example.org/local#",
        "kb": "http://example.org/kb/",
        "acme": "http://custompb.acme.org/core#",
        "uco-core": "https://ontology.unifiedcyberontology.org/uco/core/",
        "uco-location": "https://ontology.unifiedcyberontology.org/uco/location/",
        "xsd": "http://www.w3.org/2001/XMLSchema#"
    },
    "@graph": [
        {
            "@id": "kb:location-4511219e-a924-4ba5-aee7-dfad5a2c9c05",
            "@type": "uco-location:Location",
            "uco-core:hasFacet": [
                {
                    "@id": "kb:simple-address-facet-59334948-00b9-4370-85b0-4dc8e07f5384",
                    "@type": "uco-location:SimpleAddressFacet",
                    "uco-location:locality": "Seattle",
                    "uco-location:region": "WA",
                    "uco-location:postalCode": "98052",
                    "uco-location:street": "20341 Whitworth Institute 405 N. Whitworth"
                },
                {
                    "@id": "kb:acme-internal-location-facet-41fb3158-bbab-404d-97e4-ac61debb71f3",
                    "@type": [
                        "acme:InternalLocationFacet",
                        "uco-core:Facet"
                    ],
                    "acme:floor": 3,
                    "acme:roomNumber": 345
                }
            ]
        },
        {
            "@id": "kb:location-b579264d-6e30-4055-bf9b-72390364f224",
            "@type": "uco-location:Location",
            "uco-core:hasFacet": [
                {
                    "@id": "kb:simple-address-facet-258f169e-1e9c-4936-ba65-eed0f0c60788",
                    "@type": "uco-location:SimpleAddressFacet",
                    "uco-location:locality": "Paris",
                    "uco-location:country": "France",
                    "uco-location:postalCode": "F-75002",
                    "uco-location:street": "38 Bad Guy Headquarters st."
                },
                {
                    "@id": "kb:lat-long-coordinates-facet-36126f9c-0273-48fe-ad4d-6a4e2848458f",
                    "@type": "uco-location:LatLongCoordinatesFacet",
                    "uco-location:latitude": {
                        "@type": "xsd:decimal",
                        "@value": "48.860346"
                    },
                    "uco-location:longitude": {
                        "@type": "xsd:decimal",
                        "@value": "2.331199"
                    }
                }
            ]
        }
    ]
}
"#;

        // Parse the file.
        let value = Value::parse_str(file_contents)
            .expect("unable to parse file")
            .0;

        // Create a "remote" document by parsing a file manually.
        let input = RemoteDocument::new(
            // We use `IriBuf` as IRI type.
            Some(iri!("https://example.com/sample.jsonld").to_owned()),
            // Optional content type.
            Some("application/ld+json".parse().unwrap()),
            value,
        );

        // Use `NoLoader` as we won't need to load any remote document.
        let mut loader = json_ld::NoLoader;

        // Expand the "remote" document.
        let expanded = input.expand(&mut loader).await.expect("expansion failed");

        let mut expected = HashSet::new();
        let mut computed = HashSet::new();
        expected.insert(
            "http://example.org/kb/location-4511219e-a924-4ba5-aee7-dfad5a2c9c05".to_string(),
        );
        expected.insert(
            "http://example.org/kb/location-b579264d-6e30-4055-bf9b-72390364f224".to_string(),
        );

        for object in expanded {
            if let Some(id) = object.id() {
                computed.insert(id.as_iri().unwrap().to_string());
            }
        }
        assert_eq!(expected, computed,);
    }

    #[test]
    /// Adapted from [example source](https://docs.rs/oxigraph/0.4.3/oxigraph/store/struct.Store.html#method.insert)
    fn store_insert_adapted_example() -> Result<(), Box<dyn std::error::Error>> {
        let n_kb_location = NamedNodeRef::new(
            "http://example.org/kb/location-4511219e-a924-4ba5-aee7-dfad5a2c9c05",
        )?;
        let n_rdf_type = NamedNodeRef::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")?;
        let n_uco_location_location =
            NamedNodeRef::new("https://ontology.unifiedcyberontology.org/uco/location/Location")?;

        // Create three quads from the same triple-parts.
        let quad0 = QuadRef::new(
            n_kb_location,
            n_rdf_type,
            n_uco_location_location,
            GraphNameRef::DefaultGraph,
        );
        let quad1 = QuadRef::new(
            n_kb_location,
            n_rdf_type,
            n_uco_location_location,
            GraphNameRef::DefaultGraph,
        );
        let quad2 = QuadRef::new(
            n_kb_location,
            n_rdf_type,
            n_uco_location_location,
            GraphNameRef::DefaultGraph,
        );

        let store = Store::new()?;
        assert!(store.insert(quad0)?);
        assert!(!store.insert(quad1)?);

        assert!(store.contains(quad2)?);
        Result::<_, Box<dyn std::error::Error>>::Ok(())
    }

    #[test]
    /// Adapted from [example source](https://docs.rs/oxigraph/0.4.3/oxigraph/store/struct.Store.html#method.query)
    fn store_query_adapted_example() -> Result<(), Box<dyn std::error::Error>> {
        let store = Store::new()?;

        let query = r#"
PREFIX uco-location: <https://ontology.unifiedcyberontology.org/uco/location/>
SELECT ?nLocation
WHERE {
	?nLocation
		a uco-location:Location ;
		.
}
"#;

        // insertions
        let n_kb_location = NamedNodeRef::new(
            "http://example.org/kb/location-4511219e-a924-4ba5-aee7-dfad5a2c9c05",
        )?;
        let n_rdf_type = NamedNodeRef::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")?;
        let n_uco_location_location =
            NamedNodeRef::new("https://ontology.unifiedcyberontology.org/uco/location/Location")?;
        // NOTE: store.insert only takes a QuadRef, not a TripleRef.
        store.insert(QuadRef::new(
            n_kb_location,
            n_rdf_type,
            n_uco_location_location,
            GraphNameRef::DefaultGraph,
        ))?;

        // SPARQL query
        if let QueryResults::Solutions(mut solutions) = store.query(query)? {
            assert_eq!(
                solutions.next().unwrap()?.get("nLocation"),
                Some(&n_kb_location.into_owned().into())
            );
        }
        Result::<_, Box<dyn std::error::Error>>::Ok(())
    }
}
