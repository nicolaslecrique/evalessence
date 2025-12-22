from contextlib import asynccontextmanager
from dataclasses import dataclass
from typing import AsyncIterator, NewType, Protocol
from pydantic import BaseModel
from abc import abstractmethod
from pandas import DataFrame

AggregatedResult = float | DataFrame

class Sample[
    TSampleInput: BaseModel,
    TSampleAnnotation: BaseModel,
    TSampleResult: BaseModel,
    TSampleEvaluation: BaseModel,
](BaseModel, frozen=True):
    input: TSampleInput
    annotation: TSampleAnnotation
    result: TSampleResult
    comparison: TSampleEvaluation


DataSetupId = NewType("DataSetupId", str)
ConfigSetupId = NewType("ConfigSetupId", str)


class DataSetup[TExperimentData: BaseModel](Protocol):
    @abstractmethod
    @asynccontextmanager
    async def __call__(self, data: TExperimentData) -> AsyncIterator[DataSetupId]:
        yield DataSetupId("")


class ConfigSetup[TExperimentConfig: BaseModel](Protocol):
    @abstractmethod
    @asynccontextmanager
    async def __call__(
        self,
        config: TExperimentConfig,
    ) -> AsyncIterator[ConfigSetupId]:
        yield ConfigSetupId("")


class SampleRunner[
    TSampleInput: BaseModel,
    TSampleResult: BaseModel,
](Protocol):
    @abstractmethod
    async def __call__(
        self,
        data_setup_id: DataSetupId,
        config_setup_id: ConfigSetupId,
        input: TSampleInput,
    ) -> TSampleResult:
        pass


class ResultEvaluator[
    TSampleInput: BaseModel,
    TSampleAnnotation: BaseModel,
    TSampleResult: BaseModel,
    TSampleEvaluation: BaseModel,
](Protocol):
    @abstractmethod
    async def __call__(
        self,
        input: TSampleInput,
        annotation: TSampleAnnotation,
        result: TSampleResult,
    ) -> TSampleEvaluation:
        pass


class ResultAggregator[
    TSampleInput: BaseModel,
    TSampleAnnotation: BaseModel,
    TSampleResult: BaseModel,
    TSampleEvaluation: BaseModel,
](Protocol):
    @abstractmethod
    async def __call__(
        self,
        results: list[
            Sample[TSampleInput, TSampleAnnotation, TSampleResult, TSampleEvaluation]
        ],
    ) -> dict[str, AggregatedResult]:
        pass


@dataclass
class EvaluationPipeline[
    TSampleInput: BaseModel,
    TSampleAnnotation: BaseModel,
    TSampleResult: BaseModel,
    TSampleEvaluation: BaseModel,
    TExperimentConfig: BaseModel,
    TExperimentData: BaseModel,
]:
    data_setup: DataSetup[TExperimentData]
    config_setup: ConfigSetup[TExperimentConfig]
    sample_runner: SampleRunner[TSampleInput, TSampleResult]
    result_evaluator: ResultEvaluator[
        TSampleInput, TSampleAnnotation, TSampleResult, TSampleEvaluation
    ]
    result_aggregator:ResultAggregator[TSampleInput, TSampleAnnotation, TSampleResult, TSampleEvaluation
    ]
