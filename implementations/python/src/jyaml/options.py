"""JYAML parsing and dumping options with Pydantic validation."""

from collections.abc import Callable
from typing import Literal

from pydantic import BaseModel, ConfigDict, Field, field_validator, model_validator


# Predefined option presets for common use cases
class JYAMLMode:
    """Predefined JYAML parsing modes for common use cases."""

    @staticmethod
    def strict() -> "ParseOptions":
        """Strict JYAML spec compliance (default)."""
        return ParseOptions(strict_mode=True, preserve_comments=True, max_depth=1000)

    @staticmethod
    def permissive() -> "ParseOptions":
        """Permissive mode for flexibility."""
        return ParseOptions(
            strict_mode=False,
            preserve_comments=True,
            allow_duplicate_keys=True,
            max_depth=10000,
        )

    @staticmethod
    def fast() -> "ParseOptions":
        """Fast parsing mode (minimal features)."""
        return ParseOptions(strict_mode=True, preserve_comments=False, max_depth=100)

    @staticmethod
    def debug() -> "ParseOptions":
        """Debug mode with detailed information."""
        return ParseOptions(
            strict_mode=False,
            preserve_comments=True,
            include_comment_positions=True,
            allow_duplicate_keys=True,
        )


class ParseOptions(BaseModel):
    """Simple, user-friendly JYAML parsing options with Pydantic validation."""

    model_config = ConfigDict(
        extra="forbid",
        str_strip_whitespace=True,
        validate_assignment=True,
        frozen=False,
    )

    # Main mode settings
    strict_mode: bool = Field(default=True, description="Strict JYAML spec compliance")
    preserve_comments: bool = Field(
        default=True, description="Keep comments in parsed document"
    )

    # Common flexibility options
    allow_duplicate_keys: bool = Field(
        default=False, description="Allow duplicate object keys"
    )

    # Performance limits
    max_depth: int | None = Field(
        default=1000,
        gt=0,
        le=100000,
        description="Maximum nesting depth (None = unlimited)",
    )

    # Advanced options (for power users)
    include_comment_positions: bool = Field(
        default=False, description="Include line/column info for comments"
    )
    normalize_line_endings: Literal["none", "lf", "crlf"] = Field(
        default="lf",
        description="Line ending normalization: 'none', 'lf' (\\n), or 'crlf' (\\r\\n)",
    )

    @field_validator("max_depth")
    @classmethod
    def validate_max_depth(cls, v):
        """Validate max_depth is reasonable."""
        if v is not None and v <= 0:
            raise ValueError("max_depth must be positive")
        return v

    @model_validator(mode="after")
    def validate_consistency(self):
        """Validate option consistency."""
        if self.strict_mode and self.allow_duplicate_keys:
            raise ValueError("strict_mode and allow_duplicate_keys are incompatible")

        if self.include_comment_positions and not self.preserve_comments:
            raise ValueError(
                "include_comment_positions requires preserve_comments=True"
            )

        return self

    # Create from preset
    @classmethod
    def from_preset(cls, preset: str) -> "ParseOptions":
        """Create options from preset name."""
        presets = {
            "strict": JYAMLMode.strict(),
            "permissive": JYAMLMode.permissive(),
            "fast": JYAMLMode.fast(),
            "debug": JYAMLMode.debug(),
        }
        if preset not in presets:
            available = ", ".join(presets.keys())
            raise ValueError(f"Unknown preset: {preset}. Available: {available}")
        return presets[preset]


