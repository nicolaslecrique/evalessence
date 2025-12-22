from fastapi import FastAPI
from evalessence.interfaces import EvaluationPipeline
from pydantic import BaseModel


def create_rest_api[
    TSampleInput: BaseModel,
    TSampleAnnotation: BaseModel,
    TSampleResult: BaseModel,
    TSampleEvaluation: BaseModel,
    TExperimentConfig: BaseModel,
    TExperimentData: BaseModel,
](
    evaluation_pipeline: EvaluationPipeline[
        TSampleInput,
        TSampleAnnotation,
        TSampleResult,
        TSampleEvaluation,
        TExperimentConfig,
        TExperimentData,
    ]
) -> FastAPI:
    
    app = FastAPI()

    @app.post("/setup_data")
    async def setup_data(data: TExperimentData) -> None: # pyright: ignore[reportUnusedFunction]
        context = evaluation_pipeline.data_setup(data)
        app.state.data_setup_id = await context.__aenter__()


    return app