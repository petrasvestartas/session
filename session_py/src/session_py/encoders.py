import json
from typing import Any
import importlib


def _get_class_from_name(class_name: str):
    """Dynamically import a class by name from the session_py package.

    Convention: Class name maps to module name (lowercase).
    Example: "Color" -> "session_py.color", "TreeNode" -> "session_py.treenode"

    Parameters
    ----------
    class_name : str
        Name of the class to import (e.g., "Color", "Point", "TreeNode")

    Returns
    -------
    type or None
        The class object if found, None otherwise
    """
    try:
        module_name = f"session_py.{class_name.lower()}"
        module = importlib.import_module(module_name)
        return getattr(module, class_name, None)
    except (ImportError, AttributeError):
        return None


class GeometryEncoder(json.JSONEncoder):
    """Custom JSON encoder that handles geometry objects with __jsondump__ method.

    Automatically serializes:
    - Geometry objects (via __jsondump__)
    - Nested lists, dicts, tuples
    - Primitive types

    """

    def default(self, obj):
        # Check if object has __jsondump__ method
        if hasattr(obj, "__jsondump__"):
            return obj.__jsondump__()

        # Handle iterators
        if hasattr(obj, "__next__"):
            return list(obj)

        # Let the base class handle it
        return super().default(obj)


class GeometryDecoder(json.JSONDecoder):
    """Custom JSON decoder that reconstructs geometry objects from the 'type' field.

    Automatically deserializes:
    - Geometry objects (via __jsonload__)
    - Nested lists, dicts
    - Primitive types

    """

    def __init__(self, *args, **kwargs):
        super().__init__(object_hook=self.object_hook, *args, **kwargs)

    def object_hook(self, obj: dict) -> Any:
        """Hook called for every JSON object decoded.

        If the object has a 'type' field, attempts to reconstruct the
        corresponding geometry object using its __jsonload__ method.

        Parameters
        ----------
        obj : dict
            Dictionary decoded from JSON.

        Returns
        -------
        Any
            Reconstructed geometry object or original dict.

        """
        # Check if it has type field
        if "type" not in obj:
            return obj

        try:
            # Get class name from type field
            # Supports: "ClassName", "session/ClassName", or "session_py.module/ClassName"
            type_str = obj["type"]
            if "/" in type_str:
                _, class_name = type_str.rsplit("/", 1)
            else:
                class_name = type_str

            # Dynamically import the class
            cls = _get_class_from_name(class_name)
            if cls is None:
                return obj

            if hasattr(cls, "__jsonload__"):
                return cls.__jsonload__(obj, obj.get("guid"), obj.get("name"))
        except Exception:
            return obj

        return obj


def json_dump(data: Any, filepath: str, pretty: bool = True):
    """Write data to JSON file with geometry object support.

    Parameters
    ----------
    data : Any
        Any data structure (can contain geometry objects).
    filepath : str
        Path to output file.
    pretty : bool, optional
        If True, format with indentation. Defaults to True.

    """
    with open(filepath, "w") as f:
        if pretty:
            json.dump(data, f, cls=GeometryEncoder, indent=4)
        else:
            json.dump(data, f, cls=GeometryEncoder)


def json_dumps(data: Any, pretty: bool = True) -> str:
    """Serialize data to JSON string with geometry object support.

    Parameters
    ----------
    data : Any
        Any data structure (can contain geometry objects).
    pretty : bool, optional
        If True, format with indentation. Defaults to True.

    Returns
    -------
    str
        JSON string.

    """
    if pretty:
        return json.dumps(data, cls=GeometryEncoder, indent=4)
    else:
        return json.dumps(data, cls=GeometryEncoder)


def json_load(filepath: str) -> Any:
    """Load data from JSON file with geometry object reconstruction.

    Parameters
    ----------
    filepath : str
        Path to input file.

    Returns
    -------
    Any
        Reconstructed data (geometry objects are restored).

    """
    with open(filepath, "r") as f:
        return json.load(f, cls=GeometryDecoder)


def json_loads(s: str) -> Any:
    """Deserialize JSON string with geometry object reconstruction.

    Parameters
    ----------
    s : str
        JSON string.

    Returns
    -------
    Any
        Reconstructed data (geometry objects are restored).

    """
    data = json.loads(s, cls=GeometryDecoder)
    # If root decoded as Session, return its JSON mapping form
    try:
        from .session import Session  # local import to avoid cycles

        if isinstance(data, Session):
            return data.__jsondump__()
    except Exception:
        pass
    return data


def decode_node(node: Any) -> Any:
    """Recursively decode a node that may contain polymorphic objects.

    - If dict with 'type', dynamically import the class and call __jsonload__.
    - If list, decode each element.
    - If plain dict, decode values recursively.
    - Otherwise, return as-is.

    Parameters
    ----------
    node : Any
        Node to decode (can be primitive, list, dict, or geometry object).

    Returns
    -------
    Any
        Decoded node with geometry objects reconstructed.

    """
    # Primitives
    if node is None or isinstance(node, (bool, int, float, str)):
        return node
    # Lists
    if isinstance(node, list):
        return [decode_node(x) for x in node]
    # Dicts
    if isinstance(node, dict):
        # Polymorphic geometry object
        if "type" in node:
            try:
                # Get class name from type field
                # Supports: "ClassName", "session/ClassName", or "session_py.module/ClassName"
                type_str = node["type"]
                if "/" in type_str:
                    _, class_name = type_str.rsplit("/", 1)
                else:
                    class_name = type_str

                # Dynamically import the class
                cls = _get_class_from_name(class_name)
                if cls is None:
                    return node

                if hasattr(cls, "__jsonload__"):
                    return cls.__jsonload__(node, node.get("guid"), node.get("name"))
            except Exception:
                return node
        # Plain dict: decode values
        return {k: decode_node(v) for k, v in node.items()}
    # Fallback
    return node
