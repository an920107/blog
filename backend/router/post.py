from datetime import datetime
from http import HTTPStatus
from typing import Annotated
from fastapi import APIRouter, Body, HTTPException
from model.jsend import Jsend
from model.post import Post
from util.database import Database

router = APIRouter(prefix="/post", tags=["post"])

database = Database()


@router.get("", description="Returns a list of posts' id.")
async def get_post_ids(show_invisible: bool = False) -> Jsend[list[str]]:
    ids = database.get_post_ids(show_invisible=show_invisible)
    return Jsend(data=ids, message="Successfully retrieved.")


@router.put("", description="Create a new post.")
async def create_post(post: Post) -> Jsend[None]:
    post.created_time = datetime.now()
    post.updated_time = datetime.now()
    database.create_post(post)
    return Jsend(message="Successfully created.")


@router.get("/{post_id}", description="Return a post payload with the given id.")
async def get_post_by_id(post_id: str, show_invisible: bool = False) -> Jsend[Post]:
    post = database.get_post_by_id(post_id, show_invisible)
    if post == None:
        raise HTTPException(HTTPStatus.NOT_FOUND)
    return Jsend(data=post, message="Successfully retrieved.")
