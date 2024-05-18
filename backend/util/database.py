import datetime
import json
import os
import pathlib
import sqlite3
from typing import Iterable

from model.post import Post
from util.singleton import Singleton


class Database(Singleton):
    def __init__(self) -> None:
        ROOT = str(pathlib.Path().resolve())
        STATIC = os.path.join(ROOT, "static")
        os.makedirs(STATIC, exist_ok=True)

        self._conn = sqlite3.connect(os.path.join(STATIC, "data.sqlite"))
        self._cursor = self._conn.cursor()

        self._database_init()

    def _database_init(self):
        self._cursor.execute(
            """
            CREATE TABLE IF NOT EXISTS post (
                id TEXT PRIMARY KEY NOT NULL,
                title TEXT NOT NULL,
                description TEXT NOT NULL,
                content TEXT NOT NULL,
                tags JSON NOT NULL,
                preview_image TEXT NOT NULL,
                visible INTEGER NOT NULL,
                created_time TIMESTAMP NOT NULL,
                updated_time TIMESTAMP NOT NULL
            );
            """
        )
        self._conn.commit()

    # Post
    def get_post_ids(self, show_invisible: bool) -> Iterable[str]:
        self._cursor.execute(
            f"""
            SELECT id FROM post
            {"" if show_invisible else "WHERE visible=1"}
            ORDER BY created_time DESC;
            """
        )
        return [row[0] for row in self._cursor.fetchall()]

    def create_post(self, post: Post) -> None:
        self._cursor.execute(
            """
            INSERT INTO post (
                id,
                title,
                description,
                content,
                tags,
                preview_image,
                visible,
                created_time,
                updated_time
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            """,
            (
                post.id,
                post.title,
                post.description,
                post.content,
                json.dumps(post.tags),
                post.preview_image,
                post.visible,
                post.created_time,
                post.updated_time,
            ),
        )
        self._conn.commit()

    def get_post_by_id(self, id: str, show_invisible: bool) -> Post | None:
        self._cursor.execute(
            f"""
            SELECT * FROM post
            WHERE id=?
            {"" if show_invisible else "AND visible=1"}
            """,
            (id, )
        )
        row = self._cursor.fetchone()
        if row == None: return None
        return Post(
            id=row[0],
            title=row[1],
            description=row[2],
            content=row[3],
            tags=json.loads(row[4]),
            preview_image=row[5],
            visible=row[6],
            created_time=row[7],
            updated_time=row[8],
        )