class LoadOptions(BaseModel):
    """Simple, user-friendly JYAML loading options with Pydantic validation."""

    model_config = ConfigDict(
        extra="forbid",
        str_strip_whitespace=True,
        validate_assignment=True,
        frozen=False,
        arbitrary_types_allowed=True,  # Allow callable types
    )

    # Main conversion options
    as_dict: bool = Field(
        default=True, description="Convert objects to dict (vs OrderedDict/custom)"
    )
    as_native_types: bool = Field(
        default=True, description="Convert to Python native types"
    )

    # Type conversion control
    parse_numbers: bool = Field(
        default=True, description="Convert numeric strings to int/float"
    )
    parse_booleans: bool = Field(
        default=True, description="Convert 'true'/'false' to bool"
    )
    parse_null: bool = Field(default=True, description="Convert 'null' to None")

    # Advanced type options
    use_decimal: bool = Field(
        default=False, description="Use Decimal instead of float for precision"
    )
    use_ordered_dict: bool = Field(
        default=False, description="Use OrderedDict to preserve key order"
    )

    # Custom conversion hooks
    object_hook: Callable | None = Field(
        default=None, description="Custom object creation function"
    )
    number_hook: Callable | None = Field(
        default=None, description="Custom number parsing function"
    )

    # Include parsing options
    parse_options: ParseOptions | None = Field(
        default=None, description="Override default parse options"
    )

    @field_validator("object_hook", "number_hook")
    @classmethod
    def validate_callable(cls, v):
        """Validate that hooks are callable."""
        if v is not None and not callable(v):
            raise ValueError("Hook must be callable")
        return v

    @model_validator(mode="after")
    def validate_consistency(self):
        """Validate option consistency."""
        if not self.as_native_types and (self.use_decimal or self.use_ordered_dict):
            raise ValueError(
                "use_decimal and use_ordered_dict require as_native_types=True"
            )

        if self.use_decimal and not self.parse_numbers:
            raise ValueError("use_decimal requires parse_numbers=True")

        if self.as_dict and self.use_ordered_dict:
            # as_dict=False implied when use_ordered_dict=True
            self.as_dict = False

        return self

    # Create from preset
    @classmethod
    def from_preset(cls, preset: str) -> "LoadOptions":
        """Create options from preset name."""
        presets = {
            "default": cls(),
            "strict_types": cls(
                as_native_types=True, parse_numbers=True, parse_booleans=True
            ),
            "preserve_order": cls(use_ordered_dict=True),
            "high_precision": cls(use_decimal=True, use_ordered_dict=True),
            "strings_only": cls(
                as_native_types=False,
                parse_numbers=False,
                parse_booleans=False,
                parse_null=False,
            ),
        }

        if preset not in presets:
            available = ", ".join(presets.keys())
            raise ValueError(f"Unknown preset: {preset}. Available: {available}")

        return presets[preset]


class DumpOptions(BaseModel):
    """Simple, user-friendly JYAML dumping options with Pydantic validation."""

    model_config = ConfigDict(
        extra="forbid",
        str_strip_whitespace=True,
        validate_assignment=True,
        frozen=False,
    )

    # Output style
    style: Literal["flow", "block", "auto"] = Field(
        default="auto",
        description="Output style: 'flow' (JSON-like), 'block' (YAML-like), or 'auto'",
    )
    indent: int = Field(
        default=2, ge=0, le=8, description="Number of spaces for indentation"
    )

    # String formatting
    quote_style: Literal["double", "single", "auto"] = Field(
        default="double", description="Quote style for strings"
    )
    escape_unicode: bool = Field(
        default=False, description="Escape non-ASCII characters"
    )

    # Structure options
    sort_keys: bool = Field(
        default=False, description="Sort object keys alphabetically"
    )

    # Line endings and formatting
    line_ending: Literal["lf", "crlf"] = Field(
        default="lf", description="Line ending style"
    )
    ensure_ascii: bool = Field(default=False, description="Ensure ASCII-only output")

    # Comments (for future extension)
    include_comments: bool = Field(
        default=False, description="Include comments in output (future feature)"
    )

    @field_validator("indent")
    @classmethod
    def validate_indent(cls, v):
        """Validate indent is reasonable."""
        if v < 0:
            raise ValueError("indent must be non-negative")
        if v > 8:
            raise ValueError("indent should not exceed 8 spaces")
        return v
