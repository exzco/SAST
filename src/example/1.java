package org.owasp.benchmark.report.sonarqube;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;

public class SonarReport {
    private static final String SONAR_USER = "admin";
    private static final String SONAR_PASSWORD = "P4ssword!!!!";

    private static String resultFilename() throws Exception {
        return "Benchmark_" + benchmarkVersion() + "-sonarqube-v" + apiCall("server/version");
    }
    public static class a {
        
    }
}
