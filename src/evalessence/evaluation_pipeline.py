from evalessence.interfaces import ConfigSetup, ResultAggregator, ResultEvaluator, SampleRunner


from pydantic import BaseModel


from dataclasses import dataclass
from typing import Type


@dataclass
class EvaluationPipeline[
    TSampleInput: BaseModel,
    TSampleAnnotation: BaseModel,
    TSampleResult: BaseModel,
    TSampleEvaluation: BaseModel,
    TExperimentConfig: BaseModel,
]:

    sample_runner: SampleRunner[TSampleInput, TSampleResult]
    result_evaluator: ResultEvaluator[
        TSampleInput, TSampleAnnotation, TSampleResult, TSampleEvaluation
    ]
    result_aggregator:ResultAggregator[TSampleInput, TSampleAnnotation, TSampleResult, TSampleEvaluation
    ]

    config_setup: ConfigSetup[TExperimentConfig]

    SampleInput: Type[TSampleInput] | None = None
    SampleAnnotation: Type[TSampleAnnotation] | None = None
    SampleResult: Type[TSampleResult] | None = None
    SampleEvaluation: Type[TSampleEvaluation] | None = None
    ExperimentConfig: Type[TExperimentConfig] | None = None


    async def run_samples(self, config: TExperimentConfig, samples: list[TSampleInput]) -> list[TSampleResult]:
        async with self.config_setup(config) as config_setup_id:
            return [
            await self.sample_runner(config_setup_id, sample)
            for sample in samples
            ]
        
    async def evaluate_results(
        self,
        results: list[tuple[TSampleInput, TSampleAnnotation, TSampleResult]],
    ) -> list[TSampleEvaluation]:
        return [
            await self.result_evaluator(input, annotation, result)
            for input, annotation, result in results
        ]

    async def aggregate_evaluations(
        self,
        evaluations: list[
            tuple[
                TSampleInput,
                TSampleAnnotation,
                TSampleResult,
                TSampleEvaluation,
            ]
        ],
    ):
        return await self.result_aggregator(evaluations)