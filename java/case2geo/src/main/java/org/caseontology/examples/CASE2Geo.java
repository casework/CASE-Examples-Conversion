package org.caseontology.examples;

import java.io.File;
import java.io.FileWriter;
import java.io.IOException;

/**
 * The primary entrypoint for the CASE2GeoJSON application. This class is a simple command line application that
 * accepts two positional arguments: the path to an input file and the path to an output file. The input file is
 * expected to be an RDF graph containing location information. The output file will be a GeoJSON representation of
 * the input file.
 */
public class CASE2Geo {
    public static void main(String[] args) {
        // Ensure two arguments were provided and save them as the input and output
        // paths
        if (args.length != 2) {
            System.out.println("Usage: java -jar CASE2GeoJSON.jar <inputPath> <outputPath>");
            System.exit(1);
        }

        // Save the input and output paths
        String inputPath = args[0];
        String outputPath = args[1];

        // Ensure the inputPath exists
        File inputFile = new File(inputPath);
        if (!inputFile.exists()) {
            System.out.println("Input file does not exist.");
            System.exit(1);
        }

        // Ensure the directory containing the outputPath exists
        File outputFile = new File(outputPath);
        File outputDirectory = outputFile.getParentFile();
        if (!outputDirectory.exists()) {
            System.out.println("Output directory does not exist.");
            System.exit(1);
        }

        // Build an RDF graph from the input file and convert it to GeoJSON
        GeoReader reader = new GeoReader(inputPath);
        String geoJSON = reader.run();

        // Write the GeoJSON to the output file
        try {
            FileWriter fileWriter = new FileWriter(outputPath);
            fileWriter.write(geoJSON);
            fileWriter.close();
        } catch (IOException e) {
            System.out.println("Error writing output: " + e.getMessage());
            System.exit(1);
        }
    }
}
