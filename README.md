# EvalEssence

## Vision

* (V0) "Agnostic" regarding workflow, use-case, framework, programming language  
* (V0) All features are accessible by code / Rest API / UI
* (V0) The package provides the UI structure and default "json" view of client data structures
* (Later) The user is able to vibe-code it's own rendering component (as react components or html ?) to annotate and vizualize its own data structure
* (V0) The user can define it's own metrics (both for aggregation and individual sample) using "https://rhai.rs"
* (V0) It is installable as a simple binary and runnable as a simple command line
* (V0) It can run on the developper computer, or as a server
* (Later) The concept is that EvalEssence provides infrastructure (UI template, storage, versionning, search & filter, auth...) and defaults (dashboards, views, metrics, judges)


# MVP Specifications

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
-
# Future work

* LLM Judge function available to rhai
* judge alignments
* versionning and migrations of inputs / annotations
* Aggregation
* Filtering and search