# EvalEssence

## Vision

* "Agnostic" regarding workflow, use-case, framework, programming language  
* All features are accessible by code / Rest API / UI
* The package provides the UI structure and default "json" view of client data structures
* The user is able to vibe-code it's own rendering component (as react components or html ?) to annotate and vizualize its own data structure
* The user can define it's own metrics (both for aggregation and individual sample) using a scripting language (lua?)
* It is installable as a simple binary and runnable as a simple command line
* It can run on the developper computer, or as a server
* Versioning is done through data transformation (minijinja?)
* EvalEssence provides infrastructure (UI template, storage, versionning, search & filter, auth...) and defaults (dashboards, views, metrics, judges)
* est practices made easy (data lineage, versionning, reproductibilty, metrics)

## MVP Specifications

* User install evalessence
* User start its own Rest API exposing a OpenAPI documentation
* Using the web interface, the user:
    * Create an Evaluation pipeline
    * select the evalessence pipeline
    * see the list of samples in a grid, add /update / delete, and associate an annotation
    * run the pipeline to generate outputs ("experiment")
    * run evaluations on the "experiment" (to run multiple evals if needed)
    * display an experiment (list of input / output)
    * display an evaluation (list of input / output / metrics)

## Components

* Id, Name, Version on all objects.

* App (business concept == one codebase)
    * Envs: list
        * Url
    * Pipelines: list
        * Dataset: List of json pairs input / label, (imported from a jsonl file)
        * Route
        * Dataset Input -> Route Input Adapter as MiniJinja Script
        * List of metrics: Script in Lua
        * List of filters (duckdb "where" clause)
        * List of aggregations: Script in Lua
    * Experiment: list
        * Pipeline Template
        * Env (in list)
        * Dataset filters (sublist)
        * Metrics (sublist)
        * Aggregation (sublist)
        * results

## Detailed UI specifications

### Create / Select app

* a list or square with the name of apps
* a "plus" square with a textbox with the name of the app

### Side bar, after App is selected

* envs
* Pipelines
* Experiments

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

* LLM Judge function available to lua
* judge alignments
* versionning and migrations of inputs / annotations
* Aggregation
* Filtering and search

scripts to study: https://github.com/khvzak/script-bench-rs