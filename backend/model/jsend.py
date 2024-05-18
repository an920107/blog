from typing import Generic, TypeVar

from pydantic import BaseModel

T = TypeVar('T')

class Jsend(BaseModel, Generic[T]):
    data: T = None
    message: str = ""
    
    def __init__(self, *, data: object = None, message: str = None):
        super().__init__()
        if data is not None: self.data = data
        if message is not None: self.message = message
