package org.caseontology.examples;

import java.io.File;

/**
 * Hello world!
 *
 */
public class CASE2Geo
{
    public static void main( String[] args )
    {
        // Ensure two arguments were provided and save them as the input and output paths
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

        // Build an RDF graph from the input file
        GeoReader reader = new GeoReader(inputPath);

        // Convert the RDF graph to GeoJSON
        // TODO
    }
}
