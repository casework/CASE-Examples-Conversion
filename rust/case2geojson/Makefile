#!/usr/bin/make -f

# Portions of this file contributed by NIST are governed by the
# following statement:
#
# This software was developed at the National Institute of Standards
# and Technology by employees of the Federal Government in the course
# of their official duties. Pursuant to Title 17 Section 105 of the
# United States Code, this software is not subject to copyright
# protection within the United States. NIST assumes no responsibility
# whatsoever for its use by other parties, and makes no guarantees,
# expressed or implied, about its quality, reliability, or any other
# characteristic.
#
# We would appreciate acknowledgement if the software is used.

SHELL := /bin/bash

top_srcdir := ../..

all: \
  $(top_srcdir)/output/rust.geojson

.PHONY: \
  check-supply-chain \
  format

$(top_srcdir)/output/rust.geojson: \
  output.geojson
	cp $< $@

check: \
  all

check-supply-chain:
	cargo update

clean:
	@rm -f \
	  Cargo.lock
	@rm -r \
	  target

format:
	cargo fmt

output.geojson: \
  $(top_srcdir)/.venv.done.log \
  $(top_srcdir)/data/geo.json \
  $(top_srcdir)/schema/GeoJSON.json \
  target/debug/case2geojson
	target/debug/case2geojson \
	  $(top_srcdir)/data/geo.json \
	  > $@__
	python3 -m json.tool $@__ $@_
	rm $@__
	source $(top_srcdir)/venv/bin/activate \
	  && check-jsonschema \
	    --schemafile $(top_srcdir)/schema/GeoJSON.json \
	    $@_
	mv $@_ $@

target/debug/case2geojson: \
  Cargo.toml \
  src/main.rs
	cargo check
	cargo test
	cargo build
