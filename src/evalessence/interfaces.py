from contextlib import asynccontextmanager
from typing import AsyncIterator, NewType, Protocol
from pydantic import BaseModel
from abc import abstractmethod
from pandas import DataFrame

AggregatedResult = float | DataFrame


ConfigSetupId = NewType("ConfigSetupId", str)

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
            tuple[TSampleInput, TSampleAnnotation, TSampleResult, TSampleEvaluation]
        ],
    ) -> dict[str, AggregatedResult]:
        pass




