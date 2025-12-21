from typing import NewType
from pydantic import BaseModel
from abc import abstractmethod, ABC 


class SampleInput(ABC, BaseModel, frozen=True):
    pass

class SampleAnnotation(ABC, BaseModel, frozen=True):
    pass

class SampleResult(ABC, BaseModel, frozen=True):
    pass

class SampleComparison(ABC, BaseModel, frozen=True):
    pass

class ExperimentConfig(BaseModel, frozen=True):
    pass

class ExperimentData(BaseModel, frozen=True):
    pass

class AggregatedResult(BaseModel, frozen=True):
    pass

ExperimentId = NewType('ExperimentId', str)

class EvaluationPipeline[
    TSampleInput: SampleInput,
    TSampleAnnotation: SampleAnnotation,
    TSampleResult: SampleResult,
    TSampleComparison: SampleComparison,
    TExperimentConfig: ExperimentConfig,
    TExperimentData: ExperimentData,
    ](ABC):

    @abstractmethod
    def init_experiment(
        self,
        config: TExperimentConfig,
        data: TExperimentData
        ) -> ExperimentId:
        pass

    @abstractmethod
    def run_sample(
        self,
        experiment_id: ExperimentId,
        input: TSampleInput,
    ) -> TSampleResult:
        pass

    @abstractmethod
    def evaluate_result(
        self,
        input: TSampleInput,
        annotation: TSampleAnnotation,
        results: TSampleResult) -> TSampleComparison:
        pass

    @abstractmethod
    def aggregate_results(
        self,
        comparisons: list[TSampleComparison],
    ) -> AggregatedResult:
        pass




