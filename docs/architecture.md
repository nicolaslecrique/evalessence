# Architecture

## Objective

* Be able to work in local or with a centralized server
* Be able to work with API, Rest API, or UI

## Storage

* yaml files for Pipeline configs (rust-yaml ?)
* Lancedb (datasets, experiments, config and data snapshots)

## Crates and packages

* `evalessence-api`: expose an minimal interface (set of traits), not made to be user friendly, but ready to be ffi-ed (stateless functions) by language-specific clients
* `evalessence-core`: implement `api` and define the core logic of the application
* `evalessence-server` expose a Rest API server wrapping the `api`
* `evalessence-remote`: implement `api` by making requests to `server`
* `clients/rust`, `clients/python`...: idiomatic bindings in each language, wrapping an `api` implementation, just called `evalessence` from the point of view of the language specific client user.

## UI

* react / tanstack-router / Mantine