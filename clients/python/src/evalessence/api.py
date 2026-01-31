
from abc import ABC
from dataclasses import dataclass
from io import StringIO
from re import ASCII
from typing import AsyncGenerator, TypeAlias, Sequence, Literal, Any, overload
from enum import Enum, auto
from typing import TypeAlias
import pyarrow as pa 

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

class Sample:
    sample_id: str
    value: JSONValue

class ExperimentSampleResult:
    sample_id: str
    result: JSONValue

@dataclass
class SamplePage:
    items: pa.RecordBatchReader[Sample]
    cursor: Any | None # None if there is no more results to load
    total_count: int
    

class OrderDirection(Enum):
    ASC = auto()
    DESC = auto()


class Evalessence(ABC):

    # ---- App -----
    async def list_apps(self) -> list[AppHeader]: ...
    async def create_app(self, name: str) -> App:...
    async def get_app(self, app_id: str) -> App:...
    async def delete_app(self, app_id: str) -> None:...
    async def update_app(self, app: App) -> App:...

    # --- Datasets ---

    async def update_dataset(self, app_id: str, dataset_id: str, upsert_by_id: pa.RecordBatchReader[Sample], delete_by_id: pa.Array[str]) -> list[str]:...
    async def select(self, app_id: str, dataset_id: str, *, where: str | None, order_by: str = "id", order_direction: OrderDirection = OrderDirection.ASC, limit: int | None = None) -> SamplePage: ...
    async def select_next(self, app_id: str, dataset_id: str, *, cursor: Any, limit: int | None = None) -> SamplePage: ...

    # ----- Experiments -----------

    async def run_experiment(self, experiment: ExperimentConfig) -> Experiment:...
    
    async def list_experiments(self, app_id: str, pipeline_id: str) -> list[Experiment]:...
    
    async def get_experiment(self, app_id: str, experiment_id: str) -> Experiment:...

    async def stream_experiment_results(self, app_id: str, experiment_id: str) -> AsyncGenerator[ExperimentSampleResult, None]:...
    
    async def load_experiment_results(
        self, 
        experiment_id: str, 
        limit: int = 100, 
        cursor: str | None = None
    ) -> ExperimentPaginatedResults:...