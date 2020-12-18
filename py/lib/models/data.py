
import binascii
import datetime
import os

from passlib.hash import bcrypt


from sqlalchemy import (
    Boolean,
    Column,
    DateTime,
    Integer,
    String,

    ForeignKey,
)

from sqlalchemy import exc as sqlalchemy_exc
from sqlalchemy.dialects.postgresql import (
    JSONB,
    UUID,
    ARRAY,
)

from sqlalchemy.sql.expression import false, null
from sqlalchemy.orm import relationship

from abenga_site.py.lib.models.base import Base


class Author(Base):
    __tablename__ = 'authors'
    __table_args__ = {'schema': 'data'}

    id = Column(Integer, ForeignKey("people.id"), primary_key=True, name='id', quote=False)
    bio_md = Column(String, name='bio_md', quote=False)
    bio_html = Column(String, name='bio_html', quote=False)


class PostSeries(Base):
    __tablename__ = 'post_series'
    __table_args__ = {'schema': 'data'}

    id = Column(Integer, primary_key=True, name='id', quote=False)
    # uid = Column(UUID, unique=True, nullable=False, name='uid', quote=False)
    author_id = Column(Integer, ForeignKey("authors.id"), name='author_id', quote=False)
    title = Column(String, nullable=False, name='title', quote=False)
    joined_title = Column(String, unique=True, nullable=False, name='joined_title', quote=False)
    date_added = Column(DateTime, nullable=False, name='date_added', quote=False)
    last_edited = Column(DateTime, nullable=False, name='last_edited', quote=False)
    abstract_md = Column(String, nullable=False, name='abstract_md', quote=False)
    abstract_html = Column(String, nullable=False, name='abstract_html', quote=False)
    cover_image_path = Column(String, name='cover_image_path', quote=False)
    tags = Column(ARRAY(String), name='tags', quote=False)


class Post(Base):
    __tablename__ = 'posts'
    __table_args__ = {'schema': 'data'}

    id = Column(Integer, primary_key=True, name='id', quote=False)
    # uid = Column(UUID, unique=True, nullable=False, name='uid', quote=False)
    title = Column(String, nullable=False, name='title', quote=False)
    joined_title = Column(String, unique=True, nullable=False, name='joined_title', quote=False)
    date_added = Column(DateTime, nullable=False, name='date_added', quote=False)
    last_edited = Column(DateTime, nullable=False, name='last_edited', quote=False)
    year_added = Column(Integer, nullable=False, name='year_added', quote=False)
    month_added = Column(Integer, nullable=False, name='month_added', quote=False)
    day_added = Column(Integer, nullable=False, name='day_added', quote=False)
    author_id = Column(Integer, ForeignKey("authors.id"), name='author_id', quote=False)
    abstract_md = Column(String, nullable=False, name='abstract_md', quote=False)
    abstract_html = Column(String, nullable=False, name='abstract_html', quote=False)
    body_md = Column(String, nullable=False, name='body_md', quote=False)
    body_html = Column(String, nullable=False, name='body_html', quote=False)
    series_id = Column(Integer, ForeignKey("post_series.id"), nullable=True, name='series_id', quote=False)
    position_in_series = Column(Integer, name='position_in_series', quote=False)
    references_md = Column(String, nullable=False, name='references_md', quote=False)
    references_html = Column(String, nullable=False, name='references_html', quote=False)
    cover_image_path = Column(String, name='cover_image_path', quote=False)
    tags = Column(ARRAY(String), name='tags', quote=False)
