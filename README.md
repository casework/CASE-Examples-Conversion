# CASE Conversion Examples

[![Project Status: WIP – Initial development is in progress, but there has not yet been a stable, usable release suitable for the public.](https://www.repostatus.org/badges/latest/wip.svg)](https://www.repostatus.org/#wip)
![CASE Version](https://img.shields.io/badge/CASE%20Version-1.2.0-brightgreen.svg)

This repository provides example scripts for extracting information from CASE graphs and writing them to various output formats. This project contains several directories with the same general intent, but different implementations/languages. The directories are as follows:

- `dotnet` - C# implementation using dotNetRDF
- `java` - Java implementation
- `python` - Python implementation using RDFLib

Each directory contains the code to convert a CASE graph to a specific output format. The input and output files are specified as command line arguments. The input file should be a CASE graph in JSON-LD format. The output file will be written to a GeoJSON file.

Each directory contains a README with more information about the specific implementation as well as specific usage but the general usage expects the following two positional arguments:

| Argument | Description                                                |
| -------- | ---------------------------------------------------------- |
| input    | The input file containing the CASE graph in JSON-LD format |
| output   | The output file into which to write the GeoJSON file       |
