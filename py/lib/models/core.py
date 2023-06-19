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


class Person(Base):
    __tablename__ = "people"
    __table_args__ = {"schema": "core"}

    id = Column(Integer, primary_key=True, name="id", quote=False)
    uid = Column(UUID, unique=True, nullable=False, name="uid", quote=False)
    username = Column(String, unique=True, name="username", quote=False)
    email = Column(String, unique=True, nullable=False, name="email", quote=False)
    primary_phone_number = Column(String(32), name="primary_phone_number", quote=False)
    login_type = Column(String(255), name="login_type", quote=False)
    password = Column(String, name="password", quote=False)
    oauth_provider = Column(String, name="oauth_provider", quote=False)
    oauth_token = Column(String, name="oauth_token", quote=False)
    first_name = Column(String, name="first_name", quote=False)
    last_name = Column(String, name="last_name", quote=False)
    other_names = Column(String, name="other_names", quote=False)
    date_added = Column(DateTime, name="date_added", quote=False)
    contact_email = Column(String, name="contact_email", quote=False)
    other_phone_numbers = Column(JSONB, name="other_phone_numbers", quote=False)
    postal_address = Column(JSONB, name="postal_address", quote=False)
    physical_address = Column(JSONB, name="physical_address", quote=False)
    active = Column(Boolean, name="active", quote=False)

    def __repr__(self):
        return f"{self.first_name} {self.last_name}<{self.email}>"


class LoginSession(Base):
    __tablename__ = "login_sessions"
    __table_args__ = {"schema": "core"}

    id = Column(Integer, primary_key=True, name="id", quote=False)
    person_id = Column(
        Integer, ForeignKey("core.people.id"), name="person_id", quote=False
    )
    session_id = Column(
        String(128), unique=True, nullable=False, name="session_id", quote=False
    )
    time_started = Column(DateTime, nullable=False, name="time_started", quote=False)
    last_action_time = Column(
        DateTime, nullable=False, name="last_action_time", quote=False
    )
    ended = Column(Boolean, name="ended", server_default="f", quote=False)
    time_ended = Column(DateTime, nullable=False, name="time_ended", quote=False)

    def __repr__(self):
        return f"LoginSession<{self.person_id}:{self.session_id}:{self.time_started}>"
