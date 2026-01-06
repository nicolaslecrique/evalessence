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
* User implement
    - interface "run_sample(input) -> output"
    - a main.py with evalessence_pipeline = Evalessence(run_sample, "my_pipeline")
* User start evalessence and the fastAPI server.
* Using the web interface, the user can:
    * select evalessence pipeline
    * see the list of samples in a grid, add /update / delete
    * run the pipeline -> get an "experiment
    * select the experiment -> get the list of samples with answers
    * add / update / delete judges (prompt), return type OK / KO
    * run evaluation on experiment -> get judge values for all samples
-








# Future work

* How to manage judge alignments ?
* How to manage input structure updates ?
* How to manage custom metrics ?
* How to manage annotations ?
