Feature: pipeline task management

  Scenario: Empty Pipeline
    Given a fresh pipeline
    Then the pipeline has 0 tasks
    And the pipeline has 0 artifacts

  Scenario: Adding a Task
    Given a fresh pipeline
    When an empty task named bob is added
    Then the pipeline has 1 tasks
    And the pipeline has 0 artifacts
    And the pipeline has a task named bob
    And the task bob has 0 dependencies
    And the task bob has 0 outputs
