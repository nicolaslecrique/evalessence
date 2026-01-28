
from abc import ABC
from dataclasses import dataclass
from io import StringIO
from typing import AsyncGenerator, TypeAlias, Sequence, Literal
from typing import TypeAlias
# All ids are immutable and generated at the ressource creation (based on the name provided and a random value to ensure unicity)

# --- App properties, stored in yaml file ---

@dataclass
class Dataset:
    id: str
    name: str

@dataclass
class Env:
    id: str
    url: str
    name: str

@dataclass
class Pipeline:
    id: str
    name: str
    route: str
    dataset_id: str


@dataclass
class AppHeader:
    id: str
    name: str

# One yaml file by App, ({app_id}.yaml)
@dataclass
class App:
    id: str
    version: int # to prevent concurrent updates or Experiment started on an deprecated version of the App.
    name: str
    envs: list[Env]
    datasets: list[Dataset]
    pipelines: list[Pipeline]


# --- Experiments and Dataset content, stored in lancedb ----

@dataclass
class ExperimentConfig:
    app_id: str
    app_version: int
    pipeline_id: str
    name: str

@dataclass
class Experiment:
    id: str
    config: ExperimentConfig
    status: Literal["not_started","running","completed","stopped", "failed"]

JSONValue: TypeAlias = (
    dict[str, "JSONValue"] 
    | list["JSONValue"] 
    | str 
    | int 
    | float 
    | bool 
    | None
)

class ExperimentSampleResult:
    sample_id: str
    result: JSONValue

@dataclass
class ExperimentPaginatedResults:
    total_count: int
    items: list[ExperimentSampleResult]
    next_cursor: str | None

class Evalessence(ABC):

    # ---- App -----
    async def list_apps(self) -> list[AppHeader]:
        raise NotImplementedError()

    async def create_app(self, name: str) -> App:
        raise NotImplementedError()

    async def get_app(self, app_id: str) -> App:
        raise NotImplementedError()
    
    async def delete_app(self, app_id: str, app_version: int) -> None:
        raise NotImplementedError()

    async def update_app(self, app: App) -> App:
        raise NotImplementedError()

    # --- Datasets and experiments ---

    # TODO: at the level of the Code API, it should probably be a list ?
    async def upload_file_to_dataset(self, app_id: str, dataset_id: str, file: StringIO) -> None:
        "file must be a jsonl file, where each entry has a unique 'sample_id' key"
        raise NotImplementedError()

    async def run_experiment(self, experiment: ExperimentConfig) -> Experiment:
        raise NotImplementedError()
    
    async def list_experiments(self, app_id: str, pipeline_id: str) -> list[Experiment]:
        raise NotImplementedError()
    
    async def get_experiment(self, app_id: str, experiment_id: str) -> Experiment:
        raise NotImplementedError()

    async def stream_experiment_results(self, app_id: str, experiment_id: str) -> AsyncGenerator[ExperimentSampleResult, None]:
        raise NotImplementedError()
    
    async def load_experiment_results(
        self, 
        experiment_id: str, 
        limit: int = 100, 
        cursor: str | None = None
    ) -> ExperimentPaginatedResults:
        raise NotImplementedError()