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

class ExperienceConfig(BaseModel, frozen=True):
    pass

class ExperienceData(BaseModel, frozen=True):
    pass

class AggregatedResult(BaseModel, frozen=True):
    pass



class Evaluation[
    TSampleInput: SampleInput,
    TSampleAnnotation: SampleAnnotation,
    TSampleResult: SampleResult,
    TSampleComparison: SampleComparison,
    TExperienceConfig: ExperienceConfig,
    TExperienceData: ExperienceData,
    ](ABC):

    @abstractmethod
    def runExperience(
        self,
        config: TExperienceConfig,
        data: TExperienceData,
        inputs: list[TSampleInput],
    ) -> list[TSampleResult]:
        pass

    @abstractmethod
    def evaluateResult(
        self,
        input: TSampleInput,
        annotation: TSampleAnnotation,
        results: TSampleResult) -> TSampleComparison:
        pass

    @abstractmethod
    def computeAggregatedResult(
        self,
        comparisons: list[TSampleComparison],
    ) -> AggregatedResult:
        pass