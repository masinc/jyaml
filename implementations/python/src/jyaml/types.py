"""JYAML data types using Pydantic."""

from typing import Any, Dict, List, Union, Optional
from pydantic import BaseModel, Field, ConfigDict


class JYAMLValue(BaseModel):
    """Base class for JYAML values."""

    model_config = ConfigDict(
        extra="forbid", str_strip_whitespace=True, validate_assignment=True
    )


class JYAMLNull(JYAMLValue):
    """JYAML null value."""

    value: None = None


class JYAMLBool(JYAMLValue):
    """JYAML boolean value."""

    value: bool


class JYAMLNumber(JYAMLValue):
    """JYAML number value."""

    value: Union[int, float]


class JYAMLString(JYAMLValue):
    """JYAML string value."""

    value: str


class JYAMLArray(JYAMLValue):
    """JYAML array value."""

    value: List["JYAMLData"] = Field(default_factory=list)


class JYAMLObject(JYAMLValue):
    """JYAML object value."""

    value: Dict[str, "JYAMLData"] = Field(default_factory=dict)


# Union type for all JYAML data types
JYAMLData = Union[
    JYAMLNull, JYAMLBool, JYAMLNumber, JYAMLString, JYAMLArray, JYAMLObject
]

# Update forward references
JYAMLArray.model_rebuild()
JYAMLObject.model_rebuild()


class ParsedDocument(BaseModel):
    """Parsed JYAML document."""

    model_config = ConfigDict(extra="forbid")

    data: JYAMLData
    comments: List[str] = Field(default_factory=list)
    source_info: Optional[Dict[str, Any]] = None


def to_python(value: JYAMLData) -> Any:
    """Convert JYAML data to native Python types."""
    if isinstance(value, JYAMLNull):
        return None
    elif isinstance(value, JYAMLBool):
        return value.value
    elif isinstance(value, JYAMLNumber):
        return value.value
    elif isinstance(value, JYAMLString):
        return value.value
    elif isinstance(value, JYAMLArray):
        return [to_python(item) for item in value.value]
    elif isinstance(value, JYAMLObject):
        return {key: to_python(val) for key, val in value.value.items()}
    else:
        raise ValueError(f"Unknown JYAML type: {type(value)}")


def from_python(value: Any) -> JYAMLData:
    """Convert native Python types to JYAML data."""
    if value is None:
        return JYAMLNull()
    elif isinstance(value, bool):
        return JYAMLBool(value=value)
    elif isinstance(value, (int, float)):
        return JYAMLNumber(value=value)
    elif isinstance(value, str):
        return JYAMLString(value=value)
    elif isinstance(value, list):
        return JYAMLArray(value=[from_python(item) for item in value])
    elif isinstance(value, dict):
        return JYAMLObject(
            value={str(key): from_python(val) for key, val in value.items()}
        )
    else:
        raise ValueError(f"Cannot convert Python type {type(value)} to JYAML")
