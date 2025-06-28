"""JYAML serialization functionality."""

from typing import Any, Optional, Unpack, TypedDict, Literal
from .options import DumpOptions


class DumpKwargs(TypedDict, total=False):
    """Type hint for dumps() kwargs."""

    style: Literal["flow", "block", "auto"]
    indent: int
    quote_style: Literal["double", "single", "auto"]
    escape_unicode: bool
    sort_keys: bool
    line_ending: Literal["lf", "crlf"]
    ensure_ascii: bool
    include_comments: bool


def dumps(
    obj: Any,
    *,
    preset: Optional[str] = None,
    options: Optional[DumpOptions] = None,
    **kwargs: Unpack[DumpKwargs],
) -> str:
    """Serialize Python object to JYAML string.

    Args:
        obj: Python object to serialize
        preset: Preset name ('compact', 'pretty', 'minimal')
        options: Custom DumpOptions (overrides preset)
        **kwargs: Direct option overrides (style='block', indent=4, etc.)

    Returns:
        JYAML string representation

    Examples:
        # Simple usage
        jyaml_str = dumps({"key": "value"})

        # Using preset
        jyaml_str = dumps(data, preset='pretty')

        # Quick options
        jyaml_str = dumps(data, style='block', indent=4)

        # Custom options
        opts = DumpOptions(style='flow', sort_keys=True)
        jyaml_str = dumps(data, options=opts)
    """

    # Determine options to use
    if options is not None:
        dump_opts = options
    elif preset is not None:
        presets = {
            "compact": DumpOptions(style="flow", indent=0),
            "pretty": DumpOptions(style="auto", indent=2),
            "minimal": DumpOptions(style="flow", indent=0, sort_keys=True),
            "block": DumpOptions(style="block", indent=2),
        }
        if preset not in presets:
            available = ", ".join(presets.keys())
            raise ValueError(f"Unknown preset: {preset}. Available: {available}")
        dump_opts = presets[preset]
    elif kwargs:
        dump_opts = DumpOptions(**kwargs)
    else:
        dump_opts = DumpOptions()

    return _serialize_with_options(obj, dump_opts)


def _serialize_with_options(obj: Any, options: DumpOptions, depth: int = 0) -> str:
    """Serialize Python object to JYAML with DumpOptions."""
    from decimal import Decimal
    from collections import OrderedDict

    if obj is None:
        return "null"

    elif isinstance(obj, bool):
        return "true" if obj else "false"

    elif isinstance(obj, (int, float, Decimal)):
        return str(obj)

    elif isinstance(obj, str):
        # Escape string and add quotes
        escaped = _escape_string(obj, options)
        quote_char = '"' if options.quote_style in ["double", "auto"] else "'"
        return f"{quote_char}{escaped}{quote_char}"

    elif isinstance(obj, (list, tuple)):
        return _serialize_array(obj, options, depth)

    elif isinstance(obj, (dict, OrderedDict)):
        return _serialize_object(obj, options, depth)

    else:
        # For other types, convert to string
        return _serialize_with_options(str(obj), options, depth)


def _serialize_array(arr: list, options: DumpOptions, depth: int = 0) -> str:
    """Serialize array to JYAML."""
    if not arr:
        return "[]"

    # Always use flow style for arrays to ensure JYAML compatibility
    items = []
    for item in arr:
        items.append(_serialize_with_options(item, options, depth + 1))

    content = ", ".join(items)

    return f"[{content}]"


def _serialize_object(obj: dict, options: DumpOptions, depth: int = 0) -> str:
    """Serialize object to JYAML."""
    if not obj:
        return "{}"

    # Sort keys if requested
    items = sorted(obj.items()) if options.sort_keys else obj.items()

    # Determine actual style to use
    if options.style == "block" and depth == 0:
        use_block = True
    elif options.style == "flow":
        use_block = False
    elif options.style == "auto":
        # Auto mode: use flow for small/simple objects, block for large/complex
        use_block = _should_use_block_style_object(obj, depth)
    else:
        use_block = False

    if use_block:
        # Root-level block style: key: value
        lines = []
        for key, value in items:
            key_str = _serialize_with_options(key, options, depth + 1)
            value_str = _serialize_with_options(value, options, depth + 1)
            lines.append(f"{key_str}: {value_str}")

        return "\n".join(lines)

    else:
        # Flow style: {"key": "value"}
        pairs = []
        for key, value in items:
            key_str = _serialize_with_options(key, options, depth + 1)
            value_str = _serialize_with_options(value, options, depth + 1)
            pairs.append(f"{key_str}: {value_str}")

        content = ", ".join(pairs)

        return f"{{{content}}}"


def _should_use_block_style_object(obj: dict, depth: int = 0) -> bool:
    """Determine if object should use block style in auto mode."""
    # Only use block style at root level for readability
    if depth > 0:
        return False

    # Use block style for larger objects or complex values
    if len(obj) > 3:
        return True

    # Use block style if any value is complex (nested structure)
    for value in obj.values():
        if isinstance(value, (dict, list)):
            return True

    # Use block style for very long string values
    for value in obj.values():
        if isinstance(value, str) and len(value) > 50:
            return True

    return False


def _escape_string(s: str, options: DumpOptions) -> str:
    """Escape string for JYAML output."""
    # Basic JSON-compatible escaping (order matters - backslash first!)
    result = s
    result = result.replace("\\", "\\\\")  # Must be first
    result = result.replace('"', '\\"')
    result = result.replace("\b", "\\b")
    result = result.replace("\f", "\\f")
    result = result.replace("\n", "\\n")
    result = result.replace("\r", "\\r")
    result = result.replace("\t", "\\t")

    # Handle Unicode escaping if requested (use \uXXXX format)
    if options.escape_unicode or options.ensure_ascii:
        escaped_result = ""
        for char in result:
            if ord(char) > 127:
                escaped_result += f"\\u{ord(char):04x}"
            else:
                escaped_result += char
        result = escaped_result

    return result
