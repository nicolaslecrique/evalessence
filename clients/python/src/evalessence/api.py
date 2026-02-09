
from abc import ABC
from pydantic import BaseModel
from typing import AsyncGenerator, Iterable, TypeAlias, Literal, Any
from enum import Enum
from typing import TypeAlias
import pyarrow as pa 

# All ids are immutable and generated at the ressource creation (based on the name provided and a random value to ensure unicity)

# --- App properties, stored in yaml file ---

class Dataset(BaseModel):
    id: str
    name: str

class Env(BaseModel):
    id: str
    url: str
    name: str

class Pipeline(BaseModel):
    id: str
    name: str
    route: str
    env_id: str
    dataset_id: str

class AppKey(BaseModel):
    id: str
    version: int # to prevent concurrent updates or Experiment started on an deprecated version of the App.


# One yaml file by App, ({app_id}.yaml)
class App(BaseModel):
    key: AppKey
    name: str
    envs: list[Env]
    datasets: list[Dataset]
    pipelines: list[Pipeline]


# -------- add-hoc structure for API

class AppHeader(BaseModel):
    id: str
    name: str

# -------- App

class AppServices(ABC):

    async def list(self) -> list[AppHeader]: ...
    async def create(self, name: str) -> App:...
    async def get(self, app_id: str) -> App:...
    async def delete(self, app_id: str) -> None:...
    async def update(self, app: App) -> App:...



# --- Experiments and Dataset content, stored in lancedb ----


JSONValue: TypeAlias = (
    dict[str, "JSONValue"] 
    | list["JSONValue"] 
    | str 
    | int 
    | float 
    | bool 
    | None
)

class Sample(BaseModel):
    sample_id: str
    value: JSONValue

class SamplePage:
    items: pa.RecordBatchReader[Sample]
    cursor: Any | None # None if there is no more results to load
    total_count: int
    

class OrderDirection(str, Enum):
    ASC = "asc"
    DESC = "desc"


SampleSet =  pa.RecordBatchReader[Sample] | pa.RecordBatch[Sample] | pa.Table[Sample] | Iterable[Sample]
IdSet = pa.Array[str] | Iterable[str]


class DatasetServices(ABC):
    async def update(self, app_id: str, dataset_id: str, upsert_by_id: SampleSet, delete_by_id: IdSet) -> list[str]:... # create the table if it doesn't exists
    async def select(self, app_id: str, dataset_id: str, *, where: str | None, order_by: str = "id", order_direction: OrderDirection = OrderDirection.ASC, limit: int | None = None) -> SamplePage: ...
    async def select_next(self, app_id: str, dataset_id: str, *, cursor: Any, limit: int | None = None) -> SamplePage: ...





class ExperimentStatus(str, Enum):
    NOT_STARTED = "not_started"
    RUNNING = "running"
    COMPLETED = "completed"
    FAILED = "failed"

class Experiment(BaseModel):
    id: str
    name: str
    pipeline_id: str
    dataset_version: int # table version in lancedb
    app_snapshot: App
    status: ExperimentStatus

class ExperimentSampleResult(BaseModel):
    sample_id: str
    input: JSONValue
    result: JSONValue


class ExperimentSampleResultPage:
    items: pa.RecordBatchReader[ExperimentSampleResult]
    cursor: Any | None # None if there is no more results to load
    total_count: int


class ExperimentServices(ABC):

    async def create(self, app_key: AppKey, pipeline_id: str, name: str) -> str: ...
    async def run(self, experiment_id: str) -> None:... # start or continue the experiment, no-op if already started
    async def list(self, app_id: str) -> list[Experiment]:...
    async def get(self, experiment_id: str) -> Experiment:...

    async def select(self, experiment_id: str, *, where: str | None, order_by: str = "id", order_direction: OrderDirection = OrderDirection.ASC, limit: int | None = None) -> ExperimentSampleResultPage: ...
    async def select_next(self, experiment_id: str, *, cursor: Any, limit: int | None = None) -> ExperimentSampleResultPage: ...
    async def stream(self, experiment_id: str) -> AsyncGenerator[ExperimentSampleResult]: ...

