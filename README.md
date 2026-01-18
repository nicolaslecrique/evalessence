# EvalEssence

## Vision

* (V0) "Agnostic" regarding workflow, use-case, framework, programming language  
* (V0) All features are accessible by code / Rest API / UI
* (V0) The package provides the UI structure and default "json" view of client data structures
* (Later) The user is able to vibe-code it's own rendering component (as react components or html ?) to annotate and vizualize its own data structure
* (V0) The user can define it's own metrics (both for aggregation and individual sample) using "https://rhai.rs"
* (V0) It is installable as a simple binary and runnable as a simple command line
* (V0) It can run on the developper computer, or as a server
* Versioning is done through data transformation (minijinja or rhai ?)
* (Later) The concept is that EvalEssence provides infrastructure (UI template, storage, versionning, search & filter, auth...) and defaults (dashboards, views, metrics, judges)


## MVP Specifications

* User install evalessence
* User start its own Rest API exposing a OpenAPI documentation
* Using the web interface, the user:
    * Create an Evaluation pipeline by specifying a Post url, and an annotation format (as a json format), metrics as rhai functions of (input, output, annotation)
    * select the evalessence pipeline
    * see the list of samples in a grid, add /update / delete, and associate an annotation
    * run the pipeline to generate outputs ("experiment")
    * run evaluations on the "experiment" (to run multiple evals if needed)
    * display an experiment (list of input / output)
    * display an evaluation (list of input / output / metrics)

## Components

* App (business concept == one codebase)
    * Name
* Env:
    * Name
    * Url
* Format
    * Name
    * JSON Schema
* Adapter
    * Name
    * Input Format
    * Output Format
    * MiniJinja Script
* Dataset:
    * Name
    * Input Format
    * Label Format
    * List of pairs input / output
* Dataset Filter:
    * Name
    * datafusion sql "where" clause
* Metric
    * Name
    * Format
    * Output type
    * Script in Lua
* Experiment
    * Env
    * Dataset
    * Dataset filter
    * Route
    * Adapter
* Evaluation
    * Experiment
    * List of Metrics



## Detailed UI specifications

### Create / Select app

* a list or square with the name of apps
* a "plus" square with a textbox with the name of the app

### Side bar

* Environments
* Dataset




### Experiment

* choose a root


### index

* a list or square with the name of apps and the URL
* a "plus" square with a textbox with the name of the app
* If there is alreday an app (and only one), go to the "app" page

### App

* a list of square with the name of the "envs
* a "plus" square with a textbox with the name of the env



# Future work

* LLM Judge function available to rhai
* judge alignments
* versionning and migrations of inputs / annotations
* Aggregation
* Filtering and search

scripts to study: https://github.com/khvzak/script-bench-rs