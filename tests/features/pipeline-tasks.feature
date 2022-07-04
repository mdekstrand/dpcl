Feature: pipeline task management

  Scenario: Empty Pipeline
    Given a fresh pipeline
    Then the pipeline has no tasks
    And the pipeline has no artifacts
