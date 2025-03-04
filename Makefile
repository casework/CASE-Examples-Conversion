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

PYTHON3 ?= python3

all:

all-schema:
	$(MAKE) \
	  --directory schema

.venv.done.log: \
  requirements.txt
	rm -rf venv
	$(PYTHON3) -m venv \
	  venv
	source venv/bin/activate && pip install --upgrade pip
	source venv/bin/activate && pip install --requirement requirements.txt
	touch $@

check: \
  check-output

check-output: \
  .venv.done.log \
  all-schema
	$(MAKE) \
	  --directory output \
	  check

clean:
	@rm -f \
	  .venv.done.log
	@rm -rf \
	  venv
