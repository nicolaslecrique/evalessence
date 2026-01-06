# EvalEssence

## Vision

* "Agnostic" regarding workflow, use-case, framework, programming language  
* All features are accessible by code / Rest API / UI
* The package provides the UI structure and default "json" view of client data structures
* The user is able to vibe-code it's own rendering component (as react components or html ?) to annotate and vizualize its own data structure
* The user can define it's own metrics (both for aggregation and individual sample) using "https://rhai.rs"
* It is installable as a standard package install (pip install in python) and startable as a single command line
* It can run on the developper computer, or as a server
* EvalEssence provides infrastructure (storage, versionning, search & filter) and defaults (dashboards, views, metrics, judges)


# MVP Specifications

* User pip install evalessence
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