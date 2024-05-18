from model.db_object import DbObject
from pydantic import BaseModel, Field


class Post(DbObject):
    title: str
    description: str
    content: str
    tags: list[str]
    preview_image: str
    visible: bool = False
