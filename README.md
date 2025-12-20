# EvalEssence

# MVP Specifications

## V0


### Setup

Within its own service evaluated (or as a dedicated service), the user:
- Define the SampleInput structure in Pydantic
- Define the SampleAnnotation structure in Pydantic
- Define the SampleResult structure in Pydantic
- Define the SampleComparison structure un Pydantic
- Define the ExperienceConfig structure in Pydantic (with a default value)
- Define the ExperienceData structure in Pydantic
- Define the def runExperience(ExperienceConfig, ExperienceData, list[SampleInput]) -> list[SampleResult]
- Define the def evaluate(SampleInput, SampleAnnotation, SampleResult) -> SampleComparison
- Define the def computeMetrics(list[SampleComparison]) -> list[AggregatedResult]
with AggregatedResult = DataFrame, dict..

- Define EvalEssence config as json (or yaml?) (path to store all the experience data)

- Add a line in a startup script that will popup a running Service exposting a Rest API and an UI

### Features REST API

- add / remove / update a SampleInput / SampleAnnotation
- run the experience
- compute the result
- store everything

### Features UI

* Display the experiment
    * grid with columns as raw jsons:
        * the SampleInput
        * the SampleAnnotation
        * the SampleResult
        * the SampleComparison
    * List of widget, each one a AggregatedResult (just a dataFrame for now)



## Later
