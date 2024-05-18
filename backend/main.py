from fastapi import FastAPI
from model.jsend import Jsend

app = FastAPI()

@app.get("/")
async def root() -> Jsend[None]:
    return Jsend(message="/redoc and /docs provides the API documentation.")
