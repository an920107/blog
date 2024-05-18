from datetime import datetime
import uuid

from pydantic import BaseModel, Field


class DbObject(BaseModel):
    id: str = Field(..., default_factory=lambda: str(uuid.uuid4()))
    updated_time: datetime = Field(..., default_factory=lambda: datetime.now())
    created_time: datetime = Field(..., default_factory=lambda: datetime.now())
