from evalessence.interfaces import AggregatedResult, ExperimentConfig, ExperimentData, FloatResult, Sample, SampleInput, SampleAnnotation, SampleResult, SampleEvaluation, DataSetupId, ConfigSetupId, EvaluationPipeline
from typing import AsyncIterator, List, Tuple
from contextlib import asynccontextmanager


class MySampleInput(SampleInput, frozen=True):
    user_message: str

class MySampleAnnotation(SampleAnnotation, frozen=True):
    expected_answer: str

class MySampleResult(SampleResult, frozen=True):
    assistant_reply: str

class MySampleEvaluation(SampleEvaluation, frozen=True):
    is_equivalent: bool

class MyExperimentConfig(ExperimentConfig, frozen=True):
    model_name: str

class MyExperimentData(ExperimentData, frozen=True):
    faq_entries: List[Tuple[str, str]]  # List of (question, answer) pairs

@asynccontextmanager
async def setup_data(
    data: MyExperimentData
) -> AsyncIterator[DataSetupId]:
    # Simulate data setup
    yield DataSetupId("my_data_setup")


@asynccontextmanager
async def setup_config(
    config: MyExperimentConfig,
) -> AsyncIterator[ConfigSetupId]:
    # Simulate config setup
        yield ConfigSetupId("my_config_setup")


async def run_sample(
    data_setup_id: DataSetupId,
    config_setup_id: ConfigSetupId,
    input: MySampleInput) -> MySampleResult:
        # Simulate running the sample
        return MySampleResult(assistant_reply=f"Response to: {input.user_message}")
    

async def evaluate_result(
    input: MySampleInput,
    annotation: MySampleAnnotation,
    result: MySampleResult,
) -> MySampleEvaluation:
    is_equivalent = annotation.expected_answer in result.assistant_reply
    return MySampleEvaluation(is_equivalent=is_equivalent) 


async def aggregate_results(
    results: List[Sample[MySampleInput, MySampleAnnotation, MySampleResult, MySampleEvaluation]],
) -> AggregatedResult:
    total = len(results)
    correct = sum(1 for eval in results if eval.comparison.is_equivalent)
    return FloatResult(label="accuracy", value=correct / total if total > 0 else 0.0)


my_eventuation_pipeline = EvaluationPipeline[
    MySampleInput,
    MySampleAnnotation,
    MySampleResult,
    MySampleEvaluation,
    MyExperimentConfig,
    MyExperimentData,
](
    data_setup=setup_data,
    config_setup=setup_config,
    sample_runner=run_sample,
    result_evaluator=evaluate_result,
    result_aggregators=[aggregate_results],
)

