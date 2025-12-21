from evalessence.interfaces import ExperimentConfig, ExperimentData, ExperimentId, SampleInput, SampleAnnotation, SampleResult, SampleComparison, ExperienceConfig, ExperienceData, EvaluationPipeline
from dataclasses import dataclass, field
from typing import Any, Dict, Iterable, List, Tuple


class MySampleInput(SampleInput, frozen=True):
    user_message: str

class MySampleAnnotation(SampleAnnotation, frozen=True):
    expected_answer: str

class MySampleResult(SampleResult, frozen=True):
    assistant_reply: str

class MySampleComparison(SampleComparison, frozen=True):
    is_equivalent: bool

class MyExperimentConfig(ExperimentConfig, frozen=True):
    model_name: str

class MyExperimentData(ExperimentData, frozen=True):
    faq_entries: List[Tuple[str, str]]  # List of (question, answer) pairs


class MyEvaluationPipeline(
    EvaluationPipeline[
        MySampleInput,
        MySampleAnnotation,
        MySampleResult,
        MySampleComparison,
        MyExperimentConfig,
        MyExperimentData,
    ]
):

    def init_experiment(
        self,
        config: MyExperimentConfig,
        data: MyExperimentData
    ) -> ExperimentId:
        # Initialize experiment with model and data
        return ExperimentId("exp_001")

    def run_sample(
        self,
        experiment_id: ExperimentId,
        input: MySampleInput,
    ) -> MySampleResult:
        # Simulate running the model to get a response
        reply = f"Simulated reply to: {input.user_message}"
        return MySampleResult(assistant_reply=reply)

    def evaluate_result(
        self,
        input: MySampleInput,
        annotation: MySampleAnnotation,
        results: MySampleResult
    ) -> MySampleComparison:
        # Compare the model's reply to the expected answer
        is_equivalent = results.assistant_reply == annotation.expected_answer
        return MySampleComparison(is_equivalent=is_equivalent)