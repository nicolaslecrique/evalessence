from contextlib import asynccontextmanager
from dataclasses import dataclass
from typing import AsyncIterator, NewType, Protocol
from pydantic import BaseModel
from abc import abstractmethod, ABC


class SampleInput(BaseModel, frozen=True):
    pass


class SampleAnnotation(BaseModel, frozen=True):
    pass


class SampleResult(BaseModel, frozen=True):
    pass


class SampleEvaluation(BaseModel, frozen=True):
    pass


class ExperimentConfig(BaseModel, frozen=True):
    pass


class ExperimentData(BaseModel, frozen=True):
    pass


class AggregatedResult(BaseModel, frozen=True):
    label: str


class FloatResult(AggregatedResult, frozen=True):
    value: float


class Sample[
    TSampleInput: SampleInput,
    TSampleAnnotation: SampleAnnotation,
    TSampleResult: SampleResult,
    TSampleEvaluation: SampleEvaluation,
](BaseModel, frozen=True):
    input: TSampleInput
    annotation: TSampleAnnotation
    result: TSampleResult
    comparison: TSampleEvaluation


DataSetupId = NewType("DataSetupId", str)
ConfigSetupId = NewType("ConfigSetupId", str)


class DataSetup[TExperimentData: ExperimentData](Protocol):
    @abstractmethod
    @asynccontextmanager
    async def __call__(self, data: TExperimentData) -> AsyncIterator[DataSetupId]:
        yield DataSetupId("")


class ConfigSetup[TExperimentConfig: ExperimentConfig](Protocol):
    @abstractmethod
    @asynccontextmanager
    async def __call__(
        self,
        config: TExperimentConfig,
    ) -> AsyncIterator[ConfigSetupId]:
        yield ConfigSetupId("")


class SampleRunner[
    TSampleInput: SampleInput,
    TSampleResult: SampleResult,
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
    TSampleInput: SampleInput,
    TSampleAnnotation: SampleAnnotation,
    TSampleResult: SampleResult,
    TSampleEvaluation: SampleEvaluation,
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
    TSampleInput: SampleInput,
    TSampleAnnotation: SampleAnnotation,
    TSampleResult: SampleResult,
    TSampleEvaluation: SampleEvaluation,
](Protocol):
    @abstractmethod
    async def __call__(
        self,
        results: list[
            Sample[TSampleInput, TSampleAnnotation, TSampleResult, TSampleEvaluation]
        ],
    ) -> AggregatedResult:
        pass


@dataclass(frozen=True)
class EvaluationPipeline[
    TSampleInput: SampleInput,
    TSampleAnnotation: SampleAnnotation,
    TSampleResult: SampleResult,
    TSampleEvaluation: SampleEvaluation,
    TExperimentConfig: ExperimentConfig,
    TExperimentData: ExperimentData,
](BaseModel):
    data_setup: DataSetup[TExperimentData]
    config_setup: ConfigSetup[TExperimentConfig]
    sample_runner: SampleRunner[TSampleInput, TSampleResult]
    result_evaluator: ResultEvaluator[
        TSampleInput, TSampleAnnotation, TSampleResult, TSampleEvaluation
    ]
    result_aggregators: list[
        ResultAggregator[
            TSampleInput, TSampleAnnotation, TSampleResult, TSampleEvaluation
        ]
    ]
