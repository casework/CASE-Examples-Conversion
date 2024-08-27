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

top_srcdir := ..

geojson_files := $(wildcard *.geojson)

validation_files := $(foreach geojson_file,$(geojson_files),.$(geojson_file).validation.done.log)

all:

.%.validation.done.log: \
  % \
  $(top_srcdir)/.venv.done.log \
  $(top_srcdir)/schema/GeoJSON.json
	source $(top_srcdir)/venv/bin/activate \
	  && check-jsonschema \
	    --schemafile $(top_srcdir)/schema/GeoJSON.json \
	    $<
	touch $@

check: \
  $(validation_files)

clean:
	@rm -f \
	  .*.validation.done.log
