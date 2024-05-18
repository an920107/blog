from fastapi import FastAPI
from model.jsend import Jsend
from router.post import router as post_router

app = FastAPI()
app.include_router(post_router)


@app.get("/")
async def root() -> Jsend[None]:
    return Jsend(message="/redoc and /docs provides the API documentation.")
