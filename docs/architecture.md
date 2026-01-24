# Architecture

## Objective

* Be able to work in local or with a centralized server
* Be able to work with API, Rest API, or UI

## Implementation

* Storage:
    * yaml files for Pipeline configs (rust-yaml ?)
    * Lancedb (datasets, experiments, config and data snapshots)
* API definition: A rust API definition (that can be wrapper in any programming language)
* Local API: A rust API implementation manipulating lancedb
* Rest API, calling the Local API
* Remote API: implementing the API definition and calling the Rest API
* a SPA Web UI: calling the Rest API

