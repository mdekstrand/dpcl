Feature: pipeline task management

  Scenario: Empty Pipeline
    Given a fresh pipeline
    Then the pipeline has 0 tasks
    And the pipeline has 0 artifacts
