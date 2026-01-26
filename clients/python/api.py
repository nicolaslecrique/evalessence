
from abc import ABC
from dataclasses import dataclass
from typing import NewType
from uuid import UUID


DatasetId = NewType('DatasetId', str)
EnvId = NewType('EnvId', str)
PipelineId = NewType('PipelineId', str)
AppId = NewType('AppId', str)
ExperimentId = NewType('ExperimentId', str)


@dataclass
class Dataset:
    id: DatasetId
    name: str

@dataclass
class Env:
    id: EnvId
    url: str

@dataclass
class Pipeline:
    id: PipelineId
    route: str
    dataset: Dataset


@dataclass
class AppHeader:
    id: AppId
    name: str

# One yaml file by App, ({app_id}.yaml)
@dataclass
class App:
    id: AppId
    version: UUID # to prevent concurrent updates or Experiment started on an deprecated version of the App.
    name: str
    envs: list[Env]
    pipelines: list[Pipeline]

@dataclass
class Experiment:
    app_id: str
    app_version: str
    pipeline_id: str


class Evalessence(ABC):

    def get_app_list(self) -> list[AppHeader]:
        raise NotImplementedError()

    def create_app(self, name: str) -> AppId:
        raise NotImplementedError()

    def get_app(self, app_id: AppId) -> App:
        raise NotImplementedError()
    
    def delete_app(self, app_id: AppId, app_version: UUID) -> None:
        raise NotImplementedError()

    def update_app(self, app: App) -> App:
        raise NotImplementedError()

    def run_experiment(self, experiment: Experiment) -> ExperimentId:
        raise NotImplementedError()

    def get_experiment(sed, experiment_id:ExperimentId) -> TODO