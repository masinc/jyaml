"""JYAML loader implementation - converts parsed JYAML to Python objects."""

from collections.abc import Callable
from typing import Any, TypedDict, Unpack

from .options import LoadOptions, ParseOptions
from .parser import parse
from .types import JYAMLData


class LoadKwargs(TypedDict, total=False):
    """Type hint for loads() kwargs."""

    as_dict: bool
    as_native_types: bool
    parse_numbers: bool
    parse_booleans: bool
    parse_null: bool
    use_decimal: bool
    use_ordered_dict: bool
    object_hook: Callable | None
    number_hook: Callable | None
    parse_options: ParseOptions | None


def loads_strict(text: str) -> Any:
    """Parse JYAML with strict compliance."""
    return loads(text, options=LoadOptions.from_preset("strict_types"))


def loads_permissive(text: str) -> Any:
    """Parse JYAML with permissive settings."""
    parse_opts = ParseOptions.from_preset("permissive")
    return loads(text, options=LoadOptions(parse_options=parse_opts))


def loads_fast(text: str) -> Any:
    """Parse JYAML with fast settings (no comments)."""
    parse_opts = ParseOptions.from_preset("fast")
    return loads(text, options=LoadOptions(parse_options=parse_opts))


def loads_ordered(text: str) -> Any:
    """Parse JYAML preserving key order."""
    return loads(text, options=LoadOptions.from_preset("preserve_order"))


def loads(
    text: str,
    *,
    preset: str | None = None,
    options: LoadOptions | None = None,
    **kwargs: Unpack[LoadKwargs],
) -> Any:
    """Parse JYAML text and return native Python data.

    Args:
        text: JYAML text to parse
        preset: Preset name ('default', 'strict_types', 'preserve_order', etc.)
        options: Custom LoadOptions (overrides preset)
        **kwargs: Direct option overrides (as_dict=True, parse_numbers=False, etc.)

    Returns:
        Parsed Python data

    Examples:
        # Simple usage
        data = loads('{"key": "value"}')

        # Using preset
        data = loads(text, preset='preserve_order')

        # Quick options
        data = loads(text, parse_numbers=False, use_ordered_dict=True)

        # Custom options
        opts = LoadOptions(use_decimal=True)
        data = loads(text, options=opts)
    """

    # Determine options to use
    if options is not None:
        # Use provided options
        load_opts = options
    elif preset is not None:
        # Use preset
        load_opts = LoadOptions.from_preset(preset)
    elif kwargs:
        # Create options from kwargs
        load_opts = LoadOptions(**kwargs)
    else:
        # Use defaults
        load_opts = LoadOptions()

    # Parse with ParseOptions
    if load_opts.parse_options:
        document = parse(text, options=load_opts.parse_options)
    else:
        document = parse(text)

    # Convert to Python with type conversion options
    return convert_to_python(document.data, load_opts)


def convert_to_python(value: JYAMLData, options: LoadOptions) -> Any:
    """Convert JYAML data to Python with LoadOptions."""
    from collections import OrderedDict
    from decimal import Decimal

    from .types import (
        JYAMLArray,
        JYAMLBool,
        JYAMLNull,
        JYAMLNumber,
        JYAMLObject,
        JYAMLString,
    )

    if isinstance(value, JYAMLNull):
        return None if options.parse_null else "null"

    elif isinstance(value, JYAMLBool):
        if options.parse_booleans and options.as_native_types:
            return value.value
        else:
            return "true" if value.value else "false"

    elif isinstance(value, JYAMLNumber):
        if options.parse_numbers and options.as_native_types:
            if options.number_hook:
                return options.number_hook(value.value)
            elif options.use_decimal and isinstance(value.value, float):
                return Decimal(str(value.value))
            else:
                return value.value
        else:
            return str(value.value)

    elif isinstance(value, JYAMLString):
        return value.value

    elif isinstance(value, JYAMLArray):
        return [convert_to_python(item, options) for item in value.value]

    elif isinstance(value, JYAMLObject):
        # Convert key-value pairs
        pairs = [
            (key, convert_to_python(val, options)) for key, val in value.value.items()
        ]

        # Choose container type
        if options.object_hook:
            return options.object_hook(pairs)
        elif options.use_ordered_dict:
            return OrderedDict(pairs)
        else:
            return dict(pairs)

    else:
        raise ValueError(f"Unknown JYAML type: {type(value)}")
