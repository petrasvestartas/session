// Auto-generated API index for browser search
window.API_INDEX = {
  "concepts": [
    {
      "name": "Color.__init__",
      "implementations": {
        "python": {
          "sig": "__init__(r: int, g: int, b: int, a: int, name: str = \"my_color\")",
          "code": "def __init__(self, r: int, g: int, b: int, a: int, name: str = \"my_color\"):\n\n        self.guid = str(uuid.uuid4())\n        self.name = name\n        self._r = int(r)\n        self._g = int(g)\n        self._b = int(b)\n        self._a = int(a)\n\n    ###########################################################################################\n    # Operators\n    ###########################################################################################\n\n    def __deepcopy__(self, memo):\n\n        cls = self.__class__\n        result = cls.__new__(cls)\n        memo[id(self)] = result\n\n        # New guid\n        result.guid = str(uuid.uuid4())",
          "file": "color.py"
        }
      }
    },
    {
      "name": "Color.__deepcopy__",
      "implementations": {
        "python": {
          "sig": "__deepcopy__(memo)",
          "code": "def __deepcopy__(self, memo):\n\n\n        cls = self.__class__\n        result = cls.__new__(cls)\n        memo[id(self)] = result\n\n        # New guid\n        result.guid = str(uuid.uuid4())\n\n        # Copy remaining fields\n        result.name = copy.deepcopy(self.name, memo)\n        result._r = self._r\n        result._g = self._g\n        result._b = self._b\n        result._a = self._a\n        return result\n\n    def duplicate(self) -> \"Color\":\n        \"\"\"Create a deep copy of this color with a new GUID.",
          "file": "color.py"
        }
      }
    },
    {
      "name": "Color.duplicate",
      "implementations": {
        "python": {
          "sig": "duplicate() -> \"Color\"",
          "code": "def duplicate(self) -> \"Color\":\n\n        \"\"\"Create a deep copy of this color with a new GUID.\n\n        Returns\n        -------\n        :class:`Color`\n            A new Color with identical RGBA values but a different GUID.\n\n        \"\"\"\n        return copy.deepcopy(self)\n\n    ###########################################################################################\n    # No-copy Operators\n    ###########################################################################################\n\n    def __getitem__(self, index):\n        if index == 0:\n            return self._r\n        elif index == 1:\n            return self._g",
          "file": "color.py"
        },
        "rust": {
          "sig": "duplicate() -> Self",
          "code": "pub fn duplicate(&self) -> Self {\n        Color {\n            guid: Uuid::new_v4().to_string(),\n            name: self.name.clone(),\n            r: self.r,\n            g: self.g,\n            b: self.b,\n            a: self.a,\n        }\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.__getitem__",
      "implementations": {
        "python": {
          "sig": "__getitem__(index)",
          "code": "def __getitem__(self, index):\n\n        if index == 0:\n            return self._r\n        elif index == 1:\n            return self._g\n        elif index == 2:\n            return self._b\n        elif index == 3:\n            return self._a\n        else:\n            raise IndexError(\"Index out of range\")\n\n    def __setitem__(self, index, value):\n        if index == 0:\n            self._r = value\n        elif index == 1:\n            self._g = value\n        elif index == 2:\n            self._b = value\n        elif index == 3:",
          "file": "color.py"
        }
      }
    },
    {
      "name": "Color.__setitem__",
      "implementations": {
        "python": {
          "sig": "__setitem__(index, value)",
          "code": "def __setitem__(self, index, value):\n\n        if index == 0:\n            self._r = value\n        elif index == 1:\n            self._g = value\n        elif index == 2:\n            self._b = value\n        elif index == 3:\n            self._a = value\n        else:\n            raise IndexError(\"Index out of range\")\n\n    ###########################################################################################\n    # Details\n    ###########################################################################################\n\n    def to_unified_array(self) -> list[float]:\n        \"\"\"Convert to normalized float array [0-1].\n\n        Returns",
          "file": "color.py"
        }
      }
    },
    {
      "name": "Color.to_unified_array",
      "implementations": {
        "python": {
          "sig": "to_unified_array() -> list[float]",
          "code": "def to_unified_array(self) -> list[float]:\n\n        \"\"\"Convert to normalized float array [0-1].\n\n        Returns\n        -------\n        list[float]\n            Array [r, g, b, a] with values normalized to [0.0, 1.0].\n\n        \"\"\"\n        return [self[0] / 255.0, self[1] / 255.0, self[2] / 255.0, self[3] / 255.0]\n\n    @classmethod\n    def from_unified_array(cls, arr) -> \"Color\":\n        \"\"\"Create color from normalized float values [0-1].\n\n        Parameters\n        ----------\n        arr : list[float]\n            Array [r, g, b, a] with values in [0.0, 1.0] range.",
          "file": "color.py"
        }
      }
    },
    {
      "name": "Color.from_unified_array",
      "implementations": {
        "python": {
          "sig": "from_unified_array(cls, arr) -> \"Color\"",
          "code": "def from_unified_array(cls, arr) -> \"Color\":\n\n        \"\"\"Create color from normalized float values [0-1].\n\n        Parameters\n        ----------\n        arr : list[float]\n            Array [r, g, b, a] with values in [0.0, 1.0] range.\n\n        Returns\n        -------\n        :class:`Color`\n            A new Color with values converted to 0-255 range.\n\n        \"\"\"\n        return cls(int(arr[0] * 255.0 + 0.5), int(arr[1] * 255.0 + 0.5), int(arr[2] * 255.0 + 0.5), int(arr[3] * 255.0 + 0.5))\n\n    ###########################################################################################\n    # Presets\n    ###########################################################################################",
          "file": "color.py"
        },
        "cpp": {
          "sig": "Color from_unified_array(std::array<double, 4> arr)",
          "code": "Color Color::from_unified_array(std::array<double, 4> arr) {\n  return Color(static_cast<unsigned int>(arr[0] * 255.0 + 0.5),\n               static_cast<unsigned int>(arr[1] * 255.0 + 0.5),\n               static_cast<unsigned int>(arr[2] * 255.0 + 0.5),\n               static_cast<unsigned int>(arr[3] * 255.0 + 0.5));\n}",
          "file": "color.cpp"
        }
      }
    },
    {
      "name": "Color.white",
      "implementations": {
        "python": {
          "sig": "white(cls) -> \"Color\"",
          "code": "def white(cls) -> \"Color\":\n\n        \"\"\"Create a white color.\"\"\"\n        color = cls(255, 255, 255, 255)\n        color.name = \"white\"\n        return color\n\n    @classmethod\n    def black(cls) -> \"Color\":\n        \"\"\"Create a black color.\"\"\"\n        color = cls(0, 0, 0, 255)\n        color.name = \"black\"\n        return color\n\n    @classmethod\n    def grey(cls) -> \"Color\":\n        \"\"\"Create a grey color.\"\"\"\n        color = cls(128, 128, 128, 255)\n        color.name = \"grey\"\n        return color",
          "file": "color.py"
        },
        "cpp": {
          "sig": "Color white()",
          "code": "Color Color::white() { return Color(255, 255, 255, 255, \"white\"); }",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "white() -> Self",
          "code": "pub fn white() -> Self {\n        let mut color = Color::new(255, 255, 255, 255);\n        color.name = \"white\".to_string();\n        color\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.black",
      "implementations": {
        "python": {
          "sig": "black(cls) -> \"Color\"",
          "code": "def black(cls) -> \"Color\":\n\n        \"\"\"Create a black color.\"\"\"\n        color = cls(0, 0, 0, 255)\n        color.name = \"black\"\n        return color\n\n    @classmethod\n    def grey(cls) -> \"Color\":\n        \"\"\"Create a grey color.\"\"\"\n        color = cls(128, 128, 128, 255)\n        color.name = \"grey\"\n        return color\n\n    @classmethod\n    def red(cls) -> \"Color\":\n        \"\"\"Create a red color.\"\"\"\n        color = cls(255, 0, 0, 255)\n        color.name = \"red\"\n        return color",
          "file": "color.py"
        },
        "cpp": {
          "sig": "Color black()",
          "code": "Color Color::black() { return Color(0, 0, 0, 255, \"black\"); }",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "black() -> Self",
          "code": "pub fn black() -> Self {\n        let mut color = Color::new(0, 0, 0, 255);\n        color.name = \"black\".to_string();\n        color\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.grey",
      "implementations": {
        "python": {
          "sig": "grey(cls) -> \"Color\"",
          "code": "def grey(cls) -> \"Color\":\n\n        \"\"\"Create a grey color.\"\"\"\n        color = cls(128, 128, 128, 255)\n        color.name = \"grey\"\n        return color\n\n    @classmethod\n    def red(cls) -> \"Color\":\n        \"\"\"Create a red color.\"\"\"\n        color = cls(255, 0, 0, 255)\n        color.name = \"red\"\n        return color\n\n    @classmethod\n    def orange(cls) -> \"Color\":\n        \"\"\"Create an orange color.\"\"\"\n        color = cls(255, 128, 0, 255)\n        color.name = \"orange\"\n        return color",
          "file": "color.py"
        },
        "cpp": {
          "sig": "Color grey()",
          "code": "Color Color::grey() { return Color(128, 128, 128, 255, \"grey\"); }",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "grey() -> Self",
          "code": "pub fn grey() -> Self {\n        let mut color = Color::new(128, 128, 128, 255);\n        color.name = \"grey\".to_string();\n        color\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.red",
      "implementations": {
        "python": {
          "sig": "red(cls) -> \"Color\"",
          "code": "def red(cls) -> \"Color\":\n\n        \"\"\"Create a red color.\"\"\"\n        color = cls(255, 0, 0, 255)\n        color.name = \"red\"\n        return color\n\n    @classmethod\n    def orange(cls) -> \"Color\":\n        \"\"\"Create an orange color.\"\"\"\n        color = cls(255, 128, 0, 255)\n        color.name = \"orange\"\n        return color\n\n    @classmethod\n    def yellow(cls) -> \"Color\":\n        \"\"\"Create a yellow color.\"\"\"\n        color = cls(255, 255, 0, 255)\n        color.name = \"yellow\"\n        return color",
          "file": "color.py"
        },
        "cpp": {
          "sig": "Color red()",
          "code": "Color Color::red() { return Color(255, 0, 0, 255, \"red\"); }",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "red() -> Self",
          "code": "pub fn red() -> Self {\n        let mut color = Color::new(255, 0, 0, 255);\n        color.name = \"red\".to_string();\n        color\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.orange",
      "implementations": {
        "python": {
          "sig": "orange(cls) -> \"Color\"",
          "code": "def orange(cls) -> \"Color\":\n\n        \"\"\"Create an orange color.\"\"\"\n        color = cls(255, 128, 0, 255)\n        color.name = \"orange\"\n        return color\n\n    @classmethod\n    def yellow(cls) -> \"Color\":\n        \"\"\"Create a yellow color.\"\"\"\n        color = cls(255, 255, 0, 255)\n        color.name = \"yellow\"\n        return color\n\n    @classmethod\n    def lime(cls) -> \"Color\":\n        \"\"\"Create a lime color.\"\"\"\n        color = cls(128, 255, 0, 255)\n        color.name = \"lime\"\n        return color",
          "file": "color.py"
        },
        "cpp": {
          "sig": "Color orange()",
          "code": "Color Color::orange() { return Color(255, 128, 0, 255, \"orange\"); }",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "orange() -> Self",
          "code": "pub fn orange() -> Self {\n        let mut color = Color::new(255, 128, 0, 255);\n        color.name = \"orange\".to_string();\n        color\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.yellow",
      "implementations": {
        "python": {
          "sig": "yellow(cls) -> \"Color\"",
          "code": "def yellow(cls) -> \"Color\":\n\n        \"\"\"Create a yellow color.\"\"\"\n        color = cls(255, 255, 0, 255)\n        color.name = \"yellow\"\n        return color\n\n    @classmethod\n    def lime(cls) -> \"Color\":\n        \"\"\"Create a lime color.\"\"\"\n        color = cls(128, 255, 0, 255)\n        color.name = \"lime\"\n        return color\n\n    @classmethod\n    def green(cls) -> \"Color\":\n        \"\"\"Create a green color.\"\"\"\n        color = cls(0, 255, 0, 255)\n        color.name = \"green\"\n        return color",
          "file": "color.py"
        },
        "cpp": {
          "sig": "Color yellow()",
          "code": "Color Color::yellow() { return Color(255, 255, 0, 255, \"yellow\"); }",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "yellow() -> Self",
          "code": "pub fn yellow() -> Self {\n        let mut color = Color::new(255, 255, 0, 255);\n        color.name = \"yellow\".to_string();\n        color\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.lime",
      "implementations": {
        "python": {
          "sig": "lime(cls) -> \"Color\"",
          "code": "def lime(cls) -> \"Color\":\n\n        \"\"\"Create a lime color.\"\"\"\n        color = cls(128, 255, 0, 255)\n        color.name = \"lime\"\n        return color\n\n    @classmethod\n    def green(cls) -> \"Color\":\n        \"\"\"Create a green color.\"\"\"\n        color = cls(0, 255, 0, 255)\n        color.name = \"green\"\n        return color\n\n    @classmethod\n    def mint(cls) -> \"Color\":\n        \"\"\"Create a mint color.\"\"\"\n        color = cls(0, 255, 128, 255)\n        color.name = \"mint\"\n        return color",
          "file": "color.py"
        },
        "cpp": {
          "sig": "Color lime()",
          "code": "Color Color::lime() { return Color(128, 255, 0, 255, \"lime\"); }",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "lime() -> Self",
          "code": "pub fn lime() -> Self {\n        let mut color = Color::new(128, 255, 0, 255);\n        color.name = \"lime\".to_string();\n        color\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.green",
      "implementations": {
        "python": {
          "sig": "green(cls) -> \"Color\"",
          "code": "def green(cls) -> \"Color\":\n\n        \"\"\"Create a green color.\"\"\"\n        color = cls(0, 255, 0, 255)\n        color.name = \"green\"\n        return color\n\n    @classmethod\n    def mint(cls) -> \"Color\":\n        \"\"\"Create a mint color.\"\"\"\n        color = cls(0, 255, 128, 255)\n        color.name = \"mint\"\n        return color\n\n    @classmethod\n    def cyan(cls) -> \"Color\":\n        \"\"\"Create a cyan color.\"\"\"\n        color = cls(0, 255, 255, 255)\n        color.name = \"cyan\"\n        return color",
          "file": "color.py"
        },
        "cpp": {
          "sig": "Color green()",
          "code": "Color Color::green() { return Color(0, 255, 0, 255, \"green\"); }",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "green() -> Self",
          "code": "pub fn green() -> Self {\n        let mut color = Color::new(0, 255, 0, 255);\n        color.name = \"green\".to_string();\n        color\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.mint",
      "implementations": {
        "python": {
          "sig": "mint(cls) -> \"Color\"",
          "code": "def mint(cls) -> \"Color\":\n\n        \"\"\"Create a mint color.\"\"\"\n        color = cls(0, 255, 128, 255)\n        color.name = \"mint\"\n        return color\n\n    @classmethod\n    def cyan(cls) -> \"Color\":\n        \"\"\"Create a cyan color.\"\"\"\n        color = cls(0, 255, 255, 255)\n        color.name = \"cyan\"\n        return color\n\n    @classmethod\n    def azure(cls) -> \"Color\":\n        \"\"\"Create an azure color.\"\"\"\n        color = cls(0, 128, 255, 255)\n        color.name = \"azure\"\n        return color",
          "file": "color.py"
        },
        "cpp": {
          "sig": "Color mint()",
          "code": "Color Color::mint() { return Color(0, 255, 128, 255, \"mint\"); }",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "mint() -> Self",
          "code": "pub fn mint() -> Self {\n        let mut color = Color::new(0, 255, 128, 255);\n        color.name = \"mint\".to_string();\n        color\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.cyan",
      "implementations": {
        "python": {
          "sig": "cyan(cls) -> \"Color\"",
          "code": "def cyan(cls) -> \"Color\":\n\n        \"\"\"Create a cyan color.\"\"\"\n        color = cls(0, 255, 255, 255)\n        color.name = \"cyan\"\n        return color\n\n    @classmethod\n    def azure(cls) -> \"Color\":\n        \"\"\"Create an azure color.\"\"\"\n        color = cls(0, 128, 255, 255)\n        color.name = \"azure\"\n        return color\n\n    @classmethod\n    def blue(cls) -> \"Color\":\n        \"\"\"Create a blue color.\"\"\"\n        color = cls(0, 0, 255, 255)\n        color.name = \"blue\"\n        return color",
          "file": "color.py"
        },
        "cpp": {
          "sig": "Color cyan()",
          "code": "Color Color::cyan() { return Color(0, 255, 255, 255, \"cyan\"); }",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "cyan() -> Self",
          "code": "pub fn cyan() -> Self {\n        let mut color = Color::new(0, 255, 255, 255);\n        color.name = \"cyan\".to_string();\n        color\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.azure",
      "implementations": {
        "python": {
          "sig": "azure(cls) -> \"Color\"",
          "code": "def azure(cls) -> \"Color\":\n\n        \"\"\"Create an azure color.\"\"\"\n        color = cls(0, 128, 255, 255)\n        color.name = \"azure\"\n        return color\n\n    @classmethod\n    def blue(cls) -> \"Color\":\n        \"\"\"Create a blue color.\"\"\"\n        color = cls(0, 0, 255, 255)\n        color.name = \"blue\"\n        return color\n\n    @classmethod\n    def violet(cls) -> \"Color\":\n        \"\"\"Create a violet color.\"\"\"\n        color = cls(128, 0, 255, 255)\n        color.name = \"violet\"\n        return color",
          "file": "color.py"
        },
        "cpp": {
          "sig": "Color azure()",
          "code": "Color Color::azure() { return Color(0, 128, 255, 255, \"azure\"); }",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "azure() -> Self",
          "code": "pub fn azure() -> Self {\n        let mut color = Color::new(0, 128, 255, 255);\n        color.name = \"azure\".to_string();\n        color\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.blue",
      "implementations": {
        "python": {
          "sig": "blue(cls) -> \"Color\"",
          "code": "def blue(cls) -> \"Color\":\n\n        \"\"\"Create a blue color.\"\"\"\n        color = cls(0, 0, 255, 255)\n        color.name = \"blue\"\n        return color\n\n    @classmethod\n    def violet(cls) -> \"Color\":\n        \"\"\"Create a violet color.\"\"\"\n        color = cls(128, 0, 255, 255)\n        color.name = \"violet\"\n        return color\n\n    @classmethod\n    def magenta(cls) -> \"Color\":\n        \"\"\"Create a magenta color.\"\"\"\n        color = cls(255, 0, 255, 255)\n        color.name = \"magenta\"\n        return color",
          "file": "color.py"
        },
        "cpp": {
          "sig": "Color blue()",
          "code": "Color Color::blue() { return Color(0, 0, 255, 255, \"blue\"); }",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "blue() -> Self",
          "code": "pub fn blue() -> Self {\n        let mut color = Color::new(0, 0, 255, 255);\n        color.name = \"blue\".to_string();\n        color\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.violet",
      "implementations": {
        "python": {
          "sig": "violet(cls) -> \"Color\"",
          "code": "def violet(cls) -> \"Color\":\n\n        \"\"\"Create a violet color.\"\"\"\n        color = cls(128, 0, 255, 255)\n        color.name = \"violet\"\n        return color\n\n    @classmethod\n    def magenta(cls) -> \"Color\":\n        \"\"\"Create a magenta color.\"\"\"\n        color = cls(255, 0, 255, 255)\n        color.name = \"magenta\"\n        return color\n\n    @classmethod\n    def pink(cls) -> \"Color\":\n        \"\"\"Create a pink color.\"\"\"\n        color = cls(255, 0, 128, 255)\n        color.name = \"pink\"\n        return color",
          "file": "color.py"
        },
        "cpp": {
          "sig": "Color violet()",
          "code": "Color Color::violet() { return Color(128, 0, 255, 255, \"violet\"); }",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "violet() -> Self",
          "code": "pub fn violet() -> Self {\n        let mut color = Color::new(128, 0, 255, 255);\n        color.name = \"violet\".to_string();\n        color\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.magenta",
      "implementations": {
        "python": {
          "sig": "magenta(cls) -> \"Color\"",
          "code": "def magenta(cls) -> \"Color\":\n\n        \"\"\"Create a magenta color.\"\"\"\n        color = cls(255, 0, 255, 255)\n        color.name = \"magenta\"\n        return color\n\n    @classmethod\n    def pink(cls) -> \"Color\":\n        \"\"\"Create a pink color.\"\"\"\n        color = cls(255, 0, 128, 255)\n        color.name = \"pink\"\n        return color\n\n    @classmethod\n    def maroon(cls) -> \"Color\":\n        \"\"\"Create a maroon color.\"\"\"\n        color = cls(128, 0, 0, 255)\n        color.name = \"maroon\"\n        return color",
          "file": "color.py"
        },
        "cpp": {
          "sig": "Color magenta()",
          "code": "Color Color::magenta() { return Color(255, 0, 255, 255, \"magenta\"); }",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "magenta() -> Self",
          "code": "pub fn magenta() -> Self {\n        let mut color = Color::new(255, 0, 255, 255);\n        color.name = \"magenta\".to_string();\n        color\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.pink",
      "implementations": {
        "python": {
          "sig": "pink(cls) -> \"Color\"",
          "code": "def pink(cls) -> \"Color\":\n\n        \"\"\"Create a pink color.\"\"\"\n        color = cls(255, 0, 128, 255)\n        color.name = \"pink\"\n        return color\n\n    @classmethod\n    def maroon(cls) -> \"Color\":\n        \"\"\"Create a maroon color.\"\"\"\n        color = cls(128, 0, 0, 255)\n        color.name = \"maroon\"\n        return color\n\n    @classmethod\n    def brown(cls) -> \"Color\":\n        \"\"\"Create a brown color.\"\"\"\n        color = cls(128, 64, 0, 255)\n        color.name = \"brown\"\n        return color",
          "file": "color.py"
        },
        "cpp": {
          "sig": "Color pink()",
          "code": "Color Color::pink() { return Color(255, 0, 128, 255, \"pink\"); }",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "pink() -> Self",
          "code": "pub fn pink() -> Self {\n        let mut color = Color::new(255, 0, 128, 255);\n        color.name = \"pink\".to_string();\n        color\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.maroon",
      "implementations": {
        "python": {
          "sig": "maroon(cls) -> \"Color\"",
          "code": "def maroon(cls) -> \"Color\":\n\n        \"\"\"Create a maroon color.\"\"\"\n        color = cls(128, 0, 0, 255)\n        color.name = \"maroon\"\n        return color\n\n    @classmethod\n    def brown(cls) -> \"Color\":\n        \"\"\"Create a brown color.\"\"\"\n        color = cls(128, 64, 0, 255)\n        color.name = \"brown\"\n        return color\n\n    @classmethod\n    def olive(cls) -> \"Color\":\n        \"\"\"Create an olive color.\"\"\"\n        color = cls(128, 128, 0, 255)\n        color.name = \"olive\"\n        return color",
          "file": "color.py"
        },
        "cpp": {
          "sig": "Color maroon()",
          "code": "Color Color::maroon() { return Color(128, 0, 0, 255, \"maroon\"); }",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "maroon() -> Self",
          "code": "pub fn maroon() -> Self {\n        let mut color = Color::new(128, 0, 0, 255);\n        color.name = \"maroon\".to_string();\n        color\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.brown",
      "implementations": {
        "python": {
          "sig": "brown(cls) -> \"Color\"",
          "code": "def brown(cls) -> \"Color\":\n\n        \"\"\"Create a brown color.\"\"\"\n        color = cls(128, 64, 0, 255)\n        color.name = \"brown\"\n        return color\n\n    @classmethod\n    def olive(cls) -> \"Color\":\n        \"\"\"Create an olive color.\"\"\"\n        color = cls(128, 128, 0, 255)\n        color.name = \"olive\"\n        return color\n\n    @classmethod\n    def teal(cls) -> \"Color\":\n        \"\"\"Create a teal color.\"\"\"\n        color = cls(0, 128, 128, 255)\n        color.name = \"teal\"\n        return color",
          "file": "color.py"
        },
        "cpp": {
          "sig": "Color brown()",
          "code": "Color Color::brown() { return Color(128, 64, 0, 255, \"brown\"); }",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "brown() -> Self",
          "code": "pub fn brown() -> Self {\n        let mut color = Color::new(128, 64, 0, 255);\n        color.name = \"brown\".to_string();\n        color\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.olive",
      "implementations": {
        "python": {
          "sig": "olive(cls) -> \"Color\"",
          "code": "def olive(cls) -> \"Color\":\n\n        \"\"\"Create an olive color.\"\"\"\n        color = cls(128, 128, 0, 255)\n        color.name = \"olive\"\n        return color\n\n    @classmethod\n    def teal(cls) -> \"Color\":\n        \"\"\"Create a teal color.\"\"\"\n        color = cls(0, 128, 128, 255)\n        color.name = \"teal\"\n        return color\n\n    @classmethod\n    def navy(cls) -> \"Color\":\n        \"\"\"Create a navy color.\"\"\"\n        color = cls(0, 0, 128, 255)\n        color.name = \"navy\"\n        return color",
          "file": "color.py"
        },
        "cpp": {
          "sig": "Color olive()",
          "code": "Color Color::olive() { return Color(128, 128, 0, 255, \"olive\"); }",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "olive() -> Self",
          "code": "pub fn olive() -> Self {\n        let mut color = Color::new(128, 128, 0, 255);\n        color.name = \"olive\".to_string();\n        color\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.teal",
      "implementations": {
        "python": {
          "sig": "teal(cls) -> \"Color\"",
          "code": "def teal(cls) -> \"Color\":\n\n        \"\"\"Create a teal color.\"\"\"\n        color = cls(0, 128, 128, 255)\n        color.name = \"teal\"\n        return color\n\n    @classmethod\n    def navy(cls) -> \"Color\":\n        \"\"\"Create a navy color.\"\"\"\n        color = cls(0, 0, 128, 255)\n        color.name = \"navy\"\n        return color\n\n    @classmethod\n    def purple(cls) -> \"Color\":\n        \"\"\"Create a purple color.\"\"\"\n        color = cls(128, 0, 128, 255)\n        color.name = \"purple\"\n        return color",
          "file": "color.py"
        },
        "cpp": {
          "sig": "Color teal()",
          "code": "Color Color::teal() { return Color(0, 128, 128, 255, \"teal\"); }",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "teal() -> Self",
          "code": "pub fn teal() -> Self {\n        let mut color = Color::new(0, 128, 128, 255);\n        color.name = \"teal\".to_string();\n        color\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.navy",
      "implementations": {
        "python": {
          "sig": "navy(cls) -> \"Color\"",
          "code": "def navy(cls) -> \"Color\":\n\n        \"\"\"Create a navy color.\"\"\"\n        color = cls(0, 0, 128, 255)\n        color.name = \"navy\"\n        return color\n\n    @classmethod\n    def purple(cls) -> \"Color\":\n        \"\"\"Create a purple color.\"\"\"\n        color = cls(128, 0, 128, 255)\n        color.name = \"purple\"\n        return color\n\n    @classmethod\n    def silver(cls) -> \"Color\":\n        \"\"\"Create a silver color.\"\"\"\n        color = cls(192, 192, 192, 255)\n        color.name = \"silver\"\n        return color",
          "file": "color.py"
        },
        "cpp": {
          "sig": "Color navy()",
          "code": "Color Color::navy() { return Color(0, 0, 128, 255, \"navy\"); }",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "navy() -> Self",
          "code": "pub fn navy() -> Self {\n        let mut color = Color::new(0, 0, 128, 255);\n        color.name = \"navy\".to_string();\n        color\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.purple",
      "implementations": {
        "python": {
          "sig": "purple(cls) -> \"Color\"",
          "code": "def purple(cls) -> \"Color\":\n\n        \"\"\"Create a purple color.\"\"\"\n        color = cls(128, 0, 128, 255)\n        color.name = \"purple\"\n        return color\n\n    @classmethod\n    def silver(cls) -> \"Color\":\n        \"\"\"Create a silver color.\"\"\"\n        color = cls(192, 192, 192, 255)\n        color.name = \"silver\"\n        return color\n\n    ###########################################################################################\n    # JSON Serialization\n    ###########################################################################################\n\n    def __jsondump__(self):\n        \"\"\"Serialize to polymorphic JSON format with type field.\"\"\"\n        # Alphabetical order to match Rust's serde_json",
          "file": "color.py"
        },
        "cpp": {
          "sig": "Color purple()",
          "code": "Color Color::purple() { return Color(128, 0, 128, 255, \"purple\"); }",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "purple() -> Self",
          "code": "pub fn purple() -> Self {\n        let mut color = Color::new(128, 0, 128, 255);\n        color.name = \"purple\".to_string();\n        color\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.silver",
      "implementations": {
        "python": {
          "sig": "silver(cls) -> \"Color\"",
          "code": "def silver(cls) -> \"Color\":\n\n        \"\"\"Create a silver color.\"\"\"\n        color = cls(192, 192, 192, 255)\n        color.name = \"silver\"\n        return color\n\n    ###########################################################################################\n    # JSON Serialization\n    ###########################################################################################\n\n    def __jsondump__(self):\n        \"\"\"Serialize to polymorphic JSON format with type field.\"\"\"\n        # Alphabetical order to match Rust's serde_json\n        return {\n            \"a\": self[3],\n            \"b\": self[2],\n            \"g\": self[1],\n            \"guid\": self.guid,\n            \"name\": self.name,\n            \"r\": self[0],",
          "file": "color.py"
        },
        "cpp": {
          "sig": "Color silver()",
          "code": "Color Color::silver() { return Color(192, 192, 192, 255, \"silver\"); }",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "silver() -> Self",
          "code": "pub fn silver() -> Self {\n        let mut color = Color::new(192, 192, 192, 255);\n        color.name = \"silver\".to_string();\n        color\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.__jsondump__",
      "implementations": {
        "python": {
          "sig": "__jsondump__()",
          "code": "def __jsondump__(self):\n\n        \"\"\"Serialize to polymorphic JSON format with type field.\"\"\"\n        # Alphabetical order to match Rust's serde_json\n        return {\n            \"a\": self[3],\n            \"b\": self[2],\n            \"g\": self[1],\n            \"guid\": self.guid,\n            \"name\": self.name,\n            \"r\": self[0],\n            \"type\": f\"{self.__class__.__name__}\",\n        }\n\n    @classmethod\n    def __jsonload__(cls, data, guid=None, name=None):\n        \"\"\"Deserialize from polymorphic JSON format.\"\"\"\n        color = cls(data[\"r\"], data[\"g\"], data[\"b\"], data.get(\"a\", 255))\n        color.guid = guid if guid is not None else data.get(\"guid\", color.guid)\n        color.name = name if name is not None else data.get(\"name\", color.name)\n        return color",
          "file": "color.py"
        }
      }
    },
    {
      "name": "Color.__jsonload__",
      "implementations": {
        "python": {
          "sig": "__jsonload__(cls, data, guid=None, name=None)",
          "code": "def __jsonload__(cls, data, guid=None, name=None):\n\n        \"\"\"Deserialize from polymorphic JSON format.\"\"\"\n        color = cls(data[\"r\"], data[\"g\"], data[\"b\"], data.get(\"a\", 255))\n        color.guid = guid if guid is not None else data.get(\"guid\", color.guid)\n        color.name = name if name is not None else data.get(\"name\", color.name)\n        return color\n\n    def json_dump(self, filepath):\n        \"\"\"Write JSON to file.\n\n        Parameters\n        ----------\n        filepath : str or Path\n            Path to the output file.\n\n        \"\"\"\n        import json\n        with open(filepath, 'w') as f:\n            json.dump(self.__jsondump__(), f, indent=2)",
          "file": "color.py"
        }
      }
    },
    {
      "name": "Color.json_dump",
      "implementations": {
        "python": {
          "sig": "json_dump(filepath)",
          "code": "def json_dump(self, filepath):\n\n        \"\"\"Write JSON to file.\n\n        Parameters\n        ----------\n        filepath : str or Path\n            Path to the output file.\n\n        \"\"\"\n        import json\n        with open(filepath, 'w') as f:\n            json.dump(self.__jsondump__(), f, indent=2)\n\n    @classmethod\n    def json_load(cls, filepath):\n        \"\"\"Read JSON from file.\n\n        Parameters\n        ----------\n        filepath : str or Path",
          "file": "color.py"
        },
        "cpp": {
          "sig": "void json_dump(const std::string& filename)",
          "code": "void Color::json_dump(const std::string& filename) const {\n  std::ofstream file(filename);\n  file << jsondump().dump(4);\n}",
          "file": "color.cpp"
        }
      }
    },
    {
      "name": "Color.json_load",
      "implementations": {
        "python": {
          "sig": "json_load(cls, filepath)",
          "code": "def json_load(cls, filepath):\n\n        \"\"\"Read JSON from file.\n\n        Parameters\n        ----------\n        filepath : str or Path\n            Path to the JSON file.\n\n        Returns\n        -------\n        :class:`Color`\n            The deserialized Color.\n\n        \"\"\"\n        import json\n        with open(filepath, 'r') as f:\n            data = json.load(f)\n        return cls.__jsonload__(data)\n\n    ###########################################################################################",
          "file": "color.py"
        },
        "cpp": {
          "sig": "Color json_load(const std::string& filename)",
          "code": "Color Color::json_load(const std::string& filename) {\n  std::ifstream file(filename);\n  nlohmann::json data = nlohmann::json::parse(file);\n  return jsonload(data);\n}",
          "file": "color.cpp"
        }
      }
    },
    {
      "name": "Color.to_protobuf",
      "implementations": {
        "python": {
          "sig": "to_protobuf()",
          "code": "def to_protobuf(self):\n\n        \"\"\"Convert to protobuf binary format.\n\n        Returns\n        -------\n        bytes\n            Serialized protobuf data.\n\n        Raises\n        ------\n        ImportError\n            If protobuf module is not available.\n\n        \"\"\"\n        if not _HAS_PROTOBUF:\n            raise ImportError(\"protobuf not available\")\n        proto = color_pb2.Color()\n        proto.guid = self.guid\n        proto.name = self.name\n        proto.r = self[0]",
          "file": "color.py"
        },
        "cpp": {
          "sig": "std::string to_protobuf()",
          "code": "std::string Color::to_protobuf() const {\n  session_proto::Color proto;\n  proto.set_guid(guid);\n  proto.set_name(name);\n  proto.set_r(r);\n  proto.set_g(g);\n  proto.set_b(b);\n  proto.set_a(a);\n  return proto.SerializeAsString();\n}",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "to_protobuf() -> Vec<u8>",
          "code": "pub fn to_protobuf(&self) -> Vec<u8> {\n        use prost::Message;\n        \n        let proto = crate::proto::Color {\n            guid: self.guid.clone(),\n            name: self.name.clone(),\n            r: self.r as i32,\n            g: self.g as i32,\n            b: self.b as i32,\n            a: self.a as i32,\n        };\n        proto.encode_to_vec()\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.from_protobuf",
      "implementations": {
        "python": {
          "sig": "from_protobuf(cls, data)",
          "code": "def from_protobuf(cls, data):\n\n        \"\"\"Create color from protobuf binary data.\n\n        Parameters\n        ----------\n        data : bytes\n            Protobuf-encoded color data.\n\n        Returns\n        -------\n        :class:`Color`\n            The deserialized Color.\n\n        Raises\n        ------\n        ImportError\n            If protobuf module is not available.\n\n        \"\"\"\n        if not _HAS_PROTOBUF:",
          "file": "color.py"
        },
        "cpp": {
          "sig": "Color from_protobuf(const std::string& data)",
          "code": "Color Color::from_protobuf(const std::string& data) {\n  session_proto::Color proto;\n  proto.ParseFromString(data);\n  \n  Color color(proto.r(), proto.g(), proto.b(), proto.a(), proto.name());\n  color.guid = proto.guid();\n  return color;\n}",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "from_protobuf(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn from_protobuf(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {\n        use prost::Message;\n        \n        let proto = crate::proto::Color::decode(data)?;\n        \n        let mut color = Self::new(proto.r as u8, proto.g as u8, proto.b as u8, proto.a as u8);\n        color.guid = proto.guid;\n        color.name = proto.name;\n        Ok(color)\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.protobuf_dump",
      "implementations": {
        "python": {
          "sig": "protobuf_dump(filepath)",
          "code": "def protobuf_dump(self, filepath):\n\n        \"\"\"Write protobuf to file.\n\n        Parameters\n        ----------\n        filepath : str\n            Path to the output file.\n\n        \"\"\"\n        data = self.to_protobuf()\n        with open(filepath, 'wb') as f:\n            f.write(data)\n\n    @classmethod\n    def protobuf_load(cls, filepath):\n        \"\"\"Read protobuf from file.\n\n        Parameters\n        ----------\n        filepath : str",
          "file": "color.py"
        },
        "cpp": {
          "sig": "void protobuf_dump(const std::string& filename)",
          "code": "void Color::protobuf_dump(const std::string& filename) const {\n  std::string data = to_protobuf();\n  std::ofstream file(filename, std::ios::binary);\n  file.write(data.data(), data.size());\n}",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "protobuf_dump(filepath: &str)",
          "code": "pub fn protobuf_dump(&self, filepath: &str) {\n        let data = self.to_protobuf();\n        std::fs::write(filepath, data).expect(\"Failed to write protobuf file\");\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.protobuf_load",
      "implementations": {
        "python": {
          "sig": "protobuf_load(cls, filepath)",
          "code": "def protobuf_load(cls, filepath):\n\n        \"\"\"Read protobuf from file.\n\n        Parameters\n        ----------\n        filepath : str\n            Path to the protobuf file.\n\n        Returns\n        -------\n        :class:`Color`\n            The deserialized Color.\n\n        \"\"\"\n        with open(filepath, 'rb') as f:\n            data = f.read()\n        return cls.from_protobuf(data)\n\n    def __str__(self) -> str:\n        \"\"\"String representation.\"\"\"",
          "file": "color.py"
        },
        "cpp": {
          "sig": "Color protobuf_load(const std::string& filename)",
          "code": "Color Color::protobuf_load(const std::string& filename) {\n  std::ifstream file(filename, std::ios::binary);\n  std::string data((std::istreambuf_iterator<char>(file)),\n                    std::istreambuf_iterator<char>());\n  return from_protobuf(data);\n}",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "protobuf_load(filepath: &str) -> Self",
          "code": "pub fn protobuf_load(filepath: &str) -> Self {\n        let data = std::fs::read(filepath).expect(\"Failed to read protobuf file\");\n        Self::from_protobuf(&data).expect(\"Failed to parse protobuf\")\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.__str__",
      "implementations": {
        "python": {
          "sig": "__str__() -> str",
          "code": "def __str__(self) -> str:\n\n        \"\"\"String representation.\"\"\"\n        return f\"{self[0]}, {self[1]}, {self[2]}, {self[3]}\"\n\n    def __repr__(self) -> str:\n        return f\"Color({self.name}, {self[0]}, {self[1]}, {self[2]}, {self[3]})\"\n\n    def __eq__(self, other) -> bool:\n        if not isinstance(other, Color):\n            return False\n        return (\n            self.name == other.name\n            and self[0] == other[0]\n            and self[1] == other[1]\n            and self[2] == other[2]\n            and self[3] == other[3]\n        )\n\n    def __ne__(self, other) -> bool:\n        return not self == other",
          "file": "color.py"
        }
      }
    },
    {
      "name": "Color.__repr__",
      "implementations": {
        "python": {
          "sig": "__repr__() -> str",
          "code": "def __repr__(self) -> str:\n\n        return f\"Color({self.name}, {self[0]}, {self[1]}, {self[2]}, {self[3]})\"\n\n    def __eq__(self, other) -> bool:\n        if not isinstance(other, Color):\n            return False\n        return (\n            self.name == other.name\n            and self[0] == other[0]\n            and self[1] == other[1]\n            and self[2] == other[2]\n            and self[3] == other[3]\n        )\n\n    def __ne__(self, other) -> bool:\n        return not self == other",
          "file": "color.py"
        }
      }
    },
    {
      "name": "Color.__eq__",
      "implementations": {
        "python": {
          "sig": "__eq__(other) -> bool",
          "code": "def __eq__(self, other) -> bool:\n\n        if not isinstance(other, Color):\n            return False\n        return (\n            self.name == other.name\n            and self[0] == other[0]\n            and self[1] == other[1]\n            and self[2] == other[2]\n            and self[3] == other[3]\n        )\n\n    def __ne__(self, other) -> bool:\n        return not self == other",
          "file": "color.py"
        }
      }
    },
    {
      "name": "Color.__ne__",
      "implementations": {
        "python": {
          "sig": "__ne__(other) -> bool",
          "code": "def __ne__(self, other) -> bool:\n\n        return not self == other",
          "file": "color.py"
        }
      }
    },
    {
      "name": "Line.__init__",
      "implementations": {
        "python": {
          "sig": "__init__(x0=0.0, y0=0.0, z0=0.0, x1=0.0, y1=0.0, z1=1.0)",
          "code": "def __init__(self, x0=0.0, y0=0.0, z0=0.0, x1=0.0, y1=0.0, z1=1.0):\n\n        self.guid = str(uuid.uuid4())\n        self.name = \"my_line\"\n        self._x0 = x0\n        self._y0 = y0\n        self._z0 = z0\n        self._x1 = x1\n        self._y1 = y1\n        self._z1 = z1\n        self.width = 1.0\n        self.linecolor = Color.white()\n        self.xform = Xform.identity()\n\n    def duplicate(self):\n        \"\"\"Create a deep copy of this line with a new GUID.\n\n        Returns\n        -------\n        :class:`Line`\n            A new Line with identical values but a different GUID.",
          "file": "line.py"
        }
      }
    },
    {
      "name": "Line.duplicate",
      "implementations": {
        "python": {
          "sig": "duplicate()",
          "code": "def duplicate(self):\n\n        \"\"\"Create a deep copy of this line with a new GUID.\n\n        Returns\n        -------\n        :class:`Line`\n            A new Line with identical values but a different GUID.\n\n        \"\"\"\n        import copy\n        import uuid\n        result = copy.deepcopy(self)\n        result.guid = str(uuid.uuid4())\n        return result\n\n    @classmethod\n    def fit_points(cls, points, length=None):\n        \"\"\"Fit a line to a set of points using least squares (PCA).\n\n        Uses Principal Component Analysis to find the best-fit line",
          "file": "line.py"
        },
        "rust": {
          "sig": "duplicate() -> Self",
          "code": "pub fn duplicate(&self) -> Self {\n        let mut copy = self.clone();\n        copy.guid = Uuid::new_v4().to_string();\n        copy\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.fit_points",
      "implementations": {
        "python": {
          "sig": "fit_points(cls, points, length=None)",
          "code": "def fit_points(cls, points, length=None):\n\n        \"\"\"Fit a line to a set of points using least squares (PCA).\n\n        Uses Principal Component Analysis to find the best-fit line\n        that minimizes perpendicular distances to all points.\n\n        Parameters\n        ----------\n        points : list of Point\n            List of points to fit (minimum 2 points required).\n        length : float, optional\n            Length of the resulting line. If None, uses the extent\n            of points projected onto the line direction.\n\n        Returns\n        -------\n        Line\n            Best-fit line through the points.\n\n        Raises",
          "file": "line.py"
        },
        "cpp": {
          "sig": "Line fit_points(const std::vector<Point>& points, double length)",
          "code": "Line Line::fit_points(const std::vector<Point>& points, double length) {\n    if (points.size() < 2) {\n        throw std::invalid_argument(\"At least 2 points are required for line fitting\");\n    }",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "fit_points(points: &[Point], length: Option<f64>) -> Self",
          "code": "pub fn fit_points(points: &[Point], length: Option<f64>) -> Self {\n        if points.len() < 2 {\n            panic!(\"At least 2 points are required for line fitting\");\n        }\n\n        let n = points.len() as f64;\n\n        // Compute centroid\n        let (mut cx, mut cy, mut cz) = (0.0, 0.0, 0.0);\n        for p in points {\n            cx += p[0];\n            cy += p[1];\n            cz += p[2];\n        }\n        cx /= n;\n        cy /= n;\n        cz /= n;\n\n        // Compute covariance matrix elements\n        let (mut cxx, mut cyy, mut czz) = (0.0, 0.0, 0.0);\n        let (mut cxy, mut cxz, mut cyz) = (0.0, 0.0, 0.0);\n        for p in points {\n            let dx = p[0] - cx;\n            let dy = p[1] - cy;\n            let dz = p[2] - cz;\n            cxx += dx * dx;\n            cyy += dy",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.from_points",
      "implementations": {
        "python": {
          "sig": "from_points(cls, p1, p2)",
          "code": "def from_points(cls, p1, p2):\n\n        \"\"\"Create a line from two points.\n\n        Parameters\n        ----------\n        p1 : Point\n            Start point.\n        p2 : Point\n            End point.\n\n        Returns\n        -------\n        Line\n            New line from p1 to p2.\n        \"\"\"\n        return cls(p1[0], p1[1], p1[2], p2[0], p2[1], p2[2])\n\n    @classmethod\n    def from_point_and_vector(cls, point, vector):\n        \"\"\"Create a line from a point and a vector.",
          "file": "line.py"
        },
        "cpp": {
          "sig": "Line from_points(const Point& p1, const Point& p2)",
          "code": "Line Line::from_points(const Point& p1, const Point& p2) {\n    return Line(p1[0], p1[1], p1[2], p2[0], p2[1], p2[2]);\n}",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "from_points(p1: &Point, p2: &Point) -> Self",
          "code": "pub fn from_points(p1: &Point, p2: &Point) -> Self {\n        Self::new(p1[0], p1[1], p1[2], p2[0], p2[1], p2[2])\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.from_point_and_vector",
      "implementations": {
        "python": {
          "sig": "from_point_and_vector(cls, point, vector)",
          "code": "def from_point_and_vector(cls, point, vector):\n\n        \"\"\"Create a line from a point and a vector.\n\n        Parameters\n        ----------\n        point : Point\n            Start point of the line.\n        vector : Vector\n            Direction and length of the line.\n\n        Returns\n        -------\n        Line\n            New line from point to point + vector.\n        \"\"\"\n        return cls(\n            point[0], point[1], point[2],\n            point[0] + vector[0], point[1] + vector[1], point[2] + vector[2]\n        )",
          "file": "line.py"
        },
        "cpp": {
          "sig": "Line from_point_and_vector(const Point& point, const Vector& vector)",
          "code": "Line Line::from_point_and_vector(const Point& point, const Vector& vector) {\n    return Line(\n        point[0], point[1], point[2],\n        point[0] + vector[0], point[1] + vector[1], point[2] + vector[2]\n    );\n}",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "from_point_and_vector(point: &Point, vector: &Vector) -> Self",
          "code": "pub fn from_point_and_vector(point: &Point, vector: &Vector) -> Self {\n        Self::new(\n            point[0], point[1], point[2],\n            point[0] + vector[0], point[1] + vector[1], point[2] + vector[2],\n        )\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.from_point_direction_length",
      "implementations": {
        "python": {
          "sig": "from_point_direction_length(cls, point, direction, length)",
          "code": "def from_point_direction_length(cls, point, direction, length):\n\n        \"\"\"Create a line from a point, direction, and length.\n\n        Parameters\n        ----------\n        point : Point\n            Start point of the line.\n        direction : Vector\n            Direction of the line (will be normalized).\n        length : float\n            Length of the line.\n\n        Returns\n        -------\n        Line\n            New line from point in direction with given length.\n        \"\"\"\n        d = direction.normalize()\n        return cls(\n            point[0], point[1], point[2],",
          "file": "line.py"
        },
        "cpp": {
          "sig": "Line from_point_direction_length(const Point& point, const Vector& direction, double length)",
          "code": "Line Line::from_point_direction_length(const Point& point, const Vector& direction, double length) {\n    Vector d = direction.normalize();\n    return Line(\n        point[0], point[1], point[2],\n        point[0] + d[0] * length, point[1] + d[1] * length, point[2] + d[2] * length\n    );\n}",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "from_point_direction_length(point: &Point, direction: &Vector, length: f64) -> Self",
          "code": "pub fn from_point_direction_length(point: &Point, direction: &Vector, length: f64) -> Self {\n        let d = direction.normalized();\n        Self::new(\n            point[0], point[1], point[2],\n            point[0] + d[0] * length, point[1] + d[1] * length, point[2] + d[2] * length,\n        )\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.with_name",
      "implementations": {
        "python": {
          "sig": "with_name(cls, name, x0, y0, z0, x1, y1, z1)",
          "code": "def with_name(cls, name, x0, y0, z0, x1, y1, z1):\n\n        \"\"\"Create a line with a specific name.\n\n        Parameters\n        ----------\n        name : str\n            Name for the line.\n        x0, y0, z0 : float\n            Start point coordinates.\n        x1, y1, z1 : float\n            End point coordinates.\n\n        Returns\n        -------\n        Line\n            New named line.\n        \"\"\"\n        line = cls(x0, y0, z0, x1, y1, z1)\n        line.name = name\n        return line",
          "file": "line.py"
        },
        "cpp": {
          "sig": "Line with_name(const std::string& name, double x0, double y0, double z0, double x1, double y1, double z1)",
          "code": "Line Line::with_name(const std::string& name, double x0, double y0, double z0, double x1, double y1, double z1) {\n    Line line(x0, y0, z0, x1, y1, z1);\n    line.name = name;\n    return line;\n}",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "with_name(name: &str, x0: f64, y0: f64, z0: f64, x1: f64, y1: f64, z1: f64) -> Self",
          "code": "pub fn with_name(name: &str, x0: f64, y0: f64, z0: f64, x1: f64, y1: f64, z1: f64) -> Self {\n        Self {\n            name: name.to_string(),\n            _x0: x0,\n            _y0: y0,\n            _z0: z0,\n            _x1: x1,\n            _y1: y1,\n            _z1: z1,\n            ..Default::default()\n        }\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.length",
      "implementations": {
        "python": {
          "sig": "length()",
          "code": "def length(self):\n\n        \"\"\"Calculate the length of the line.\n\n        Returns\n        -------\n        float\n            Length of the line.\n        \"\"\"\n        dx = self._x1 - self._x0\n        dy = self._y1 - self._y0\n        dz = self._z1 - self._z0\n        return (dx * dx + dy * dy + dz * dz) ** 0.5\n\n    def squared_length(self):\n        \"\"\"Calculate the squared length of the line.\n\n        Returns\n        -------\n        float\n            Squared length of the line.",
          "file": "line.py"
        },
        "cpp": {
          "sig": "double length()",
          "code": "double Line::length() const {\n    double dx = _x1 - _x0;\n    double dy = _y1 - _y0;\n    double dz = _z1 - _z0;\n    return std::sqrt(dx * dx + dy * dy + dz * dz);\n}",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "length() -> f64",
          "code": "pub fn length(&self) -> f64 {\n        let dx = self._x1 - self._x0;\n        let dy = self._y1 - self._y0;\n        let dz = self._z1 - self._z0;\n        (dx * dx + dy * dy + dz * dz).sqrt()\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.squared_length",
      "implementations": {
        "python": {
          "sig": "squared_length()",
          "code": "def squared_length(self):\n\n        \"\"\"Calculate the squared length of the line.\n\n        Returns\n        -------\n        float\n            Squared length of the line.\n        \"\"\"\n        dx = self._x1 - self._x0\n        dy = self._y1 - self._y0\n        dz = self._z1 - self._z0\n        return dx * dx + dy * dy + dz * dz\n\n    def to_vector(self):\n        \"\"\"Convert line to vector from start to end.\n\n        Returns\n        -------\n        Vector\n            Direction vector of the line.",
          "file": "line.py"
        },
        "cpp": {
          "sig": "double squared_length()",
          "code": "double Line::squared_length() const {\n    double dx = _x1 - _x0;\n    double dy = _y1 - _y0;\n    double dz = _z1 - _z0;\n    return dx * dx + dy * dy + dz * dz;\n}",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "squared_length() -> f64",
          "code": "pub fn squared_length(&self) -> f64 {\n        let dx = self._x1 - self._x0;\n        let dy = self._y1 - self._y0;\n        let dz = self._z1 - self._z0;\n        dx * dx + dy * dy + dz * dz\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.to_vector",
      "implementations": {
        "python": {
          "sig": "to_vector()",
          "code": "def to_vector(self):\n\n        \"\"\"Convert line to vector from start to end.\n\n        Returns\n        -------\n        Vector\n            Direction vector of the line.\n        \"\"\"\n        return Vector(self._x1 - self._x0, self._y1 - self._y0, self._z1 - self._z0)\n\n    def to_direction(self):\n        \"\"\"Convert line to unit direction vector.\n\n        Returns\n        -------\n        Vector\n            Normalized direction vector from start to end.\n        \"\"\"\n        return self.to_vector().normalize()",
          "file": "line.py"
        },
        "cpp": {
          "sig": "Vector to_vector()",
          "code": "Vector Line::to_vector() const {\n    return Vector(_x1 - _x0, _y1 - _y0, _z1 - _z0);\n}",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "to_vector() -> Vector",
          "code": "pub fn to_vector(&self) -> Vector {\n        Vector::new(\n            self._x1 - self._x0,\n            self._y1 - self._y0,\n            self._z1 - self._z0,\n        )\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.to_direction",
      "implementations": {
        "python": {
          "sig": "to_direction()",
          "code": "def to_direction(self):\n\n        \"\"\"Convert line to unit direction vector.\n\n        Returns\n        -------\n        Vector\n            Normalized direction vector from start to end.\n        \"\"\"\n        return self.to_vector().normalize()\n\n    def point_at(self, t):\n        \"\"\"Get point at parameter t along the line.\n\n        Parameters\n        ----------\n        t : float\n            Parameter value (0.0 = start, 1.0 = end).\n\n        Returns\n        -------",
          "file": "line.py"
        },
        "cpp": {
          "sig": "Vector to_direction()",
          "code": "Vector Line::to_direction() const {\n    return to_vector().normalize();\n}",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "to_direction() -> Vector",
          "code": "pub fn to_direction(&self) -> Vector {\n        self.to_vector().normalized()\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.point_at",
      "implementations": {
        "python": {
          "sig": "point_at(t)",
          "code": "def point_at(self, t):\n\n        \"\"\"Get point at parameter t along the line.\n\n        Parameters\n        ----------\n        t : float\n            Parameter value (0.0 = start, 1.0 = end).\n\n        Returns\n        -------\n        Point\n            Point at parameter t.\n        \"\"\"\n        s = 1.0 - t\n        return Point(\n            s * self._x0 + t * self._x1,\n            s * self._y0 + t * self._y1,\n            s * self._z0 + t * self._z1,\n        )",
          "file": "line.py"
        },
        "cpp": {
          "sig": "Point point_at(double t)",
          "code": "Point Line::point_at(double t) const {\n    double s = 1.0 - t;\n    return Point(s * _x0 + t * _x1, s * _y0 + t * _y1, s * _z0 + t * _z1);\n}",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "point_at(t: f64) -> Point",
          "code": "pub fn point_at(&self, t: f64) -> Point {\n        let s = 1.0 - t;\n        Point::new(\n            s * self._x0 + t * self._x1,\n            s * self._y0 + t * self._y1,\n            s * self._z0 + t * self._z1,\n        )\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.subdivide",
      "implementations": {
        "python": {
          "sig": "subdivide(n)",
          "code": "def subdivide(self, n):\n\n        \"\"\"Subdivide line into n points.\n\n        Parameters\n        ----------\n        n : int\n            Number of points (must be >= 2).\n\n        Returns\n        -------\n        list of Point\n            List of n points along the line, including start and end.\n        \"\"\"\n        if n < 2:\n            raise ValueError(\"n must be at least 2\")\n        points = []\n        for i in range(n):\n            t = i / (n - 1)\n            points.append(self.point_at(t))\n        return points",
          "file": "line.py"
        },
        "cpp": {
          "sig": "std::vector<Point> subdivide(int n)",
          "code": "std::vector<Point> Line::subdivide(int n) const {\n    if (n < 2) {\n        throw std::invalid_argument(\"n must be at least 2\");\n    }",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "subdivide(n: usize) -> Vec<Point>",
          "code": "pub fn subdivide(&self, n: usize) -> Vec<Point> {\n        if n < 2 {\n            panic!(\"n must be at least 2\");\n        }\n        let mut points = Vec::with_capacity(n);\n        for i in 0..n {\n            let t = i as f64 / (n - 1) as f64;\n            points.push(self.point_at(t));\n        }\n        points\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.subdivide_by_distance",
      "implementations": {
        "python": {
          "sig": "subdivide_by_distance(distance)",
          "code": "def subdivide_by_distance(self, distance):\n\n        \"\"\"Subdivide line by approximate distance between points.\n\n        Parameters\n        ----------\n        distance : float\n            Target distance between consecutive points.\n\n        Returns\n        -------\n        list of Point\n            List of points along the line, including start and end.\n        \"\"\"\n        if distance <= 0:\n            raise ValueError(\"distance must be positive\")\n        length = self.length()\n        if length < 1e-10:\n            return [self.start(), self.end()]\n        n = max(2, int(length / distance + 0.5) + 1)\n        return self.subdivide(n)",
          "file": "line.py"
        },
        "cpp": {
          "sig": "std::vector<Point> subdivide_by_distance(double distance)",
          "code": "std::vector<Point> Line::subdivide_by_distance(double distance) const {\n    if (distance <= 0) {\n        throw std::invalid_argument(\"distance must be positive\");\n    }",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "subdivide_by_distance(distance: f64) -> Vec<Point>",
          "code": "pub fn subdivide_by_distance(&self, distance: f64) -> Vec<Point> {\n        if distance <= 0.0 {\n            panic!(\"distance must be positive\");\n        }\n        let len = self.length();\n        if len < 1e-10 {\n            return vec![self.start(), self.end()];\n        }\n        let n = 2.max((len / distance + 0.5) as usize + 1);\n        self.subdivide(n)\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.start",
      "implementations": {
        "python": {
          "sig": "start()",
          "code": "def start(self):\n\n        \"\"\"Get start point.\n\n        Returns\n        -------\n        Point\n            Start point of the line.\n        \"\"\"\n        return Point(self._x0, self._y0, self._z0)\n\n    def end(self):\n        \"\"\"Get end point.\n\n        Returns\n        -------\n        Point\n            End point of the line.\n        \"\"\"\n        return Point(self._x1, self._y1, self._z1)",
          "file": "line.py"
        },
        "cpp": {
          "sig": "Point start()",
          "code": "Point Line::start() const {\n    return Point(_x0, _y0, _z0);\n}",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "start() -> Point",
          "code": "pub fn start(&self) -> Point {\n        Point::new(self._x0, self._y0, self._z0)\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.end",
      "implementations": {
        "python": {
          "sig": "end()",
          "code": "def end(self):\n\n        \"\"\"Get end point.\n\n        Returns\n        -------\n        Point\n            End point of the line.\n        \"\"\"\n        return Point(self._x1, self._y1, self._z1)\n\n    def center(self):\n        \"\"\"Get center point (average of start and end).\n\n        Returns\n        -------\n        Point\n            Center point of the line.\n        \"\"\"\n        return Point(\n            (self._x0 + self._x1) * 0.5,",
          "file": "line.py"
        },
        "cpp": {
          "sig": "Point end()",
          "code": "Point Line::end() const {\n    return Point(_x1, _y1, _z1);\n}",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "end() -> Point",
          "code": "pub fn end(&self) -> Point {\n        Point::new(self._x1, self._y1, self._z1)\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.center",
      "implementations": {
        "python": {
          "sig": "center()",
          "code": "def center(self):\n\n        \"\"\"Get center point (average of start and end).\n\n        Returns\n        -------\n        Point\n            Center point of the line.\n        \"\"\"\n        return Point(\n            (self._x0 + self._x1) * 0.5,\n            (self._y0 + self._y1) * 0.5,\n            (self._z0 + self._z1) * 0.5,\n        )\n\n    def closest_point(self, point):\n        \"\"\"Find the closest point on the line to a given point.\n\n        Parameters\n        ----------\n        point : Point",
          "file": "line.py"
        },
        "cpp": {
          "sig": "Point center()",
          "code": "Point Line::center() const {\n    return Point(\n        (_x0 + _x1) * 0.5,\n        (_y0 + _y1) * 0.5,\n        (_z0 + _z1) * 0.5\n    );\n}",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "center() -> Point",
          "code": "pub fn center(&self) -> Point {\n        Point::new(\n            (self._x0 + self._x1) * 0.5,\n            (self._y0 + self._y1) * 0.5,\n            (self._z0 + self._z1) * 0.5,\n        )\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.closest_point",
      "implementations": {
        "python": {
          "sig": "closest_point(point)",
          "code": "def closest_point(self, point):\n\n        \"\"\"Find the closest point on the line to a given point.\n\n        Parameters\n        ----------\n        point : Point\n            The point to find the closest point to.\n\n        Returns\n        -------\n        Point\n            The closest point on the line segment.\n        \"\"\"\n        dx = self._x1 - self._x0\n        dy = self._y1 - self._y0\n        dz = self._z1 - self._z0\n        len_sq = dx * dx + dy * dy + dz * dz\n        if len_sq < 1e-20:\n            return self.start()\n        t = ((point[0] - self._x0) * dx + (point[1] - self._y0) * dy + (point[2] - self._z0) * dz) / len_sq",
          "file": "line.py"
        },
        "cpp": {
          "sig": "Point closest_point(const Point& point)",
          "code": "Point Line::closest_point(const Point& point) const {\n    double dx = _x1 - _x0;\n    double dy = _y1 - _y0;\n    double dz = _z1 - _z0;\n    double len_sq = dx * dx + dy * dy + dz * dz;\n    if (len_sq < 1e-20) {\n        return start();\n    }",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "closest_point(point: &Point) -> Point",
          "code": "pub fn closest_point(&self, point: &Point) -> Point {\n        let dx = self._x1 - self._x0;\n        let dy = self._y1 - self._y0;\n        let dz = self._z1 - self._z0;\n        let len_sq = dx * dx + dy * dy + dz * dz;\n        if len_sq < 1e-20 {\n            return self.start();\n        }\n        let t = ((point[0] - self._x0) * dx + (point[1] - self._y0) * dy + (point[2] - self._z0) * dz) / len_sq;\n        let t = t.clamp(0.0, 1.0);\n        self.point_at(t)\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.get_middle_line",
      "implementations": {
        "python": {
          "sig": "get_middle_line(line0_start: Point, line0_end: Point, line1_start: Point, line1_end: Point)",
          "code": "def get_middle_line(line0_start: Point, line0_end: Point, line1_start: Point, line1_end: Point):\n\n        \"\"\"Calculate middle line between two line segments.\n\n        Returns\n        -------\n        tuple\n            (start_point, end_point) of the middle line.\n        \"\"\"\n        p0 = Point(\n            (line0_start.x + line1_start.x) * 0.5,\n            (line0_start.y + line1_start.y) * 0.5,\n            (line0_start.z + line1_start.z) * 0.5,\n        )\n        p1 = Point(\n            (line0_end.x + line1_end.x) * 0.5,\n            (line0_end.y + line1_end.y) * 0.5,\n            (line0_end.z + line1_end.z) * 0.5,\n        )\n        return p0, p1",
          "file": "line.py"
        },
        "cpp": {
          "sig": "void get_middle_line(const Point& line0_start, const Point& line0_end,\n                          const Point& line1_start, const Point& line1_end,\n                          Point& output_start, Point& output_end)",
          "code": "void Line::get_middle_line(const Point& line0_start, const Point& line0_end,\n                          const Point& line1_start, const Point& line1_end,\n                          Point& output_start, Point& output_end) {\n    output_start = Point(\n        (line0_start[0] + line1_start[0]) * 0.5,\n        (line0_start[1] + line1_start[1]) * 0.5,\n        (line0_start[2] + line1_start[2]) * 0.5\n    );\n    output_end = Point(\n        (line0_end[0] + line1_end[0]) * 0.5,\n        (line0_end[1] + line1_end[1]) * 0.5,\n        (line0_end[2] + line1_end[2]) * 0.5\n    );\n}",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "get_middle_line(\n        line0_start: &Point,\n        line0_end: &Point,\n        line1_start: &Point,\n        line1_end: &Point,\n    ) -> (Point, Point)",
          "code": "pub fn get_middle_line(\n        line0_start: &Point,\n        line0_end: &Point,\n        line1_start: &Point,\n        line1_end: &Point,\n    ) -> (Point, Point) {\n        let p0 = Point::new(\n            (line0_start[0] + line1_start[0]) * 0.5,\n            (line0_start[1] + line1_start[1]) * 0.5,\n            (line0_start[2] + line1_start[2]) * 0.5,\n        );\n        let p1 = Point::new(\n            (line0_end[0] + line1_end[0]) * 0.5,\n            (line0_end[1] + line1_end[1]) * 0.5,\n            (line0_end[2] + line1_end[2]) * 0.5,\n        );\n        (p0, p1)\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.__getitem__",
      "implementations": {
        "python": {
          "sig": "__getitem__(index)",
          "code": "def __getitem__(self, index):\n\n        \"\"\"Get coordinate by index (0-5).\"\"\"\n        coords = [self._x0, self._y0, self._z0, self._x1, self._y1, self._z1]\n        return coords[index]\n\n    def __setitem__(self, index, value):\n        \"\"\"Set coordinate by index (0-5).\"\"\"\n        if index == 0:\n            self._x0 = value\n        elif index == 1:\n            self._y0 = value\n        elif index == 2:\n            self._z0 = value\n        elif index == 3:\n            self._x1 = value\n        elif index == 4:\n            self._y1 = value\n        elif index == 5:\n            self._z1 = value\n        else:",
          "file": "line.py"
        }
      }
    },
    {
      "name": "Line.__setitem__",
      "implementations": {
        "python": {
          "sig": "__setitem__(index, value)",
          "code": "def __setitem__(self, index, value):\n\n        \"\"\"Set coordinate by index (0-5).\"\"\"\n        if index == 0:\n            self._x0 = value\n        elif index == 1:\n            self._y0 = value\n        elif index == 2:\n            self._z0 = value\n        elif index == 3:\n            self._x1 = value\n        elif index == 4:\n            self._y1 = value\n        elif index == 5:\n            self._z1 = value\n        else:\n            raise IndexError(\"Index out of bounds\")\n\n    def __iadd__(self, other):\n        \"\"\"Add vector to line in place.\"\"\"\n        if isinstance(other, Vector):",
          "file": "line.py"
        }
      }
    },
    {
      "name": "Line.__iadd__",
      "implementations": {
        "python": {
          "sig": "__iadd__(other)",
          "code": "def __iadd__(self, other):\n\n        \"\"\"Add vector to line in place.\"\"\"\n        if isinstance(other, Vector):\n            self._x0 += other[0]\n            self._y0 += other[1]\n            self._z0 += other[2]\n            self._x1 += other[0]\n            self._y1 += other[1]\n            self._z1 += other[2]\n        return self\n\n    def __isub__(self, other):\n        \"\"\"Subtract vector from line in place.\"\"\"\n        if isinstance(other, Vector):\n            self._x0 -= other[0]\n            self._y0 -= other[1]\n            self._z0 -= other[2]\n            self._x1 -= other[0]\n            self._y1 -= other[1]\n            self._z1 -= other[2]",
          "file": "line.py"
        }
      }
    },
    {
      "name": "Line.__isub__",
      "implementations": {
        "python": {
          "sig": "__isub__(other)",
          "code": "def __isub__(self, other):\n\n        \"\"\"Subtract vector from line in place.\"\"\"\n        if isinstance(other, Vector):\n            self._x0 -= other[0]\n            self._y0 -= other[1]\n            self._z0 -= other[2]\n            self._x1 -= other[0]\n            self._y1 -= other[1]\n            self._z1 -= other[2]\n        return self\n\n    def __imul__(self, factor):\n        \"\"\"Multiply line coordinates by scalar in place.\"\"\"\n        self._x0 *= factor\n        self._y0 *= factor\n        self._z0 *= factor\n        self._x1 *= factor\n        self._y1 *= factor\n        self._z1 *= factor\n        return self",
          "file": "line.py"
        }
      }
    },
    {
      "name": "Line.__imul__",
      "implementations": {
        "python": {
          "sig": "__imul__(factor)",
          "code": "def __imul__(self, factor):\n\n        \"\"\"Multiply line coordinates by scalar in place.\"\"\"\n        self._x0 *= factor\n        self._y0 *= factor\n        self._z0 *= factor\n        self._x1 *= factor\n        self._y1 *= factor\n        self._z1 *= factor\n        return self\n\n    def __itruediv__(self, factor):\n        \"\"\"Divide line coordinates by scalar in place.\"\"\"\n        self._x0 /= factor\n        self._y0 /= factor\n        self._z0 /= factor\n        self._x1 /= factor\n        self._y1 /= factor\n        self._z1 /= factor\n        return self",
          "file": "line.py"
        }
      }
    },
    {
      "name": "Line.__itruediv__",
      "implementations": {
        "python": {
          "sig": "__itruediv__(factor)",
          "code": "def __itruediv__(self, factor):\n\n        \"\"\"Divide line coordinates by scalar in place.\"\"\"\n        self._x0 /= factor\n        self._y0 /= factor\n        self._z0 /= factor\n        self._x1 /= factor\n        self._y1 /= factor\n        self._z1 /= factor\n        return self\n\n    def __add__(self, other):\n        \"\"\"Add vector to line.\"\"\"\n        if isinstance(other, Vector):\n            return Line(\n                self._x0 + other[0],\n                self._y0 + other[1],\n                self._z0 + other[2],\n                self._x1 + other[0],\n                self._y1 + other[1],\n                self._z1 + other[2],",
          "file": "line.py"
        }
      }
    },
    {
      "name": "Line.__add__",
      "implementations": {
        "python": {
          "sig": "__add__(other)",
          "code": "def __add__(self, other):\n\n        \"\"\"Add vector to line.\"\"\"\n        if isinstance(other, Vector):\n            return Line(\n                self._x0 + other[0],\n                self._y0 + other[1],\n                self._z0 + other[2],\n                self._x1 + other[0],\n                self._y1 + other[1],\n                self._z1 + other[2],\n            )\n        return NotImplemented\n\n    def __sub__(self, other):\n        \"\"\"Subtract vector from line.\"\"\"\n        if isinstance(other, Vector):\n            return Line(\n                self._x0 - other[0],\n                self._y0 - other[1],\n                self._z0 - other[2],",
          "file": "line.py"
        }
      }
    },
    {
      "name": "Line.__sub__",
      "implementations": {
        "python": {
          "sig": "__sub__(other)",
          "code": "def __sub__(self, other):\n\n        \"\"\"Subtract vector from line.\"\"\"\n        if isinstance(other, Vector):\n            return Line(\n                self._x0 - other[0],\n                self._y0 - other[1],\n                self._z0 - other[2],\n                self._x1 - other[0],\n                self._y1 - other[1],\n                self._z1 - other[2],\n            )\n        return NotImplemented\n\n    def __mul__(self, factor):\n        \"\"\"Multiply line by scalar.\"\"\"\n        return Line(\n            self._x0 * factor,\n            self._y0 * factor,\n            self._z0 * factor,\n            self._x1 * factor,",
          "file": "line.py"
        }
      }
    },
    {
      "name": "Line.__mul__",
      "implementations": {
        "python": {
          "sig": "__mul__(factor)",
          "code": "def __mul__(self, factor):\n\n        \"\"\"Multiply line by scalar.\"\"\"\n        return Line(\n            self._x0 * factor,\n            self._y0 * factor,\n            self._z0 * factor,\n            self._x1 * factor,\n            self._y1 * factor,\n            self._z1 * factor,\n        )\n\n    def __truediv__(self, factor):\n        \"\"\"Divide line by scalar.\"\"\"\n        return Line(\n            self._x0 / factor,\n            self._y0 / factor,\n            self._z0 / factor,\n            self._x1 / factor,\n            self._y1 / factor,\n            self._z1 / factor,",
          "file": "line.py"
        }
      }
    },
    {
      "name": "Line.__truediv__",
      "implementations": {
        "python": {
          "sig": "__truediv__(factor)",
          "code": "def __truediv__(self, factor):\n\n        \"\"\"Divide line by scalar.\"\"\"\n        return Line(\n            self._x0 / factor,\n            self._y0 / factor,\n            self._z0 / factor,\n            self._x1 / factor,\n            self._y1 / factor,\n            self._z1 / factor,\n        )\n\n    def __neg__(self):\n        \"\"\"Negate line (flip direction).\"\"\"\n        return Line(self._x1, self._y1, self._z1, self._x0, self._y0, self._z0)\n\n    def transform(self):\n        \"\"\"Apply the stored xform transformation to the line coordinates.\n\n        Transforms the line in-place and resets xform to identity.\n        \"\"\"",
          "file": "line.py"
        }
      }
    },
    {
      "name": "Line.__neg__",
      "implementations": {
        "python": {
          "sig": "__neg__()",
          "code": "def __neg__(self):\n\n        \"\"\"Negate line (flip direction).\"\"\"\n        return Line(self._x1, self._y1, self._z1, self._x0, self._y0, self._z0)\n\n    def transform(self):\n        \"\"\"Apply the stored xform transformation to the line coordinates.\n\n        Transforms the line in-place and resets xform to identity.\n        \"\"\"\n        start = Point(self._x0, self._y0, self._z0)\n        end = Point(self._x1, self._y1, self._z1)\n\n        self.xform.transform_point(start)\n        self.xform.transform_point(end)\n\n        self._x0 = start[0]\n        self._y0 = start[1]\n        self._z0 = start[2]\n        self._x1 = end[0]\n        self._y1 = end[1]",
          "file": "line.py"
        }
      }
    },
    {
      "name": "Line.transform",
      "implementations": {
        "python": {
          "sig": "transform()",
          "code": "def transform(self):\n\n        \"\"\"Apply the stored xform transformation to the line coordinates.\n\n        Transforms the line in-place and resets xform to identity.\n        \"\"\"\n        start = Point(self._x0, self._y0, self._z0)\n        end = Point(self._x1, self._y1, self._z1)\n\n        self.xform.transform_point(start)\n        self.xform.transform_point(end)\n\n        self._x0 = start[0]\n        self._y0 = start[1]\n        self._z0 = start[2]\n        self._x1 = end[0]\n        self._y1 = end[1]\n        self._z1 = end[2]\n        self.xform = Xform.identity()\n\n    def transformed(self):",
          "file": "line.py"
        },
        "cpp": {
          "sig": "void transform()",
          "code": "void Line::transform() {\n  Point start(_x0, _y0, _z0);\n  Point end(_x1, _y1, _z1);\n  \n  xform.transform_point(start);\n  xform.transform_point(end);\n  \n  _x0 = start[0];\n  _y0 = start[1];\n  _z0 = start[2];\n  _x1 = end[0];\n  _y1 = end[1];\n  _z1 = end[2];\n  xform = Xform::identity();\n}",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "transform()",
          "code": "pub fn transform(&mut self) {\n        let mut start = Point::new(self._x0, self._y0, self._z0);\n        let mut end = Point::new(self._x1, self._y1, self._z1);\n\n        // No clone needed - transform_point takes &self\n        self.xform.transform_point(&mut start);\n        self.xform.transform_point(&mut end);\n\n        self._x0 = start[0];\n        self._y0 = start[1];\n        self._z0 = start[2];\n        self._x1 = end[0];\n        self._y1 = end[1];\n        self._z1 = end[2];\n        self.xform = Xform::identity();\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.transformed",
      "implementations": {
        "python": {
          "sig": "transformed()",
          "code": "def transformed(self):\n\n        \"\"\"Return a transformed copy of the line.\n\n        Returns a new line with the transformation applied.\n        The original line and its xform remain unchanged.\n\n        Returns\n        -------\n        Line\n            A new transformed line.\n        \"\"\"\n        import copy\n\n        result = copy.deepcopy(self)\n        result.transform()\n        return result\n\n    ###########################################################################################\n    # Polymorphic JSON Serialization\n    ###########################################################################################",
          "file": "line.py"
        },
        "cpp": {
          "sig": "Line transformed()",
          "code": "Line Line::transformed() const {\n  Line result = *this;\n  result.transform();\n  return result;\n}",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "transformed() -> Self",
          "code": "pub fn transformed(&self) -> Self {\n        let mut result = self.clone();\n        result.transform();\n        result\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.__jsondump__",
      "implementations": {
        "python": {
          "sig": "__jsondump__()",
          "code": "def __jsondump__(self):\n\n        \"\"\"Serialize to polymorphic JSON format with type field.\n\n        Returns\n        -------\n        dict\n            Dictionary with 'type', 'guid', 'name', and object fields.\n\n        \"\"\"\n        # Alphabetical order to match Rust's serde_json\n        return {\n            \"guid\": self.guid,\n            \"linecolor\": self.linecolor.__jsondump__(),\n            \"name\": self.name,\n            \"type\": f\"{self.__class__.__name__}\",\n            \"width\": self.width,\n            \"x0\": self._x0,\n            \"x1\": self._x1,\n            \"xform\": self.xform.__jsondump__(),\n            \"y0\": self._y0,",
          "file": "line.py"
        }
      }
    },
    {
      "name": "Line.json_dump",
      "implementations": {
        "python": {
          "sig": "json_dump(filepath)",
          "code": "def json_dump(self, filepath):\n\n        \"\"\"Write JSON to file.\n\n        Parameters\n        ----------\n        filepath : str or Path\n            Path to the output file.\n\n        \"\"\"\n        import json\n        with open(filepath, 'w') as f:\n            json.dump(self.__jsondump__(), f, indent=2)\n\n    @classmethod\n    def json_load(cls, filepath):\n        \"\"\"Read JSON from file.\n\n        Parameters\n        ----------\n        filepath : str or Path",
          "file": "line.py"
        },
        "cpp": {
          "sig": "void json_dump(const std::string& filename)",
          "code": "void Line::json_dump(const std::string& filename) const {\n    std::ofstream ofs(filename);\n    ofs << jsondump().dump(4);\n    ofs.close();\n}",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "json_dump(filepath: &str) -> Result<(), Box<dyn std::error::Error>>",
          "code": "pub fn json_dump(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {\n        self.to_json(filepath)\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.json_load",
      "implementations": {
        "python": {
          "sig": "json_load(cls, filepath)",
          "code": "def json_load(cls, filepath):\n\n        \"\"\"Read JSON from file.\n\n        Parameters\n        ----------\n        filepath : str or Path\n            Path to the JSON file.\n\n        Returns\n        -------\n        :class:`Line`\n            The deserialized Line.\n\n        \"\"\"\n        import json\n        with open(filepath, 'r') as f:\n            data = json.load(f)\n        return cls.__jsonload__(data)\n\n    @classmethod",
          "file": "line.py"
        },
        "cpp": {
          "sig": "Line json_load(const std::string& filename)",
          "code": "Line Line::json_load(const std::string& filename) {\n    std::ifstream ifs(filename);\n    nlohmann::json data = nlohmann::json::parse(ifs);\n    ifs.close();\n    return jsonload(data);\n}",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "json_load(filepath: &str) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn json_load(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {\n        Self::from_json(filepath)\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.__jsonload__",
      "implementations": {
        "python": {
          "sig": "__jsonload__(cls, data, guid=None, name=None)",
          "code": "def __jsonload__(cls, data, guid=None, name=None):\n\n        \"\"\"Deserialize from polymorphic JSON format.\n\n        Parameters\n        ----------\n        data : dict\n            Dictionary containing line data.\n        guid : str, optional\n            GUID for the line.\n        name : str, optional\n            Name for the line.\n\n        Returns\n        -------\n        :class:`Line`\n            Reconstructed line instance.\n\n        \"\"\"\n        from .encoders import decode_node",
          "file": "line.py"
        }
      }
    },
    {
      "name": "Line.to_protobuf",
      "implementations": {
        "python": {
          "sig": "to_protobuf()",
          "code": "def to_protobuf(self):\n\n        \"\"\"Convert to protobuf binary format.\n\n        Returns\n        -------\n        bytes\n            Serialized protobuf data.\n\n        \"\"\"\n        from .proto import line_pb2\n        from .proto import point_pb2\n\n        proto = line_pb2.Line()\n        proto.guid = self.guid\n        proto.name = self.name\n\n        # Set start point\n        proto.start.x = self._x0\n        proto.start.y = self._y0\n        proto.start.z = self._z0",
          "file": "line.py"
        },
        "cpp": {
          "sig": "std::string to_protobuf()",
          "code": "std::string Line::to_protobuf() const {\n    throw std::runtime_error(\"Protobuf support not enabled\");\n}",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "to_protobuf() -> Vec<u8>",
          "code": "pub fn to_protobuf(&self) -> Vec<u8> {\n        use prost::Message;\n        let proto = crate::proto::Line {\n            start: Some(crate::proto::Point {\n                x: self._x0,\n                y: self._y0,\n                z: self._z0,\n                guid: String::new(),\n                name: String::new(),\n                width: 1.0,\n                pointcolor: None,\n                xform: None,\n            }),\n            end: Some(crate::proto::Point {\n                x: self._x1,\n                y: self._y1,\n                z: self._z1,\n                guid: String::new(),\n                name: String::new(),\n                width: 1.0,\n                pointcolor: None,\n                xform: None,\n            }),\n            guid: self.guid.clone(),\n            name: self.na",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.from_protobuf",
      "implementations": {
        "python": {
          "sig": "from_protobuf(cls, data)",
          "code": "def from_protobuf(cls, data):\n\n        \"\"\"Create Line from protobuf binary data.\n\n        Parameters\n        ----------\n        data : bytes\n            Protobuf-encoded line data.\n\n        Returns\n        -------\n        :class:`Line`\n            The deserialized Line.\n\n        \"\"\"\n        from .proto import line_pb2\n\n        proto = line_pb2.Line()\n        proto.ParseFromString(data)\n\n        line = cls(",
          "file": "line.py"
        },
        "cpp": {
          "sig": "Line from_protobuf(const std::string& data)",
          "code": "Line Line::from_protobuf(const std::string& data) {\n    throw std::runtime_error(\"Protobuf support not enabled\");\n}",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "from_protobuf(data: &[u8]) -> Result<Self, prost::DecodeError>",
          "code": "pub fn from_protobuf(data: &[u8]) -> Result<Self, prost::DecodeError> {\n        use prost::Message;\n        let proto = crate::proto::Line::decode(data)?;\n        let start = proto.start.unwrap_or_default();\n        let end = proto.end.unwrap_or_default();\n        let mut line = Self::new(start.x, start.y, start.z, end.x, end.y, end.z);\n        line.guid = proto.guid;\n        line.name = proto.name;\n        Ok(line)\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.protobuf_dump",
      "implementations": {
        "python": {
          "sig": "protobuf_dump(filepath)",
          "code": "def protobuf_dump(self, filepath):\n\n        \"\"\"Write protobuf to file.\n\n        Parameters\n        ----------\n        filepath : str or Path\n            Path to the output file.\n\n        \"\"\"\n        data = self.to_protobuf()\n        with open(filepath, 'wb') as f:\n            f.write(data)\n\n    @classmethod\n    def protobuf_load(cls, filepath):\n        \"\"\"Read protobuf from file.\n\n        Parameters\n        ----------\n        filepath : str or Path",
          "file": "line.py"
        },
        "cpp": {
          "sig": "void protobuf_dump(const std::string& filename)",
          "code": "void Line::protobuf_dump(const std::string& filename) const {\n    throw std::runtime_error(\"Protobuf support not enabled\");\n}",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "protobuf_dump(filepath: &str)",
          "code": "pub fn protobuf_dump(&self, filepath: &str) {\n        let data = self.to_protobuf();\n        std::fs::write(filepath, data).expect(\"Failed to write protobuf file\");\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.protobuf_load",
      "implementations": {
        "python": {
          "sig": "protobuf_load(cls, filepath)",
          "code": "def protobuf_load(cls, filepath):\n\n        \"\"\"Read protobuf from file.\n\n        Parameters\n        ----------\n        filepath : str or Path\n            Path to the protobuf file.\n\n        Returns\n        -------\n        :class:`Line`\n            The deserialized Line.\n\n        \"\"\"\n        with open(filepath, 'rb') as f:\n            data = f.read()\n        return cls.from_protobuf(data)\n\n    def __str__(self):\n        \"\"\"String representation.\"\"\"",
          "file": "line.py"
        },
        "cpp": {
          "sig": "Line protobuf_load(const std::string& filename)",
          "code": "Line Line::protobuf_load(const std::string& filename) {\n    throw std::runtime_error(\"Protobuf support not enabled\");\n}",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "protobuf_load(filepath: &str) -> Self",
          "code": "pub fn protobuf_load(filepath: &str) -> Self {\n        let data = std::fs::read(filepath).expect(\"Failed to read protobuf file\");\n        Self::from_protobuf(&data).expect(\"Failed to parse protobuf\")\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.__str__",
      "implementations": {
        "python": {
          "sig": "__str__()",
          "code": "def __str__(self):\n\n        \"\"\"String representation.\"\"\"\n        return f\"Line({self._x0}, {self._y0}, {self._z0}, {self._x1}, {self._y1}, {self._z1})\"\n\n    def __repr__(self):\n        \"\"\"Detailed representation.\"\"\"\n        return f\"Line({self.name}, {self._x0}, {self._y0}, {self._z0}, {self._x1}, {self._y1}, {self._z1}, {repr(self.linecolor)}, {self.width})\"",
          "file": "line.py"
        }
      }
    },
    {
      "name": "Line.__repr__",
      "implementations": {
        "python": {
          "sig": "__repr__()",
          "code": "def __repr__(self):\n\n        \"\"\"Detailed representation.\"\"\"\n        return f\"Line({self.name}, {self._x0}, {self._y0}, {self._z0}, {self._x1}, {self._y1}, {self._z1}, {repr(self.linecolor)}, {self.width})\"",
          "file": "line.py"
        }
      }
    },
    {
      "name": "VertexData.__init__",
      "implementations": {
        "python": {
          "sig": "__init__(point: Point = None)",
          "code": "def __init__(self, point: Point = None):\n\n        if point is None:\n            point = Point(0.0, 0.0, 0.0)\n        self.x = point[0]\n        self.y = point[1]\n        self.z = point[2]\n        self.attributes = {}\n\n    def __getitem__(self, index):\n        \"\"\"Access coordinate by index (0=x, 1=y, 2=z).\"\"\"\n        if index == 0:\n            return self.x\n        elif index == 1:\n            return self.y\n        elif index == 2:\n            return self.z\n        else:\n            raise IndexError(\"Index out of range\")\n\n    def __setitem__(self, index, value):",
          "file": "mesh.py"
        }
      }
    },
    {
      "name": "VertexData.__getitem__",
      "implementations": {
        "python": {
          "sig": "__getitem__(index)",
          "code": "def __getitem__(self, index):\n\n        \"\"\"Access coordinate by index (0=x, 1=y, 2=z).\"\"\"\n        if index == 0:\n            return self.x\n        elif index == 1:\n            return self.y\n        elif index == 2:\n            return self.z\n        else:\n            raise IndexError(\"Index out of range\")\n\n    def __setitem__(self, index, value):\n        \"\"\"Set coordinate by index (0=x, 1=y, 2=z).\"\"\"\n        if index == 0:\n            self.x = value\n        elif index == 1:\n            self.y = value\n        elif index == 2:\n            self.z = value\n        else:",
          "file": "mesh.py"
        }
      }
    },
    {
      "name": "VertexData.__setitem__",
      "implementations": {
        "python": {
          "sig": "__setitem__(index, value)",
          "code": "def __setitem__(self, index, value):\n\n        \"\"\"Set coordinate by index (0=x, 1=y, 2=z).\"\"\"\n        if index == 0:\n            self.x = value\n        elif index == 1:\n            self.y = value\n        elif index == 2:\n            self.z = value\n        else:\n            raise IndexError(\"Index out of range\")\n\n    def position(self) -> Point:\n        \"\"\"Get the vertex position as a Point.\"\"\"\n        return Point(self.x, self.y, self.z)\n\n    def set_position(self, point: Point):\n        \"\"\"Set the vertex position from a Point.\"\"\"\n        self.x = point[0]\n        self.y = point[1]\n        self.z = point[2]",
          "file": "mesh.py"
        }
      }
    },
    {
      "name": "VertexData.position",
      "implementations": {
        "python": {
          "sig": "position() -> Point",
          "code": "def position(self) -> Point:\n\n        \"\"\"Get the vertex position as a Point.\"\"\"\n        return Point(self.x, self.y, self.z)\n\n    def set_position(self, point: Point):\n        \"\"\"Set the vertex position from a Point.\"\"\"\n        self.x = point[0]\n        self.y = point[1]\n        self.z = point[2]\n\n    def color(self) -> List[float]:\n        \"\"\"Get the vertex color as [r, g, b].\"\"\"\n        return [\n            self.attributes.get(\"r\", 0.5),\n            self.attributes.get(\"g\", 0.5),\n            self.attributes.get(\"b\", 0.5),\n        ]\n\n    def set_color(self, r: float, g: float, b: float):\n        \"\"\"Set the vertex color.\"\"\"",
          "file": "mesh.py"
        },
        "rust": {
          "sig": "position() -> Point",
          "code": "pub fn position(&self) -> Point {\n        Point::new(self.x, self.y, self.z)\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "VertexData.set_position",
      "implementations": {
        "python": {
          "sig": "set_position(point: Point)",
          "code": "def set_position(self, point: Point):\n\n        \"\"\"Set the vertex position from a Point.\"\"\"\n        self.x = point[0]\n        self.y = point[1]\n        self.z = point[2]\n\n    def color(self) -> List[float]:\n        \"\"\"Get the vertex color as [r, g, b].\"\"\"\n        return [\n            self.attributes.get(\"r\", 0.5),\n            self.attributes.get(\"g\", 0.5),\n            self.attributes.get(\"b\", 0.5),\n        ]\n\n    def set_color(self, r: float, g: float, b: float):\n        \"\"\"Set the vertex color.\"\"\"\n        self.attributes[\"r\"] = r\n        self.attributes[\"g\"] = g\n        self.attributes[\"b\"] = b",
          "file": "mesh.py"
        },
        "rust": {
          "sig": "set_position(point: Point)",
          "code": "pub fn set_position(&mut self, point: Point) {\n        self.x = point[0];\n        self.y = point[1];\n        self.z = point[2];\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "VertexData.color",
      "implementations": {
        "python": {
          "sig": "color() -> List[float]",
          "code": "def color(self) -> List[float]:\n\n        \"\"\"Get the vertex color as [r, g, b].\"\"\"\n        return [\n            self.attributes.get(\"r\", 0.5),\n            self.attributes.get(\"g\", 0.5),\n            self.attributes.get(\"b\", 0.5),\n        ]\n\n    def set_color(self, r: float, g: float, b: float):\n        \"\"\"Set the vertex color.\"\"\"\n        self.attributes[\"r\"] = r\n        self.attributes[\"g\"] = g\n        self.attributes[\"b\"] = b\n\n    def normal(self) -> Optional[List[float]]:\n        \"\"\"Get the vertex normal as [nx, ny, nz].\"\"\"\n        if (\n            \"nx\" in self.attributes\n            and \"ny\" in self.attributes\n            and \"nz\" in self.attributes",
          "file": "mesh.py"
        },
        "rust": {
          "sig": "color() -> [f64; 3]",
          "code": "pub fn color(&self) -> [f64; 3] {\n        [\n            self.attributes.get(\"r\").copied().unwrap_or(0.5),\n            self.attributes.get(\"g\").copied().unwrap_or(0.5),\n            self.attributes.get(\"b\").copied().unwrap_or(0.5),\n        ]\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "VertexData.set_color",
      "implementations": {
        "python": {
          "sig": "set_color(r: float, g: float, b: float)",
          "code": "def set_color(self, r: float, g: float, b: float):\n\n        \"\"\"Set the vertex color.\"\"\"\n        self.attributes[\"r\"] = r\n        self.attributes[\"g\"] = g\n        self.attributes[\"b\"] = b\n\n    def normal(self) -> Optional[List[float]]:\n        \"\"\"Get the vertex normal as [nx, ny, nz].\"\"\"\n        if (\n            \"nx\" in self.attributes\n            and \"ny\" in self.attributes\n            and \"nz\" in self.attributes\n        ):\n            return [self.attributes[\"nx\"], self.attributes[\"ny\"], self.attributes[\"nz\"]]\n        return None\n\n    def set_normal(self, nx: float, ny: float, nz: float):\n        \"\"\"Set the vertex normal.\"\"\"\n        self.attributes[\"nx\"] = nx\n        self.attributes[\"ny\"] = ny",
          "file": "mesh.py"
        },
        "rust": {
          "sig": "set_color(r: f64, g: f64, b: f64)",
          "code": "pub fn set_color(&mut self, r: f64, g: f64, b: f64) {\n        self.attributes.insert(\"r\".to_string(), r);\n        self.attributes.insert(\"g\".to_string(), g);\n        self.attributes.insert(\"b\".to_string(), b);\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "VertexData.normal",
      "implementations": {
        "python": {
          "sig": "normal() -> Optional[List[float]]",
          "code": "def normal(self) -> Optional[List[float]]:\n\n        \"\"\"Get the vertex normal as [nx, ny, nz].\"\"\"\n        if (\n            \"nx\" in self.attributes\n            and \"ny\" in self.attributes\n            and \"nz\" in self.attributes\n        ):\n            return [self.attributes[\"nx\"], self.attributes[\"ny\"], self.attributes[\"nz\"]]\n        return None\n\n    def set_normal(self, nx: float, ny: float, nz: float):\n        \"\"\"Set the vertex normal.\"\"\"\n        self.attributes[\"nx\"] = nx\n        self.attributes[\"ny\"] = ny\n        self.attributes[\"nz\"] = nz\n\n\nclass Mesh:\n    \"\"\"A halfedge mesh data structure for representing polygonal surfaces.",
          "file": "mesh.py"
        },
        "rust": {
          "sig": "normal() -> Option<[f64; 3]>",
          "code": "pub fn normal(&self) -> Option<[f64; 3]> {\n        let nx = self.attributes.get(\"nx\")?;\n        let ny = self.attributes.get(\"ny\")?;\n        let nz = self.attributes.get(\"nz\")?;\n        Some([*nx, *ny, *nz])\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "VertexData.set_normal",
      "implementations": {
        "python": {
          "sig": "set_normal(nx: float, ny: float, nz: float)",
          "code": "def set_normal(self, nx: float, ny: float, nz: float):\n\n        \"\"\"Set the vertex normal.\"\"\"\n        self.attributes[\"nx\"] = nx\n        self.attributes[\"ny\"] = ny\n        self.attributes[\"nz\"] = nz\n\n\nclass Mesh:\n    \"\"\"A halfedge mesh data structure for representing polygonal surfaces.\n\n    Attributes\n    ----------\n    halfedge : dict\n        Halfedge connectivity structure mapping vertex pairs to faces.\n    vertex : dict\n        Vertex data dictionary mapping vertex keys to VertexData.\n    face : dict\n        Face vertex lists mapping face keys to vertex key lists.\n    facedata : dict\n        Face attributes dictionary.",
          "file": "mesh.py"
        },
        "rust": {
          "sig": "set_normal(nx: f64, ny: f64, nz: f64)",
          "code": "pub fn set_normal(&mut self, nx: f64, ny: f64, nz: f64) {\n        self.attributes.insert(\"nx\".to_string(), nx);\n        self.attributes.insert(\"ny\".to_string(), ny);\n        self.attributes.insert(\"nz\".to_string(), nz);\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.__init__",
      "implementations": {
        "python": {
          "sig": "__init__()",
          "code": "def __init__(self):\n\n        self.halfedge = {}\n        self.vertex = {}\n        self.face = {}\n        self.facedata = {}\n        self.edgedata = {}\n        self.default_vertex_attributes = {\"x\": 0.0, \"y\": 0.0, \"z\": 0.0}\n        self.default_face_attributes = {}\n        self.default_edge_attributes = {}\n        self.triangulation = {}\n        self._max_vertex = 0\n        self._max_face = 0\n        self.guid = str(uuid.uuid4())\n        self.name = \"my_mesh\"\n        self.pointcolors = []\n        self.facecolors = []\n        self.linecolors = []\n        self.widths = []\n        self.xform = Xform.identity()",
          "file": "mesh.py"
        }
      }
    },
    {
      "name": "Mesh.number_of_vertices",
      "implementations": {
        "python": {
          "sig": "number_of_vertices() -> int",
          "code": "def number_of_vertices(self) -> int:\n\n        \"\"\"Get the number of vertices.\"\"\"\n        return len(self.vertex)\n\n    def number_of_faces(self) -> int:\n        \"\"\"Get the number of faces.\"\"\"\n        return len(self.face)\n\n    def number_of_edges(self) -> int:\n        \"\"\"Get the number of edges.\"\"\"\n        seen = set()\n        count = 0\n        for u in self.halfedge:\n            for v in self.halfedge[u]:\n                edge = tuple(sorted([u, v]))\n                if edge not in seen:\n                    seen.add(edge)\n                    count += 1\n        return count",
          "file": "mesh.py"
        },
        "rust": {
          "sig": "number_of_vertices() -> usize",
          "code": "pub fn number_of_vertices(&self) -> usize {\n        self.vertex.len()\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.number_of_faces",
      "implementations": {
        "python": {
          "sig": "number_of_faces() -> int",
          "code": "def number_of_faces(self) -> int:\n\n        \"\"\"Get the number of faces.\"\"\"\n        return len(self.face)\n\n    def number_of_edges(self) -> int:\n        \"\"\"Get the number of edges.\"\"\"\n        seen = set()\n        count = 0\n        for u in self.halfedge:\n            for v in self.halfedge[u]:\n                edge = tuple(sorted([u, v]))\n                if edge not in seen:\n                    seen.add(edge)\n                    count += 1\n        return count\n\n    def is_empty(self) -> bool:\n        \"\"\"Check if the mesh is empty.\"\"\"\n        return len(self.vertex) == 0",
          "file": "mesh.py"
        },
        "rust": {
          "sig": "number_of_faces() -> usize",
          "code": "pub fn number_of_faces(&self) -> usize {\n        self.face.len()\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.number_of_edges",
      "implementations": {
        "python": {
          "sig": "number_of_edges() -> int",
          "code": "def number_of_edges(self) -> int:\n\n        \"\"\"Get the number of edges.\"\"\"\n        seen = set()\n        count = 0\n        for u in self.halfedge:\n            for v in self.halfedge[u]:\n                edge = tuple(sorted([u, v]))\n                if edge not in seen:\n                    seen.add(edge)\n                    count += 1\n        return count\n\n    def is_empty(self) -> bool:\n        \"\"\"Check if the mesh is empty.\"\"\"\n        return len(self.vertex) == 0\n\n    def euler(self) -> int:\n        \"\"\"Calculate Euler characteristic (V - E + F).\"\"\"\n        return (\n            self.number_of_vertices() - self.number_of_edges() + self.number_of_faces()",
          "file": "mesh.py"
        },
        "cpp": {
          "sig": "size_t number_of_edges()",
          "code": "size_t Mesh::number_of_edges() const {\n    std::set<std::pair<size_t, size_t>> seen;\n    size_t count = 0;\n    \n    for (const auto& [u, neighbors] : halfedge) {\n        for (const auto& [v, _] : neighbors) {\n            auto edge = std::minmax(u, v);\n            if (seen.insert(edge).second) {\n                count++;\n            }",
          "file": "mesh.cpp"
        },
        "rust": {
          "sig": "number_of_edges() -> usize",
          "code": "pub fn number_of_edges(&self) -> usize {\n        let mut seen = HashSet::new();\n        let mut count = 0;\n\n        for u in self.halfedge.keys() {\n            if let Some(neighbors) = self.halfedge.get(u) {\n                for v in neighbors.keys() {\n                    let edge = if u < v { (*u, *v) } else { (*v, *u) };\n                    if seen.insert(edge) {\n                        count += 1;\n                    }\n                }\n            }\n        }\n\n        count\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.is_empty",
      "implementations": {
        "python": {
          "sig": "is_empty() -> bool",
          "code": "def is_empty(self) -> bool:\n\n        \"\"\"Check if the mesh is empty.\"\"\"\n        return len(self.vertex) == 0\n\n    def euler(self) -> int:\n        \"\"\"Calculate Euler characteristic (V - E + F).\"\"\"\n        return (\n            self.number_of_vertices() - self.number_of_edges() + self.number_of_faces()\n        )\n\n    def clear(self):\n        \"\"\"Clear all mesh data.\"\"\"\n        self.halfedge.clear()\n        self.vertex.clear()\n        self.face.clear()\n        self.facedata.clear()\n        self.edgedata.clear()\n        self.triangulation.clear()\n        self._max_vertex = 0\n        self._max_face = 0",
          "file": "mesh.py"
        },
        "rust": {
          "sig": "is_empty() -> bool",
          "code": "pub fn is_empty(&self) -> bool {\n        self.vertex.is_empty() && self.face.is_empty()\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.euler",
      "implementations": {
        "python": {
          "sig": "euler() -> int",
          "code": "def euler(self) -> int:\n\n        \"\"\"Calculate Euler characteristic (V - E + F).\"\"\"\n        return (\n            self.number_of_vertices() - self.number_of_edges() + self.number_of_faces()\n        )\n\n    def clear(self):\n        \"\"\"Clear all mesh data.\"\"\"\n        self.halfedge.clear()\n        self.vertex.clear()\n        self.face.clear()\n        self.facedata.clear()\n        self.edgedata.clear()\n        self.triangulation.clear()\n        self._max_vertex = 0\n        self._max_face = 0\n        self.pointcolors.clear()\n        self.facecolors.clear()\n        self.linecolors.clear()\n        self.widths.clear()",
          "file": "mesh.py"
        },
        "cpp": {
          "sig": "int euler()",
          "code": "int Mesh::euler() const {\n    return static_cast<int>(number_of_vertices()) - \n           static_cast<int>(number_of_edges()) + \n           static_cast<int>(number_of_faces());\n}",
          "file": "mesh.cpp"
        },
        "rust": {
          "sig": "euler() -> i32",
          "code": "pub fn euler(&self) -> i32 {\n        let v = self.number_of_vertices() as i32;\n        let e = self.number_of_edges() as i32;\n        let f = self.number_of_faces() as i32;\n        v - e + f\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.clear",
      "implementations": {
        "python": {
          "sig": "clear()",
          "code": "def clear(self):\n\n        \"\"\"Clear all mesh data.\"\"\"\n        self.halfedge.clear()\n        self.vertex.clear()\n        self.face.clear()\n        self.facedata.clear()\n        self.edgedata.clear()\n        self.triangulation.clear()\n        self._max_vertex = 0\n        self._max_face = 0\n        self.pointcolors.clear()\n        self.facecolors.clear()\n        self.linecolors.clear()\n        self.widths.clear()\n\n    ###########################################################################################\n    # Vertex and Face Operations\n    ###########################################################################################\n\n    def add_vertex(self, position: Point, vkey: Optional[int] = None) -> int:",
          "file": "mesh.py"
        },
        "cpp": {
          "sig": "void clear()",
          "code": "void Mesh::clear() {\n    halfedge.clear();\n    vertex.clear();\n    face.clear();\n    facedata.clear();\n    edgedata.clear();\n    triangulation.clear();\n    max_vertex = 0;\n    max_face = 0;\n    pointcolors.clear();\n    facecolors.clear();\n    linecolors.clear();\n    widths.clear();\n    triangle_bvh_built = false;\n    triangle_bvh.reset();\n    triangle_boxes_cache.clear();\n    triangle_aabbs_cache.clear();\n    triangle_indices_cache.clear();\n    triangle_face_subidx_cache.clear();\n    vertices_cache.clear();\n}",
          "file": "mesh.cpp"
        },
        "rust": {
          "sig": "clear()",
          "code": "pub fn clear(&mut self) {\n        self.halfedge.clear();\n        self.vertex.clear();\n        self.face.clear();\n        self.facedata.clear();\n        self.edgedata.clear();\n        self.triangulation.clear();\n        self.max_vertex = 0;\n        self.max_face = 0;\n        self.pointcolors.clear();\n        self.facecolors.clear();\n        self.linecolors.clear();\n        self.widths.clear();\n        self.invalidate_triangle_bvh();\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.add_vertex",
      "implementations": {
        "python": {
          "sig": "add_vertex(position: Point, vkey: Optional[int] = None) -> int",
          "code": "def add_vertex(self, position: Point, vkey: Optional[int] = None) -> int:\n\n        \"\"\"Add a vertex to the mesh.\n\n        Parameters\n        ----------\n        position : Point\n            The position of the vertex.\n        vkey : int, optional\n            Optional vertex key. If None, auto-generated.\n\n        Returns\n        -------\n        int\n            The vertex key.\n        \"\"\"\n        if vkey is None:\n            self._max_vertex += 1\n            vertex_key = self._max_vertex\n        else:\n            vertex_key = vkey",
          "file": "mesh.py"
        },
        "cpp": {
          "sig": "size_t add_vertex(const Point& position, std::optional<size_t> vkey)",
          "code": "size_t Mesh::add_vertex(const Point& position, std::optional<size_t> vkey) {\n    size_t vertex_key = vkey.value_or(max_vertex + 1);\n    \n    if (vertex_key >= max_vertex) {\n        max_vertex = vertex_key + 1;\n    }",
          "file": "mesh.cpp"
        },
        "rust": {
          "sig": "add_vertex(position: Point, key: Option<usize>) -> usize",
          "code": "pub fn add_vertex(&mut self, position: Point, key: Option<usize>) -> usize {\n        let vertex_key = key.unwrap_or_else(|| {\n            self.max_vertex += 1;\n            self.max_vertex\n        });\n\n        if vertex_key >= self.max_vertex {\n            self.max_vertex = vertex_key + 1;\n        }\n\n        let vertex_data = VertexData::new(position);\n        self.vertex.insert(vertex_key, vertex_data);\n        self.halfedge.entry(vertex_key).or_default();\n        self.pointcolors.push(Color::white());\n        self.invalidate_triangle_bvh();\n\n        vertex_key\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.add_face",
      "implementations": {
        "python": {
          "sig": "add_face(\n        vertices: List[int], fkey: Optional[int] = None\n    ) -> Optional[int]",
          "code": "def add_face(\n        self, vertices: List[int], fkey: Optional[int] = None\n    ) -> Optional[int]:\n\n        \"\"\"Add a face to the mesh.\n\n        Parameters\n        ----------\n        vertices : list of int\n            The vertex keys forming the face.\n        fkey : int, optional\n            Optional face key. If None, auto-generated.\n\n        Returns\n        -------\n        int or None\n            The face key, or None if the face is invalid.\n        \"\"\"\n        if len(vertices) < 3:\n            return None\n\n        if not all(v in self.vertex for v in vertices):\n            return None",
          "file": "mesh.py"
        },
        "cpp": {
          "sig": "std::optional<size_t> add_face(const std::vector<size_t>& vertices, std::optional<size_t> fkey)",
          "code": "std::optional<size_t> Mesh::add_face(const std::vector<size_t>& vertices, std::optional<size_t> fkey) {\n    if (vertices.size() < 3) {\n        return std::nullopt;\n    }",
          "file": "mesh.cpp"
        },
        "rust": {
          "sig": "add_face(vertices: Vec<usize>, fkey: Option<usize>) -> Option<usize>",
          "code": "pub fn add_face(&mut self, vertices: Vec<usize>, fkey: Option<usize>) -> Option<usize> {\n        if vertices.len() < 3 {\n            return None;\n        }\n\n        if !vertices.iter().all(|v| self.vertex.contains_key(v)) {\n            return None;\n        }\n\n        let mut unique_vertices = HashSet::new();\n        for vertex in &vertices {\n            if !unique_vertices.insert(*vertex) {\n                return None;\n            }\n        }\n\n        let face_key = fkey.unwrap_or_else(|| {\n            self.max_face += 1;\n            self.max_face\n        });\n\n        if face_key >= self.max_face {\n            self.max_face = face_key + 1;\n        }\n\n        self.face.insert(face_key, vertices.clone());\n        self.triangulation.remove(&face_key);\n        self.facecolors.push(Color::",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.vertex_position",
      "implementations": {
        "python": {
          "sig": "vertex_position(vertex_key: int) -> Optional[Point]",
          "code": "def vertex_position(self, vertex_key: int) -> Optional[Point]:\n\n        \"\"\"Get the position of a vertex.\"\"\"\n        if vertex_key not in self.vertex:\n            return None\n        return self.vertex[vertex_key].position()\n\n    def face_vertices(self, face_key: int) -> Optional[List[int]]:\n        \"\"\"Get the vertices of a face.\"\"\"\n        return self.face.get(face_key)\n\n    def vertex_neighbors(self, vertex_key: int) -> List[int]:\n        \"\"\"Get the neighboring vertices of a vertex.\"\"\"\n        if vertex_key not in self.halfedge:\n            return []\n        return list(self.halfedge[vertex_key].keys())\n\n    def vertex_faces(self, vertex_key: int) -> List[int]:\n        \"\"\"Get the faces incident to a vertex.\"\"\"\n        faces = []\n        for face_key, face_vertices in self.face.items():",
          "file": "mesh.py"
        },
        "cpp": {
          "sig": "std::optional<Point> vertex_position(size_t vertex_key)",
          "code": "std::optional<Point> Mesh::vertex_position(size_t vertex_key) const {\n    auto it = vertex.find(vertex_key);\n    if (it == vertex.end()) {\n        return std::nullopt;\n    }",
          "file": "mesh.cpp"
        },
        "rust": {
          "sig": "vertex_position(vertex_key: usize) -> Option<Point>",
          "code": "pub fn vertex_position(&self, vertex_key: usize) -> Option<Point> {\n        self.vertex.get(&vertex_key).map(|v| v.position())\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.face_vertices",
      "implementations": {
        "python": {
          "sig": "face_vertices(face_key: int) -> Optional[List[int]]",
          "code": "def face_vertices(self, face_key: int) -> Optional[List[int]]:\n\n        \"\"\"Get the vertices of a face.\"\"\"\n        return self.face.get(face_key)\n\n    def vertex_neighbors(self, vertex_key: int) -> List[int]:\n        \"\"\"Get the neighboring vertices of a vertex.\"\"\"\n        if vertex_key not in self.halfedge:\n            return []\n        return list(self.halfedge[vertex_key].keys())\n\n    def vertex_faces(self, vertex_key: int) -> List[int]:\n        \"\"\"Get the faces incident to a vertex.\"\"\"\n        faces = []\n        for face_key, face_vertices in self.face.items():\n            if vertex_key in face_vertices:\n                faces.append(face_key)\n        return faces\n\n    def is_vertex_on_boundary(self, vertex_key: int) -> bool:\n        \"\"\"Check if a vertex is on the boundary.\"\"\"",
          "file": "mesh.py"
        },
        "cpp": {
          "sig": "std::optional<std::vector<size_t>> face_vertices(size_t face_key)",
          "code": "std::optional<std::vector<size_t>> Mesh::face_vertices(size_t face_key) const {\n    auto it = face.find(face_key);\n    if (it == face.end()) {\n        return std::nullopt;\n    }",
          "file": "mesh.cpp"
        },
        "rust": {
          "sig": "face_vertices(face_key: usize) -> Option<&Vec<usize>>",
          "code": "pub fn face_vertices(&self, face_key: usize) -> Option<&Vec<usize>> {\n        self.face.get(&face_key)\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.vertex_neighbors",
      "implementations": {
        "python": {
          "sig": "vertex_neighbors(vertex_key: int) -> List[int]",
          "code": "def vertex_neighbors(self, vertex_key: int) -> List[int]:\n\n        \"\"\"Get the neighboring vertices of a vertex.\"\"\"\n        if vertex_key not in self.halfedge:\n            return []\n        return list(self.halfedge[vertex_key].keys())\n\n    def vertex_faces(self, vertex_key: int) -> List[int]:\n        \"\"\"Get the faces incident to a vertex.\"\"\"\n        faces = []\n        for face_key, face_vertices in self.face.items():\n            if vertex_key in face_vertices:\n                faces.append(face_key)\n        return faces\n\n    def is_vertex_on_boundary(self, vertex_key: int) -> bool:\n        \"\"\"Check if a vertex is on the boundary.\"\"\"\n        if vertex_key not in self.halfedge:\n            return False\n\n        for v, face_opt in self.halfedge[vertex_key].items():",
          "file": "mesh.py"
        },
        "cpp": {
          "sig": "std::vector<size_t> vertex_neighbors(size_t vertex_key)",
          "code": "std::vector<size_t> Mesh::vertex_neighbors(size_t vertex_key) const {\n    std::vector<size_t> neighbors;\n    auto it = halfedge.find(vertex_key);\n    if (it != halfedge.end()) {\n        for (const auto& [v, _] : it->second) {\n            neighbors.push_back(v);\n        }",
          "file": "mesh.cpp"
        },
        "rust": {
          "sig": "vertex_neighbors(vertex_key: usize) -> Vec<usize>",
          "code": "pub fn vertex_neighbors(&self, vertex_key: usize) -> Vec<usize> {\n        self.halfedge\n            .get(&vertex_key)\n            .map(|neighbors| neighbors.keys().copied().collect())\n            .unwrap_or_default()\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.vertex_faces",
      "implementations": {
        "python": {
          "sig": "vertex_faces(vertex_key: int) -> List[int]",
          "code": "def vertex_faces(self, vertex_key: int) -> List[int]:\n\n        \"\"\"Get the faces incident to a vertex.\"\"\"\n        faces = []\n        for face_key, face_vertices in self.face.items():\n            if vertex_key in face_vertices:\n                faces.append(face_key)\n        return faces\n\n    def is_vertex_on_boundary(self, vertex_key: int) -> bool:\n        \"\"\"Check if a vertex is on the boundary.\"\"\"\n        if vertex_key not in self.halfedge:\n            return False\n\n        for v, face_opt in self.halfedge[vertex_key].items():\n            if face_opt is None:\n                return True\n\n        for u, neighbors in self.halfedge.items():\n            if vertex_key in neighbors and neighbors[vertex_key] is None:\n                return True",
          "file": "mesh.py"
        },
        "cpp": {
          "sig": "std::vector<size_t> vertex_faces(size_t vertex_key)",
          "code": "std::vector<size_t> Mesh::vertex_faces(size_t vertex_key) const {\n    std::vector<size_t> faces;\n    for (const auto& [face_key, face_vertices] : face) {\n        if (std::find(face_vertices.begin(), face_vertices.end(), vertex_key) != face_vertices.end()) {\n            faces.push_back(face_key);\n        }",
          "file": "mesh.cpp"
        },
        "rust": {
          "sig": "vertex_faces(vertex_key: usize) -> Vec<usize>",
          "code": "pub fn vertex_faces(&self, vertex_key: usize) -> Vec<usize> {\n        let mut faces = Vec::new();\n        for (face_key, face_vertices) in &self.face {\n            if face_vertices.contains(&vertex_key) {\n                faces.push(*face_key);\n            }\n        }\n        faces\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.is_vertex_on_boundary",
      "implementations": {
        "python": {
          "sig": "is_vertex_on_boundary(vertex_key: int) -> bool",
          "code": "def is_vertex_on_boundary(self, vertex_key: int) -> bool:\n\n        \"\"\"Check if a vertex is on the boundary.\"\"\"\n        if vertex_key not in self.halfedge:\n            return False\n\n        for v, face_opt in self.halfedge[vertex_key].items():\n            if face_opt is None:\n                return True\n\n        for u, neighbors in self.halfedge.items():\n            if vertex_key in neighbors and neighbors[vertex_key] is None:\n                return True\n\n        return False\n\n    ###########################################################################################\n    # Geometric Properties\n    ###########################################################################################\n\n    def face_normal(self, face_key: int) -> Optional[Vector]:",
          "file": "mesh.py"
        },
        "cpp": {
          "sig": "bool is_vertex_on_boundary(size_t vertex_key)",
          "code": "bool Mesh::is_vertex_on_boundary(size_t vertex_key) const {\n    auto it = halfedge.find(vertex_key);\n    if (it == halfedge.end()) {\n        return false;\n    }",
          "file": "mesh.cpp"
        },
        "rust": {
          "sig": "is_vertex_on_boundary(vertex_key: usize) -> bool",
          "code": "pub fn is_vertex_on_boundary(&self, vertex_key: usize) -> bool {\n        if let Some(neigh) = self.halfedge.get(&vertex_key) {\n            for (_v, face_opt) in neigh.iter() {\n                if face_opt.is_none() {\n                    return true;\n                }\n            }\n        }\n\n        for (_u, neigh) in self.halfedge.iter() {\n            if let Some(face_opt) = neigh.get(&vertex_key) {\n                if face_opt.is_none() {\n                    return true;\n                }\n            }\n        }\n        false\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.face_normal",
      "implementations": {
        "python": {
          "sig": "face_normal(face_key: int) -> Optional[Vector]",
          "code": "def face_normal(self, face_key: int) -> Optional[Vector]:\n\n        \"\"\"Calculate the normal of a face.\"\"\"\n        vertices = self.face_vertices(face_key)\n        if vertices is None or len(vertices) < 3:\n            return None\n\n        p0 = self.vertex_position(vertices[0])\n        p1 = self.vertex_position(vertices[1])\n        p2 = self.vertex_position(vertices[2])\n\n        if p0 is None or p1 is None or p2 is None:\n            return None\n\n        u = Vector(p1.x - p0.x, p1.y - p0.y, p1.z - p0.z)\n        v = Vector(p2.x - p0.x, p2.y - p0.y, p2.z - p0.z)\n\n        normal = u.cross(v)\n        length = normal.magnitude()\n\n        if length > Tolerance.ZERO_TOLERANCE:",
          "file": "mesh.py"
        },
        "cpp": {
          "sig": "std::optional<Vector> face_normal(size_t face_key)",
          "code": "std::optional<Vector> Mesh::face_normal(size_t face_key) const {\n    auto vertices_opt = face_vertices(face_key);\n    if (!vertices_opt.has_value() || vertices_opt->size() < 3) {\n        return std::nullopt;\n    }",
          "file": "mesh.cpp"
        },
        "rust": {
          "sig": "face_normal(face_key: usize) -> Option<Vector>",
          "code": "pub fn face_normal(&self, face_key: usize) -> Option<Vector> {\n        let vertices = self.face.get(&face_key)?;\n        if vertices.len() < 3 {\n            return None;\n        }\n\n        let p0 = self.vertex_position(vertices[0])?;\n        let p1 = self.vertex_position(vertices[1])?;\n        let p2 = self.vertex_position(vertices[2])?;\n\n        let u = Vector::new(p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]);\n        let v = Vector::new(p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]);\n\n        let normal = u.cross(&v);\n        let len = normal.magnitude();\n        if len > Tolerance::ZERO_TOLERANCE {\n            Some(Vector::new(\n                normal[0] / len,\n                normal[1] / len,\n                normal[2] / len,\n            ))\n        } else {\n            None\n        }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.vertex_normal",
      "implementations": {
        "python": {
          "sig": "vertex_normal(vertex_key: int) -> Optional[Vector]",
          "code": "def vertex_normal(self, vertex_key: int) -> Optional[Vector]:\n\n        \"\"\"Calculate the normal of a vertex (area-weighted).\"\"\"\n        return self.vertex_normal_weighted(vertex_key, NormalWeighting.AREA)\n\n    def vertex_normal_weighted(\n        self, vertex_key: int, weighting: NormalWeighting\n    ) -> Optional[Vector]:\n        \"\"\"Calculate the normal of a vertex with specified weighting.\"\"\"\n        faces = self.vertex_faces(vertex_key)\n        if not faces:\n            return None\n\n        normal_acc = Vector(0.0, 0.0, 0.0)\n\n        for face_key in faces:\n            face_normal = self.face_normal(face_key)\n            if face_normal is None:\n                continue\n\n            if weighting == NormalWeighting.AREA:",
          "file": "mesh.py"
        },
        "cpp": {
          "sig": "std::optional<Vector> vertex_normal(size_t vertex_key)",
          "code": "std::optional<Vector> Mesh::vertex_normal(size_t vertex_key) const {\n    return vertex_normal_weighted(vertex_key, NormalWeighting::Area);\n}",
          "file": "mesh.cpp"
        },
        "rust": {
          "sig": "vertex_normal(vertex_key: usize) -> Option<Vector>",
          "code": "pub fn vertex_normal(&self, vertex_key: usize) -> Option<Vector> {\n        self.vertex_normal_weighted(vertex_key, NormalWeighting::Area)\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.vertex_normal_weighted",
      "implementations": {
        "python": {
          "sig": "vertex_normal_weighted(\n        vertex_key: int, weighting: NormalWeighting\n    ) -> Optional[Vector]",
          "code": "def vertex_normal_weighted(\n        self, vertex_key: int, weighting: NormalWeighting\n    ) -> Optional[Vector]:\n\n        \"\"\"Calculate the normal of a vertex with specified weighting.\"\"\"\n        faces = self.vertex_faces(vertex_key)\n        if not faces:\n            return None\n\n        normal_acc = Vector(0.0, 0.0, 0.0)\n\n        for face_key in faces:\n            face_normal = self.face_normal(face_key)\n            if face_normal is None:\n                continue\n\n            if weighting == NormalWeighting.AREA:\n                weight = self.face_area(face_key) or 1.0\n            elif weighting == NormalWeighting.ANGLE:\n                weight = self.vertex_angle_in_face(vertex_key, face_key) or 1.0\n            else:  # UNIFORM\n                weight = 1.0",
          "file": "mesh.py"
        },
        "cpp": {
          "sig": "std::optional<Vector> vertex_normal_weighted(size_t vertex_key, NormalWeighting weighting)",
          "code": "std::optional<Vector> Mesh::vertex_normal_weighted(size_t vertex_key, NormalWeighting weighting) const {\n    auto faces = vertex_faces(vertex_key);\n    if (faces.empty()) {\n        return std::nullopt;\n    }",
          "file": "mesh.cpp"
        },
        "rust": {
          "sig": "vertex_normal_weighted(\n        ,\n        vertex_key: usize,\n        weighting: NormalWeighting,\n    ) -> Option<Vector>",
          "code": "pub fn vertex_normal_weighted(\n        &self,\n        vertex_key: usize,\n        weighting: NormalWeighting,\n    ) -> Option<Vector> {\n        let faces = self.vertex_faces(vertex_key);\n        if faces.is_empty() {\n            return None;\n        }\n\n        let mut normal_acc = Vector::new(0.0, 0.0, 0.0);\n\n        for face_key in faces {\n            if let Some(face_normal) = self.face_normal(face_key) {\n                let weight = match weighting {\n                    NormalWeighting::Area => self.face_area(face_key).unwrap_or(1.0),\n                    NormalWeighting::Angle => self\n                        .vertex_angle_in_face(vertex_key, face_key)\n                        .unwrap_or(1.0),\n                    NormalWeighting::Uniform => 1.0,\n                };\n\n                nor",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.face_area",
      "implementations": {
        "python": {
          "sig": "face_area(face_key: int) -> Optional[float]",
          "code": "def face_area(self, face_key: int) -> Optional[float]:\n\n        \"\"\"Calculate the area of a face.\"\"\"\n        vertices = self.face_vertices(face_key)\n        if vertices is None or len(vertices) < 3:\n            return 0.0\n\n        area = 0.0\n        p0 = self.vertex_position(vertices[0])\n        if p0 is None:\n            return None\n\n        for i in range(1, len(vertices) - 1):\n            p1 = self.vertex_position(vertices[i])\n            p2 = self.vertex_position(vertices[i + 1])\n            if p1 is None or p2 is None:\n                return None\n\n            u = Vector(p1.x - p0.x, p1.y - p0.y, p1.z - p0.z)\n            v = Vector(p2.x - p0.x, p2.y - p0.y, p2.z - p0.z)",
          "file": "mesh.py"
        },
        "cpp": {
          "sig": "std::optional<double> face_area(size_t face_key)",
          "code": "std::optional<double> Mesh::face_area(size_t face_key) const {\n    auto vertices_opt = face_vertices(face_key);\n    if (!vertices_opt.has_value() || vertices_opt->size() < 3) {\n        return 0.0;\n    }",
          "file": "mesh.cpp"
        },
        "rust": {
          "sig": "face_area(face_key: usize) -> Option<f64>",
          "code": "pub fn face_area(&self, face_key: usize) -> Option<f64> {\n        let vertices = self.face.get(&face_key)?;\n        if vertices.len() < 3 {\n            return Some(0.0);\n        }\n\n        let mut area = 0.0;\n        let p0 = self.vertex_position(vertices[0])?;\n\n        for i in 1..(vertices.len() - 1) {\n            let p1 = self.vertex_position(vertices[i])?;\n            let p2 = self.vertex_position(vertices[i + 1])?;\n\n            let u = Vector::new(p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]);\n            let v = Vector::new(p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]);\n\n            area += u.cross(&v).magnitude() * 0.5;\n        }\n\n        Some(area)\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.vertex_angle_in_face",
      "implementations": {
        "python": {
          "sig": "vertex_angle_in_face(vertex_key: int, face_key: int) -> Optional[float]",
          "code": "def vertex_angle_in_face(self, vertex_key: int, face_key: int) -> Optional[float]:\n\n        \"\"\"Calculate the angle at a vertex in a face.\"\"\"\n        vertices = self.face_vertices(face_key)\n        if vertices is None or vertex_key not in vertices:\n            return None\n\n        vertex_index = vertices.index(vertex_key)\n        n = len(vertices)\n        prev_vertex = vertices[(vertex_index - 1) % n]\n        next_vertex = vertices[(vertex_index + 1) % n]\n\n        center = self.vertex_position(vertex_key)\n        prev_pos = self.vertex_position(prev_vertex)\n        next_pos = self.vertex_position(next_vertex)\n\n        if center is None or prev_pos is None or next_pos is None:\n            return None\n\n        u = Vector(prev_pos.x - center.x, prev_pos.y - center.y, prev_pos.z - center.z)\n        v = Vector(next_pos.x - center.x, next_pos.y - center.y, next_pos.z - center.z)",
          "file": "mesh.py"
        },
        "cpp": {
          "sig": "std::optional<double> vertex_angle_in_face(size_t vertex_key, size_t face_key)",
          "code": "std::optional<double> Mesh::vertex_angle_in_face(size_t vertex_key, size_t face_key) const {\n    auto vertices_opt = face_vertices(face_key);\n    if (!vertices_opt) return std::nullopt;\n    \n    const auto& vertices = *vertices_opt;\n    auto it = std::find(vertices.begin(), vertices.end(), vertex_key);\n    if (it == vertices.end()) return std::nullopt;\n    \n    size_t vertex_index = std::distance(vertices.begin(), it);\n    size_t n = vertices.size();\n    size_t prev_vertex = vertices[(vertex_index + n - 1) % n];\n    size_t next_vertex = vertices[(vertex_index + 1) % n];\n    \n    auto center_opt = vertex_position(vertex_key);\n    auto prev_opt = vertex_position(prev_vertex);\n    auto next_opt = vertex_position(next_vertex);\n    \n    if (!center_opt || !prev_opt || !next_opt) return std::nu",
          "file": "mesh.cpp"
        },
        "rust": {
          "sig": "vertex_angle_in_face(vertex_key: usize, face_key: usize) -> Option<f64>",
          "code": "pub fn vertex_angle_in_face(&self, vertex_key: usize, face_key: usize) -> Option<f64> {\n        let vertices = self.face.get(&face_key)?;\n        let vertex_index = vertices.iter().position(|&v| v == vertex_key)?;\n\n        let n = vertices.len();\n        let prev_vertex = vertices[(vertex_index + n - 1) % n];\n        let next_vertex = vertices[(vertex_index + 1) % n];\n\n        let center = self.vertex_position(vertex_key)?;\n        let prev_pos = self.vertex_position(prev_vertex)?;\n        let next_pos = self.vertex_position(next_vertex)?;\n\n        let u = Vector::new(\n            prev_pos[0] - center[0],\n            prev_pos[1] - center[1],\n            prev_pos[2] - center[2],\n        );\n        let v = Vector::new(\n            next_pos[0] - center[0],\n            next_pos[1] - cente",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.face_normals",
      "implementations": {
        "python": {
          "sig": "face_normals() -> Dict[int, Vector]",
          "code": "def face_normals(self) -> Dict[int, Vector]:\n\n        \"\"\"Calculate normals for all faces.\"\"\"\n        normals = {}\n        for face_key in self.face:\n            normal = self.face_normal(face_key)\n            if normal is not None:\n                normals[face_key] = normal\n        return normals\n\n    def vertex_normals(self) -> Dict[int, Vector]:\n        \"\"\"Calculate normals for all vertices (area-weighted).\"\"\"\n        return self.vertex_normals_weighted(NormalWeighting.AREA)\n\n    def vertex_normals_weighted(self, weighting: NormalWeighting) -> Dict[int, Vector]:\n        \"\"\"Calculate normals for all vertices with specified weighting.\"\"\"\n        normals = {}\n        for vertex_key in self.vertex:\n            normal = self.vertex_normal_weighted(vertex_key, weighting)\n            if normal is not None:\n                normals[vertex_key] = normal",
          "file": "mesh.py"
        },
        "rust": {
          "sig": "face_normals() -> HashMap<usize, Vector>",
          "code": "pub fn face_normals(&self) -> HashMap<usize, Vector> {\n        let mut normals = HashMap::new();\n        for face_key in self.face.keys() {\n            if let Some(normal) = self.face_normal(*face_key) {\n                normals.insert(*face_key, normal);\n            }\n        }\n        normals\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.vertex_normals",
      "implementations": {
        "python": {
          "sig": "vertex_normals() -> Dict[int, Vector]",
          "code": "def vertex_normals(self) -> Dict[int, Vector]:\n\n        \"\"\"Calculate normals for all vertices (area-weighted).\"\"\"\n        return self.vertex_normals_weighted(NormalWeighting.AREA)\n\n    def vertex_normals_weighted(self, weighting: NormalWeighting) -> Dict[int, Vector]:\n        \"\"\"Calculate normals for all vertices with specified weighting.\"\"\"\n        normals = {}\n        for vertex_key in self.vertex:\n            normal = self.vertex_normal_weighted(vertex_key, weighting)\n            if normal is not None:\n                normals[vertex_key] = normal\n        return normals\n\n    ###########################################################################################\n    # Construction\n    ###########################################################################################\n\n    @staticmethod\n    def from_polygons(\n        polygons: List[List[Point]], precision: Optional[float] = None",
          "file": "mesh.py"
        },
        "rust": {
          "sig": "vertex_normals() -> HashMap<usize, Vector>",
          "code": "pub fn vertex_normals(&self) -> HashMap<usize, Vector> {\n        self.vertex_normals_weighted(NormalWeighting::Area)\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.vertex_normals_weighted",
      "implementations": {
        "python": {
          "sig": "vertex_normals_weighted(weighting: NormalWeighting) -> Dict[int, Vector]",
          "code": "def vertex_normals_weighted(self, weighting: NormalWeighting) -> Dict[int, Vector]:\n\n        \"\"\"Calculate normals for all vertices with specified weighting.\"\"\"\n        normals = {}\n        for vertex_key in self.vertex:\n            normal = self.vertex_normal_weighted(vertex_key, weighting)\n            if normal is not None:\n                normals[vertex_key] = normal\n        return normals\n\n    ###########################################################################################\n    # Construction\n    ###########################################################################################\n\n    @staticmethod\n    def from_polygons(\n        polygons: List[List[Point]], precision: Optional[float] = None\n    ) -> \"Mesh\":\n        \"\"\"Create a mesh from a list of polygons.\n\n        Parameters",
          "file": "mesh.py"
        },
        "rust": {
          "sig": "vertex_normals_weighted(weighting: NormalWeighting) -> HashMap<usize, Vector>",
          "code": "pub fn vertex_normals_weighted(&self, weighting: NormalWeighting) -> HashMap<usize, Vector> {\n        let mut normals = HashMap::new();\n        for vertex_key in self.vertex.keys() {\n            if let Some(normal) = self.vertex_normal_weighted(*vertex_key, weighting) {\n                normals.insert(*vertex_key, normal);\n            }\n        }\n        normals\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.from_polygons",
      "implementations": {
        "python": {
          "sig": "from_polygons(\n        polygons: List[List[Point]], precision: Optional[float] = None\n    ) -> \"Mesh\"",
          "code": "def from_polygons(\n        polygons: List[List[Point]], precision: Optional[float] = None\n    ) -> \"Mesh\":\n\n        \"\"\"Create a mesh from a list of polygons.\n\n        Parameters\n        ----------\n        polygons : list of list of Point\n            List of polygons, each polygon is a list of points.\n        precision : float, optional\n            Precision for vertex merging. If None, exact matching is used.\n\n        Returns\n        -------\n        Mesh\n            The constructed mesh with merged vertices.\n        \"\"\"\n        mesh = Mesh()\n        map_eps = {}\n        map_exact = {}\n\n        def get_vkey(p: Point) -> int:",
          "file": "mesh.py"
        },
        "cpp": {
          "sig": "Mesh from_polygons(const std::vector<std::vector<Point>>& polygons, std::optional<double> precision)",
          "code": "Mesh Mesh::from_polygons(const std::vector<std::vector<Point>>& polygons, std::optional<double> precision) {\n    Mesh mesh;\n    \n    std::map<std::tuple<int64_t, int64_t, int64_t>, size_t> map_eps;\n    std::map<std::tuple<uint64_t, uint64_t, uint64_t>, size_t> map_exact;\n    \n    auto get_vkey = [&](const Point& p) -> size_t {\n        if (precision.has_value()) {\n            double eps = *precision;\n            int64_t kx = static_cast<int64_t>(std::round(p[0] / eps));\n            int64_t ky = static_cast<int64_t>(std::round(p[1] / eps));\n            int64_t kz = static_cast<int64_t>(std::round(p[2] / eps));\n            auto key = std::make_tuple(kx, ky, kz);\n            \n            auto it = map_eps.find(key);\n            if (it != map_eps.end()) {\n                return it->second;",
          "file": "mesh.cpp"
        },
        "rust": {
          "sig": "from_polygons(polygons: Vec<Vec<Point>>, precision: Option<f64>) -> Self",
          "code": "pub fn from_polygons(polygons: Vec<Vec<Point>>, precision: Option<f64>) -> Self {\n        let mut mesh = Mesh::new();\n        let mut map_eps: HashMap<(i64, i64, i64), usize> = HashMap::new();\n        let mut map_exact: HashMap<(u64, u64, u64), usize> = HashMap::new();\n        let eps = precision.unwrap_or(0.0);\n        let use_eps = eps > 0.0;\n\n        let mut get_vkey = |p: &Point, mesh: &mut Mesh| -> usize {\n            if use_eps {\n                let kx = (p[0] / eps).round() as i64;\n                let ky = (p[1] / eps).round() as i64;\n                let kz = (p[2] / eps).round() as i64;\n                let key = (kx, ky, kz);\n                if let Some(&vk) = map_eps.get(&key) {\n                    return vk;\n                }\n                let vk = mesh.add_vertex(p.clone(",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.get_vkey",
      "implementations": {
        "python": {
          "sig": "get_vkey(p: Point) -> int",
          "code": "def get_vkey(p: Point) -> int:\n\n            if precision is not None:\n                kx = round(p.x / precision)\n                ky = round(p.y / precision)\n                kz = round(p.z / precision)\n                key = (kx, ky, kz)\n                if key in map_eps:\n                    return map_eps[key]\n                vk = mesh.add_vertex(p)\n                map_eps[key] = vk\n                return vk\n            else:\n                key = (p.x, p.y, p.z)\n                if key in map_exact:\n                    return map_exact[key]\n                vk = mesh.add_vertex(p)\n                map_exact[key] = vk\n                return vk\n\n        for poly in polygons:",
          "file": "mesh.py"
        }
      }
    },
    {
      "name": "Mesh.vertex_index",
      "implementations": {
        "python": {
          "sig": "vertex_index() -> Dict[int, int]",
          "code": "def vertex_index(self) -> Dict[int, int]:\n\n        \"\"\"Create a mapping from sparse vertex keys to sequential indices.\n\n        Returns\n        -------\n        dict[int, int]\n            A dictionary mapping vertex_key -> sequential_index (0, 1, 2, ...).\n        \"\"\"\n        # Sort keys to ensure consistent ordering\n        sorted_keys = sorted(self.vertex.keys())\n        return {key: index for index, key in enumerate(sorted_keys)}\n\n    def to_vertices_and_faces(self) -> Tuple[List[Point], List[List[int]]]:\n        \"\"\"Export vertices and faces with sequential 0-based indices.\n\n        Returns\n        -------\n        tuple\n            A tuple of (vertices, faces) where:\n            - vertices: List of Point objects in sequential order",
          "file": "mesh.py"
        },
        "rust": {
          "sig": "vertex_index() -> HashMap<usize, usize>",
          "code": "pub fn vertex_index(&self) -> HashMap<usize, usize> {\n        let mut keys: Vec<usize> = self.vertex.keys().copied().collect();\n        keys.sort();\n        keys.iter()\n            .enumerate()\n            .map(|(index, &key)| (key, index))\n            .collect()\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.to_vertices_and_faces",
      "implementations": {
        "python": {
          "sig": "to_vertices_and_faces() -> Tuple[List[Point], List[List[int]]]",
          "code": "def to_vertices_and_faces(self) -> Tuple[List[Point], List[List[int]]]:\n\n        \"\"\"Export vertices and faces with sequential 0-based indices.\n\n        Returns\n        -------\n        tuple\n            A tuple of (vertices, faces) where:\n            - vertices: List of Point objects in sequential order\n            - faces: List of face vertex lists using sequential indices\n        \"\"\"\n        vertex_idx = self.vertex_index()\n        vertices = [None] * len(self.vertex)\n\n        for key, vdata in self.vertex.items():\n            idx = vertex_idx[key]\n            vertices[idx] = vdata.position()\n\n        # Sort face keys to ensure consistent ordering\n        sorted_face_keys = sorted(self.face.keys())\n        faces = []",
          "file": "mesh.py"
        },
        "rust": {
          "sig": "to_vertices_and_faces() -> (Vec<Point>, Vec<Vec<usize>>)",
          "code": "pub fn to_vertices_and_faces(&self) -> (Vec<Point>, Vec<Vec<usize>>) {\n        let vertex_index = self.vertex_index();\n        let mut vertices: Vec<Point> = vec![Point::default(); self.vertex.len()];\n\n        for (&key, data) in &self.vertex {\n            let idx = vertex_index[&key];\n            vertices[idx] = data.position();\n        }\n\n        // Sort face keys to ensure consistent ordering\n        let mut face_keys: Vec<usize> = self.face.keys().copied().collect();\n        face_keys.sort();\n\n        let mut faces = Vec::new();\n        for face_key in face_keys {\n            let face_vertices = &self.face[&face_key];\n            let remapped: Vec<usize> = face_vertices.iter().map(|v| vertex_index[v]).collect();\n            faces.push(remapped);\n        }\n\n        (vertices, faces",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.__jsondump__",
      "implementations": {
        "python": {
          "sig": "__jsondump__()",
          "code": "def __jsondump__(self):\n\n        \"\"\"Serialize to polymorphic JSON format with type field.\n\n        Returns\n        -------\n        dict\n            Dictionary with fields in alphabetical order (matching Rust).\n\n        \"\"\"\n        # Halfedge connectivity\n        halfedge_data = {}\n        for u, neighbors in self.halfedge.items():\n            halfedge_data[str(u)] = {\n                str(v): face_key for v, face_key in neighbors.items()\n            }\n\n        # Vertex data (alphabetical: attributes, x, y, z)\n        vertex_data = {}\n        for key, vdata in self.vertex.items():\n            vertex_data[str(key)] = {",
          "file": "mesh.py"
        }
      }
    },
    {
      "name": "Mesh.__jsonload__",
      "implementations": {
        "python": {
          "sig": "__jsonload__(cls, data, guid=None, name=None)",
          "code": "def __jsonload__(cls, data, guid=None, name=None):\n\n        \"\"\"Deserialize from polymorphic JSON format.\n\n        Parameters\n        ----------\n        data : dict\n            Dictionary containing mesh data.\n        guid : str, optional\n            GUID for the mesh.\n        name : str, optional\n            Name for the mesh.\n\n        Returns\n        -------\n        :class:`Mesh`\n            Reconstructed mesh instance.\n\n        \"\"\"\n        mesh = cls()\n        mesh.guid = guid if guid is not None else data.get(\"guid\", mesh.guid)",
          "file": "mesh.py"
        }
      }
    },
    {
      "name": "Mesh.json_dump",
      "implementations": {
        "python": {
          "sig": "json_dump(filepath)",
          "code": "def json_dump(self, filepath):\n\n        \"\"\"Write JSON to file.\"\"\"\n        import json\n        with open(filepath, 'w') as f:\n            json.dump(self.__jsondump__(), f, indent=2)\n\n    @classmethod\n    def json_load(cls, filepath):\n        \"\"\"Read JSON from file.\"\"\"\n        import json\n        with open(filepath, 'r') as f:\n            data = json.load(f)\n        return cls.__jsonload__(data)\n\n    ###########################################################################################\n    # Transformation\n    ###########################################################################################\n\n    def transform(self):\n        \"\"\"Apply the stored xform transformation to the mesh.",
          "file": "mesh.py"
        }
      }
    },
    {
      "name": "Mesh.json_load",
      "implementations": {
        "python": {
          "sig": "json_load(cls, filepath)",
          "code": "def json_load(cls, filepath):\n\n        \"\"\"Read JSON from file.\"\"\"\n        import json\n        with open(filepath, 'r') as f:\n            data = json.load(f)\n        return cls.__jsonload__(data)\n\n    ###########################################################################################\n    # Transformation\n    ###########################################################################################\n\n    def transform(self):\n        \"\"\"Apply the stored xform transformation to the mesh.\n\n        Transforms all vertices in-place and resets xform to identity.\n        \"\"\"\n        from .xform import Xform\n\n        for vdata in self.vertex.values():\n            pos = vdata.position()",
          "file": "mesh.py"
        }
      }
    },
    {
      "name": "Mesh.transform",
      "implementations": {
        "python": {
          "sig": "transform()",
          "code": "def transform(self):\n\n        \"\"\"Apply the stored xform transformation to the mesh.\n\n        Transforms all vertices in-place and resets xform to identity.\n        \"\"\"\n        from .xform import Xform\n\n        for vdata in self.vertex.values():\n            pos = vdata.position()\n            self.xform.transform_point(pos)\n            vdata[0] = pos.x\n            vdata[1] = pos.y\n            vdata[2] = pos.z\n        self.xform = Xform.identity()\n\n    def transformed(self):\n        \"\"\"Return a transformed copy of the mesh.\"\"\"\n        import copy\n\n        result = copy.deepcopy(self)",
          "file": "mesh.py"
        },
        "cpp": {
          "sig": "void transform()",
          "code": "void Mesh::transform() {\n  for (auto& [idx, vdata] : vertex) {\n    Point pt(vdata.x, vdata.y, vdata.z);\n    xform.transform_point(pt);\n    vdata.x = pt[0];\n    vdata.y = pt[1];\n    vdata.z = pt[2];\n  }",
          "file": "mesh.cpp"
        },
        "rust": {
          "sig": "transform()",
          "code": "pub fn transform(&mut self) {\n        let xform = self.xform.clone();\n        for v in self.vertex.values_mut() {\n            let mut pt = Point::new(v.x, v.y, v.z);\n            xform.transform_point(&mut pt);\n            v.x = pt[0];\n            v.y = pt[1];\n            v.z = pt[2];\n        }\n        self.xform = Xform::identity();\n        self.invalidate_triangle_bvh();\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.transformed",
      "implementations": {
        "python": {
          "sig": "transformed()",
          "code": "def transformed(self):\n\n        \"\"\"Return a transformed copy of the mesh.\"\"\"\n        import copy\n\n        result = copy.deepcopy(self)\n        result.transform()\n        return result\n\n    ###########################################################################################\n    # Color and Width Management\n    ###########################################################################################\n\n    def set_vertex_color(self, index: int, color: Color):\n        \"\"\"Set color for a specific vertex.\"\"\"\n        if 0 <= index < len(self.pointcolors):\n            self.pointcolors[index] = color\n\n    def set_face_color(self, index: int, color: Color):\n        \"\"\"Set color for a specific face.\"\"\"\n        if 0 <= index < len(self.facecolors):",
          "file": "mesh.py"
        },
        "cpp": {
          "sig": "Mesh transformed()",
          "code": "Mesh Mesh::transformed() const {\n  Mesh result = *this;\n  result.transform();\n  return result;\n}",
          "file": "mesh.cpp"
        },
        "rust": {
          "sig": "transformed() -> Self",
          "code": "pub fn transformed(&self) -> Self {\n        let mut result = self.clone();\n        result.transform();\n        result\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.set_vertex_color",
      "implementations": {
        "python": {
          "sig": "set_vertex_color(index: int, color: Color)",
          "code": "def set_vertex_color(self, index: int, color: Color):\n\n        \"\"\"Set color for a specific vertex.\"\"\"\n        if 0 <= index < len(self.pointcolors):\n            self.pointcolors[index] = color\n\n    def set_face_color(self, index: int, color: Color):\n        \"\"\"Set color for a specific face.\"\"\"\n        if 0 <= index < len(self.facecolors):\n            self.facecolors[index] = color\n\n    def set_edge_color(self, index: int, color: Color):\n        \"\"\"Set color for a specific edge.\"\"\"\n        if 0 <= index < len(self.linecolors):\n            self.linecolors[index] = color\n\n    def set_edge_width(self, index: int, width: float):\n        \"\"\"Set width for a specific edge.\"\"\"\n        if 0 <= index < len(self.widths):\n            self.widths[index] = width",
          "file": "mesh.py"
        },
        "rust": {
          "sig": "set_vertex_color(index: usize, color: Color)",
          "code": "pub fn set_vertex_color(&mut self, index: usize, color: Color) {\n        if index < self.pointcolors.len() {\n            self.pointcolors[index] = color;\n        }\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.set_face_color",
      "implementations": {
        "python": {
          "sig": "set_face_color(index: int, color: Color)",
          "code": "def set_face_color(self, index: int, color: Color):\n\n        \"\"\"Set color for a specific face.\"\"\"\n        if 0 <= index < len(self.facecolors):\n            self.facecolors[index] = color\n\n    def set_edge_color(self, index: int, color: Color):\n        \"\"\"Set color for a specific edge.\"\"\"\n        if 0 <= index < len(self.linecolors):\n            self.linecolors[index] = color\n\n    def set_edge_width(self, index: int, width: float):\n        \"\"\"Set width for a specific edge.\"\"\"\n        if 0 <= index < len(self.widths):\n            self.widths[index] = width\n\n    ###########################################################################################\n    # Protobuf Serialization\n    ###########################################################################################\n\n    def to_protobuf(self):",
          "file": "mesh.py"
        },
        "rust": {
          "sig": "set_face_color(index: usize, color: Color)",
          "code": "pub fn set_face_color(&mut self, index: usize, color: Color) {\n        if index < self.facecolors.len() {\n            self.facecolors[index] = color;\n        }\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.set_edge_color",
      "implementations": {
        "python": {
          "sig": "set_edge_color(index: int, color: Color)",
          "code": "def set_edge_color(self, index: int, color: Color):\n\n        \"\"\"Set color for a specific edge.\"\"\"\n        if 0 <= index < len(self.linecolors):\n            self.linecolors[index] = color\n\n    def set_edge_width(self, index: int, width: float):\n        \"\"\"Set width for a specific edge.\"\"\"\n        if 0 <= index < len(self.widths):\n            self.widths[index] = width\n\n    ###########################################################################################\n    # Protobuf Serialization\n    ###########################################################################################\n\n    def to_protobuf(self):\n        \"\"\"Convert to protobuf binary format.\"\"\"\n        from .proto import mesh_pb2\n\n        proto = mesh_pb2.Mesh()\n        proto.guid = self.guid",
          "file": "mesh.py"
        },
        "rust": {
          "sig": "set_edge_color(index: usize, color: Color)",
          "code": "pub fn set_edge_color(&mut self, index: usize, color: Color) {\n        if index < self.linecolors.len() {\n            self.linecolors[index] = color;\n        }\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.set_edge_width",
      "implementations": {
        "python": {
          "sig": "set_edge_width(index: int, width: float)",
          "code": "def set_edge_width(self, index: int, width: float):\n\n        \"\"\"Set width for a specific edge.\"\"\"\n        if 0 <= index < len(self.widths):\n            self.widths[index] = width\n\n    ###########################################################################################\n    # Protobuf Serialization\n    ###########################################################################################\n\n    def to_protobuf(self):\n        \"\"\"Convert to protobuf binary format.\"\"\"\n        from .proto import mesh_pb2\n\n        proto = mesh_pb2.Mesh()\n        proto.guid = self.guid\n        proto.name = self.name\n\n        # Vertices\n        for vkey, vdata in self.vertex.items():\n            vertex_proto = proto.vertices[vkey]",
          "file": "mesh.py"
        },
        "rust": {
          "sig": "set_edge_width(index: usize, width: f64)",
          "code": "pub fn set_edge_width(&mut self, index: usize, width: f64) {\n        if index < self.widths.len() {\n            self.widths[index] = width;\n        }\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.to_protobuf",
      "implementations": {
        "python": {
          "sig": "to_protobuf()",
          "code": "def to_protobuf(self):\n\n        \"\"\"Convert to protobuf binary format.\"\"\"\n        from .proto import mesh_pb2\n\n        proto = mesh_pb2.Mesh()\n        proto.guid = self.guid\n        proto.name = self.name\n\n        # Vertices\n        for vkey, vdata in self.vertex.items():\n            vertex_proto = proto.vertices[vkey]\n            vertex_proto.x = vdata.x\n            vertex_proto.y = vdata.y\n            vertex_proto.z = vdata.z\n            for k, v in vdata.attributes.items():\n                vertex_proto.attributes[k] = v\n\n        # Faces\n        for fkey, fverts in self.face.items():\n            face_proto = proto.faces[fkey]",
          "file": "mesh.py"
        },
        "cpp": {
          "sig": "std::string to_protobuf()",
          "code": "std::string Mesh::to_protobuf() const {\n    session_proto::Mesh proto;\n    proto.set_guid(this->guid);\n    proto.set_name(this->name);\n\n    // Vertices\n    for (const auto& [vkey, vdata] : vertex) {\n        auto& vertex_proto = (*proto.mutable_vertices())[vkey];\n        vertex_proto.set_x(vdata.x);\n        vertex_proto.set_y(vdata.y);\n        vertex_proto.set_z(vdata.z);\n        for (const auto& [k, v] : vdata.attributes) {\n            (*vertex_proto.mutable_attributes())[k] = v;\n        }",
          "file": "mesh.cpp"
        },
        "rust": {
          "sig": "to_protobuf() -> Vec<u8>",
          "code": "pub fn to_protobuf(&self) -> Vec<u8> {\n        use prost::Message;\n        use std::collections::HashMap;\n\n        let mut vertices: HashMap<u64, crate::proto::VertexData> = HashMap::new();\n        for (&vkey, vdata) in &self.vertex {\n            let mut attrs: HashMap<String, f64> = HashMap::new();\n            for (k, v) in &vdata.attributes {\n                attrs.insert(k.clone(), *v);\n            }\n            vertices.insert(vkey as u64, crate::proto::VertexData {\n                x: vdata.x,\n                y: vdata.y,\n                z: vdata.z,\n                attributes: attrs,\n            });\n        }\n\n        let mut faces: HashMap<u64, crate::proto::FaceData> = HashMap::new();\n        for (&fkey, fverts) in &self.face {\n            let mut attrs: HashMap<String, f64> = Hash",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.from_protobuf",
      "implementations": {
        "python": {
          "sig": "from_protobuf(cls, data)",
          "code": "def from_protobuf(cls, data):\n\n        \"\"\"Create Mesh from protobuf binary data.\"\"\"\n        from .proto import mesh_pb2\n        from .color import Color\n        from .xform import Xform\n\n        proto = mesh_pb2.Mesh()\n        proto.ParseFromString(data)\n\n        mesh = cls()\n        mesh.guid = proto.guid\n        mesh.name = proto.name\n\n        # Vertices\n        for vkey, vdata in proto.vertices.items():\n            attrs = dict(vdata.attributes)\n            mesh.vertex[vkey] = VertexData(Point(vdata.x, vdata.y, vdata.z))\n            mesh.vertex[vkey].attributes = attrs\n            if vkey not in mesh.halfedge:\n                mesh.halfedge[vkey] = {}",
          "file": "mesh.py"
        },
        "cpp": {
          "sig": "Mesh from_protobuf(const std::string& data)",
          "code": "Mesh Mesh::from_protobuf(const std::string& data) {\n    session_proto::Mesh proto;\n    proto.ParseFromString(data);\n\n    Mesh mesh;\n    mesh.guid = proto.guid();\n    mesh.name = proto.name();\n\n    // Vertices\n    for (const auto& [vkey, vdata] : proto.vertices()) {\n        VertexData vd;\n        vd.x = vdata.x();\n        vd.y = vdata.y();\n        vd.z = vdata.z();\n        for (const auto& [k, v] : vdata.attributes()) {\n            vd.attributes[k] = v;\n        }",
          "file": "mesh.cpp"
        },
        "rust": {
          "sig": "from_protobuf(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn from_protobuf(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {\n        use prost::Message;\n\n        let proto = crate::proto::Mesh::decode(data)?;\n        let mut mesh = Self::new();\n        mesh.guid = proto.guid;\n        mesh.name = proto.name;\n\n        for (vkey, vdata) in proto.vertices {\n            let mut attrs: std::collections::HashMap<String, f64> = std::collections::HashMap::new();\n            for (k, v) in vdata.attributes {\n                attrs.insert(k, v);\n            }\n            mesh.vertex.insert(vkey as usize, VertexData {\n                x: vdata.x,\n                y: vdata.y,\n                z: vdata.z,\n                attributes: attrs,\n            });\n            mesh.halfedge.entry(vkey as usize).or_insert_with(std::collections::HashMap::new);",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.protobuf_dump",
      "implementations": {
        "python": {
          "sig": "protobuf_dump(filepath)",
          "code": "def protobuf_dump(self, filepath):\n\n        \"\"\"Write protobuf to file.\"\"\"\n        data = self.to_protobuf()\n        with open(filepath, 'wb') as f:\n            f.write(data)\n\n    @classmethod\n    def protobuf_load(cls, filepath):\n        \"\"\"Read protobuf from file.\"\"\"\n        with open(filepath, 'rb') as f:\n            data = f.read()\n        return cls.from_protobuf(data)",
          "file": "mesh.py"
        },
        "cpp": {
          "sig": "void protobuf_dump(const std::string& filename)",
          "code": "void Mesh::protobuf_dump(const std::string& filename) const {\n    std::string data = to_protobuf();\n    std::ofstream file(filename, std::ios::binary);\n    file.write(data.data(), data.size());\n}",
          "file": "mesh.cpp"
        },
        "rust": {
          "sig": "protobuf_dump(filepath: &str)",
          "code": "pub fn protobuf_dump(&self, filepath: &str) {\n        let data = self.to_protobuf();\n        std::fs::write(filepath, data).expect(\"Failed to write protobuf file\");\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.protobuf_load",
      "implementations": {
        "python": {
          "sig": "protobuf_load(cls, filepath)",
          "code": "def protobuf_load(cls, filepath):\n\n        \"\"\"Read protobuf from file.\"\"\"\n        with open(filepath, 'rb') as f:\n            data = f.read()\n        return cls.from_protobuf(data)",
          "file": "mesh.py"
        },
        "cpp": {
          "sig": "Mesh protobuf_load(const std::string& filename)",
          "code": "Mesh Mesh::protobuf_load(const std::string& filename) {\n    std::ifstream file(filename, std::ios::binary);\n    std::string data((std::istreambuf_iterator<char>(file)), std::istreambuf_iterator<char>());\n    return from_protobuf(data);\n}",
          "file": "mesh.cpp"
        },
        "rust": {
          "sig": "protobuf_load(filepath: &str) -> Self",
          "code": "pub fn protobuf_load(filepath: &str) -> Self {\n        let data = std::fs::read(filepath).expect(\"Failed to read protobuf file\");\n        Self::from_protobuf(&data).expect(\"Failed to parse protobuf\")\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.__init__",
      "implementations": {
        "python": {
          "sig": "__init__(dimension: int = 3, is_rational: bool = False, \n                 order: int = 4, cv_count: int = 0)",
          "code": "def __init__(self, dimension: int = 3, is_rational: bool = False, \n                 order: int = 4, cv_count: int = 0):\n\n        self.guid = str(uuid.uuid4())\n        self.name = \"nurbscurve\"\n        \n        # Core NURBS data\n        self.m_dim = dimension\n        self.m_is_rat = 1 if is_rational else 0\n        self.m_order = order\n        self.m_cv_count = cv_count\n        self.m_cv_stride = (dimension + 1) if is_rational else dimension\n        \n        # Data arrays\n        self.m_knot = np.array([], dtype=np.float64)\n        self.m_cv = np.array([], dtype=np.float64)\n    \n    #############################################################################\n    # STATIC FACTORY METHODS\n    #############################################################################\n    \n    @staticmethod",
          "file": "nurbscurve.py"
        }
      }
    },
    {
      "name": "NurbsCurve.create",
      "implementations": {
        "python": {
          "sig": "create(periodic: bool, degree: int, points: List[Point], \n               dimension: int = 3, knot_delta: float = 1.0) -> 'NurbsCurve'",
          "code": "def create(periodic: bool, degree: int, points: List[Point], \n               dimension: int = 3, knot_delta: float = 1.0) -> 'NurbsCurve':\n\n        \"\"\"Create NURBS curve from points.\n\n        Parameters\n        ----------\n        periodic : bool\n            If True, creates a periodic curve; otherwise clamped.\n        degree : int\n            The degree of the curve.\n        points : list of Point\n            Control points for the curve.\n        dimension : int, optional\n            Dimension of the curve. Defaults to 3.\n        knot_delta : float, optional\n            Spacing between knots. Defaults to 1.0.\n\n        Returns\n        -------\n        NurbsCurve\n            The created NURBS curve.",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool create(int dimension, bool is_rational, int order, int cv_count)",
          "code": "bool NurbsCurve::create(int dimension, bool is_rational, int order, int cv_count) {\n    if (dimension < 1 || order < 2 || cv_count < order) {\n        return false;\n    }",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "create(periodic: bool, degree: usize, points: &[Point]) -> Self",
          "code": "pub fn create(periodic: bool, degree: usize, points: &[Point]) -> Self {\n        let order = degree + 1;\n        \n        if periodic {\n            Self::create_periodic_uniform(3, order, points, 1.0)\n        } else {\n            Self::create_clamped_uniform(3, order, points, 1.0)\n        }\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.initialize",
      "implementations": {
        "python": {
          "sig": "initialize()",
          "code": "def initialize(self):\n\n        \"\"\"Initialize all fields to zero/empty.\n        \n        Returns\n        -------\n        None\n        \"\"\"\n        self.m_dim = 0\n        self.m_is_rat = 0\n        self.m_order = 0\n        self.m_cv_count = 0\n        self.m_cv_stride = 0\n        self.m_knot = np.array([], dtype=np.float64)\n        self.m_cv = np.array([], dtype=np.float64)\n    \n    def create_curve(self, dimension: int, is_rational: bool, \n                    order: int, cv_count: int) -> bool:\n        \"\"\"Create NURBS curve with specified parameters\"\"\"\n        if dimension < 1 or order < 2 or cv_count < order:\n            return False",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "void initialize()",
          "code": "void NurbsCurve::initialize() {\n    m_dim = 0;\n    m_is_rat = 0;\n    m_order = 0;\n    m_cv_count = 0;\n    m_cv_stride = 0;\n    m_cv_capacity = 0;\n    m_knot.clear();\n    m_cv.clear();\n}",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.create_curve",
      "implementations": {
        "python": {
          "sig": "create_curve(dimension: int, is_rational: bool, \n                    order: int, cv_count: int) -> bool",
          "code": "def create_curve(self, dimension: int, is_rational: bool, \n                    order: int, cv_count: int) -> bool:\n\n        \"\"\"Create NURBS curve with specified parameters\"\"\"\n        if dimension < 1 or order < 2 or cv_count < order:\n            return False\n        \n        self.m_dim = dimension\n        self.m_is_rat = 1 if is_rational else 0\n        self.m_order = order\n        self.m_cv_count = cv_count\n        self.m_cv_stride = (dimension + 1) if is_rational else dimension\n        \n        # Allocate arrays\n        knot_count = order + cv_count - 2\n        self.m_knot = np.zeros(knot_count, dtype=np.float64)\n        self.m_cv = np.zeros(cv_count * self.m_cv_stride, dtype=np.float64)\n        \n        # Set weights to 1.0 if rational\n        if is_rational:\n            for i in range(cv_count):\n                self.m_cv[i * self.m_cv_stride + dimension] = 1.0",
          "file": "nurbscurve.py"
        }
      }
    },
    {
      "name": "NurbsCurve.create_clamped_uniform",
      "implementations": {
        "python": {
          "sig": "create_clamped_uniform(dimension: int, order: int, \n                              points: List[Point], knot_delta: float = 1.0) -> bool",
          "code": "def create_clamped_uniform(self, dimension: int, order: int, \n                              points: List[Point], knot_delta: float = 1.0) -> bool:\n\n        \"\"\"Create clamped uniform NURBS curve from control points\"\"\"\n        if not points or len(points) < order:\n            return False\n        \n        if not self.create_curve(dimension, False, order, len(points)):\n            return False\n        \n        # Set control points\n        for i, pt in enumerate(points):\n            self.set_cv(i, pt)\n        \n        # Create clamped uniform knot vector\n        self.make_clamped_uniform_knot_vector(knot_delta)\n        \n        return True\n    \n    def create_periodic_uniform(self, dimension: int, order: int,\n                               points: List[Point], knot_delta: float = 1.0) -> bool:\n        \"\"\"Create periodic uniform NURBS curve from control points\"\"\"",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool create_clamped_uniform(int dimension, int order, \n                                        const std::vector<Point>& points,\n                                        double knot_delta)",
          "code": "bool NurbsCurve::create_clamped_uniform(int dimension, int order, \n                                        const std::vector<Point>& points,\n                                        double knot_delta) {\n    int point_count = static_cast<int>(points.size());\n    if (!create(dimension, false, order, point_count)) {\n        return false;\n    }",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "create_clamped_uniform(\n        dimension: usize,\n        order: usize,\n        points: &[Point],\n        knot_delta: f64,\n    ) -> Self",
          "code": "pub fn create_clamped_uniform(\n        dimension: usize,\n        order: usize,\n        points: &[Point],\n        knot_delta: f64,\n    ) -> Self {\n        let point_count = points.len();\n        \n        if order < 2 || point_count < order {\n            return Self::default();\n        }\n\n        let mut curve = Self::default();\n        if !curve.initialize_curve(dimension, false, order, point_count) {\n            return Self::default();\n        }\n\n        // Set control points\n        for (i, point) in points.iter().enumerate() {\n            curve.set_cv(i, point);\n        }\n\n        // Create clamped uniform knot vector - matches OpenNURBS exactly\n        let knot_count = order + point_count - 2;\n\n        // Fill interior knots with uniform spacing\n        // Start from index (order-2)",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.create_periodic_uniform",
      "implementations": {
        "python": {
          "sig": "create_periodic_uniform(dimension: int, order: int,\n                               points: List[Point], knot_delta: float = 1.0) -> bool",
          "code": "def create_periodic_uniform(self, dimension: int, order: int,\n                               points: List[Point], knot_delta: float = 1.0) -> bool:\n\n        \"\"\"Create periodic uniform NURBS curve from control points\"\"\"\n        if not points or len(points) < order:\n            return False\n        \n        if not self.create_curve(dimension, False, order, len(points)):\n            return False\n        \n        # Set control points\n        for i, pt in enumerate(points):\n            self.set_cv(i, pt)\n        \n        # Create periodic uniform knot vector\n        self.make_periodic_uniform_knot_vector(knot_delta)\n        \n        return True\n    \n    def destroy(self):\n        \"\"\"Deallocate all memory and reset to empty state\"\"\"\n        self.initialize()",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool create_periodic_uniform(int dimension, int order,\n                                        const std::vector<Point>& points,\n                                        double knot_delta)",
          "code": "bool NurbsCurve::create_periodic_uniform(int dimension, int order,\n                                        const std::vector<Point>& points,\n                                        double knot_delta) {\n    int point_count = static_cast<int>(points.size());\n    if (!create(dimension, false, order, point_count + order - 1)) {\n        return false;\n    }",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "create_periodic_uniform(\n        dimension: usize,\n        order: usize,\n        points: &[Point],\n        knot_delta: f64,\n    ) -> Self",
          "code": "pub fn create_periodic_uniform(\n        dimension: usize,\n        order: usize,\n        points: &[Point],\n        knot_delta: f64,\n    ) -> Self {\n        let point_count = points.len();\n        \n        if order < 2 || point_count < order {\n            return Self::default();\n        }\n\n        let mut curve = Self::default();\n        let cv_count = point_count + order - 1;\n        \n        if !curve.initialize_curve(dimension, false, order, cv_count) {\n            return Self::default();\n        }\n\n        // Set control points with wrapping\n        for (i, point) in points.iter().enumerate() {\n            curve.set_cv(i, point);\n        }\n        \n        // Wrap control points for periodicity\n        for i in 0..(order - 1) {\n            let idx = i % point_count;\n            curve",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.destroy",
      "implementations": {
        "python": {
          "sig": "destroy()",
          "code": "def destroy(self):\n\n        \"\"\"Deallocate all memory and reset to empty state\"\"\"\n        self.initialize()\n    \n    #############################################################################\n    # VALIDATION\n    #############################################################################\n    \n    def is_valid(self) -> bool:\n        \"\"\"Check if NURBS curve is valid\"\"\"\n        if self.m_dim < 1:\n            return False\n        if self.m_order < 2:\n            return False\n        if self.m_cv_count < self.m_order:\n            return False\n        if len(self.m_knot) != self.m_order + self.m_cv_count - 2:\n            return False\n        if len(self.m_cv) < self.m_cv_count * self.m_cv_stride:\n            return False",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "void destroy()",
          "code": "void NurbsCurve::destroy() {\n    m_knot.clear();\n    m_cv.clear();\n    initialize();\n}",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.is_valid",
      "implementations": {
        "python": {
          "sig": "is_valid() -> bool",
          "code": "def is_valid(self) -> bool:\n\n        \"\"\"Check if NURBS curve is valid\"\"\"\n        if self.m_dim < 1:\n            return False\n        if self.m_order < 2:\n            return False\n        if self.m_cv_count < self.m_order:\n            return False\n        if len(self.m_knot) != self.m_order + self.m_cv_count - 2:\n            return False\n        if len(self.m_cv) < self.m_cv_count * self.m_cv_stride:\n            return False\n        \n        # Check knot vector is non-decreasing\n        for i in range(len(self.m_knot) - 1):\n            if self.m_knot[i] > self.m_knot[i + 1] + Tolerance.ZERO_TOLERANCE:\n                return False\n        \n        return True",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool is_valid()",
          "code": "bool NurbsCurve::is_valid() const {\n    if (m_dim <= 0) return false;\n    if (m_order < 2) return false;\n    if (m_cv_count < m_order) return false;\n    if (m_cv_stride < cv_size()) return false;\n    if (m_cv.empty() || m_knot.empty()) return false;\n    if (!is_valid_knot_vector()) return false;\n    \n    // Check CVs for valid values\n    for (size_t i = 0; i < m_cv.size(); i++) {\n        if (!std::isfinite(m_cv[i])) return false;\n    }",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "is_valid() -> bool",
          "code": "pub fn is_valid(&self) -> bool {\n        if self.m_order < 2 || self.m_cv_count < self.m_order {\n            return false;\n        }\n        if self.m_knot.len() != self.m_order + self.m_cv_count - 2 {\n            return false;\n        }\n        // Check for sufficient distinct knots\n        if self.m_order >= 2 && self.m_cv_count >= self.m_order {\n            let idx1 = self.m_order - 2;\n            let idx2 = self.m_cv_count - 1;\n            if idx2 < self.m_knot.len() && self.m_knot[idx1] >= self.m_knot[idx2] {\n                return false;\n            }\n        }\n        true\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.dimension",
      "implementations": {
        "python": {
          "sig": "dimension() -> int",
          "code": "def dimension(self) -> int:\n\n        return self.m_dim\n    \n    def is_rational(self) -> bool:\n        return self.m_is_rat != 0\n    \n    def order(self) -> int:\n        return self.m_order\n    \n    def degree(self) -> int:\n        return self.m_order - 1\n    \n    def cv_count(self) -> int:\n        return self.m_cv_count\n    \n    def cv_size(self) -> int:\n        \"\"\"Size of each control vertex\"\"\"\n        return (self.m_dim + 1) if self.m_is_rat else self.m_dim\n    \n    def knot_count(self) -> int:",
          "file": "nurbscurve.py"
        },
        "rust": {
          "sig": "dimension() -> usize",
          "code": "pub fn dimension(&self) -> usize {\n        self.m_dim\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.is_rational",
      "implementations": {
        "python": {
          "sig": "is_rational() -> bool",
          "code": "def is_rational(self) -> bool:\n\n        return self.m_is_rat != 0\n    \n    def order(self) -> int:\n        return self.m_order\n    \n    def degree(self) -> int:\n        return self.m_order - 1\n    \n    def cv_count(self) -> int:\n        return self.m_cv_count\n    \n    def cv_size(self) -> int:\n        \"\"\"Size of each control vertex\"\"\"\n        return (self.m_dim + 1) if self.m_is_rat else self.m_dim\n    \n    def knot_count(self) -> int:\n        return self.m_order + self.m_cv_count - 2\n    \n    def span_count(self) -> int:",
          "file": "nurbscurve.py"
        },
        "rust": {
          "sig": "is_rational() -> bool",
          "code": "pub fn is_rational(&self) -> bool {\n        self.m_is_rat\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.order",
      "implementations": {
        "python": {
          "sig": "order() -> int",
          "code": "def order(self) -> int:\n\n        return self.m_order\n    \n    def degree(self) -> int:\n        return self.m_order - 1\n    \n    def cv_count(self) -> int:\n        return self.m_cv_count\n    \n    def cv_size(self) -> int:\n        \"\"\"Size of each control vertex\"\"\"\n        return (self.m_dim + 1) if self.m_is_rat else self.m_dim\n    \n    def knot_count(self) -> int:\n        return self.m_order + self.m_cv_count - 2\n    \n    def span_count(self) -> int:\n        return self.m_cv_count - self.m_order + 1\n    \n    def cv_capacity(self) -> int:",
          "file": "nurbscurve.py"
        },
        "rust": {
          "sig": "order() -> usize",
          "code": "pub fn order(&self) -> usize {\n        self.m_order\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.degree",
      "implementations": {
        "python": {
          "sig": "degree() -> int",
          "code": "def degree(self) -> int:\n\n        return self.m_order - 1\n    \n    def cv_count(self) -> int:\n        return self.m_cv_count\n    \n    def cv_size(self) -> int:\n        \"\"\"Size of each control vertex\"\"\"\n        return (self.m_dim + 1) if self.m_is_rat else self.m_dim\n    \n    def knot_count(self) -> int:\n        return self.m_order + self.m_cv_count - 2\n    \n    def span_count(self) -> int:\n        return self.m_cv_count - self.m_order + 1\n    \n    def cv_capacity(self) -> int:\n        return len(self.m_cv) // self.m_cv_stride\n    \n    def knot_capacity(self) -> int:",
          "file": "nurbscurve.py"
        },
        "rust": {
          "sig": "degree() -> usize",
          "code": "pub fn degree(&self) -> usize {\n        if self.m_order < 2 {\n            0\n        } else {\n            self.m_order - 1\n        }\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.cv_count",
      "implementations": {
        "python": {
          "sig": "cv_count() -> int",
          "code": "def cv_count(self) -> int:\n\n        return self.m_cv_count\n    \n    def cv_size(self) -> int:\n        \"\"\"Size of each control vertex\"\"\"\n        return (self.m_dim + 1) if self.m_is_rat else self.m_dim\n    \n    def knot_count(self) -> int:\n        return self.m_order + self.m_cv_count - 2\n    \n    def span_count(self) -> int:\n        return self.m_cv_count - self.m_order + 1\n    \n    def cv_capacity(self) -> int:\n        return len(self.m_cv) // self.m_cv_stride\n    \n    def knot_capacity(self) -> int:\n        return len(self.m_knot)\n    \n    #############################################################################",
          "file": "nurbscurve.py"
        },
        "rust": {
          "sig": "cv_count() -> usize",
          "code": "pub fn cv_count(&self) -> usize {\n        self.m_cv_count\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.cv_size",
      "implementations": {
        "python": {
          "sig": "cv_size() -> int",
          "code": "def cv_size(self) -> int:\n\n        \"\"\"Size of each control vertex\"\"\"\n        return (self.m_dim + 1) if self.m_is_rat else self.m_dim\n    \n    def knot_count(self) -> int:\n        return self.m_order + self.m_cv_count - 2\n    \n    def span_count(self) -> int:\n        return self.m_cv_count - self.m_order + 1\n    \n    def cv_capacity(self) -> int:\n        return len(self.m_cv) // self.m_cv_stride\n    \n    def knot_capacity(self) -> int:\n        return len(self.m_knot)\n    \n    #############################################################################\n    # CONTROL VERTEX ACCESS  \n    #############################################################################",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "int cv_size()",
          "code": "int NurbsCurve::cv_size() const {\n    return (m_dim > 0) ? (m_is_rat ? (m_dim + 1) : m_dim) : 0;\n}",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "cv_size() -> usize",
          "code": "pub fn cv_size(&self) -> usize {\n        self.m_cv_stride\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.knot_count",
      "implementations": {
        "python": {
          "sig": "knot_count() -> int",
          "code": "def knot_count(self) -> int:\n\n        return self.m_order + self.m_cv_count - 2\n    \n    def span_count(self) -> int:\n        return self.m_cv_count - self.m_order + 1\n    \n    def cv_capacity(self) -> int:\n        return len(self.m_cv) // self.m_cv_stride\n    \n    def knot_capacity(self) -> int:\n        return len(self.m_knot)\n    \n    #############################################################################\n    # CONTROL VERTEX ACCESS  \n    #############################################################################\n    \n    def get_cv(self, cv_index: int) -> Optional[Point]:\n        \"\"\"Get control point at index as Point\"\"\"\n        if cv_index < 0 or cv_index >= self.m_cv_count:\n            return None",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "int knot_count()",
          "code": "int NurbsCurve::knot_count() const {\n    return m_order + m_cv_count - 2;\n}",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "knot_count() -> usize",
          "code": "pub fn knot_count(&self) -> usize {\n        self.m_knot.len()\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.span_count",
      "implementations": {
        "python": {
          "sig": "span_count() -> int",
          "code": "def span_count(self) -> int:\n\n        return self.m_cv_count - self.m_order + 1\n    \n    def cv_capacity(self) -> int:\n        return len(self.m_cv) // self.m_cv_stride\n    \n    def knot_capacity(self) -> int:\n        return len(self.m_knot)\n    \n    #############################################################################\n    # CONTROL VERTEX ACCESS  \n    #############################################################################\n    \n    def get_cv(self, cv_index: int) -> Optional[Point]:\n        \"\"\"Get control point at index as Point\"\"\"\n        if cv_index < 0 or cv_index >= self.m_cv_count:\n            return None\n        \n        idx = cv_index * self.m_cv_stride\n        if self.m_is_rat:",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "int span_count()",
          "code": "int NurbsCurve::span_count() const {\n    int count = 0;\n    int kc = knot_count();\n    for (int i = m_order - 2; i < m_cv_count - 1; i++) {\n        if (i >= 0 && i + 1 < kc && m_knot[i] < m_knot[i + 1]) {\n            count++;\n        }",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "span_count() -> usize",
          "code": "pub fn span_count(&self) -> usize {\n        if !self.is_valid() {\n            return 0;\n        }\n        let spans = self.get_span_vector();\n        if spans.len() > 1 {\n            spans.len() - 1\n        } else {\n            0\n        }\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.cv_capacity",
      "implementations": {
        "python": {
          "sig": "cv_capacity() -> int",
          "code": "def cv_capacity(self) -> int:\n\n        return len(self.m_cv) // self.m_cv_stride\n    \n    def knot_capacity(self) -> int:\n        return len(self.m_knot)\n    \n    #############################################################################\n    # CONTROL VERTEX ACCESS  \n    #############################################################################\n    \n    def get_cv(self, cv_index: int) -> Optional[Point]:\n        \"\"\"Get control point at index as Point\"\"\"\n        if cv_index < 0 or cv_index >= self.m_cv_count:\n            return None\n        \n        idx = cv_index * self.m_cv_stride\n        if self.m_is_rat:\n            w = self.m_cv[idx + self.m_dim]\n            if abs(w) < Tolerance.ZERO_TOLERANCE:\n                return Point(0, 0, 0)",
          "file": "nurbscurve.py"
        }
      }
    },
    {
      "name": "NurbsCurve.knot_capacity",
      "implementations": {
        "python": {
          "sig": "knot_capacity() -> int",
          "code": "def knot_capacity(self) -> int:\n\n        return len(self.m_knot)\n    \n    #############################################################################\n    # CONTROL VERTEX ACCESS  \n    #############################################################################\n    \n    def get_cv(self, cv_index: int) -> Optional[Point]:\n        \"\"\"Get control point at index as Point\"\"\"\n        if cv_index < 0 or cv_index >= self.m_cv_count:\n            return None\n        \n        idx = cv_index * self.m_cv_stride\n        if self.m_is_rat:\n            w = self.m_cv[idx + self.m_dim]\n            if abs(w) < Tolerance.ZERO_TOLERANCE:\n                return Point(0, 0, 0)\n            return Point(\n                self.m_cv[idx] / w if self.m_dim > 0 else 0,\n                self.m_cv[idx + 1] / w if self.m_dim > 1 else 0,",
          "file": "nurbscurve.py"
        }
      }
    },
    {
      "name": "NurbsCurve.get_cv",
      "implementations": {
        "python": {
          "sig": "get_cv(cv_index: int) -> Optional[Point]",
          "code": "def get_cv(self, cv_index: int) -> Optional[Point]:\n\n        \"\"\"Get control point at index as Point\"\"\"\n        if cv_index < 0 or cv_index >= self.m_cv_count:\n            return None\n        \n        idx = cv_index * self.m_cv_stride\n        if self.m_is_rat:\n            w = self.m_cv[idx + self.m_dim]\n            if abs(w) < Tolerance.ZERO_TOLERANCE:\n                return Point(0, 0, 0)\n            return Point(\n                self.m_cv[idx] / w if self.m_dim > 0 else 0,\n                self.m_cv[idx + 1] / w if self.m_dim > 1 else 0,\n                self.m_cv[idx + 2] / w if self.m_dim > 2 else 0\n            )\n        else:\n            return Point(\n                self.m_cv[idx] if self.m_dim > 0 else 0,\n                self.m_cv[idx + 1] if self.m_dim > 1 else 0,\n                self.m_cv[idx + 2] if self.m_dim > 2 else 0",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "Point get_cv(int cv_index)",
          "code": "Point NurbsCurve::get_cv(int cv_index) const {\n    const double* cv_ptr = cv(cv_index);\n    if (!cv_ptr) return Point(0, 0, 0);\n    \n    if (m_is_rat) {\n        double w = cv_ptr[m_dim];\n        if (w != 0.0) {\n            return Point(cv_ptr[0]/w, cv_ptr[1]/w, m_dim > 2 ? cv_ptr[2]/w : 0.0);\n        }",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "get_cv(index: usize) -> Option<Point>",
          "code": "pub fn get_cv(&self, index: usize) -> Option<Point> {\n        if index >= self.m_cv_count {\n            return None;\n        }\n\n        let idx = index * self.m_cv_stride;\n        let x = self.m_cv[idx];\n        let y = if self.m_dim > 1 { self.m_cv[idx + 1] } else { 0.0 };\n        let z = if self.m_dim > 2 { self.m_cv[idx + 2] } else { 0.0 };\n\n        Some(Point::new(x, y, z))\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.set_cv",
      "implementations": {
        "python": {
          "sig": "set_cv(cv_index: int, point: Point) -> bool",
          "code": "def set_cv(self, cv_index: int, point: Point) -> bool:\n\n        \"\"\"Set control point at index from Point\"\"\"\n        if cv_index < 0 or cv_index >= self.m_cv_count:\n            return False\n        \n        idx = cv_index * self.m_cv_stride\n        if self.m_dim > 0:\n            self.m_cv[idx] = point.x\n        if self.m_dim > 1:\n            self.m_cv[idx + 1] = point.y\n        if self.m_dim > 2:\n            self.m_cv[idx + 2] = point.z\n        \n        # Keep weight unchanged if rational\n        if self.m_is_rat:\n            w = self.m_cv[idx + self.m_dim]\n            if self.m_dim > 0:\n                self.m_cv[idx] *= w\n            if self.m_dim > 1:\n                self.m_cv[idx + 1] *= w",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool set_cv(int cv_index, const Point& point)",
          "code": "bool NurbsCurve::set_cv(int cv_index, const Point& point) {\n    double* cv_ptr = cv(cv_index);\n    if (!cv_ptr) return false;\n    \n    cv_ptr[0] = point[0];\n    if (m_dim > 1) cv_ptr[1] = point[1];\n    if (m_dim > 2) cv_ptr[2] = point[2];\n    if (m_is_rat) cv_ptr[m_dim] = 1.0;\n    \n    return true;\n}",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.get_cv_4d",
      "implementations": {
        "python": {
          "sig": "get_cv_4d(cv_index: int) -> Optional[Tuple[float, float, float, float]]",
          "code": "def get_cv_4d(self, cv_index: int) -> Optional[Tuple[float, float, float, float]]:\n\n        \"\"\"Get control point as homogeneous coordinates (x, y, z, w)\"\"\"\n        if cv_index < 0 or cv_index >= self.m_cv_count:\n            return None\n        \n        idx = cv_index * self.m_cv_stride\n        x = self.m_cv[idx] if self.m_dim > 0 else 0.0\n        y = self.m_cv[idx + 1] if self.m_dim > 1 else 0.0\n        z = self.m_cv[idx + 2] if self.m_dim > 2 else 0.0\n        w = self.m_cv[idx + self.m_dim] if self.m_is_rat else 1.0\n        \n        return (x, y, z, w)\n    \n    def set_cv_4d(self, cv_index: int, x: float, y: float, z: float, w: float) -> bool:\n        \"\"\"Set control point from homogeneous coordinates\"\"\"\n        if cv_index < 0 or cv_index >= self.m_cv_count:\n            return False\n        \n        idx = cv_index * self.m_cv_stride\n        if self.m_dim > 0:",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool get_cv_4d(int cv_index, double& x, double& y, double& z, double& w)",
          "code": "bool NurbsCurve::get_cv_4d(int cv_index, double& x, double& y, double& z, double& w) const {\n    const double* cv_ptr = cv(cv_index);\n    if (!cv_ptr) return false;\n    \n    x = cv_ptr[0];\n    y = m_dim > 1 ? cv_ptr[1] : 0.0;\n    z = m_dim > 2 ? cv_ptr[2] : 0.0;\n    w = m_is_rat ? cv_ptr[m_dim] : 1.0;\n    return true;\n}",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.set_cv_4d",
      "implementations": {
        "python": {
          "sig": "set_cv_4d(cv_index: int, x: float, y: float, z: float, w: float) -> bool",
          "code": "def set_cv_4d(self, cv_index: int, x: float, y: float, z: float, w: float) -> bool:\n\n        \"\"\"Set control point from homogeneous coordinates\"\"\"\n        if cv_index < 0 or cv_index >= self.m_cv_count:\n            return False\n        \n        idx = cv_index * self.m_cv_stride\n        if self.m_dim > 0:\n            self.m_cv[idx] = x\n        if self.m_dim > 1:\n            self.m_cv[idx + 1] = y\n        if self.m_dim > 2:\n            self.m_cv[idx + 2] = z\n        if self.m_is_rat:\n            self.m_cv[idx + self.m_dim] = w\n        \n        return True\n    \n    def weight(self, cv_index: int) -> float:\n        \"\"\"Get weight at control vertex index\"\"\"\n        if cv_index < 0 or cv_index >= self.m_cv_count:",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool set_cv_4d(int cv_index, double x, double y, double z, double w)",
          "code": "bool NurbsCurve::set_cv_4d(int cv_index, double x, double y, double z, double w) {\n    double* cv_ptr = cv(cv_index);\n    if (!cv_ptr) return false;\n    \n    if (m_is_rat) {\n        cv_ptr[0] = x;\n        if (m_dim > 1) cv_ptr[1] = y;\n        if (m_dim > 2) cv_ptr[2] = z;\n        cv_ptr[m_dim] = w;\n    }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.weight",
      "implementations": {
        "python": {
          "sig": "weight(cv_index: int) -> float",
          "code": "def weight(self, cv_index: int) -> float:\n\n        \"\"\"Get weight at control vertex index\"\"\"\n        if cv_index < 0 or cv_index >= self.m_cv_count:\n            return 1.0\n        \n        if not self.m_is_rat:\n            return 1.0\n        \n        idx = cv_index * self.m_cv_stride\n        return self.m_cv[idx + self.m_dim]\n    \n    def set_weight(self, cv_index: int, weight: float) -> bool:\n        \"\"\"Set weight at control vertex index\"\"\"\n        if cv_index < 0 or cv_index >= self.m_cv_count:\n            return False\n        \n        if not self.m_is_rat:\n            # Convert to rational if setting non-1 weight\n            if abs(weight - 1.0) > Tolerance.ZERO_TOLERANCE:\n                self.make_rational()",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "double weight(int cv_index)",
          "code": "double NurbsCurve::weight(int cv_index) const {\n    if (!m_is_rat) return 1.0;\n    const double* cv_ptr = cv(cv_index);\n    return cv_ptr ? cv_ptr[m_dim] : 1.0;\n}",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "weight(cv_index: usize) -> f64",
          "code": "pub fn weight(&self, cv_index: usize) -> f64 {\n        if !self.m_is_rat || cv_index >= self.m_cv_count {\n            return 1.0;\n        }\n        let idx = cv_index * self.m_cv_stride + self.m_dim;\n        self.m_cv[idx]\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.set_weight",
      "implementations": {
        "python": {
          "sig": "set_weight(cv_index: int, weight: float) -> bool",
          "code": "def set_weight(self, cv_index: int, weight: float) -> bool:\n\n        \"\"\"Set weight at control vertex index\"\"\"\n        if cv_index < 0 or cv_index >= self.m_cv_count:\n            return False\n        \n        if not self.m_is_rat:\n            # Convert to rational if setting non-1 weight\n            if abs(weight - 1.0) > Tolerance.ZERO_TOLERANCE:\n                self.make_rational()\n        \n        if self.m_is_rat:\n            idx = cv_index * self.m_cv_stride\n            old_w = self.m_cv[idx + self.m_dim]\n            \n            # Scale CVs by weight ratio\n            if abs(old_w) > Tolerance.ZERO_TOLERANCE:\n                ratio = weight / old_w\n                for i in range(self.m_dim):\n                    self.m_cv[idx + i] *= ratio",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool set_weight(int cv_index, double w)",
          "code": "bool NurbsCurve::set_weight(int cv_index, double w) {\n    if (!m_is_rat) {\n        if (!make_rational()) return false;\n    }",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "set_weight(cv_index: usize, weight: f64) -> bool",
          "code": "pub fn set_weight(&mut self, cv_index: usize, weight: f64) -> bool {\n        if cv_index >= self.m_cv_count {\n            return false;\n        }\n        if !self.m_is_rat {\n            // Would need to convert to rational - not implemented yet\n            return false;\n        }\n        let idx = cv_index * self.m_cv_stride + self.m_dim;\n        self.m_cv[idx] = weight;\n        true\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.knot",
      "implementations": {
        "python": {
          "sig": "knot(knot_index: int) -> float",
          "code": "def knot(self, knot_index: int) -> float:\n\n        \"\"\"Get knot value at index\"\"\"\n        if knot_index < 0 or knot_index >= len(self.m_knot):\n            return 0.0\n        return self.m_knot[knot_index]\n    \n    def set_knot(self, knot_index: int, knot_value: float) -> bool:\n        \"\"\"Set knot value at index\"\"\"\n        if knot_index < 0 or knot_index >= len(self.m_knot):\n            return False\n        self.m_knot[knot_index] = knot_value\n        return True\n    \n    def knot_multiplicity(self, knot_index: int) -> int:\n        \"\"\"Get knot multiplicity at index\"\"\"\n        if knot_index < 0 or knot_index >= len(self.m_knot):\n            return 0\n        \n        knot_value = self.m_knot[knot_index]\n        mult = 1",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "double knot(int knot_index)",
          "code": "double NurbsCurve::knot(int knot_index) const {\n    if (knot_index < 0 || knot_index >= static_cast<int>(m_knot.size())) {\n        return 0.0;\n    }",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "knot(knot_index: usize) -> Option<f64>",
          "code": "pub fn knot(&self, knot_index: usize) -> Option<f64> {\n        if knot_index >= self.m_knot.len() {\n            return None;\n        }\n        Some(self.m_knot[knot_index])\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.set_knot",
      "implementations": {
        "python": {
          "sig": "set_knot(knot_index: int, knot_value: float) -> bool",
          "code": "def set_knot(self, knot_index: int, knot_value: float) -> bool:\n\n        \"\"\"Set knot value at index\"\"\"\n        if knot_index < 0 or knot_index >= len(self.m_knot):\n            return False\n        self.m_knot[knot_index] = knot_value\n        return True\n    \n    def knot_multiplicity(self, knot_index: int) -> int:\n        \"\"\"Get knot multiplicity at index\"\"\"\n        if knot_index < 0 or knot_index >= len(self.m_knot):\n            return 0\n        \n        knot_value = self.m_knot[knot_index]\n        mult = 1\n        \n        # Count after\n        for i in range(knot_index + 1, len(self.m_knot)):\n            if abs(self.m_knot[i] - knot_value) < Tolerance.ZERO_TOLERANCE:\n                mult += 1\n            else:",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool set_knot(int knot_index, double knot_value)",
          "code": "bool NurbsCurve::set_knot(int knot_index, double knot_value) {\n    if (knot_index < 0 || knot_index >= static_cast<int>(m_knot.size())) {\n        return false;\n    }",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "set_knot(knot_index: usize, knot_value: f64) -> bool",
          "code": "pub fn set_knot(&mut self, knot_index: usize, knot_value: f64) -> bool {\n        if knot_index >= self.m_knot.len() {\n            return false;\n        }\n        self.m_knot[knot_index] = knot_value;\n        true\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.knot_multiplicity",
      "implementations": {
        "python": {
          "sig": "knot_multiplicity(knot_index: int) -> int",
          "code": "def knot_multiplicity(self, knot_index: int) -> int:\n\n        \"\"\"Get knot multiplicity at index\"\"\"\n        if knot_index < 0 or knot_index >= len(self.m_knot):\n            return 0\n        \n        knot_value = self.m_knot[knot_index]\n        mult = 1\n        \n        # Count after\n        for i in range(knot_index + 1, len(self.m_knot)):\n            if abs(self.m_knot[i] - knot_value) < Tolerance.ZERO_TOLERANCE:\n                mult += 1\n            else:\n                break\n        \n        # Count before\n        for i in range(knot_index - 1, -1, -1):\n            if abs(self.m_knot[i] - knot_value) < Tolerance.ZERO_TOLERANCE:\n                mult += 1\n            else:",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "int knot_multiplicity(int knot_index)",
          "code": "int NurbsCurve::knot_multiplicity(int knot_index) const {\n    if (knot_index < 0 || knot_index >= knot_count()) return 0;\n    \n    double knot_value = m_knot[knot_index];\n    int mult = 1;\n    \n    // Count knots equal to this value after current index\n    for (int i = knot_index + 1; i < knot_count(); i++) {\n        if (std::abs(m_knot[i] - knot_value) < Tolerance::ZERO_TOLERANCE) {\n            mult++;\n        }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.get_knots",
      "implementations": {
        "python": {
          "sig": "get_knots() -> np.ndarray",
          "code": "def get_knots(self) -> np.ndarray:\n\n        \"\"\"Get all knot values\"\"\"\n        return self.m_knot.copy()\n    \n    def knot_array(self) -> np.ndarray:\n        \"\"\"Get pointer to knot array\"\"\"\n        return self.m_knot\n    \n    def cv_array(self) -> np.ndarray:\n        \"\"\"Get pointer to CV array\"\"\"\n        return self.m_cv\n    \n    def is_valid_knot_vector(self) -> bool:\n        \"\"\"Check if knot vector is valid\"\"\"\n        if len(self.m_knot) != self.knot_count():\n            return False\n        \n        for i in range(len(self.m_knot) - 1):\n            if self.m_knot[i] > self.m_knot[i + 1] + Tolerance.ZERO_TOLERANCE:\n                return False",
          "file": "nurbscurve.py"
        },
        "rust": {
          "sig": "get_knots() -> Vec<f64>",
          "code": "pub fn get_knots(&self) -> Vec<f64> {\n        self.m_knot.clone()\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.knot_array",
      "implementations": {
        "python": {
          "sig": "knot_array() -> np.ndarray",
          "code": "def knot_array(self) -> np.ndarray:\n\n        \"\"\"Get pointer to knot array\"\"\"\n        return self.m_knot\n    \n    def cv_array(self) -> np.ndarray:\n        \"\"\"Get pointer to CV array\"\"\"\n        return self.m_cv\n    \n    def is_valid_knot_vector(self) -> bool:\n        \"\"\"Check if knot vector is valid\"\"\"\n        if len(self.m_knot) != self.knot_count():\n            return False\n        \n        for i in range(len(self.m_knot) - 1):\n            if self.m_knot[i] > self.m_knot[i + 1] + Tolerance.ZERO_TOLERANCE:\n                return False\n        \n        return True\n    \n    #############################################################################",
          "file": "nurbscurve.py"
        },
        "rust": {
          "sig": "knot_array() -> &[f64]",
          "code": "pub fn knot_array(&self) -> &[f64] {\n        &self.m_knot\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.cv_array",
      "implementations": {
        "python": {
          "sig": "cv_array() -> np.ndarray",
          "code": "def cv_array(self) -> np.ndarray:\n\n        \"\"\"Get pointer to CV array\"\"\"\n        return self.m_cv\n    \n    def is_valid_knot_vector(self) -> bool:\n        \"\"\"Check if knot vector is valid\"\"\"\n        if len(self.m_knot) != self.knot_count():\n            return False\n        \n        for i in range(len(self.m_knot) - 1):\n            if self.m_knot[i] > self.m_knot[i + 1] + Tolerance.ZERO_TOLERANCE:\n                return False\n        \n        return True\n    \n    #############################################################################\n    # DOMAIN & PARAMETERIZATION\n    #############################################################################\n    \n    def domain(self) -> Tuple[float, float]:",
          "file": "nurbscurve.py"
        },
        "rust": {
          "sig": "cv_array() -> &[f64]",
          "code": "pub fn cv_array(&self) -> &[f64] {\n        &self.m_cv\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.is_valid_knot_vector",
      "implementations": {
        "python": {
          "sig": "is_valid_knot_vector() -> bool",
          "code": "def is_valid_knot_vector(self) -> bool:\n\n        \"\"\"Check if knot vector is valid\"\"\"\n        if len(self.m_knot) != self.knot_count():\n            return False\n        \n        for i in range(len(self.m_knot) - 1):\n            if self.m_knot[i] > self.m_knot[i + 1] + Tolerance.ZERO_TOLERANCE:\n                return False\n        \n        return True\n    \n    #############################################################################\n    # DOMAIN & PARAMETERIZATION\n    #############################################################################\n    \n    def domain(self) -> Tuple[float, float]:\n        \"\"\"Get curve domain [start_param, end_param]\"\"\"\n        if not self.is_valid():\n            return (0.0, 0.0)\n        return (self.m_knot[self.m_order - 2], self.m_knot[self.m_cv_count - 1])",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool is_valid_knot_vector()",
          "code": "bool NurbsCurve::is_valid_knot_vector() const {\n    int kc = knot_count();\n    if (static_cast<int>(m_knot.size()) != kc) return false;\n    \n    // Check for non-decreasing knot values\n    for (int i = 1; i < kc; i++) {\n        if (m_knot[i] < m_knot[i-1]) return false;\n    }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.domain",
      "implementations": {
        "python": {
          "sig": "domain() -> Tuple[float, float]",
          "code": "def domain(self) -> Tuple[float, float]:\n\n        \"\"\"Get curve domain [start_param, end_param]\"\"\"\n        if not self.is_valid():\n            return (0.0, 0.0)\n        return (self.m_knot[self.m_order - 2], self.m_knot[self.m_cv_count - 1])\n    \n    def set_domain(self, t0: float, t1: float) -> bool:\n        \"\"\"Set curve domain\"\"\"\n        if not self.is_valid():\n            return False\n        if t0 >= t1:\n            return False\n        \n        old_t0, old_t1 = self.domain()\n        if abs(old_t1 - old_t0) < Tolerance.ZERO_TOLERANCE:\n            return False\n        \n        # Linear remap of knots\n        scale = (t1 - t0) / (old_t1 - old_t0)\n        for i in range(len(self.m_knot)):",
          "file": "nurbscurve.py"
        },
        "rust": {
          "sig": "domain() -> (f64, f64)",
          "code": "pub fn domain(&self) -> (f64, f64) {\n        if !self.is_valid() {\n            return (0.0, 0.0);\n        }\n        let t0 = self.m_knot[self.m_order - 2];\n        let t1 = self.m_knot[self.m_cv_count - 1];\n        (t0, t1)\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.set_domain",
      "implementations": {
        "python": {
          "sig": "set_domain(t0: float, t1: float) -> bool",
          "code": "def set_domain(self, t0: float, t1: float) -> bool:\n\n        \"\"\"Set curve domain\"\"\"\n        if not self.is_valid():\n            return False\n        if t0 >= t1:\n            return False\n        \n        old_t0, old_t1 = self.domain()\n        if abs(old_t1 - old_t0) < Tolerance.ZERO_TOLERANCE:\n            return False\n        \n        # Linear remap of knots\n        scale = (t1 - t0) / (old_t1 - old_t0)\n        for i in range(len(self.m_knot)):\n            self.m_knot[i] = t0 + (self.m_knot[i] - old_t0) * scale\n        \n        return True\n    \n    def get_span_vector(self) -> List[float]:\n        \"\"\"Get span (distinct knot intervals) values\"\"\"",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool set_domain(double t0, double t1)",
          "code": "bool NurbsCurve::set_domain(double t0, double t1) {\n    if (t0 >= t1 || !is_valid()) return false;\n    \n    auto [d0, d1] = domain();\n    if (d0 >= d1) return false;\n    \n    double scale = (t1 - t0) / (d1 - d0);\n    \n    for (auto& k : m_knot) {\n        k = t0 + (k - d0) * scale;\n    }",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "set_domain(t0: f64, t1: f64) -> bool",
          "code": "pub fn set_domain(&mut self, t0: f64, t1: f64) -> bool {\n        if !self.is_valid() || t0 >= t1 {\n            return false;\n        }\n        \n        let (old_t0, old_t1) = self.domain();\n        if (old_t0 - old_t1).abs() < 1e-14 {\n            return false;\n        }\n        \n        let scale = (t1 - t0) / (old_t1 - old_t0);\n        \n        // Reparameterize knots\n        for i in 0..self.m_knot.len() {\n            self.m_knot[i] = t0 + (self.m_knot[i] - old_t0) * scale;\n        }\n        \n        true\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.get_span_vector",
      "implementations": {
        "python": {
          "sig": "get_span_vector() -> List[float]",
          "code": "def get_span_vector(self) -> List[float]:\n\n        \"\"\"Get span (distinct knot intervals) values\"\"\"\n        if not self.is_valid():\n            return []\n        \n        spans = []\n        for i in range(self.m_order - 2, self.m_cv_count):\n            if i == self.m_order - 2 or abs(self.m_knot[i] - self.m_knot[i-1]) > Tolerance.ZERO_TOLERANCE:\n                spans.append(self.m_knot[i])\n        \n        return spans\n\n    #############################################################################\n    # KNOT VECTOR OPERATIONS (CONTINUED)\n    #############################################################################\n    \n    def make_clamped_uniform_knot_vector(self, delta: float = 1.0) -> bool:\n        \"\"\"Make knot vector a clamped uniform knot vector.\n        \n        Implementation matches OpenNURBS ON_MakeClampedUniformKnotVector.",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "std::vector<double> get_span_vector()",
          "code": "std::vector<double> NurbsCurve::get_span_vector() const {\n    std::vector<double> spans;\n    spans.push_back(m_knot[m_order-2]);\n    \n    for (int i = m_order - 1; i < m_cv_count; i++) {\n        if (m_knot[i] > spans.back()) {\n            spans.push_back(m_knot[i]);\n        }",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "get_span_vector() -> Vec<f64>",
          "code": "pub fn get_span_vector(&self) -> Vec<f64> {\n        let mut spans = Vec::new();\n        if !self.is_valid() {\n            return spans;\n        }\n\n        let offset = self.m_order - 2;\n        spans.push(self.m_knot[offset]);\n\n        for i in (offset + 1)..self.m_cv_count {\n            if i == offset || (self.m_knot[i] - self.m_knot[i - 1]).abs() > Tolerance::ZERO_TOLERANCE {\n                spans.push(self.m_knot[i]);\n            }\n        }\n\n        spans\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.make_clamped_uniform_knot_vector",
      "implementations": {
        "python": {
          "sig": "make_clamped_uniform_knot_vector(delta: float = 1.0) -> bool",
          "code": "def make_clamped_uniform_knot_vector(self, delta: float = 1.0) -> bool:\n\n        \"\"\"Make knot vector a clamped uniform knot vector.\n        \n        Implementation matches OpenNURBS ON_MakeClampedUniformKnotVector.\n        \"\"\"\n        if delta <= 0.0:\n            return False\n        if self.m_order < 2 or self.m_cv_count < self.m_order:\n            return False\n        \n        knot_count = self.m_order + self.m_cv_count - 2\n        self.m_knot = np.zeros(knot_count, dtype=np.float64)\n        \n        # Fill interior knots with uniform spacing\n        # Start from index (order-2) up to (cv_count-1)\n        k = 0.0\n        for i in range(self.m_order - 2, self.m_cv_count):\n            self.m_knot[i] = k\n            k += delta",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool make_clamped_uniform_knot_vector(double delta)",
          "code": "bool NurbsCurve::make_clamped_uniform_knot_vector(double delta) {\n    // Don't call is_valid() here as it checks knot vector which we're about to create\n    if (delta <= 0.0) return false;\n    if (m_dim <= 0) return false;\n    if (m_order < 2 || m_cv_count < m_order) return false;\n    \n    int knot_count = m_order + m_cv_count - 2;\n    m_knot.resize(knot_count);\n    \n    // Create clamped uniform knot vector\n    // Fill interior knots with uniform spacing starting from index (order-2)\n    double k = 0.0;\n    for (int i = m_order - 2; i < m_cv_count; i++, k += delta) {\n        m_knot[i] = k;\n    }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.make_periodic_uniform_knot_vector",
      "implementations": {
        "python": {
          "sig": "make_periodic_uniform_knot_vector(delta: float = 1.0) -> bool",
          "code": "def make_periodic_uniform_knot_vector(self, delta: float = 1.0) -> bool:\n\n        \"\"\"Make knot vector a periodic uniform knot vector\"\"\"\n        if delta <= 0.0:\n            return False\n        if self.m_order < 2 or self.m_cv_count < self.m_order:\n            return False\n        \n        knot_count = self.m_order + self.m_cv_count - 2\n        self.m_knot = np.zeros(knot_count, dtype=np.float64)\n        \n        # All knots equally spaced\n        for i in range(knot_count):\n            self.m_knot[i] = i * delta\n        \n        return True\n    \n    #############################################################################\n    # EVALUATION\n    #############################################################################",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool make_periodic_uniform_knot_vector(double delta)",
          "code": "bool NurbsCurve::make_periodic_uniform_knot_vector(double delta) {\n    // Don't call is_valid() here as it checks knot vector which we're about to create\n    if (delta <= 0.0) return false;\n    if (m_dim <= 0) return false;\n    if (m_order < 2 || m_cv_count < m_order) return false;\n    \n    int knot_count = m_order + m_cv_count - 2;\n    m_knot.resize(knot_count);\n    \n    // Create periodic uniform knot vector\n    // All knots are distinct and equally spaced\n    for (int i = 0; i < knot_count; i++) {\n        m_knot[i] = i * delta;\n    }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.point_at",
      "implementations": {
        "python": {
          "sig": "point_at(t: float) -> Point",
          "code": "def point_at(self, t: float) -> Point:\n\n        \"\"\"Evaluate point at parameter t.\n        \n        Implementation matches OpenNURBS evaluation approach.\n        \"\"\"\n        if not self.is_valid():\n            return Point(0, 0, 0)\n        \n        # Find span (returns index relative to shifted knot array)\n        span = self._find_span(t)\n        if span < 0:\n            return Point(0, 0, 0)\n        \n        # Evaluate using Cox-de Boor algorithm\n        N = self._basis_functions(span, t)\n        \n        # Compute point\n        pt = np.zeros(self.m_dim)\n        \n        if self.m_is_rat:",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "Point point_at(double t)",
          "code": "Point NurbsCurve::point_at(double t) const {\n    if (!is_valid()) return Point(0, 0, 0);\n    \n    // find_span returns index relative to shifted knot array\n    int span = find_span(t);\n    std::vector<double> basis;\n    basis_functions(span, t, basis);\n    \n    double x = 0.0, y = 0.0, z = 0.0, w = 0.0;\n    \n    // In OpenNURBS, span index directly corresponds to CV starting index\n    for (int i = 0; i < m_order; i++) {\n        int cv_idx = span + i;\n        const double* cv_ptr = cv(cv_idx);\n        if (!cv_ptr) continue;\n        \n        double N = basis[i];\n        if (m_is_rat) {\n            double ww = cv_ptr[m_dim];\n            x += N * cv_ptr[0];\n            y += N * (m_dim > 1 ? cv_ptr[1] : 0.0);\n            z += N * (m_dim > 2 ? cv_ptr[2] : 0.0);\n            w += N * ww;\n        }",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "point_at(t: f64) -> Point",
          "code": "pub fn point_at(&self, t: f64) -> Point {\n        if !self.is_valid() {\n            return Point::new(0.0, 0.0, 0.0);\n        }\n\n        // Find span (returns index relative to shifted knot array)\n        let span = self.find_span(t);\n\n        // Evaluate using Cox-de Boor algorithm\n        let basis = self.basis_functions(span, t);\n\n        // Compute point\n        let mut x = 0.0;\n        let mut y = 0.0;\n        let mut z = 0.0;\n        let mut w = 0.0;\n\n        // In OpenNURBS, span index directly corresponds to CV starting index\n        for i in 0..self.m_order {\n            let cv_idx = span + i;\n            if cv_idx >= self.m_cv_count {\n                continue;\n            }\n\n            let idx = cv_idx * self.m_cv_stride;\n            let n = basis[i];\n\n            if self.m_",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.point_at_start",
      "implementations": {
        "python": {
          "sig": "point_at_start() -> Point",
          "code": "def point_at_start(self) -> Point:\n\n        \"\"\"Evaluate point at curve start\"\"\"\n        t0, _ = self.domain()\n        return self.point_at(t0)\n    \n    def point_at_end(self) -> Point:\n        \"\"\"Evaluate point at curve end\"\"\"\n        _, t1 = self.domain()\n        return self.point_at(t1)\n    \n    def tangent_at(self, t: float) -> Vector:\n        \"\"\"Evaluate tangent vector at parameter t\"\"\"\n        if not self.is_valid():\n            return Vector(0, 0, 0)\n        \n        # Use finite differences for simplicity\n        eps = 1e-8\n        p1 = self.point_at(t - eps)\n        p2 = self.point_at(t + eps)",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "Point point_at_start()",
          "code": "Point NurbsCurve::point_at_start() const {\n    auto [t0, t1] = domain();\n    return point_at(t0);\n}",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "point_at_start() -> Point",
          "code": "pub fn point_at_start(&self) -> Point {\n        let (t0, _) = self.domain();\n        self.point_at(t0)\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.point_at_end",
      "implementations": {
        "python": {
          "sig": "point_at_end() -> Point",
          "code": "def point_at_end(self) -> Point:\n\n        \"\"\"Evaluate point at curve end\"\"\"\n        _, t1 = self.domain()\n        return self.point_at(t1)\n    \n    def tangent_at(self, t: float) -> Vector:\n        \"\"\"Evaluate tangent vector at parameter t\"\"\"\n        if not self.is_valid():\n            return Vector(0, 0, 0)\n        \n        # Use finite differences for simplicity\n        eps = 1e-8\n        p1 = self.point_at(t - eps)\n        p2 = self.point_at(t + eps)\n        \n        return Vector(\n            (p2.x - p1.x) / (2 * eps),\n            (p2.y - p1.y) / (2 * eps),\n            (p2.z - p1.z) / (2 * eps)\n        )",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "Point point_at_end()",
          "code": "Point NurbsCurve::point_at_end() const {\n    auto [t0, t1] = domain();\n    return point_at(t1);\n}",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "point_at_end() -> Point",
          "code": "pub fn point_at_end(&self) -> Point {\n        let (_, t1) = self.domain();\n        self.point_at(t1)\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.tangent_at",
      "implementations": {
        "python": {
          "sig": "tangent_at(t: float) -> Vector",
          "code": "def tangent_at(self, t: float) -> Vector:\n\n        \"\"\"Evaluate tangent vector at parameter t\"\"\"\n        if not self.is_valid():\n            return Vector(0, 0, 0)\n        \n        # Use finite differences for simplicity\n        eps = 1e-8\n        p1 = self.point_at(t - eps)\n        p2 = self.point_at(t + eps)\n        \n        return Vector(\n            (p2.x - p1.x) / (2 * eps),\n            (p2.y - p1.y) / (2 * eps),\n            (p2.z - p1.z) / (2 * eps)\n        )\n    \n    def _find_span(self, t: float) -> int:\n        \"\"\"Find knot span index for parameter t using binary search.\n        \n        Implementation matches OpenNURBS ON_NurbsSpanIndex.",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "Vector tangent_at(double t)",
          "code": "Vector NurbsCurve::tangent_at(double t) const {\n    auto ders = evaluate(t, 1);\n    if (ders.size() < 2) {\n        return Vector(0, 0, 0);\n    }",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "tangent_at(t: f64) -> Vector",
          "code": "pub fn tangent_at(&self, t: f64) -> Vector {\n        if !self.is_valid() {\n            return Vector::new(0.0, 0.0, 0.0);\n        }\n\n        // Use numerical differentiation for simplicity\n        let (t0, t1) = self.domain();\n        let eps = (t1 - t0) * 1e-8;\n        \n        let p1 = self.point_at((t - eps).max(t0));\n        let p2 = self.point_at((t + eps).min(t1));\n        \n        let tangent = Vector::new(\n            (p2[0] - p1[0]) / (2.0 * eps),\n            (p2[1] - p1[1]) / (2.0 * eps),\n            (p2[2] - p1[2]) / (2.0 * eps),\n        );\n        \n        // Normalize\n        tangent.normalized()\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve._find_span",
      "implementations": {
        "python": {
          "sig": "_find_span(t: float) -> int",
          "code": "def _find_span(self, t: float) -> int:\n\n        \"\"\"Find knot span index for parameter t using binary search.\n        \n        Implementation matches OpenNURBS ON_NurbsSpanIndex.\n        OpenNURBS shifts knot pointer by (order-2) to work with compressed format.\n        Domain is knot[order-2] to knot[cv_count-1].\n        \n        Returns\n        -------\n        int\n            Span index relative to shifted knot array (0-based from domain start)\n        \"\"\"\n        if not self.is_valid():\n            return -1\n        \n        # OpenNURBS shifts knot pointer by (order-2) to work with compressed format\n        # Domain is knot[order-2] to knot[cv_count-1]\n        offset = self.m_order - 2\n        knot_len = self.m_cv_count - self.m_order + 2",
          "file": "nurbscurve.py"
        }
      }
    },
    {
      "name": "NurbsCurve._basis_functions",
      "implementations": {
        "python": {
          "sig": "_basis_functions(span: int, t: float) -> np.ndarray",
          "code": "def _basis_functions(self, span: int, t: float) -> np.ndarray:\n\n        \"\"\"Compute non-zero basis functions at parameter t.\n        \n        Implementation matches OpenNURBS Cox-de Boor algorithm.\n        \n        Parameters\n        ----------\n        span : int\n            Knot span index from _find_span() (relative to shifted array).\n        t : float\n            Parameter value.\n            \n        Returns\n        -------\n        np.ndarray\n            Array of m_order non-zero basis function values.\n        \"\"\"\n        N = np.zeros(self.m_order)\n        left = np.zeros(self.m_order)\n        right = np.zeros(self.m_order)",
          "file": "nurbscurve.py"
        }
      }
    },
    {
      "name": "NurbsCurve.is_closed",
      "implementations": {
        "python": {
          "sig": "is_closed() -> bool",
          "code": "def is_closed(self) -> bool:\n\n        \"\"\"Check if curve is closed\"\"\"\n        if not self.is_valid():\n            return False\n        \n        p_start = self.point_at_start()\n        p_end = self.point_at_end()\n        return p_start.distance(p_end) < Tolerance.ZERO_TOLERANCE\n    \n    def is_periodic(self) -> bool:\n        \"\"\"Check if curve is periodic\"\"\"\n        if not self.is_valid():\n            return False\n        \n        # Check if knots and CVs wrap around\n        if not self.is_closed():\n            return False\n        \n        # Check if first order-1 CVs match last order-1 CVs\n        for i in range(self.m_order - 1):",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool is_closed()",
          "code": "bool NurbsCurve::is_closed() const {\n    if (!is_valid()) return false;\n    Point p0 = point_at_start();\n    Point p1 = point_at_end();\n    return p0.distance(p1) < Tolerance::ZERO_TOLERANCE;\n}",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "is_closed() -> bool",
          "code": "pub fn is_closed(&self) -> bool {\n        if !self.is_valid() {\n            return false;\n        }\n        \n        let start = self.point_at_start();\n        let end = self.point_at_end();\n        \n        start.distance(&end, None) < Tolerance::ZERO_TOLERANCE\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.is_periodic",
      "implementations": {
        "python": {
          "sig": "is_periodic() -> bool",
          "code": "def is_periodic(self) -> bool:\n\n        \"\"\"Check if curve is periodic\"\"\"\n        if not self.is_valid():\n            return False\n        \n        # Check if knots and CVs wrap around\n        if not self.is_closed():\n            return False\n        \n        # Check if first order-1 CVs match last order-1 CVs\n        for i in range(self.m_order - 1):\n            p1 = self.get_cv(i)\n            p2 = self.get_cv(self.m_cv_count - self.m_order + 1 + i)\n            if p1 and p2 and p1.distance(p2) > Tolerance.ZERO_TOLERANCE:\n                return False\n        \n        return True\n    \n    def length(self) -> float:\n        \"\"\"Compute curve length\"\"\"",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool is_periodic()",
          "code": "bool NurbsCurve::is_periodic() const {\n    if (m_order < 2) return false;\n    \n    // Check if last degree CVs match first degree CVs\n    int deg = degree();\n    for (int i = 0; i < deg; i++) {\n        Point p0 = get_cv(i);\n        Point p1 = get_cv(m_cv_count - deg + i);\n        if (p0.distance(p1) > Tolerance::ZERO_TOLERANCE) {\n            return false;\n        }",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "is_periodic() -> bool",
          "code": "pub fn is_periodic(&self) -> bool {\n        // For now, return false - full implementation would check\n        // if the curve is clamped and if removing end knots makes it periodic\n        false\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.length",
      "implementations": {
        "python": {
          "sig": "length() -> float",
          "code": "def length(self) -> float:\n\n        \"\"\"Compute curve length\"\"\"\n        if not self.is_valid():\n            return 0.0\n        \n        t0, t1 = self.domain()\n        num_samples = max(100, self.m_cv_count * 10)\n        dt = (t1 - t0) / num_samples\n        \n        total_length = 0.0\n        p_prev = self.point_at(t0)\n        \n        for i in range(1, num_samples + 1):\n            t = t0 + i * dt\n            p_curr = self.point_at(t)\n            total_length += p_prev.distance(p_curr)\n            p_prev = p_curr\n        \n        return total_length",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "double length(double tolerance)",
          "code": "double NurbsCurve::length(double tolerance) const {\n    if (!is_valid()) return 0.0;\n    \n    auto [t0, t1] = domain();\n    \n    // Adaptive sampling based on tolerance\n    // Smaller tolerance = more samples for better accuracy\n    int num_samples = std::max(50, static_cast<int>(100.0 / (tolerance + 1e-10)));\n    num_samples = std::min(num_samples, 1000); // Cap at 1000 samples\n    \n    double dt = (t1 - t0) / num_samples;\n    double total_length = 0.0;\n    \n    Point prev = point_at(t0);\n    for (int i = 1; i <= num_samples; i++) {\n        Point curr = point_at(t0 + i * dt);\n        total_length += prev.distance(curr);\n        prev = curr;\n    }",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "length(tolerance: Option<f64>) -> f64",
          "code": "pub fn length(&self, tolerance: Option<f64>) -> f64 {\n        if !self.is_valid() {\n            return 0.0;\n        }\n\n        let tol = tolerance.unwrap_or(1e-6);\n        let (t0, t1) = self.domain();\n        \n        // Use adaptive Simpson's rule for length computation\n        self.length_adaptive(t0, t1, tol)\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.make_rational",
      "implementations": {
        "python": {
          "sig": "make_rational() -> bool",
          "code": "def make_rational(self) -> bool:\n\n        \"\"\"Convert to rational curve\"\"\"\n        if self.m_is_rat:\n            return True\n        \n        new_stride = self.m_dim + 1\n        new_cv = np.zeros(self.m_cv_count * new_stride)\n        \n        for i in range(self.m_cv_count):\n            old_idx = i * self.m_cv_stride\n            new_idx = i * new_stride\n            \n            for j in range(self.m_dim):\n                new_cv[new_idx + j] = self.m_cv[old_idx + j]\n            new_cv[new_idx + self.m_dim] = 1.0  # Weight\n        \n        self.m_is_rat = 1\n        self.m_cv_stride = new_stride\n        self.m_cv = new_cv",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool make_rational()",
          "code": "bool NurbsCurve::make_rational() {\n    if (m_is_rat) return true;\n    \n    int new_stride = m_dim + 1;\n    std::vector<double> new_cv(m_cv_count * new_stride);\n    \n    for (int i = 0; i < m_cv_count; i++) {\n        const double* old_cv = cv(i);\n        double* new_cv_ptr = &new_cv[i * new_stride];\n        for (int j = 0; j < m_dim; j++) {\n            new_cv_ptr[j] = old_cv[j];\n        }",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "make_rational() -> bool",
          "code": "pub fn make_rational(&mut self) -> bool {\n        if self.m_is_rat {\n            return true; // Already rational\n        }\n        if !self.is_valid() {\n            return false;\n        }\n\n        // Create new CV array with weights\n        let new_stride = self.m_dim + 1;\n        let mut new_cv = vec![0.0; self.m_cv_count * new_stride];\n        \n        for i in 0..self.m_cv_count {\n            let old_idx = i * self.m_cv_stride;\n            let new_idx = i * new_stride;\n            \n            // Copy coordinates\n            for j in 0..self.m_dim {\n                new_cv[new_idx + j] = self.m_cv[old_idx + j];\n            }\n            // Set weight to 1.0\n            new_cv[new_idx + self.m_dim] = 1.0;\n        }\n\n        self.m_cv = new_cv;\n        self.m_cv_stride = new_stride;",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.make_non_rational",
      "implementations": {
        "python": {
          "sig": "make_non_rational() -> bool",
          "code": "def make_non_rational(self) -> bool:\n\n        \"\"\"Convert to non-rational curve\"\"\"\n        if not self.m_is_rat:\n            return True\n        \n        new_stride = self.m_dim\n        new_cv = np.zeros(self.m_cv_count * new_stride)\n        \n        for i in range(self.m_cv_count):\n            old_idx = i * self.m_cv_stride\n            new_idx = i * new_stride\n            w = self.m_cv[old_idx + self.m_dim]\n            \n            if abs(w) > Tolerance.ZERO_TOLERANCE:\n                for j in range(self.m_dim):\n                    new_cv[new_idx + j] = self.m_cv[old_idx + j] / w\n            else:\n                for j in range(self.m_dim):\n                    new_cv[new_idx + j] = self.m_cv[old_idx + j]",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool make_non_rational()",
          "code": "bool NurbsCurve::make_non_rational() {\n    if (!m_is_rat) return true;\n    \n    // Check if all weights are equal\n    double w0 = weight(0);\n    for (int i = 1; i < m_cv_count; i++) {\n        if (std::abs(weight(i) - w0) > Tolerance::ZERO_TOLERANCE) {\n            return false;\n        }",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "make_non_rational() -> bool",
          "code": "pub fn make_non_rational(&mut self) -> bool {\n        if !self.m_is_rat {\n            return true; // Already non-rational\n        }\n        if !self.is_valid() {\n            return false;\n        }\n\n        // Check if all weights are 1.0\n        for i in 0..self.m_cv_count {\n            let w = self.weight(i);\n            if (w - 1.0).abs() > Tolerance::ZERO_TOLERANCE {\n                return false; // Cannot make non-rational\n            }\n        }\n\n        // Create new CV array without weights\n        let new_stride = self.m_dim;\n        let mut new_cv = vec![0.0; self.m_cv_count * new_stride];\n        \n        for i in 0..self.m_cv_count {\n            let old_idx = i * self.m_cv_stride;\n            let new_idx = i * new_stride;\n            \n            // Copy coordinates only",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.reverse",
      "implementations": {
        "python": {
          "sig": "reverse() -> bool",
          "code": "def reverse(self) -> bool:\n\n        \"\"\"Reverse curve direction\"\"\"\n        if not self.is_valid():\n            return False\n        \n        # Reverse knots\n        t0, t1 = self.domain()\n        for i in range(len(self.m_knot)):\n            self.m_knot[i] = t0 + t1 - self.m_knot[i]\n        self.m_knot = np.flip(self.m_knot).copy()\n        \n        # Reverse CVs\n        cvs = self.cv_size()\n        for i in range(self.m_cv_count // 2):\n            j = self.m_cv_count - 1 - i\n            for k in range(cvs):\n                temp = self.m_cv[i * cvs + k]\n                self.m_cv[i * cvs + k] = self.m_cv[j * cvs + k]\n                self.m_cv[j * cvs + k] = temp",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool reverse()",
          "code": "bool NurbsCurve::reverse() {\n    if (!is_valid()) return false;\n    \n    // Reverse knots\n    auto [d0, d1] = domain();\n    for (auto& k : m_knot) {\n        k = d0 + d1 - k;\n    }",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "reverse() -> bool",
          "code": "pub fn reverse(&mut self) -> bool {\n        if !self.is_valid() {\n            return false;\n        }\n\n        // Reverse control points\n        let mut temp_cv = vec![0.0; self.m_cv_stride];\n        for i in 0..(self.m_cv_count / 2) {\n            let j = self.m_cv_count - 1 - i;\n            \n            // Swap CVs\n            for k in 0..self.m_cv_stride {\n                temp_cv[k] = self.m_cv[i * self.m_cv_stride + k];\n                self.m_cv[i * self.m_cv_stride + k] = self.m_cv[j * self.m_cv_stride + k];\n                self.m_cv[j * self.m_cv_stride + k] = temp_cv[k];\n            }\n        }\n\n        // Reverse and negate knots\n        let (t0, t1) = self.domain();\n        let knot_count = self.m_knot.len();\n        for i in 0..(knot_count / 2) {\n            let j = knot_count",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.intersect_plane",
      "implementations": {
        "python": {
          "sig": "intersect_plane(plane: Plane, tolerance: float = None) -> List[float]",
          "code": "def intersect_plane(self, plane: Plane, tolerance: float = None) -> List[float]:\n\n        \"\"\"Find all intersections between curve and plane (standard method).\n        \n        Implementation matches C++ version with endpoint checking.\n        \"\"\"\n        if tolerance is None:\n            tolerance = Tolerance.ZERO_TOLERANCE\n        \n        if not self.is_valid():\n            return []\n        \n        def signed_distance(p: Point) -> float:\n            \"\"\"Signed distance from point to plane\"\"\"\n            v = Vector(p.x - plane.origin.x, p.y - plane.origin.y, p.z - plane.origin.z)\n            return v.dot(plane.z_axis)\n        \n        results = []\n        t_start, t_end = self.domain()\n        \n        # Get span parameters for better subdivision",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "std::vector<double> intersect_plane(const Plane& plane, double tolerance)",
          "code": "std::vector<double> NurbsCurve::intersect_plane(const Plane& plane, double tolerance) const {\n    std::vector<double> intersections;\n    \n    if (!is_valid()) return intersections;\n    if (tolerance <= 0.0) tolerance = Tolerance::ZERO_TOLERANCE;\n    \n    auto [t_start, t_end] = domain();\n    \n    // Subdivide curve into spans and look for sign changes\n    std::vector<double> span_params = get_span_vector();\n    \n    for (size_t i = 0; i < span_params.size() - 1; i++) {\n        double t0 = span_params[i];\n        double t1 = span_params[i + 1];\n        \n        // Skip zero-length spans\n        if (std::abs(t1 - t0) < tolerance) continue;\n        \n        // Check for sign change (intersection) in this span\n        double d0 = signed_distance_to_plane(point_at(t0), plane);\n        double d1",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "intersect_plane(plane: &Plane, tolerance: Option<f64>) -> Vec<f64>",
          "code": "pub fn intersect_plane(&self, plane: &Plane, tolerance: Option<f64>) -> Vec<f64> {\n        let tol = tolerance.unwrap_or(Tolerance::ZERO_TOLERANCE);\n        let mut results = Vec::new();\n\n        if !self.is_valid() {\n            return results;\n        }\n\n        let signed_distance = |p: &Point| -> f64 {\n            let v = Vector::new(\n                p[0] - plane.origin()[0],\n                p[1] - plane.origin()[1],\n                p[2] - plane.origin()[2],\n            );\n            v.dot(&plane.z_axis())\n        };\n\n        let (_t_start, t_end) = self.domain();\n        let span_params = self.get_span_vector();\n\n        // Check each span for intersections\n        for i in 0..(span_params.len() - 1) {\n            let t0 = span_params[i];\n            let t1 = span_params[i + 1];",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.signed_distance",
      "implementations": {
        "python": {
          "sig": "signed_distance(p: Point) -> float",
          "code": "def signed_distance(p: Point) -> float:\n\n            \"\"\"Signed distance from point to plane\"\"\"\n            v = Vector(p.x - plane.origin.x, p.y - plane.origin.y, p.z - plane.origin.z)\n            return v.dot(plane.z_axis)\n        \n        def signed_distance_derivative(t: float) -> float:\n            \"\"\"Derivative of signed distance: df/dt = n \u00b7 C'(t)\"\"\"\n            tan = self.tangent_at(t)\n            return plane.z_axis.dot(tan)\n        \n        results = []\n        \n        # Process each Bezier span separately\n        spans = self.get_span_vector()\n        \n        for span_idx in range(len(spans) - 1):\n            span_t0 = spans[span_idx]\n            span_t1 = spans[span_idx + 1]\n            \n            # Skip degenerate spans",
          "file": "nurbscurve.py"
        }
      }
    },
    {
      "name": "NurbsCurve.intersect_plane_points",
      "implementations": {
        "python": {
          "sig": "intersect_plane_points(plane: Plane, tolerance: float = None) -> List[Point]",
          "code": "def intersect_plane_points(self, plane: Plane, tolerance: float = None) -> List[Point]:\n\n        \"\"\"Find all intersection points between curve and plane.\n        \n        Parameters\n        ----------\n        plane : Plane\n            The plane to intersect with.\n        tolerance : float, optional\n            Intersection tolerance. Defaults to Tolerance.ZERO_TOLERANCE.\n            \n        Returns\n        -------\n        list of Point\n            Intersection points.\n        \"\"\"\n        params = self.intersect_plane(plane, tolerance)\n        return [self.point_at(t) for t in params]\n    \n    def intersect_plane_bezier_clipping(self, plane: Plane, tolerance: float = None) -> List[float]:\n        \"\"\"Curve-plane intersection using B\u00e9zier clipping (faster for multiple intersections).",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "std::vector<Point> intersect_plane_points(const Plane& plane, double tolerance)",
          "code": "std::vector<Point> NurbsCurve::intersect_plane_points(const Plane& plane, double tolerance) const {\n    std::vector<double> params = intersect_plane(plane, tolerance);\n    std::vector<Point> points;\n    points.reserve(params.size());\n    \n    for (double t : params) {\n        points.push_back(point_at(t));\n    }",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "intersect_plane_points(plane: &Plane, tolerance: Option<f64>) -> Vec<Point>",
          "code": "pub fn intersect_plane_points(&self, plane: &Plane, tolerance: Option<f64>) -> Vec<Point> {\n        self.intersect_plane(plane, tolerance)\n            .iter()\n            .map(|&t| self.point_at(t))\n            .collect()\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.intersect_plane_bezier_clipping",
      "implementations": {
        "python": {
          "sig": "intersect_plane_bezier_clipping(plane: Plane, tolerance: float = None) -> List[float]",
          "code": "def intersect_plane_bezier_clipping(self, plane: Plane, tolerance: float = None) -> List[float]:\n\n        \"\"\"Curve-plane intersection using B\u00e9zier clipping (faster for multiple intersections).\n        \n        Parameters\n        ----------\n        plane : Plane\n            The plane to intersect with.\n        tolerance : float, optional\n            Intersection tolerance. Defaults to Tolerance.ZERO_TOLERANCE.\n            \n        Returns\n        -------\n        list of float\n            Parameter values where curve intersects plane.\n            \n        Notes\n        -----\n        This is an advanced method using B\u00e9zier clipping for interval reduction.\n        It's 2-5x faster than the standard method for curves with many intersections.\n        Used by Rhino, SolidWorks, and other professional CAD software.",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "std::vector<double> intersect_plane_bezier_clipping(const Plane& plane, double tolerance)",
          "code": "std::vector<double> NurbsCurve::intersect_plane_bezier_clipping(const Plane& plane, double tolerance) const {\n    std::vector<double> results;\n    \n    if (!is_valid()) return results;\n    if (tolerance <= 0.0) tolerance = Tolerance::ZERO_TOLERANCE;\n    \n    auto [t0, t1] = domain();\n    \n    // Helper: compute signed distance from point to plane\n    auto signed_distance = [&](const Point& p) -> double {\n        Vector v(p[0] - plane.origin()[0],\n                p[1] - plane.origin()[1],\n                p[2] - plane.origin()[2]);\n        return v.dot(plane.z_axis());\n    }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.clip_recursive",
      "implementations": {
        "python": {
          "sig": "clip_recursive(ta: float, tb: float, depth: int)",
          "code": "def clip_recursive(ta: float, tb: float, depth: int):\n\n            \"\"\"Recursive B\u00e9zier clipping on interval [ta, tb]\"\"\"\n            # Prevent infinite recursion\n            if depth > 50:\n                tm = (ta + tb) * 0.5\n                pm = self.point_at(tm)\n                dist = signed_distance(pm)\n                if abs(dist) < tolerance:\n                    results.append(tm)\n                return\n            \n            # Check if interval is small enough\n            if abs(tb - ta) < tolerance * 0.01:\n                tm = (ta + tb) * 0.5\n                pm = self.point_at(tm)\n                dist = signed_distance(pm)\n                \n                if abs(dist) < tolerance:\n                    # Newton refinement for final precision\n                    t = tm",
          "file": "nurbscurve.py"
        }
      }
    },
    {
      "name": "NurbsCurve.intersect_plane_algebraic",
      "implementations": {
        "python": {
          "sig": "intersect_plane_algebraic(plane: Plane, tolerance: float = None) -> List[float]",
          "code": "def intersect_plane_algebraic(self, plane: Plane, tolerance: float = None) -> List[float]:\n\n        \"\"\"Curve-plane intersection using algebraic/polynomial method (most precise).\n        \n        Parameters\n        ----------\n        plane : Plane\n            The plane to intersect with.\n        tolerance : float, optional\n            Intersection tolerance. Defaults to Tolerance.ZERO_TOLERANCE.\n            \n        Returns\n        -------\n        list of float\n            Parameter values where curve intersects plane.\n            \n        Notes\n        -----\n        This method converts the intersection problem to polynomial root finding.\n        It's the most mathematically precise but can be slower for high-degree curves.\n        Uses the hodograph (derivative) for Newton refinement with quadratic convergence.",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "std::vector<double> intersect_plane_algebraic(const Plane& plane, double tolerance)",
          "code": "std::vector<double> NurbsCurve::intersect_plane_algebraic(const Plane& plane, double tolerance) const {\n    if (!is_valid()) return {}",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.divide_by_count",
      "implementations": {
        "python": {
          "sig": "divide_by_count(count: int, include_endpoints: bool = True) -> Tuple[List[Point], List[float]]",
          "code": "def divide_by_count(self, count: int, include_endpoints: bool = True) -> Tuple[List[Point], List[float]]:\n\n        \"\"\"Divide curve into uniform number of points.\n        \n        Parameters\n        ----------\n        count : int\n            Number of points to generate (must be >= 2).\n        include_endpoints : bool, optional\n            If True, includes curve endpoints in the result. Defaults to True.\n            \n        Returns\n        -------\n        tuple of (list of Point, list of float)\n            The points and their parameters on the curve.\n        \"\"\"\n        points = []\n        params = []\n        \n        if not self.is_valid() or count < 2:\n            return points, params",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool divide_by_count(int count, std::vector<Point>& points,\n                                std::vector<double>* params,\n                                bool include_endpoints)",
          "code": "bool NurbsCurve::divide_by_count(int count, std::vector<Point>& points,\n                                std::vector<double>* params,\n                                bool include_endpoints) const {\n    points.clear();\n    if (params) params->clear();\n    \n    if (!is_valid()) return false;\n    if (count < 2) return false;\n    \n    auto [t0, t1] = domain();\n    double range = t1 - t0;\n    \n    if (include_endpoints) {\n        // Divide into count points including endpoints\n        // This gives (count-1) equal segments\n        for (int i = 0; i < count; i++) {\n            double t = t0 + (range * i) / (count - 1);\n            points.push_back(point_at(t));\n            if (params) params->push_back(t);\n        }",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "divide_by_count(count: usize, include_endpoints: bool) -> (Vec<Point>, Vec<f64>)",
          "code": "pub fn divide_by_count(&self, count: usize, include_endpoints: bool) -> (Vec<Point>, Vec<f64>) {\n        let mut points = Vec::new();\n        let mut params = Vec::new();\n\n        if !self.is_valid() || count == 0 {\n            return (points, params);\n        }\n\n        let (t0, t1) = self.domain();\n        let n = if include_endpoints { count - 1 } else { count + 1 };\n        let dt = (t1 - t0) / n as f64;\n\n        for i in 0..count {\n            let offset = if include_endpoints { 0 } else { 1 };\n            let t = t0 + (i + offset) as f64 * dt;\n            params.push(t);\n            points.push(self.point_at(t));\n        }\n\n        (points, params)\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.get_bounding_box",
      "implementations": {
        "python": {
          "sig": "get_bounding_box() -> Optional[BoundingBox]",
          "code": "def get_bounding_box(self) -> Optional[BoundingBox]:\n\n        \"\"\"Get the bounding box of the curve.\n        \n        Returns\n        -------\n        BoundingBox or None\n            The bounding box containing all control points, or None if invalid.\n        \"\"\"\n        if not self.is_valid():\n            return None\n        \n        min_pt = [float('inf')] * 3\n        max_pt = [float('-inf')] * 3\n        \n        for i in range(self.m_cv_count):\n            pt = self.get_cv(i)\n            if pt:\n                min_pt[0] = min(min_pt[0], pt.x)\n                min_pt[1] = min(min_pt[1], pt.y)\n                min_pt[2] = min(min_pt[2], pt.z)",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "BoundingBox get_bounding_box()",
          "code": "BoundingBox NurbsCurve::get_bounding_box() const {\n    if (!is_valid() || m_cv_count == 0) {\n        return BoundingBox();\n    }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.zero_cvs",
      "implementations": {
        "python": {
          "sig": "zero_cvs() -> bool",
          "code": "def zero_cvs(self) -> bool:\n\n        \"\"\"Zero all control vertices and set weights to 1 if rational.\n        \n        Returns\n        -------\n        bool\n            True if successful.\n        \"\"\"\n        if not self.is_valid():\n            return False\n        \n        self.m_cv.fill(0.0)\n        \n        if self.m_is_rat:\n            for i in range(self.m_cv_count):\n                self.m_cv[i * self.m_cv_stride + self.m_dim] = 1.0\n        \n        return True\n    \n    def is_clamped(self, end: int = 2) -> bool:",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool zero_cvs()",
          "code": "bool NurbsCurve::zero_cvs() {\n    if (!is_valid()) return false;\n    \n    for (auto& val : m_cv) {\n        val = 0.0;\n    }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.is_clamped",
      "implementations": {
        "python": {
          "sig": "is_clamped(end: int = 2) -> bool",
          "code": "def is_clamped(self, end: int = 2) -> bool:\n\n        \"\"\"Check if knot vector is clamped at ends.\n        \n        Parameters\n        ----------\n        end : int, optional\n            0 for start, 1 for end, 2 for both. Defaults to 2.\n            \n        Returns\n        -------\n        bool\n            True if clamped at specified end(s).\n        \"\"\"\n        if not self.is_valid():\n            return False\n        \n        check_start = (end == 0 or end == 2)\n        check_end = (end == 1 or end == 2)\n        \n        if check_start:",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool is_clamped(int end)",
          "code": "bool NurbsCurve::is_clamped(int end) const {\n    if (!is_valid()) return false;\n    \n    // end: 0 = start, 1 = end, 2 = both\n    bool start_clamped = true;\n    bool end_clamped = true;\n    \n    if (end == 0 || end == 2) {\n        // Check start: first m_order knots should be equal\n        for (int i = 1; i < m_order - 1; i++) {\n            if (m_knot[i] != m_knot[0]) {\n                start_clamped = false;\n                break;\n            }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.control_polygon_length",
      "implementations": {
        "python": {
          "sig": "control_polygon_length() -> float",
          "code": "def control_polygon_length(self) -> float:\n\n        \"\"\"Get the length of the control polygon.\n        \n        Returns\n        -------\n        float\n            Total length of control polygon edges.\n        \"\"\"\n        if not self.is_valid() or self.m_cv_count < 2:\n            return 0.0\n        \n        total_length = 0.0\n        for i in range(self.m_cv_count - 1):\n            p1 = self.get_cv(i)\n            p2 = self.get_cv(i + 1)\n            if p1 and p2:\n                total_length += p1.distance(p2)\n        \n        return total_length",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "double control_polygon_length()",
          "code": "double NurbsCurve::control_polygon_length() const {\n    if (!is_valid() || m_cv_count < 2) return 0.0;\n    \n    double length = 0.0;\n    Point prev = get_cv(0);\n    \n    for (int i = 1; i < m_cv_count; i++) {\n        Point curr = get_cv(i);\n        length += prev.distance(curr);\n        prev = curr;\n    }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.greville_abcissa",
      "implementations": {
        "python": {
          "sig": "greville_abcissa(cv_index: int) -> float",
          "code": "def greville_abcissa(self, cv_index: int) -> float:\n\n        \"\"\"Get Greville abcissa for a control point.\n        \n        Parameters\n        ----------\n        cv_index : int\n            Index of the control vertex.\n            \n        Returns\n        -------\n        float\n            The Greville abcissa parameter value.\n        \"\"\"\n        if cv_index < 0 or cv_index >= self.m_cv_count:\n            return 0.0\n        \n        total = 0.0\n        for i in range(self.m_order - 1):\n            total += self.m_knot[cv_index + i]",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "double greville_abcissa(int cv_index)",
          "code": "double NurbsCurve::greville_abcissa(int cv_index) const {\n    if (cv_index < 0 || cv_index >= m_cv_count) return 0.0;\n    \n    // Greville abcissa is the average of p consecutive knots starting at cv_index\n    int p = degree();\n    double sum = 0.0;\n    for (int i = 0; i < p; i++) {\n        sum += m_knot[cv_index + i];\n    }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.get_greville_abcissae",
      "implementations": {
        "python": {
          "sig": "get_greville_abcissae() -> List[float]",
          "code": "def get_greville_abcissae(self) -> List[float]:\n\n        \"\"\"Get all Greville abcissae.\n        \n        Returns\n        -------\n        list of float\n            Greville parameters for all control vertices.\n        \"\"\"\n        return [self.greville_abcissa(i) for i in range(self.m_cv_count)]\n    \n    def is_linear(self, tolerance: float = None) -> bool:\n        \"\"\"Check if curve is a straight line.\n        \n        Parameters\n        ----------\n        tolerance : float, optional\n            Maximum deviation from line. Defaults to Tolerance.ZERO_TOLERANCE.\n            \n        Returns\n        -------",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool get_greville_abcissae(std::vector<double>& abcissae)",
          "code": "bool NurbsCurve::get_greville_abcissae(std::vector<double>& abcissae) const {\n    if (!is_valid()) return false;\n    \n    abcissae.resize(m_cv_count);\n    for (int i = 0; i < m_cv_count; i++) {\n        abcissae[i] = greville_abcissa(i);\n    }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.is_linear",
      "implementations": {
        "python": {
          "sig": "is_linear(tolerance: float = None) -> bool",
          "code": "def is_linear(self, tolerance: float = None) -> bool:\n\n        \"\"\"Check if curve is a straight line.\n        \n        Parameters\n        ----------\n        tolerance : float, optional\n            Maximum deviation from line. Defaults to Tolerance.ZERO_TOLERANCE.\n            \n        Returns\n        -------\n        bool\n            True if curve is linear within tolerance.\n        \"\"\"\n        if tolerance is None:\n            tolerance = Tolerance.ZERO_TOLERANCE\n        \n        if not self.is_valid() or self.m_cv_count < 2:\n            return False\n        \n        p_start = self.point_at_start()",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool is_linear(double tolerance)",
          "code": "bool NurbsCurve::is_linear(double tolerance) const {\n    if (!is_valid() || m_cv_count < 2) return false;\n    \n    Point p0 = get_cv(0);\n    Point p1 = get_cv(m_cv_count - 1);\n    Vector line_vec(p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]);\n    double line_length = line_vec.magnitude();\n    \n    if (line_length < tolerance) return true;\n    \n    for (int i = 1; i < m_cv_count - 1; i++) {\n        Point p = get_cv(i);\n        Vector v(p[0] - p0[0], p[1] - p0[1], p[2] - p0[2]);\n        Vector cross = line_vec.cross(v);\n        double dist = cross.magnitude() / line_length;\n        if (dist > tolerance) return false;\n    }",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "is_linear(tolerance: Option<f64>) -> bool",
          "code": "pub fn is_linear(&self, tolerance: Option<f64>) -> bool {\n        let tol = tolerance.unwrap_or(Tolerance::ZERO_TOLERANCE);\n        \n        if !self.is_valid() || self.m_cv_count < 2 {\n            return false;\n        }\n\n        if self.m_cv_count == 2 {\n            return true;\n        }\n\n        // Check if all control points are collinear\n        let p0 = self.get_cv(0).unwrap();\n        let p1 = self.get_cv(self.m_cv_count - 1).unwrap();\n        \n        let line_vec = Vector::new(p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]);\n        let line_len = line_vec.magnitude();\n        \n        if line_len < tol {\n            return true; // Degenerate to a point\n        }\n\n        for i in 1..(self.m_cv_count - 1) {\n            let p = self.get_cv(i).unwrap();\n            let v = Vector",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.is_planar",
      "implementations": {
        "python": {
          "sig": "is_planar(tolerance: float = None) -> bool",
          "code": "def is_planar(self, tolerance: float = None) -> bool:\n\n        \"\"\"Check if curve lies in a plane.\n        \n        Parameters\n        ----------\n        tolerance : float, optional\n            Maximum deviation from plane. Defaults to Tolerance.ZERO_TOLERANCE.\n            \n        Returns\n        -------\n        bool\n            True if curve is planar within tolerance.\n        \"\"\"\n        if tolerance is None:\n            tolerance = Tolerance.ZERO_TOLERANCE\n        \n        if not self.is_valid() or self.m_cv_count < 3:\n            return True\n        \n        p0 = self.get_cv(0)",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool is_planar(Plane* plane, double tolerance)",
          "code": "bool NurbsCurve::is_planar(Plane* plane, double tolerance) const {\n    // Simplified planar check\n    if (!is_valid() || m_cv_count < 3) return true;\n    \n    // Get three non-collinear points\n    Point p0 = get_cv(0);\n    Point p1 = get_cv(m_cv_count / 2);\n    Point p2 = get_cv(m_cv_count - 1);\n    \n    Vector v1(p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]);\n    Vector v2(p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]);\n    Vector normal = v1.cross(v2);\n    \n    if (normal.magnitude() < tolerance) return true;\n    \n    // Check all CVs against this plane\n    for (int i = 0; i < m_cv_count; i++) {\n        Point p = get_cv(i);\n        Vector v(p[0] - p0[0], p[1] - p0[1], p[2] - p0[2]);\n        double dist = std::abs(v.dot(normal)) / normal.magnitude();\n        if (dist > tolerance) return fal",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.closest_point",
      "implementations": {
        "python": {
          "sig": "closest_point(test_point: Point, tolerance: float = None) -> Tuple[Point, float]",
          "code": "def closest_point(self, test_point: Point, tolerance: float = None) -> Tuple[Point, float]:\n\n        \"\"\"Find closest point on curve to test point.\n        \n        Parameters\n        ----------\n        test_point : Point\n            The point to find the closest curve point to.\n        tolerance : float, optional\n            Convergence tolerance. Defaults to Tolerance.ZERO_TOLERANCE.\n            \n        Returns\n        -------\n        tuple of (Point, float)\n            The closest point and its parameter value.\n        \"\"\"\n        if tolerance is None:\n            tolerance = Tolerance.ZERO_TOLERANCE\n        \n        if not self.is_valid():\n            return Point(0, 0, 0), 0.0",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "Point closest_point(const Point& point, double& t_out)",
          "code": "Point NurbsCurve::closest_point(const Point& point, double& t_out) const {\n    if (!is_valid()) {\n        t_out = 0.0;\n        return Point(0, 0, 0);\n    }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.increase_degree",
      "implementations": {
        "python": {
          "sig": "increase_degree(desired_degree: int) -> bool",
          "code": "def increase_degree(self, desired_degree: int) -> bool:\n\n        \"\"\"Increase the degree of the curve using degree elevation.\n        \n        Parameters\n        ----------\n        desired_degree : int\n            Target degree (must be >= current degree).\n            \n        Returns\n        -------\n        bool\n            True if successful.\n        \"\"\"\n        if not self.is_valid():\n            return False\n        if desired_degree <= self.degree():\n            return True\n        \n        degree_inc = desired_degree - self.degree()",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool increase_degree(int desired_degree)",
          "code": "bool NurbsCurve::increase_degree(int desired_degree) {\n    if (!is_valid()) return false;\n    if (desired_degree <= degree()) return true; // Already at or above desired degree\n    \n    int degree_inc = desired_degree - degree();\n    \n    // Increase degree one at a time\n    for (int inc = 0; inc < degree_inc; inc++) {\n        int old_order = m_order;\n        int old_cv_count = m_cv_count;\n        int new_order = old_order + 1;\n        int new_cv_count = old_cv_count + old_cv_count - old_order + 1;\n        \n        // Get old data\n        std::vector<double> old_knots = m_knot;\n        std::vector<double> old_cvs = m_cv;\n        \n        // Prepare new data\n        int new_knot_count = new_order + new_cv_count - 2;\n        std::vector<double> new_knots(new_knot_count);\n        std::vector<",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.trim",
      "implementations": {
        "python": {
          "sig": "trim(t0: float, t1: float) -> bool",
          "code": "def trim(self, t0: float, t1: float) -> bool:\n\n        \"\"\"Trim curve to a parameter sub-interval.\n        \n        Parameters\n        ----------\n        t0 : float\n            Start parameter.\n        t1 : float\n            End parameter.\n            \n        Returns\n        -------\n        bool\n            True if successful.\n        \"\"\"\n        if not self.is_valid() or t0 >= t1:\n            return False\n        \n        domain_t0, domain_t1 = self.domain()\n        if t0 < domain_t0 or t1 > domain_t1:",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool trim(double t0, double t1)",
          "code": "bool NurbsCurve::trim(double t0, double t1) {\n    if (!is_valid() || t0 >= t1) return false;\n    \n    auto [d0, d1] = domain();\n    if (t0 == d0 && t1 == d1) return true; // Already at desired domain\n    \n    // This is a simplified trim - for production use, need full de Boor algorithm\n    // For now, just adjust domain\n    return set_domain(t0, t1);\n}",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.divide_by_length",
      "implementations": {
        "python": {
          "sig": "divide_by_length(segment_length: float) -> Tuple[List[Point], List[float]]",
          "code": "def divide_by_length(self, segment_length: float) -> Tuple[List[Point], List[float]]:\n\n        \"\"\"Divide curve by approximate arc length.\n        \n        Parameters\n        ----------\n        segment_length : float\n            Target length between points.\n            \n        Returns\n        -------\n        tuple of (list of Point, list of float)\n            Points and parameters approximately spaced by segment_length.\n        \"\"\"\n        points = []\n        params = []\n        \n        if not self.is_valid() or segment_length <= 0.0:\n            return points, params\n        \n        curve_len = self.length()",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool divide_by_length(double segment_length, std::vector<Point>& points,\n                                 std::vector<double>* params)",
          "code": "bool NurbsCurve::divide_by_length(double segment_length, std::vector<Point>& points,\n                                 std::vector<double>* params) const {\n    points.clear();\n    if (params) params->clear();\n    \n    if (!is_valid()) return false;\n    if (segment_length <= 0.0) return false;\n    \n    double curve_len = length();\n    int approx_count = static_cast<int>(std::ceil(curve_len / segment_length)) + 1;\n    \n    // Use adaptive approach to get approximately equal arc lengths\n    auto [t0, t1] = domain();\n    \n    points.push_back(point_at(t0));\n    if (params) params->push_back(t0);\n    \n    double accumulated_length = 0.0;\n    double target_length = segment_length;\n    Point p_current = point_at(t0);\n    \n    // Sample densely and accumulate arc length\n    int num_samples = std::m",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.split",
      "implementations": {
        "python": {
          "sig": "split(t: float) -> Tuple[Optional['NurbsCurve'], Optional['NurbsCurve']]",
          "code": "def split(self, t: float) -> Tuple[Optional['NurbsCurve'], Optional['NurbsCurve']]:\n\n        \"\"\"Split curve at parameter t into left and right parts.\n        \n        Parameters\n        ----------\n        t : float\n            Parameter value to split at.\n            \n        Returns\n        -------\n        tuple of (NurbsCurve, NurbsCurve) or (None, None)\n            Left and right curves, or None if invalid.\n        \"\"\"\n        if not self.is_valid():\n            return None, None\n        \n        t0, t1 = self.domain()\n        if t <= t0 or t >= t1:\n            return None, None",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool split(double t, NurbsCurve& left, NurbsCurve& right)",
          "code": "bool NurbsCurve::split(double t, NurbsCurve& left, NurbsCurve& right) const {\n    if (!is_valid()) return false;\n    \n    auto [t0, t1] = domain();\n    if (t <= t0 || t >= t1) return false;\n    \n    // Simplified split - copy curve and trim each half\n    left = *this;\n    right = *this;\n    \n    left.trim(t0, t);\n    right.trim(t, t1);\n    \n    return true;\n}",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.extend",
      "implementations": {
        "python": {
          "sig": "extend(t0: float, t1: float) -> bool",
          "code": "def extend(self, t0: float, t1: float) -> bool:\n\n        \"\"\"Extend curve to include domain [t0, t1].\n        \n        Parameters\n        ----------\n        t0 : float\n            New start parameter (can be before current start).\n        t1 : float\n            New end parameter (can be after current end).\n            \n        Returns\n        -------\n        bool\n            True if successful.\n        \"\"\"\n        if not self.is_valid():\n            return False\n        \n        domain_t0, domain_t1 = self.domain()",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool extend(double t0, double t1)",
          "code": "bool NurbsCurve::extend(double t0, double t1) {\n    if (!is_valid() || is_closed()) return false;\n    \n    auto [d0, d1] = domain();\n    bool changed = false;\n    \n    if (t0 < d0) {\n        clamp_end(0);\n        // Adjust start knots\n        for (int i = 0; i < m_order - 1; i++) {\n            m_knot[i] = t0;\n        }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.swap_coordinates",
      "implementations": {
        "python": {
          "sig": "swap_coordinates(axis_i: int, axis_j: int) -> bool",
          "code": "def swap_coordinates(self, axis_i: int, axis_j: int) -> bool:\n\n        \"\"\"Swap two coordinate axes.\n        \n        Parameters\n        ----------\n        axis_i : int\n            First axis index (0=x, 1=y, 2=z).\n        axis_j : int\n            Second axis index (0=x, 1=y, 2=z).\n            \n        Returns\n        -------\n        bool\n            True if successful.\n        \"\"\"\n        if not self.is_valid():\n            return False\n        if axis_i < 0 or axis_i >= self.m_dim or axis_j < 0 or axis_j >= self.m_dim:\n            return False\n        if axis_i == axis_j:",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool swap_coordinates(int axis_i, int axis_j)",
          "code": "bool NurbsCurve::swap_coordinates(int axis_i, int axis_j) {\n    if (!is_valid()) return false;\n    if (axis_i < 0 || axis_i >= m_dim) return false;\n    if (axis_j < 0 || axis_j >= m_dim) return false;\n    if (axis_i == axis_j) return true;\n    \n    // Swap coordinates in all control vertices\n    for (int cv_idx = 0; cv_idx < m_cv_count; cv_idx++) {\n        double* cv_ptr = cv(cv_idx);\n        std::swap(cv_ptr[axis_i], cv_ptr[axis_j]);\n    }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.set_start_point",
      "implementations": {
        "python": {
          "sig": "set_start_point(start_point: Point) -> bool",
          "code": "def set_start_point(self, start_point: Point) -> bool:\n\n        \"\"\"Force curve to start at specified point.\n        \n        Parameters\n        ----------\n        start_point : Point\n            New start point.\n            \n        Returns\n        -------\n        bool\n            True if successful.\n        \"\"\"\n        if not self.is_valid():\n            return False\n        \n        return self.set_cv(0, start_point)\n    \n    def set_end_point(self, end_point: Point) -> bool:\n        \"\"\"Force curve to end at specified point.",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool set_start_point(const Point& start_point)",
          "code": "bool NurbsCurve::set_start_point(const Point& start_point) {\n    if (!is_valid()) return false;\n    \n    clamp_end(2);\n    \n    double w = 1.0;\n    if (m_is_rat) {\n        w = weight(0);\n    }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.set_end_point",
      "implementations": {
        "python": {
          "sig": "set_end_point(end_point: Point) -> bool",
          "code": "def set_end_point(self, end_point: Point) -> bool:\n\n        \"\"\"Force curve to end at specified point.\n        \n        Parameters\n        ----------\n        end_point : Point\n            New end point.\n            \n        Returns\n        -------\n        bool\n            True if successful.\n        \"\"\"\n        if not self.is_valid():\n            return False\n        \n        return self.set_cv(self.m_cv_count - 1, end_point)\n    \n    def transform(self, xform: Xform) -> bool:\n        \"\"\"Apply transformation to the curve.",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool set_end_point(const Point& end_point)",
          "code": "bool NurbsCurve::set_end_point(const Point& end_point) {\n    if (!is_valid()) return false;\n    \n    clamp_end(2);\n    \n    double w = 1.0;\n    if (m_is_rat) {\n        w = weight(m_cv_count - 1);\n    }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.transform",
      "implementations": {
        "python": {
          "sig": "transform(xform: Xform) -> bool",
          "code": "def transform(self, xform: Xform) -> bool:\n\n        \"\"\"Apply transformation to the curve.\n        \n        Parameters\n        ----------\n        xform : Xform\n            Transformation to apply.\n            \n        Returns\n        -------\n        bool\n            True if successful.\n        \"\"\"\n        if not self.is_valid():\n            return False\n        \n        for i in range(self.m_cv_count):\n            pt = self.get_cv(i)\n            if pt:\n                transformed_pt = xform.transform_point(pt)",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool transform(const Xform& xf)",
          "code": "bool NurbsCurve::transform(const Xform& xf) {\n    for (int i = 0; i < m_cv_count; i++) {\n        Point p = get_cv(i);\n        // Apply xform matrix (column-major: m[col*4 + row])\n        double x = xf.m[0] * p[0] + xf.m[4] * p[1] + xf.m[8] * p[2] + xf.m[12];\n        double y = xf.m[1] * p[0] + xf.m[5] * p[1] + xf.m[9] * p[2] + xf.m[13];\n        double z = xf.m[2] * p[0] + xf.m[6] * p[1] + xf.m[10] * p[2] + xf.m[14];\n        set_cv(i, Point(x, y, z));\n    }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.transformed",
      "implementations": {
        "python": {
          "sig": "transformed(xform: Xform = None) -> 'NurbsCurve'",
          "code": "def transformed(self, xform: Xform = None) -> 'NurbsCurve':\n\n        \"\"\"Get transformed copy of the curve.\n        \n        Parameters\n        ----------\n        xform : Xform, optional\n            Transformation to apply. If None, uses stored self.xform.\n            \n        Returns\n        -------\n        NurbsCurve\n            Transformed copy of the curve.\n        \"\"\"\n        result = NurbsCurve()\n        result.m_dim = self.m_dim\n        result.m_is_rat = self.m_is_rat\n        result.m_order = self.m_order\n        result.m_cv_count = self.m_cv_count\n        result.m_cv_stride = self.m_cv_stride\n        result.m_knot = self.m_knot.copy()",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "NurbsCurve transformed(const Xform& xf)",
          "code": "NurbsCurve NurbsCurve::transformed(const Xform& xf) const {\n    NurbsCurve result = *this;\n    result.transform(xf);\n    return result;\n}",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.superfluous_knot",
      "implementations": {
        "python": {
          "sig": "superfluous_knot(end: int) -> float",
          "code": "def superfluous_knot(self, end: int) -> float:\n\n        \"\"\"Get superfluous knot value at end.\n        \n        Parameters\n        ----------\n        end : int\n            0 for start, 1 for end.\n            \n        Returns\n        -------\n        float\n            The superfluous knot value.\n        \"\"\"\n        if not self.is_valid():\n            return 0.0\n        \n        if end == 0:\n            # Start: return knot[order-2] - (knot[order-1] - knot[order-2])\n            if self.m_order >= 2:\n                return 2.0 * self.m_knot[self.m_order - 2] - self.m_knot[self.m_order - 1]",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "double superfluous_knot(int end)",
          "code": "double NurbsCurve::superfluous_knot(int end) const {\n    if (!is_valid()) return 0.0;\n    \n    if (end == 0) {\n        // Start: return knot before first domain knot\n        if (m_order >= 3) {\n            return 2.0 * m_knot[m_order-2] - m_knot[m_order-1];\n        }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.is_in_plane",
      "implementations": {
        "python": {
          "sig": "is_in_plane(test_plane: Plane, tolerance: float = None) -> bool",
          "code": "def is_in_plane(self, test_plane: Plane, tolerance: float = None) -> bool:\n\n        \"\"\"Check if curve lies in a specific plane.\n        \n        Parameters\n        ----------\n        test_plane : Plane\n            The plane to test against.\n        tolerance : float, optional\n            Maximum deviation. Defaults to Tolerance.ZERO_TOLERANCE.\n            \n        Returns\n        -------\n        bool\n            True if curve lies in the plane.\n        \"\"\"\n        if tolerance is None:\n            tolerance = Tolerance.ZERO_TOLERANCE\n        \n        if not self.is_valid():\n            return False",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool is_in_plane(const Plane& test_plane, double tolerance)",
          "code": "bool NurbsCurve::is_in_plane(const Plane& test_plane, double tolerance) const {\n    if (!is_valid()) return false;\n    \n    // Check if all control points lie in the plane\n    for (int i = 0; i < m_cv_count; i++) {\n        Point pt = get_cv(i);\n        Vector v(pt[0] - test_plane.origin()[0],\n                pt[1] - test_plane.origin()[1],\n                pt[2] - test_plane.origin()[2]);\n        \n        double dist = std::abs(v.dot(test_plane.z_axis()));\n        if (dist > tolerance) {\n            return false;\n        }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.is_singular",
      "implementations": {
        "python": {
          "sig": "is_singular() -> bool",
          "code": "def is_singular(self) -> bool:\n\n        \"\"\"Check if entire curve is singular (collapsed to a point).\n        \n        Returns\n        -------\n        bool\n            True if curve is singular.\n        \"\"\"\n        if not self.is_valid():\n            return False\n        \n        p_first = self.point_at_start()\n        \n        # Check if all sample points are at same location\n        t0, t1 = self.domain()\n        num_samples = max(10, self.m_cv_count)\n        dt = (t1 - t0) / num_samples\n        \n        for i in range(1, num_samples + 1):\n            t = t0 + i * dt",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool is_singular()",
          "code": "bool NurbsCurve::is_singular() const {\n    if (!is_valid()) return false;\n    \n    int span_count = this->span_count();\n    for (int i = 0; i < span_count; i++) {\n        if (!span_is_singular(i)) {\n            return false;\n        }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.has_bezier_spans",
      "implementations": {
        "python": {
          "sig": "has_bezier_spans() -> bool",
          "code": "def has_bezier_spans(self) -> bool:\n\n        \"\"\"Check if curve has bezier spans (all distinct knots have multiplicity = degree).\n        \n        Returns\n        -------\n        bool\n            True if curve has bezier spans.\n        \"\"\"\n        if not self.is_valid():\n            return False\n        \n        degree = self.degree()\n        \n        # Check interior knots\n        i = self.m_order - 1\n        while i < self.m_cv_count - 1:\n            mult = self.knot_multiplicity(i)\n            if mult != degree:\n                return False\n            i += mult",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool has_bezier_spans()",
          "code": "bool NurbsCurve::has_bezier_spans() const {\n    if (!is_valid()) return false;\n    \n    int p = degree();\n    int kc = knot_count();\n    \n    // Check each distinct knot has multiplicity = degree\n    std::vector<double> distinct_knots;\n    std::vector<int> multiplicities;\n    \n    distinct_knots.push_back(m_knot[0]);\n    multiplicities.push_back(1);\n    \n    for (int i = 1; i < kc; i++) {\n        if (std::abs(m_knot[i] - distinct_knots.back()) < Tolerance::ZERO_TOLERANCE) {\n            multiplicities.back()++;\n        }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.append",
      "implementations": {
        "python": {
          "sig": "append(other: 'NurbsCurve') -> bool",
          "code": "def append(self, other: 'NurbsCurve') -> bool:\n\n        \"\"\"Append another NURBS curve to this one.\n        \n        Parameters\n        ----------\n        other : NurbsCurve\n            The curve to append.\n            \n        Returns\n        -------\n        bool\n            True if successful.\n        \"\"\"\n        if not self.is_valid() or not other.is_valid():\n            return False\n        if self.m_dim != other.m_dim:\n            return False\n        if self.m_is_rat != other.m_is_rat:\n            return False",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool append(const NurbsCurve& other)",
          "code": "bool NurbsCurve::append(const NurbsCurve& other) {\n    if (!is_valid() || !other.is_valid()) return false;\n    if (m_dim != other.m_dim) return false;\n    if (m_is_rat != other.m_is_rat) return false;\n    \n    // Check if curves are connected\n    Point this_end = point_at_end();\n    Point other_start = other.point_at_start();\n    double gap = this_end.distance(other_start);\n    if (gap > Tolerance::ZERO_TOLERANCE * 10.0) {\n        return false; // Curves must be connected\n    }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.clean_knots",
      "implementations": {
        "python": {
          "sig": "clean_knots(tolerance: float = 0.0) -> bool",
          "code": "def clean_knots(self, tolerance: float = 0.0) -> bool:\n\n        \"\"\"Clean up invalid knots (remove duplicates within tolerance).\n        \n        Parameters\n        ----------\n        tolerance : float, optional\n            Knot comparison tolerance. Defaults to 0.0.\n            \n        Returns\n        -------\n        bool\n            True if successful.\n        \"\"\"\n        if not self.is_valid():\n            return False\n        \n        if tolerance <= 0.0:\n            tolerance = Tolerance.ZERO_TOLERANCE\n        \n        # Remove knots that are too close together",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool clean_knots(double knot_tolerance)",
          "code": "bool NurbsCurve::clean_knots(double knot_tolerance) {\n    if (!is_valid()) return false;\n    if (knot_tolerance < 0.0) knot_tolerance = Tolerance::ZERO_TOLERANCE;\n    \n    // Remove duplicate knots within tolerance\n    std::vector<double> cleaned_knots;\n    cleaned_knots.push_back(m_knot[0]);\n    \n    for (size_t i = 1; i < m_knot.size(); i++) {\n        if (std::abs(m_knot[i] - cleaned_knots.back()) > knot_tolerance) {\n            cleaned_knots.push_back(m_knot[i]);\n        }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.clamp_end",
      "implementations": {
        "python": {
          "sig": "clamp_end(end: int) -> bool",
          "code": "def clamp_end(self, end: int) -> bool:\n\n        \"\"\"Clamp ends (add multiplicity to end knots).\n        \n        Parameters\n        ----------\n        end : int\n            0 for start, 1 for end, 2 for both.\n            \n        Returns\n        -------\n        bool\n            True if successful.\n        \"\"\"\n        if not self.is_valid():\n            return False\n        \n        # This is a simplified implementation\n        # Full implementation would insert knots to achieve full multiplicity\n        return True",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool clamp_end(int end)",
          "code": "bool NurbsCurve::clamp_end(int end) {\n    if (!is_valid()) return false;\n    \n    // end: 0 = start, 1 = end, 2 = both\n    if (end < 0 || end > 2) return false;\n    \n    // Clamp start\n    if (end == 0 || end == 2) {\n        for (int i = 0; i < m_order - 1; i++) {\n            m_knot[i] = m_knot[m_order - 2];\n        }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.evaluate",
      "implementations": {
        "python": {
          "sig": "evaluate(t: float, derivative_count: int = 0) -> List[Vector]",
          "code": "def evaluate(self, t: float, derivative_count: int = 0) -> List[Vector]:\n\n        \"\"\"Evaluate point and derivatives on curve at parameter t.\n        \n        Parameters\n        ----------\n        t : float\n            Parameter value.\n        derivative_count : int, optional\n            Number of derivatives to compute. Defaults to 0 (point only).\n            \n        Returns\n        -------\n        list of Vector\n            [point, 1st_derivative, 2nd_derivative, ...].\n        \"\"\"\n        if not self.is_valid():\n            return []\n        \n        results = []",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "std::vector<Vector> evaluate(double t, int derivative_count)",
          "code": "std::vector<Vector> NurbsCurve::evaluate(double t, int derivative_count) const {\n    std::vector<Vector> result;\n\n    if (!is_valid()) {\n        result.push_back(Vector(0, 0, 0));\n        return result;\n    }",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "evaluate(t: f64, derivative_count: usize) -> Vec<Vector>",
          "code": "pub fn evaluate(&self, t: f64, derivative_count: usize) -> Vec<Vector> {\n        let mut result = Vec::new();\n        if !self.is_valid() {\n            result.push(Vector::new(0.0, 0.0, 0.0));\n            return result;\n        }\n\n        let p = self.point_at(t);\n        result.push(Vector::new(p[0], p[1], p[2]));\n\n        if derivative_count == 0 {\n            return result;\n        }\n\n        // Numerical derivatives (consistent with tangent_at semantics)\n        let (t0, t1) = self.domain();\n        let eps = (t1 - t0) * 1e-8;\n        let ta = (t - eps).max(t0);\n        let tb = (t + eps).min(t1);\n\n        let pa = self.point_at(ta);\n        let pb = self.point_at(tb);\n\n        let d1 = Vector::new(\n            (pb[0] - pa[0]) / (tb - ta),\n            (pb[1] - pa[1]) / (tb - ta),",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.closest_point_to",
      "implementations": {
        "python": {
          "sig": "closest_point_to(test_point: Point, t0: float = None, t1: float = None) -> Tuple[float, float]",
          "code": "def closest_point_to(self, test_point: Point, t0: float = None, t1: float = None) -> Tuple[float, float]:\n\n        \"\"\"Find closest point with parameter bounds.\n        \n        Parameters\n        ----------\n        test_point : Point\n            Point to find closest curve point to.\n        t0 : float, optional\n            Start of search interval. Defaults to curve start.\n        t1 : float, optional\n            End of search interval. Defaults to curve end.\n            \n        Returns\n        -------\n        tuple of (float, float)\n            (parameter, distance) of closest point.\n        \"\"\"\n        domain_t0, domain_t1 = self.domain()\n        \n        if t0 is None:",
          "file": "nurbscurve.py"
        }
      }
    },
    {
      "name": "NurbsCurve.get_nurbs_form",
      "implementations": {
        "python": {
          "sig": "get_nurbs_form() -> int",
          "code": "def get_nurbs_form(self) -> int:\n\n        \"\"\"Get NURBS form (always returns 1 for NURBS curve).\n        \n        Returns\n        -------\n        int\n            1 (NURBS form).\n        \"\"\"\n        return 1\n    \n    def has_nurbs_form(self) -> int:\n        \"\"\"Check if has NURBS form (always returns 1).\n        \n        Returns\n        -------\n        int\n            1 (has NURBS form).\n        \"\"\"\n        return 1",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "int get_nurbs_form(NurbsCurve& nurbs_form, double tolerance)",
          "code": "int NurbsCurve::get_nurbs_form(NurbsCurve& nurbs_form, double tolerance) const {\n    (void)tolerance;  // Not used for NURBS curve\n    \n    if (!is_valid()) return 0;\n    \n    // For a NURBS curve, the NURBS form is itself\n    nurbs_form.deep_copy_from(*this);\n    \n    return 1;  // Perfect parameterization match\n}",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.has_nurbs_form",
      "implementations": {
        "python": {
          "sig": "has_nurbs_form() -> int",
          "code": "def has_nurbs_form(self) -> int:\n\n        \"\"\"Check if has NURBS form (always returns 1).\n        \n        Returns\n        -------\n        int\n            1 (has NURBS form).\n        \"\"\"\n        return 1\n    \n    def to_string(self) -> str:\n        \"\"\"Convert curve to string representation.\n        \n        Returns\n        -------\n        str\n            String description of the curve.\n        \"\"\"\n        return (f\"NurbsCurve(dim={self.m_dim}, rational={bool(self.m_is_rat)}, \"\n                f\"order={self.m_order}, cvs={self.m_cv_count}, \"",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "int has_nurbs_form()",
          "code": "int NurbsCurve::has_nurbs_form() const {\n    return is_valid() ? 1 : 0;\n}",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.to_string",
      "implementations": {
        "python": {
          "sig": "to_string() -> str",
          "code": "def to_string(self) -> str:\n\n        \"\"\"Convert curve to string representation.\n        \n        Returns\n        -------\n        str\n            String description of the curve.\n        \"\"\"\n        return (f\"NurbsCurve(dim={self.m_dim}, rational={bool(self.m_is_rat)}, \"\n                f\"order={self.m_order}, cvs={self.m_cv_count}, \"\n                f\"knots={self.knot_count()}, valid={self.is_valid()})\")\n    \n    def __str__(self) -> str:\n        \"\"\"String representation.\"\"\"\n        return self.to_string()\n    \n    def __repr__(self) -> str:\n        \"\"\"Representation string.\"\"\"\n        return self.to_string()",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "std::string to_string()",
          "code": "std::string NurbsCurve::to_string() const {\n    std::ostringstream oss;\n    oss << \"NurbsCurve(dim=\" << m_dim << \", order=\" << m_order \n        << \", cv_count=\" << m_cv_count << \", rational=\" << (m_is_rat ? \"true\" : \"false\") << \")\";\n    return oss.str();\n}",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.__str__",
      "implementations": {
        "python": {
          "sig": "__str__() -> str",
          "code": "def __str__(self) -> str:\n\n        \"\"\"String representation.\"\"\"\n        return self.to_string()\n    \n    def __repr__(self) -> str:\n        \"\"\"Representation string.\"\"\"\n        return self.to_string()\n    \n    def is_arc(self, tolerance: float = None) -> bool:\n        \"\"\"Check if curve is an arc.\n        \n        Parameters\n        ----------\n        tolerance : float, optional\n            Tolerance for arc test. Defaults to Tolerance.ZERO_TOLERANCE.\n            \n        Returns\n        -------\n        bool\n            True if curve is an arc.",
          "file": "nurbscurve.py"
        }
      }
    },
    {
      "name": "NurbsCurve.__repr__",
      "implementations": {
        "python": {
          "sig": "__repr__() -> str",
          "code": "def __repr__(self) -> str:\n\n        \"\"\"Representation string.\"\"\"\n        return self.to_string()\n    \n    def is_arc(self, tolerance: float = None) -> bool:\n        \"\"\"Check if curve is an arc.\n        \n        Parameters\n        ----------\n        tolerance : float, optional\n            Tolerance for arc test. Defaults to Tolerance.ZERO_TOLERANCE.\n            \n        Returns\n        -------\n        bool\n            True if curve is an arc.\n        \"\"\"\n        if tolerance is None:\n            tolerance = Tolerance.ZERO_TOLERANCE",
          "file": "nurbscurve.py"
        }
      }
    },
    {
      "name": "NurbsCurve.is_arc",
      "implementations": {
        "python": {
          "sig": "is_arc(tolerance: float = None) -> bool",
          "code": "def is_arc(self, tolerance: float = None) -> bool:\n\n        \"\"\"Check if curve is an arc.\n        \n        Parameters\n        ----------\n        tolerance : float, optional\n            Tolerance for arc test. Defaults to Tolerance.ZERO_TOLERANCE.\n            \n        Returns\n        -------\n        bool\n            True if curve is an arc.\n        \"\"\"\n        if tolerance is None:\n            tolerance = Tolerance.ZERO_TOLERANCE\n        \n        if not self.is_valid() or not self.is_planar(tolerance):\n            return False\n        \n        # Sample curve and check if all points are equidistant from center",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool is_arc(Plane* plane, double tolerance)",
          "code": "bool NurbsCurve::is_arc(Plane* plane, double tolerance) const {\n    if (!is_valid()) return false;\n    if (m_dim != 2 && m_dim != 3) return false;\n    if (m_order < 3) return false;\n    \n    // First check if it's linear (can't be both line and arc)\n    if (is_linear(tolerance)) return false;\n    \n    // Check if planar\n    Plane test_plane;\n    if (!is_planar(&test_plane, tolerance)) {\n        return false;\n    }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.is_natural",
      "implementations": {
        "python": {
          "sig": "is_natural(end: int = 2) -> bool",
          "code": "def is_natural(self, end: int = 2) -> bool:\n\n        \"\"\"Test if curve has natural end (zero 2nd derivative).\n        \n        Parameters\n        ----------\n        end : int, optional\n            0 for start, 1 for end, 2 for both. Defaults to 2.\n            \n        Returns\n        -------\n        bool\n            True if has natural end.\n        \"\"\"\n        if not self.is_valid():\n            return False\n        \n        t0, t1 = self.domain()\n        \n        check_start = (end == 0 or end == 2)\n        check_end = (end == 1 or end == 2)",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool is_natural(int end)",
          "code": "bool NurbsCurve::is_natural(int end) const {\n    if (!is_valid()) return false;\n    \n    const double tol_factor = 1e-8;\n    auto [t0, t1] = domain();\n    \n    for (int pass = ((end == 0 || end == 2) ? 0 : 1); pass < ((end == 1 || end == 2) ? 2 : 1); ++pass) {\n        double t = (pass == 0) ? t0 : t1;\n        \n        // Evaluate 2nd derivative\n        auto derivs = evaluate(t, 2);\n        if (derivs.size() < 3) return false;\n        \n        Vector d2(derivs[2][0], derivs[2][1], derivs[2][2]);\n        double d2_len = d2.magnitude();\n        \n        // Get control polygon length for tolerance\n        Point cv0 = get_cv((pass == 0) ? 0 : m_cv_count - 1);\n        Point cv2 = get_cv((pass == 0) ? std::min(2, m_cv_count - 1) : std::max(0, m_cv_count - 3));\n        double tol = cv0.distance(cv",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.is_polyline",
      "implementations": {
        "python": {
          "sig": "is_polyline() -> Tuple[bool, List[Point], List[float]]",
          "code": "def is_polyline(self) -> Tuple[bool, List[Point], List[float]]:\n\n        \"\"\"Check if curve can be represented as a polyline.\n        \n        Returns\n        -------\n        tuple of (bool, list of Point, list of float)\n            (is_polyline, points, parameters) or (False, [], []).\n        \"\"\"\n        if not self.is_valid():\n            return False, [], []\n        \n        # Check if curve is linear\n        if self.is_linear():\n            points = [self.point_at_start(), self.point_at_end()]\n            t0, t1 = self.domain()\n            params = [t0, t1]\n            return True, points, params\n        \n        return False, [], []",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "int is_polyline(std::vector<Point>* points, \n                           std::vector<double>* params)",
          "code": "int NurbsCurve::is_polyline(std::vector<Point>* points, \n                           std::vector<double>* params) const {\n    if (!is_valid()) return 0;\n    \n    // Degree 1 curves are polylines\n    if (m_order == 2) {\n        if (points) {\n            points->clear();\n            points->reserve(m_cv_count);\n            for (int i = 0; i < m_cv_count; i++) {\n                points->push_back(get_cv(i));\n            }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.to_polyline_adaptive",
      "implementations": {
        "python": {
          "sig": "to_polyline_adaptive(angle_tolerance: float = 0.1, \n                            min_edge_length: float = 0.0,\n                            max_edge_length: float = 0.0) -> Tuple[List[Point], List[float]]",
          "code": "def to_polyline_adaptive(self, angle_tolerance: float = 0.1, \n                            min_edge_length: float = 0.0,\n                            max_edge_length: float = 0.0) -> Tuple[List[Point], List[float]]:\n\n        \"\"\"Convert curve to polyline with adaptive sampling (curvature-based).\n        \n        Parameters\n        ----------\n        angle_tolerance : float, optional\n            Maximum angle between segments in radians. Defaults to 0.1.\n        min_edge_length : float, optional\n            Minimum distance between points. Defaults to 0.0.\n        max_edge_length : float, optional\n            Maximum distance between points. Defaults to 0.0 (no limit).\n            \n        Returns\n        -------\n        tuple of (list of Point, list of float)\n            Points and parameters.\n        \"\"\"\n        if not self.is_valid():\n            return [], []",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool to_polyline_adaptive(std::vector<Point>& points,\n                                     std::vector<double>* params,\n                                     double angle_tolerance,\n                                     double min_edge_length,\n                                     double max_edge_length)",
          "code": "bool NurbsCurve::to_polyline_adaptive(std::vector<Point>& points,\n                                     std::vector<double>* params,\n                                     double angle_tolerance,\n                                     double min_edge_length,\n                                     double max_edge_length) const {\n    points.clear();\n    if (params) params->clear();\n    \n    if (!is_valid()) return false;\n    if (angle_tolerance <= 0.0) angle_tolerance = 0.1;  // ~5.7 degrees\n    \n    auto [t0, t1] = domain();\n    double curve_len = length();\n    \n    // Set reasonable defaults for edge lengths if not specified\n    if (max_edge_length <= 0.0) {\n        max_edge_length = curve_len / 10.0;  // At least 10 segments\n    }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.span_is_linear",
      "implementations": {
        "python": {
          "sig": "span_is_linear(span_index: int, min_length: float = 0.0, \n                      tolerance: float = None) -> bool",
          "code": "def span_is_linear(self, span_index: int, min_length: float = 0.0, \n                      tolerance: float = None) -> bool:\n\n        \"\"\"Check if span is linear within tolerance.\n        \n        Parameters\n        ----------\n        span_index : int\n            Index of the span.\n        min_length : float, optional\n            Minimum length to consider. Defaults to 0.0.\n        tolerance : float, optional\n            Tolerance for linearity. Defaults to Tolerance.ZERO_TOLERANCE.\n            \n        Returns\n        -------\n        bool\n            True if span is linear.\n        \"\"\"\n        if tolerance is None:\n            tolerance = Tolerance.ZERO_TOLERANCE",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool span_is_linear(int span_index, double min_length, double tolerance,\n                               Point* line_start, Point* line_end)",
          "code": "bool NurbsCurve::span_is_linear(int span_index, double min_length, double tolerance,\n                               Point* line_start, Point* line_end) const {\n    bool is_lin = span_is_linear(span_index, min_length, tolerance);\n    \n    if (is_lin && line_start && line_end) {\n        *line_start = get_cv(span_index);\n        *line_end = get_cv(span_index + m_order - 1);\n    }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.span_is_singular",
      "implementations": {
        "python": {
          "sig": "span_is_singular(span_index: int) -> bool",
          "code": "def span_is_singular(self, span_index: int) -> bool:\n\n        \"\"\"Check if span is singular (collapsed to a point).\n        \n        Parameters\n        ----------\n        span_index : int\n            Index of the span.\n            \n        Returns\n        -------\n        bool\n            True if span is singular.\n        \"\"\"\n        if not self.is_valid():\n            return False\n        \n        spans = self.get_span_vector()\n        if span_index < 0 or span_index >= len(spans) - 1:\n            return False",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool span_is_singular(int span_index)",
          "code": "bool NurbsCurve::span_is_singular(int span_index) const {\n    if (!is_valid()) return false;\n    if (span_index < 0 || span_index >= m_cv_count - m_order) return false;\n    \n    // Check if span is non-empty\n    int ki = span_index + m_order - 2;\n    if (m_knot[ki] >= m_knot[ki + 1]) return true; // Empty span\n    \n    // Check if all CVs in span are coincident\n    Point p0 = get_cv(span_index);\n    for (int i = 1; i < m_order; i++) {\n        Point p = get_cv(span_index + i);\n        if (p0.distance(p) > Tolerance::ZERO_TOLERANCE) {\n            return false;\n        }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.repair_bad_knots",
      "implementations": {
        "python": {
          "sig": "repair_bad_knots(tolerance: float = 0.0, repair: bool = True) -> bool",
          "code": "def repair_bad_knots(self, tolerance: float = 0.0, repair: bool = True) -> bool:\n\n        \"\"\"Repair bad knots (too close, high multiplicity).\n        \n        Parameters\n        ----------\n        tolerance : float, optional\n            Knot tolerance. Defaults to 0.0.\n        repair : bool, optional\n            If True, repairs knots; if False, only checks. Defaults to True.\n            \n        Returns\n        -------\n        bool\n            True if knots are valid or repaired.\n        \"\"\"\n        if not self.is_valid():\n            return False\n        \n        if repair:\n            return self.clean_knots(tolerance)",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool repair_bad_knots(double knot_tolerance, bool repair)",
          "code": "bool NurbsCurve::repair_bad_knots(double knot_tolerance, bool repair) {\n    if (!is_valid()) return false;\n    if (knot_tolerance < 0.0) knot_tolerance = 0.0;\n    \n    bool found_bad_knots = false;\n    int kc = knot_count();\n    \n    // Check for knots that are too close together\n    for (int i = 1; i < kc; i++) {\n        double delta = m_knot[i] - m_knot[i-1];\n        if (delta < 0.0) {\n            found_bad_knots = true;\n            break;\n        }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.make_piecewise_bezier",
      "implementations": {
        "python": {
          "sig": "make_piecewise_bezier(set_end_weights_to_one: bool = False) -> bool",
          "code": "def make_piecewise_bezier(self, set_end_weights_to_one: bool = False) -> bool:\n\n        \"\"\"Make curve have piecewise bezier spans.\n        \n        Parameters\n        ----------\n        set_end_weights_to_one : bool, optional\n            Whether to set end weights to 1. Defaults to False.\n            \n        Returns\n        -------\n        bool\n            True if successful.\n        \"\"\"\n        if not self.is_valid():\n            return False\n        \n        # This is a complex operation requiring knot insertion\n        # Simplified implementation\n        if set_end_weights_to_one and self.m_is_rat:\n            self.set_weight(0, 1.0)",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool make_piecewise_bezier(bool set_end_weights_to_one)",
          "code": "bool NurbsCurve::make_piecewise_bezier(bool set_end_weights_to_one) {\n    if (has_bezier_spans()) return true;\n    if (!is_valid()) return false;\n    \n    // First clamp the ends\n    if (!clamp_end(2)) return false;\n    \n    // For each span, insert knots to achieve multiplicity = degree\n    int p = degree();\n    std::vector<double> span_params = get_span_vector();\n    \n    // Insert knots at each interior span parameter\n    for (size_t i = 1; i < span_params.size() - 1; i++) {\n        double t = span_params[i];\n        \n        // Count current multiplicity\n        int mult = 0;\n        for (int j = 0; j < knot_count(); j++) {\n            if (std::abs(m_knot[j] - t) < Tolerance::ZERO_TOLERANCE) {\n                mult++;\n            }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.change_closed_curve_seam",
      "implementations": {
        "python": {
          "sig": "change_closed_curve_seam(t: float) -> bool",
          "code": "def change_closed_curve_seam(self, t: float) -> bool:\n\n        \"\"\"Change seam point of closed periodic curve.\n        \n        Parameters\n        ----------\n        t : float\n            New seam parameter.\n            \n        Returns\n        -------\n        bool\n            True if successful.\n        \"\"\"\n        if not self.is_valid() or not self.is_closed():\n            return False\n        \n        t0, t1 = self.domain()\n        if t < t0 or t > t1:\n            return False",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool change_closed_curve_seam(double t)",
          "code": "bool NurbsCurve::change_closed_curve_seam(double t) {\n    if (!is_valid()) return false;\n    if (!is_closed()) return false;\n    \n    auto [t0, t1] = domain();\n    if (t <= t0 || t >= t1) return false;\n    \n    // For periodic curves, this would rotate the control points\n    // Simplified implementation: insert full multiplicity knot and reorganize\n    if (!is_periodic()) return false;\n    \n    // Find the span\n    int span = find_span(t);\n    if (span < 0) return false;\n    \n    // Insert knot with full multiplicity\n    int p = degree();\n    if (!insert_knot(t, p)) return false;\n    \n    // Now reorganize CVs and knots to make t the new seam\n    // This is complex - simplified version\n    return true;\n}",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.get_parameter_tolerance",
      "implementations": {
        "python": {
          "sig": "get_parameter_tolerance(t: float) -> Tuple[float, float]",
          "code": "def get_parameter_tolerance(self, t: float) -> Tuple[float, float]:\n\n        \"\"\"Get parameter tolerance at point.\n        \n        Parameters\n        ----------\n        t : float\n            Parameter value.\n            \n        Returns\n        -------\n        tuple of (float, float)\n            (t_minus, t_plus) tolerance bounds.\n        \"\"\"\n        if not self.is_valid():\n            return (0.0, 0.0)\n        \n        # Simple implementation: use small epsilon\n        eps = Tolerance.ZERO_TOLERANCE * 10.0\n        return (t - eps, t + eps)",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool get_parameter_tolerance(double t, double* tminus, double* tplus)",
          "code": "bool NurbsCurve::get_parameter_tolerance(double t, double* tminus, double* tplus) const {\n    if (!is_valid() || !tminus || !tplus) return false;\n    \n    auto [t0, t1] = domain();\n    if (t < t0 || t > t1) return false;\n    \n    // Simple implementation: use knot spacing as tolerance\n    double delta = (t1 - t0) * std::sqrt(std::numeric_limits<double>::epsilon());\n    \n    *tminus = t - delta;\n    *tplus = t + delta;\n    \n    // Clamp to domain\n    if (*tminus < t0) *tminus = t0;\n    if (*tplus > t1) *tplus = t1;\n    \n    return true;\n}",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.convert_span_to_bezier",
      "implementations": {
        "python": {
          "sig": "convert_span_to_bezier(span_index: int) -> Optional[List[Point]]",
          "code": "def convert_span_to_bezier(self, span_index: int) -> Optional[List[Point]]:\n\n        \"\"\"Convert a NURBS span to Bezier curve (OpenNURBS-compatible).\n        \n        Parameters\n        ----------\n        span_index : int\n            Index of the span to convert (0 <= span_index <= cv_count - order).\n            \n        Returns\n        -------\n        list of Point or None\n            Bezier control points, or None if invalid.\n            \n        Notes\n        -----\n        This implements the OpenNURBS algorithm:\n        1. Extract CVs for the span\n        2. Apply de Boor's algorithm to convert to Bezier basis\n        3. Return the resulting Bezier control points",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "bool convert_span_to_bezier(int span_index, std::vector<Point>& bezier_cvs)",
          "code": "bool NurbsCurve::convert_span_to_bezier(int span_index, std::vector<Point>& bezier_cvs) const {\n    bezier_cvs.clear();\n    \n    if (!is_valid()) return false;\n    if (span_index < 0 || span_index > m_cv_count - m_order) return false;\n    \n    // Check if span is non-empty\n    int ki0 = span_index + m_order - 2;\n    int ki1 = span_index + m_order - 1;\n    if (ki0 >= knot_count() || ki1 >= knot_count()) return false;\n    if (m_knot[ki0] >= m_knot[ki1]) return false;  // Empty span\n    \n    // For a proper Bezier extraction, we need the span to have full multiplicity knots\n    // Simple implementation: just return the order CVs that define this span\n    bezier_cvs.reserve(m_order);\n    for (int i = 0; i < m_order; i++) {\n        bezier_cvs.push_back(get_cv(span_index + i));\n    }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.intersect_plane_production",
      "implementations": {
        "python": {
          "sig": "intersect_plane_production(plane: Plane, tolerance: float = None) -> List[float]",
          "code": "def intersect_plane_production(self, plane: Plane, tolerance: float = None) -> List[float]:\n\n        \"\"\"Curve-plane intersection using production CAD kernel method.\n        \n        This is the INDUSTRY STANDARD method used in Rhino, Parasolid, ACIS, etc.\n        \n        Parameters\n        ----------\n        plane : Plane\n            The plane to intersect with.\n        tolerance : float, optional\n            Intersection tolerance. Defaults to Tolerance.ZERO_TOLERANCE.\n            \n        Returns\n        -------\n        list of float\n            Parameter values where curve intersects plane.\n            \n        Notes\n        -----\n        **Algorithm (Industry Standard - Subdivision + Newton Hybrid):**",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "std::vector<double> intersect_plane_production(const Plane& plane, double tolerance)",
          "code": "std::vector<double> NurbsCurve::intersect_plane_production(const Plane& plane, double tolerance) const {\n    if (!is_valid()) return {}",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.signed_distance_derivative",
      "implementations": {
        "python": {
          "sig": "signed_distance_derivative(t: float) -> float",
          "code": "def signed_distance_derivative(t: float) -> float:\n\n            \"\"\"Derivative of signed distance: df/dt = n \u00b7 C'(t)\"\"\"\n            tan = self.tangent_at(t)\n            return plane.z_axis.dot(tan)\n        \n        results = []\n        \n        # Process each Bezier span separately\n        spans = self.get_span_vector()\n        \n        for span_idx in range(len(spans) - 1):\n            span_t0 = spans[span_idx]\n            span_t1 = spans[span_idx + 1]\n            \n            # Skip degenerate spans\n            if abs(span_t1 - span_t0) < tolerance:\n                continue\n            \n            # Get Bezier representation of this span\n            bezier_cvs = self.convert_span_to_bezier(span_idx)",
          "file": "nurbscurve.py"
        }
      }
    },
    {
      "name": "NurbsCurve.subdivide_and_solve",
      "implementations": {
        "python": {
          "sig": "subdivide_and_solve(ta: float, tb: float, depth: int)",
          "code": "def subdivide_and_solve(ta: float, tb: float, depth: int):\n\n                \"\"\"Recursively subdivide until nearly linear, then solve\"\"\"\n                \n                MAX_DEPTH = 30\n                if depth > MAX_DEPTH:\n                    return\n                \n                # Evaluate at endpoints\n                pa = self.point_at(ta)\n                pb = self.point_at(tb)\n                da = signed_distance(pa)\n                db = signed_distance(pb)\n                \n                # Check if root exists in this interval\n                if da * db > tolerance * tolerance:\n                    # Same sign, no root (or even number of roots)\n                    return\n                \n                # Check if segment is nearly linear (subdivision stopping criterion)\n                segment_length = pa.distance(pb)",
          "file": "nurbscurve.py"
        }
      }
    },
    {
      "name": "NurbsCurve.__jsondump__",
      "implementations": {
        "python": {
          "sig": "__jsondump__()",
          "code": "def __jsondump__(self):\n\n        \"\"\"Return a JSON-serializable dictionary representation.\"\"\"\n        return {\n            \"guid\": self.guid,\n            \"name\": self.name,\n            \"m_dim\": int(self.m_dim),\n            \"m_is_rat\": int(self.m_is_rat),\n            \"m_order\": int(self.m_order),\n            \"m_cv_count\": int(self.m_cv_count),\n            \"m_cv_stride\": int(self.m_cv_stride),\n            \"m_knot\": self.m_knot.tolist() if hasattr(self.m_knot, 'tolist') else list(self.m_knot),\n            \"m_cv\": self.m_cv.tolist() if hasattr(self.m_cv, 'tolist') else list(self.m_cv),\n        }\n\n    @classmethod\n    def __jsonload__(cls, data):\n        \"\"\"Create NurbsCurve from JSON dictionary.\"\"\"\n        curve = cls()\n        curve.guid = data.get(\"guid\", curve.guid)\n        curve.name = data.get(\"name\", curve.name)",
          "file": "nurbscurve.py"
        }
      }
    },
    {
      "name": "NurbsCurve.__jsonload__",
      "implementations": {
        "python": {
          "sig": "__jsonload__(cls, data)",
          "code": "def __jsonload__(cls, data):\n\n        \"\"\"Create NurbsCurve from JSON dictionary.\"\"\"\n        curve = cls()\n        curve.guid = data.get(\"guid\", curve.guid)\n        curve.name = data.get(\"name\", curve.name)\n        curve.m_dim = data.get(\"m_dim\", 0)\n        curve.m_is_rat = data.get(\"m_is_rat\", 0)\n        curve.m_order = data.get(\"m_order\", 0)\n        curve.m_cv_count = data.get(\"m_cv_count\", 0)\n        curve.m_cv_stride = data.get(\"m_cv_stride\", 0)\n        curve.m_knot = np.array(data.get(\"m_knot\", []), dtype=np.float64)\n        curve.m_cv = np.array(data.get(\"m_cv\", []), dtype=np.float64)\n        return curve\n\n    def json_dump(self, filepath):\n        \"\"\"Write JSON to file.\"\"\"\n        import json\n        with open(filepath, 'w') as f:\n            json.dump(self.__jsondump__(), f, indent=2)",
          "file": "nurbscurve.py"
        }
      }
    },
    {
      "name": "NurbsCurve.json_dump",
      "implementations": {
        "python": {
          "sig": "json_dump(filepath)",
          "code": "def json_dump(self, filepath):\n\n        \"\"\"Write JSON to file.\"\"\"\n        import json\n        with open(filepath, 'w') as f:\n            json.dump(self.__jsondump__(), f, indent=2)\n\n    @classmethod\n    def json_load(cls, filepath):\n        \"\"\"Read JSON from file.\"\"\"\n        import json\n        with open(filepath, 'r') as f:\n            data = json.load(f)\n        return cls.__jsonload__(data)\n\n    ###########################################################################################\n    # Protobuf Serialization\n    ###########################################################################################\n\n    def protobuf_dump(self, filepath):\n        \"\"\"Write protobuf binary to file.\"\"\"",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "void json_dump(const std::string& filename)",
          "code": "void NurbsCurve::json_dump(const std::string& filename) const {\n    std::ofstream file(filename);\n    file << jsondump().dump(4);\n}",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "json_dump(filename: &str)",
          "code": "pub fn json_dump(&self, filename: &str) {\n        use std::fs::File;\n        use std::io::Write;\n        if let Ok(json) = serde_json::to_string_pretty(self) {\n            if let Ok(mut file) = File::create(filename) {\n                let _ = file.write_all(json.as_bytes());\n            }\n        }\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.json_load",
      "implementations": {
        "python": {
          "sig": "json_load(cls, filepath)",
          "code": "def json_load(cls, filepath):\n\n        \"\"\"Read JSON from file.\"\"\"\n        import json\n        with open(filepath, 'r') as f:\n            data = json.load(f)\n        return cls.__jsonload__(data)\n\n    ###########################################################################################\n    # Protobuf Serialization\n    ###########################################################################################\n\n    def protobuf_dump(self, filepath):\n        \"\"\"Write protobuf binary to file.\"\"\"\n        try:\n            from .proto import nurbscurve_pb2\n            proto = nurbscurve_pb2.NurbsCurve()\n            proto.guid = self.guid\n            proto.name = self.name\n            proto.dimension = int(self.m_dim)\n            proto.is_rational = bool(self.m_is_rat)",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "NurbsCurve json_load(const std::string& filename)",
          "code": "NurbsCurve NurbsCurve::json_load(const std::string& filename) {\n    std::ifstream file(filename);\n    nlohmann::json data;\n    file >> data;\n    return jsonload(data);\n}",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "json_load(filename: &str) -> Self",
          "code": "pub fn json_load(filename: &str) -> Self {\n        use std::fs::File;\n        use std::io::Read;\n        let mut file = match File::open(filename) {\n            Ok(f) => f,\n            Err(_) => return Self::default(),\n        };\n        let mut contents = String::new();\n        if file.read_to_string(&mut contents).is_err() {\n            return Self::default();\n        }\n        serde_json::from_str(&contents).unwrap_or_else(|_| Self::default())\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.protobuf_dump",
      "implementations": {
        "python": {
          "sig": "protobuf_dump(filepath)",
          "code": "def protobuf_dump(self, filepath):\n\n        \"\"\"Write protobuf binary to file.\"\"\"\n        try:\n            from .proto import nurbscurve_pb2\n            proto = nurbscurve_pb2.NurbsCurve()\n            proto.guid = self.guid\n            proto.name = self.name\n            proto.dimension = int(self.m_dim)\n            proto.is_rational = bool(self.m_is_rat)\n            proto.order = int(self.m_order)\n            proto.cv_count = int(self.m_cv_count)\n            proto.cv_stride = int(self.m_cv_stride)\n            proto.knots.extend(self.m_knot.tolist() if hasattr(self.m_knot, 'tolist') else list(self.m_knot))\n            proto.cvs.extend(self.m_cv.tolist() if hasattr(self.m_cv, 'tolist') else list(self.m_cv))\n            with open(filepath, 'wb') as f:\n                f.write(proto.SerializeToString())\n        except ImportError:\n            raise ImportError(\"protobuf not available - run ./protobuf.sh to install\")\n\n    @classmethod",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "void protobuf_dump(const std::string& filename)",
          "code": "void NurbsCurve::protobuf_dump(const std::string& filename) const {\n    // Stub: uses JSON fallback (replace .bin with .json)\n    std::string json_filename = filename;\n    size_t pos = json_filename.rfind(\".bin\");\n    if (pos != std::string::npos) {\n        json_filename.replace(pos, 4, \".json\");\n    }",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "protobuf_dump(filename: &str)",
          "code": "pub fn protobuf_dump(&self, filename: &str) {\n        // For now, just use JSON as a fallback\n        self.json_dump(&filename.replace(\".bin\", \".json\"));\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.protobuf_load",
      "implementations": {
        "python": {
          "sig": "protobuf_load(cls, filepath)",
          "code": "def protobuf_load(cls, filepath):\n\n        \"\"\"Read protobuf binary from file.\"\"\"\n        try:\n            from .proto import nurbscurve_pb2\n            proto = nurbscurve_pb2.NurbsCurve()\n            with open(filepath, 'rb') as f:\n                proto.ParseFromString(f.read())\n            curve = cls()\n            curve.guid = proto.guid\n            curve.name = proto.name\n            curve.m_dim = proto.dimension\n            curve.m_is_rat = 1 if proto.is_rational else 0\n            curve.m_order = proto.order\n            curve.m_cv_count = proto.cv_count\n            curve.m_cv_stride = proto.cv_stride\n            curve.m_knot = np.array(list(proto.knots), dtype=np.float64)\n            curve.m_cv = np.array(list(proto.cvs), dtype=np.float64)\n            return curve\n        except ImportError:\n            raise ImportError(\"protobuf not available - run ./protobuf.sh to install\")",
          "file": "nurbscurve.py"
        },
        "cpp": {
          "sig": "NurbsCurve protobuf_load(const std::string& filename)",
          "code": "NurbsCurve NurbsCurve::protobuf_load(const std::string& filename) {\n    // Stub: uses JSON fallback (replace .bin with .json)\n    std::string json_filename = filename;\n    size_t pos = json_filename.rfind(\".bin\");\n    if (pos != std::string::npos) {\n        json_filename.replace(pos, 4, \".json\");\n    }",
          "file": "nurbscurve.cpp"
        },
        "rust": {
          "sig": "protobuf_load(filename: &str) -> Self",
          "code": "pub fn protobuf_load(filename: &str) -> Self {\n        // For now, just use JSON as a fallback\n        Self::json_load(&filename.replace(\".bin\", \".json\"))\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "Plane.__init__",
      "implementations": {
        "python": {
          "sig": "__init__(origin=None, x_axis=None, y_axis=None, name=\"my_plane\", width=1.0)",
          "code": "def __init__(self, origin=None, x_axis=None, y_axis=None, name=\"my_plane\", width=1.0):\n\n        self.guid = str(uuid.uuid4())\n        self.name = name\n        self.width = width\n        self.xform = Xform.identity()\n\n        if origin is None:\n            self._origin = Point(0.0, 0.0, 0.0)\n        else:\n            self._origin = origin\n\n        if x_axis is None:\n            self._x_axis = Vector.x_axis()\n        else:\n            self._x_axis = x_axis\n            self._x_axis.normalize_self()\n\n        if y_axis is None:\n            self._y_axis = Vector.y_axis()\n        else:",
          "file": "plane.py"
        }
      }
    },
    {
      "name": "Plane._update_equation",
      "implementations": {
        "python": {
          "sig": "_update_equation()",
          "code": "def _update_equation(self):\n\n        \"\"\"Update plane equation coefficients from z_axis and origin.\"\"\"\n        self._a = self._z_axis[0]\n        self._b = self._z_axis[1]\n        self._c = self._z_axis[2]\n        self._d = -(\n            self._a * self._origin[0]\n            + self._b * self._origin[1]\n            + self._c * self._origin[2]\n        )\n\n    @property\n    def origin(self):\n        \"\"\"Get the origin point.\"\"\"\n        return self._origin\n\n    @property\n    def x_axis(self):\n        \"\"\"Get the X-axis vector.\"\"\"\n        return self._x_axis",
          "file": "plane.py"
        }
      }
    },
    {
      "name": "Plane.origin",
      "implementations": {
        "python": {
          "sig": "origin()",
          "code": "def origin(self):\n\n        \"\"\"Get the origin point.\"\"\"\n        return self._origin\n\n    @property\n    def x_axis(self):\n        \"\"\"Get the X-axis vector.\"\"\"\n        return self._x_axis\n\n    @property\n    def y_axis(self):\n        \"\"\"Get the Y-axis vector.\"\"\"\n        return self._y_axis\n\n    @property\n    def z_axis(self):\n        \"\"\"Get the Z-axis vector (normal).\"\"\"\n        return self._z_axis\n\n    @property",
          "file": "plane.py"
        },
        "rust": {
          "sig": "origin() -> Point",
          "code": "pub fn origin(&self) -> Point {\n        self._origin.clone()\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.x_axis",
      "implementations": {
        "python": {
          "sig": "x_axis()",
          "code": "def x_axis(self):\n\n        \"\"\"Get the X-axis vector.\"\"\"\n        return self._x_axis\n\n    @property\n    def y_axis(self):\n        \"\"\"Get the Y-axis vector.\"\"\"\n        return self._y_axis\n\n    @property\n    def z_axis(self):\n        \"\"\"Get the Z-axis vector (normal).\"\"\"\n        return self._z_axis\n\n    @property\n    def a(self):\n        \"\"\"Get plane equation coefficient a.\"\"\"\n        return self._a\n\n    @property",
          "file": "plane.py"
        },
        "rust": {
          "sig": "x_axis() -> Vector",
          "code": "pub fn x_axis(&self) -> Vector {\n        self._x_axis.clone()\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.y_axis",
      "implementations": {
        "python": {
          "sig": "y_axis()",
          "code": "def y_axis(self):\n\n        \"\"\"Get the Y-axis vector.\"\"\"\n        return self._y_axis\n\n    @property\n    def z_axis(self):\n        \"\"\"Get the Z-axis vector (normal).\"\"\"\n        return self._z_axis\n\n    @property\n    def a(self):\n        \"\"\"Get plane equation coefficient a.\"\"\"\n        return self._a\n\n    @property\n    def b(self):\n        \"\"\"Get plane equation coefficient b.\"\"\"\n        return self._b\n\n    @property",
          "file": "plane.py"
        },
        "rust": {
          "sig": "y_axis() -> Vector",
          "code": "pub fn y_axis(&self) -> Vector {\n        self._y_axis.clone()\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.z_axis",
      "implementations": {
        "python": {
          "sig": "z_axis()",
          "code": "def z_axis(self):\n\n        \"\"\"Get the Z-axis vector (normal).\"\"\"\n        return self._z_axis\n\n    @property\n    def a(self):\n        \"\"\"Get plane equation coefficient a.\"\"\"\n        return self._a\n\n    @property\n    def b(self):\n        \"\"\"Get plane equation coefficient b.\"\"\"\n        return self._b\n\n    @property\n    def c(self):\n        \"\"\"Get plane equation coefficient c.\"\"\"\n        return self._c\n\n    @property",
          "file": "plane.py"
        },
        "rust": {
          "sig": "z_axis() -> Vector",
          "code": "pub fn z_axis(&self) -> Vector {\n        self._z_axis.clone()\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.a",
      "implementations": {
        "python": {
          "sig": "a()",
          "code": "def a(self):\n\n        \"\"\"Get plane equation coefficient a.\"\"\"\n        return self._a\n\n    @property\n    def b(self):\n        \"\"\"Get plane equation coefficient b.\"\"\"\n        return self._b\n\n    @property\n    def c(self):\n        \"\"\"Get plane equation coefficient c.\"\"\"\n        return self._c\n\n    @property\n    def d(self):\n        \"\"\"Get plane equation coefficient d.\"\"\"\n        return self._d\n\n    @staticmethod",
          "file": "plane.py"
        },
        "rust": {
          "sig": "a() -> f64",
          "code": "pub fn a(&self) -> f64 {\n        self._a\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.b",
      "implementations": {
        "python": {
          "sig": "b()",
          "code": "def b(self):\n\n        \"\"\"Get plane equation coefficient b.\"\"\"\n        return self._b\n\n    @property\n    def c(self):\n        \"\"\"Get plane equation coefficient c.\"\"\"\n        return self._c\n\n    @property\n    def d(self):\n        \"\"\"Get plane equation coefficient d.\"\"\"\n        return self._d\n\n    @staticmethod\n    def from_point_normal(point, normal):\n        \"\"\"Create a plane from a point and normal vector.\n\n        Parameters\n        ----------",
          "file": "plane.py"
        },
        "rust": {
          "sig": "b() -> f64",
          "code": "pub fn b(&self) -> f64 {\n        self._b\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.c",
      "implementations": {
        "python": {
          "sig": "c()",
          "code": "def c(self):\n\n        \"\"\"Get plane equation coefficient c.\"\"\"\n        return self._c\n\n    @property\n    def d(self):\n        \"\"\"Get plane equation coefficient d.\"\"\"\n        return self._d\n\n    @staticmethod\n    def from_point_normal(point, normal):\n        \"\"\"Create a plane from a point and normal vector.\n\n        Parameters\n        ----------\n        point : Point\n            Point on the plane.\n        normal : Vector\n            Normal vector of the plane.",
          "file": "plane.py"
        },
        "rust": {
          "sig": "c() -> f64",
          "code": "pub fn c(&self) -> f64 {\n        self._c\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.d",
      "implementations": {
        "python": {
          "sig": "d()",
          "code": "def d(self):\n\n        \"\"\"Get plane equation coefficient d.\"\"\"\n        return self._d\n\n    @staticmethod\n    def from_point_normal(point, normal):\n        \"\"\"Create a plane from a point and normal vector.\n\n        Parameters\n        ----------\n        point : Point\n            Point on the plane.\n        normal : Vector\n            Normal vector of the plane.\n\n        Returns\n        -------\n        Plane\n            The constructed plane.\n        \"\"\"",
          "file": "plane.py"
        },
        "rust": {
          "sig": "d() -> f64",
          "code": "pub fn d(&self) -> f64 {\n        self._d\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.from_point_normal",
      "implementations": {
        "python": {
          "sig": "from_point_normal(point, normal)",
          "code": "def from_point_normal(point, normal):\n\n        \"\"\"Create a plane from a point and normal vector.\n\n        Parameters\n        ----------\n        point : Point\n            Point on the plane.\n        normal : Vector\n            Normal vector of the plane.\n\n        Returns\n        -------\n        Plane\n            The constructed plane.\n        \"\"\"\n        plane = Plane.__new__(Plane)\n        plane.guid = str(uuid.uuid4())\n        plane.name = \"my_plane\"\n        plane.width = 1.0\n        plane.xform = Xform.identity()",
          "file": "plane.py"
        },
        "cpp": {
          "sig": "Plane from_point_normal(Point& point, Vector& normal)",
          "code": "Plane Plane::from_point_normal(Point& point, Vector& normal) {\n    Plane plane;\n    plane._origin = point;\n    plane._z_axis = normal;\n    plane._z_axis.normalize_self();\n    plane._x_axis.perpendicular_to(plane._z_axis);\n    plane._x_axis.normalize_self();\n    plane._y_axis = plane._z_axis.cross(plane._x_axis);\n    plane._y_axis.normalize_self();\n    \n    plane._a = plane._z_axis[0];\n    plane._b = plane._z_axis[1];\n    plane._c = plane._z_axis[2];\n    plane._d = -(plane._a * plane._origin[0] + plane._b * plane._origin[1] + plane._c * plane._origin[2]);\n    return plane;\n}",
          "file": "plane.cpp"
        },
        "rust": {
          "sig": "from_point_normal(point: Point, normal: Vector) -> Self",
          "code": "pub fn from_point_normal(point: Point, normal: Vector) -> Self {\n        let origin = point.clone();\n        let mut z_axis = normal;\n        z_axis.normalize();\n        let mut x_axis = Vector::default();\n        x_axis.perpendicular_to(&z_axis);\n        x_axis.normalize();\n        let mut y_axis = z_axis.cross(&x_axis);\n        y_axis.normalize();\n\n        let a = z_axis[0];\n        let b = z_axis[1];\n        let c = z_axis[2];\n        let d = -(a * origin[0] + b * origin[1] + c * origin[2]);\n\n        Self {\n            guid: Uuid::new_v4().to_string(),\n            name: \"my_plane\".to_string(),\n            width: 1.0,\n            _origin: origin,\n            _x_axis: x_axis,\n            _y_axis: y_axis,\n            _z_axis: z_axis,\n            _a: a,\n            _b: b,\n            _",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.from_points",
      "implementations": {
        "python": {
          "sig": "from_points(points)",
          "code": "def from_points(points):\n\n        \"\"\"Create a plane from three or more points.\n\n        Parameters\n        ----------\n        points : list of Point\n            List of at least 3 points.\n\n        Returns\n        -------\n        Plane\n            The constructed plane.\n        \"\"\"\n        if len(points) < 3:\n            return Plane()\n\n        plane = Plane.__new__(Plane)\n        plane.guid = str(uuid.uuid4())\n        plane.name = \"my_plane\"\n        plane.width = 1.0",
          "file": "plane.py"
        },
        "cpp": {
          "sig": "Plane from_points(std::vector<Point>& points)",
          "code": "Plane Plane::from_points(std::vector<Point>& points) {\n    if (points.size() < 3) {\n        return Plane();\n    }",
          "file": "plane.cpp"
        },
        "rust": {
          "sig": "from_points(points: Vec<Point>) -> Self",
          "code": "pub fn from_points(points: Vec<Point>) -> Self {\n        if points.len() < 3 {\n            return Self::default();\n        }\n\n        let point1 = &points[0];\n        let point2 = &points[1];\n        let point3 = &points[2];\n        let v1 = point2.clone() - point1.clone();\n        let v2 = point3.clone() - point1.clone();\n        let mut z_axis = v1.cross(&v2);\n        z_axis.normalize();\n        let mut x_axis = Vector::default();\n        x_axis.perpendicular_to(&z_axis);\n        x_axis.normalize();\n        let mut y_axis = z_axis.cross(&x_axis);\n        y_axis.normalize();\n        let origin = point1.clone();\n\n        let a = z_axis[0];\n        let b = z_axis[1];\n        let c = z_axis[2];\n        let d = -(a * origin[0] + b * origin[1] + c * origin[2]);\n\n        Self {",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.from_two_points",
      "implementations": {
        "python": {
          "sig": "from_two_points(point1, point2)",
          "code": "def from_two_points(point1, point2):\n\n        \"\"\"Create a plane from two points.\n\n        Parameters\n        ----------\n        point1 : Point\n            First point.\n        point2 : Point\n            Second point.\n\n        Returns\n        -------\n        Plane\n            The constructed plane.\n        \"\"\"\n        plane = Plane.__new__(Plane)\n        plane.guid = str(uuid.uuid4())\n        plane.name = \"my_plane\"\n        plane.width = 1.0\n        plane.xform = Xform.identity()",
          "file": "plane.py"
        },
        "cpp": {
          "sig": "Plane from_two_points(Point& point1, Point& point2)",
          "code": "Plane Plane::from_two_points(Point& point1, Point& point2) {\n    Plane plane;\n    plane._origin = point1;\n    \n    Vector direction = point2 - point1;\n    direction.normalize_self();\n    plane._z_axis.perpendicular_to(direction);\n    plane._z_axis.normalize_self();\n    \n    plane._x_axis = direction;\n    \n    plane._y_axis = plane._z_axis.cross(plane._x_axis);\n    plane._y_axis.normalize_self();\n    \n    plane._a = plane._z_axis[0];\n    plane._b = plane._z_axis[1];\n    plane._c = plane._z_axis[2];\n    plane._d = -(plane._a * plane._origin[0] + plane._b * plane._origin[1] + plane._c * plane._origin[2]);\n    return plane;\n}",
          "file": "plane.cpp"
        },
        "rust": {
          "sig": "from_two_points(point1: Point, point2: Point) -> Self",
          "code": "pub fn from_two_points(point1: Point, point2: Point) -> Self {\n        let origin = point1.clone();\n\n        let mut direction = point2.clone() - point1.clone();\n        direction.normalize();\n        let mut z_axis = Vector::default();\n        z_axis.perpendicular_to(&direction);\n        z_axis.normalize();\n\n        let x_axis = direction;\n        let mut y_axis = z_axis.cross(&x_axis);\n        y_axis.normalize();\n\n        let a = z_axis[0];\n        let b = z_axis[1];\n        let c = z_axis[2];\n        let d = -(a * origin[0] + b * origin[1] + c * origin[2]);\n\n        Self {\n            guid: Uuid::new_v4().to_string(),\n            name: \"my_plane\".to_string(),\n            width: 1.0,\n            _origin: origin,\n            _x_axis: x_axis,\n            _y_axis: y_axis,\n            _",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.xy_plane",
      "implementations": {
        "python": {
          "sig": "xy_plane()",
          "code": "def xy_plane():\n\n        \"\"\"Create the XY plane.\n\n        Returns\n        -------\n        Plane\n            XY plane at origin.\n        \"\"\"\n        plane = Plane.__new__(Plane)\n        plane.guid = str(uuid.uuid4())\n        plane.name = \"xy_plane\"\n        plane.width = 1.0\n        plane.xform = Xform.identity()\n        plane._origin = Point(0.0, 0.0, 0.0)\n        plane._x_axis = Vector.x_axis()\n        plane._y_axis = Vector.y_axis()\n        plane._z_axis = Vector.z_axis()\n        plane._a = 0.0\n        plane._b = 0.0\n        plane._c = 1.0",
          "file": "plane.py"
        },
        "cpp": {
          "sig": "Plane xy_plane()",
          "code": "Plane Plane::xy_plane() {\n    Plane plane;\n    plane.name = \"xy_plane\";\n    plane._origin = Point(0.0, 0.0, 0.0);\n    plane._x_axis = Vector::x_axis();\n    plane._y_axis = Vector::y_axis();\n    plane._z_axis = Vector::z_axis();\n    plane._a = 0.0;\n    plane._b = 0.0;\n    plane._c = 1.0;\n    plane._d = 0.0;\n    return plane;\n}",
          "file": "plane.cpp"
        },
        "rust": {
          "sig": "xy_plane() -> Self",
          "code": "pub fn xy_plane() -> Self {\n        Self {\n            guid: Uuid::new_v4().to_string(),\n            name: \"xy_plane\".to_string(),\n            width: 1.0,\n            _origin: Point::new(0.0, 0.0, 0.0),\n            _x_axis: Vector::x_axis(),\n            _y_axis: Vector::y_axis(),\n            _z_axis: Vector::z_axis(),\n            _a: 0.0,\n            _b: 0.0,\n            _c: 1.0,\n            _d: 0.0,\n            xform: Xform::identity(),\n        }\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.yz_plane",
      "implementations": {
        "python": {
          "sig": "yz_plane()",
          "code": "def yz_plane():\n\n        \"\"\"Create the YZ plane.\n\n        Returns\n        -------\n        Plane\n            YZ plane at origin.\n        \"\"\"\n        plane = Plane.__new__(Plane)\n        plane.guid = str(uuid.uuid4())\n        plane.name = \"yz_plane\"\n        plane.width = 1.0\n        plane.xform = Xform.identity()\n        plane._origin = Point(0.0, 0.0, 0.0)\n        plane._x_axis = Vector.y_axis()\n        plane._y_axis = Vector.z_axis()\n        plane._z_axis = Vector.x_axis()\n        plane._a = 1.0\n        plane._b = 0.0\n        plane._c = 0.0",
          "file": "plane.py"
        },
        "cpp": {
          "sig": "Plane yz_plane()",
          "code": "Plane Plane::yz_plane() {\n    Plane plane;\n    plane.name = \"yz_plane\";\n    plane._origin = Point(0.0, 0.0, 0.0);\n    plane._x_axis = Vector::y_axis();\n    plane._y_axis = Vector::z_axis();\n    plane._z_axis = Vector::x_axis();\n    plane._a = 1.0;\n    plane._b = 0.0;\n    plane._c = 0.0;\n    plane._d = 0.0;\n    return plane;\n}",
          "file": "plane.cpp"
        },
        "rust": {
          "sig": "yz_plane() -> Self",
          "code": "pub fn yz_plane() -> Self {\n        Self {\n            guid: Uuid::new_v4().to_string(),\n            name: \"yz_plane\".to_string(),\n            width: 1.0,\n            _origin: Point::new(0.0, 0.0, 0.0),\n            _x_axis: Vector::y_axis(),\n            _y_axis: Vector::z_axis(),\n            _z_axis: Vector::x_axis(),\n            _a: 1.0,\n            _b: 0.0,\n            _c: 0.0,\n            _d: 0.0,\n            xform: Xform::identity(),\n        }\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.xz_plane",
      "implementations": {
        "python": {
          "sig": "xz_plane()",
          "code": "def xz_plane():\n\n        \"\"\"Create the XZ plane.\n\n        Returns\n        -------\n        Plane\n            XZ plane at origin.\n        \"\"\"\n        plane = Plane.__new__(Plane)\n        plane.guid = str(uuid.uuid4())\n        plane.name = \"xz_plane\"\n        plane.width = 1.0\n        plane.xform = Xform.identity()\n        plane._origin = Point(0.0, 0.0, 0.0)\n        plane._x_axis = Vector.x_axis()\n        plane._y_axis = Vector(0.0, 0.0, -1.0)\n        plane._z_axis = Vector(0.0, 1.0, 0.0)\n        plane._a = 0.0\n        plane._b = 1.0\n        plane._c = 0.0",
          "file": "plane.py"
        },
        "cpp": {
          "sig": "Plane xz_plane()",
          "code": "Plane Plane::xz_plane() {\n    Plane plane;\n    plane.name = \"xz_plane\";\n    plane._origin = Point(0.0, 0.0, 0.0);\n    plane._x_axis = Vector::x_axis();\n    plane._y_axis = Vector(0.0, 0.0, -1.0);\n    plane._z_axis = Vector(0.0, 1.0, 0.0);\n    plane._a = 0.0;\n    plane._b = 1.0;\n    plane._c = 0.0;\n    plane._d = 0.0;\n    return plane;\n}",
          "file": "plane.cpp"
        },
        "rust": {
          "sig": "xz_plane() -> Self",
          "code": "pub fn xz_plane() -> Self {\n        Self {\n            guid: Uuid::new_v4().to_string(),\n            name: \"xz_plane\".to_string(),\n            width: 1.0,\n            _origin: Point::new(0.0, 0.0, 0.0),\n            _x_axis: Vector::x_axis(),\n            _y_axis: Vector::new(0.0, 0.0, -1.0),\n            _z_axis: Vector::new(0.0, 1.0, 0.0),\n            _a: 0.0,\n            _b: 1.0,\n            _c: 0.0,\n            _d: 0.0,\n            xform: Xform::identity(),\n        }\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.transform",
      "implementations": {
        "python": {
          "sig": "transform()",
          "code": "def transform(self):\n\n        \"\"\"Apply the stored xform transformation to the plane.\n\n        Transforms the plane in-place and resets xform to identity.\n        \"\"\"\n        self.xform.transform_point(self._origin)\n        self.xform.transform_vector(self._x_axis)\n        self.xform.transform_vector(self._y_axis)\n        self.xform.transform_vector(self._z_axis)\n        self.xform = Xform.identity()\n\n    def transformed(self):\n        \"\"\"Return a transformed copy of the plane.\"\"\"\n        import copy\n\n        result = copy.deepcopy(self)\n        result.transform()\n        return result\n\n    def duplicate(self):",
          "file": "plane.py"
        },
        "cpp": {
          "sig": "void transform()",
          "code": "void Plane::transform() {\n  xform.transform_point(_origin);\n  xform.transform_vector(_x_axis);\n  xform.transform_vector(_y_axis);\n  xform.transform_vector(_z_axis);\n  xform = Xform::identity();\n}",
          "file": "plane.cpp"
        },
        "rust": {
          "sig": "transform()",
          "code": "pub fn transform(&mut self) {\n        // No clone needed - transform methods take &self\n        self.xform.transform_point(&mut self._origin);\n        self.xform.transform_vector(&mut self._x_axis);\n        self.xform.transform_vector(&mut self._y_axis);\n        self.xform.transform_vector(&mut self._z_axis);\n        self.xform = Xform::identity();\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.transformed",
      "implementations": {
        "python": {
          "sig": "transformed()",
          "code": "def transformed(self):\n\n        \"\"\"Return a transformed copy of the plane.\"\"\"\n        import copy\n\n        result = copy.deepcopy(self)\n        result.transform()\n        return result\n\n    def duplicate(self):\n        \"\"\"Create a deep copy with a new GUID.\"\"\"\n        import copy\n\n        result = copy.deepcopy(self)\n        result.guid = str(uuid.uuid4())\n        return result\n\n    def str(self):\n        \"\"\"Return minimal string representation.\"\"\"\n        return f\"{self._origin[0]}, {self._origin[1]}, {self._origin[2]}\"",
          "file": "plane.py"
        },
        "cpp": {
          "sig": "Plane transformed()",
          "code": "Plane Plane::transformed() const {\n  Plane result = *this;\n  result.transform();\n  return result;\n}",
          "file": "plane.cpp"
        },
        "rust": {
          "sig": "transformed() -> Self",
          "code": "pub fn transformed(&self) -> Self {\n        let mut result = self.clone();\n        result.transform();\n        result\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.duplicate",
      "implementations": {
        "python": {
          "sig": "duplicate()",
          "code": "def duplicate(self):\n\n        \"\"\"Create a deep copy with a new GUID.\"\"\"\n        import copy\n\n        result = copy.deepcopy(self)\n        result.guid = str(uuid.uuid4())\n        return result\n\n    def str(self):\n        \"\"\"Return minimal string representation.\"\"\"\n        return f\"{self._origin[0]}, {self._origin[1]}, {self._origin[2]}\"\n\n    def repr(self):\n        \"\"\"Return full string representation.\"\"\"\n        return f\"Plane({self.name}, {self._origin[0]}, {self._origin[1]}, {self._origin[2]}, {self._z_axis[0]}, {self._z_axis[1]}, {self._z_axis[2]})\"\n\n    def __str__(self):\n        return self.str()\n\n    def __repr__(self):",
          "file": "plane.py"
        },
        "rust": {
          "sig": "duplicate() -> Self",
          "code": "pub fn duplicate(&self) -> Self {\n        let mut result = self.clone();\n        result.guid = Uuid::new_v4().to_string();\n        result\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.str",
      "implementations": {
        "python": {
          "sig": "str()",
          "code": "def str(self):\n\n        \"\"\"Return minimal string representation.\"\"\"\n        return f\"{self._origin[0]}, {self._origin[1]}, {self._origin[2]}\"\n\n    def repr(self):\n        \"\"\"Return full string representation.\"\"\"\n        return f\"Plane({self.name}, {self._origin[0]}, {self._origin[1]}, {self._origin[2]}, {self._z_axis[0]}, {self._z_axis[1]}, {self._z_axis[2]})\"\n\n    def __str__(self):\n        return self.str()\n\n    def __repr__(self):\n        return self.repr()\n\n    def __eq__(self, other):\n        if isinstance(other, Plane):\n            return (self.name == other.name and\n                    self._origin == other._origin and\n                    self._x_axis == other._x_axis and\n                    self._y_axis == other._y_axis and",
          "file": "plane.py"
        },
        "cpp": {
          "sig": "std::string str()",
          "code": "std::string Plane::str() const {\n    int prec = static_cast<int>(Tolerance::ROUNDING);\n    return fmt::format(\"{}",
          "file": "plane.cpp"
        },
        "rust": {
          "sig": "str() -> String",
          "code": "pub fn str(&self) -> String {\n        use crate::tolerance::TOLERANCE;\n        let prec = crate::tolerance::Tolerance::ROUNDING;\n        format!(\n            \"{}, {}, {}\",\n            TOLERANCE.format_number(self._origin[0], prec),\n            TOLERANCE.format_number(self._origin[1], prec),\n            TOLERANCE.format_number(self._origin[2], prec),\n        )\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.repr",
      "implementations": {
        "python": {
          "sig": "repr()",
          "code": "def repr(self):\n\n        \"\"\"Return full string representation.\"\"\"\n        return f\"Plane({self.name}, {self._origin[0]}, {self._origin[1]}, {self._origin[2]}, {self._z_axis[0]}, {self._z_axis[1]}, {self._z_axis[2]})\"\n\n    def __str__(self):\n        return self.str()\n\n    def __repr__(self):\n        return self.repr()\n\n    def __eq__(self, other):\n        if isinstance(other, Plane):\n            return (self.name == other.name and\n                    self._origin == other._origin and\n                    self._x_axis == other._x_axis and\n                    self._y_axis == other._y_axis and\n                    self._z_axis == other._z_axis)\n        return False\n\n    def __ne__(self, other):",
          "file": "plane.py"
        },
        "cpp": {
          "sig": "std::string repr()",
          "code": "std::string Plane::repr() const {\n    int prec = static_cast<int>(Tolerance::ROUNDING);\n    return fmt::format(\"Plane({}",
          "file": "plane.cpp"
        },
        "rust": {
          "sig": "repr() -> String",
          "code": "pub fn repr(&self) -> String {\n        use crate::tolerance::TOLERANCE;\n        let prec = crate::tolerance::Tolerance::ROUNDING;\n        format!(\n            \"Plane({}, {}, {}, {}, {}, {}, {})\",\n            self.name,\n            TOLERANCE.format_number(self._origin[0], prec),\n            TOLERANCE.format_number(self._origin[1], prec),\n            TOLERANCE.format_number(self._origin[2], prec),\n            TOLERANCE.format_number(self._z_axis[0], prec),\n            TOLERANCE.format_number(self._z_axis[1], prec),\n            TOLERANCE.format_number(self._z_axis[2], prec),\n        )\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.__str__",
      "implementations": {
        "python": {
          "sig": "__str__()",
          "code": "def __str__(self):\n\n        return self.str()\n\n    def __repr__(self):\n        return self.repr()\n\n    def __eq__(self, other):\n        if isinstance(other, Plane):\n            return (self.name == other.name and\n                    self._origin == other._origin and\n                    self._x_axis == other._x_axis and\n                    self._y_axis == other._y_axis and\n                    self._z_axis == other._z_axis)\n        return False\n\n    def __ne__(self, other):\n        return not self.__eq__(other)\n\n    def __getitem__(self, index):\n        \"\"\"Get axis by index (0=x, 1=y, 2=z).\"\"\"",
          "file": "plane.py"
        }
      }
    },
    {
      "name": "Plane.__repr__",
      "implementations": {
        "python": {
          "sig": "__repr__()",
          "code": "def __repr__(self):\n\n        return self.repr()\n\n    def __eq__(self, other):\n        if isinstance(other, Plane):\n            return (self.name == other.name and\n                    self._origin == other._origin and\n                    self._x_axis == other._x_axis and\n                    self._y_axis == other._y_axis and\n                    self._z_axis == other._z_axis)\n        return False\n\n    def __ne__(self, other):\n        return not self.__eq__(other)\n\n    def __getitem__(self, index):\n        \"\"\"Get axis by index (0=x, 1=y, 2=z).\"\"\"\n        if index == 0:\n            return self._x_axis\n        elif index == 1:",
          "file": "plane.py"
        }
      }
    },
    {
      "name": "Plane.__eq__",
      "implementations": {
        "python": {
          "sig": "__eq__(other)",
          "code": "def __eq__(self, other):\n\n        if isinstance(other, Plane):\n            return (self.name == other.name and\n                    self._origin == other._origin and\n                    self._x_axis == other._x_axis and\n                    self._y_axis == other._y_axis and\n                    self._z_axis == other._z_axis)\n        return False\n\n    def __ne__(self, other):\n        return not self.__eq__(other)\n\n    def __getitem__(self, index):\n        \"\"\"Get axis by index (0=x, 1=y, 2=z).\"\"\"\n        if index == 0:\n            return self._x_axis\n        elif index == 1:\n            return self._y_axis\n        elif index == 2:\n            return self._z_axis",
          "file": "plane.py"
        }
      }
    },
    {
      "name": "Plane.__ne__",
      "implementations": {
        "python": {
          "sig": "__ne__(other)",
          "code": "def __ne__(self, other):\n\n        return not self.__eq__(other)\n\n    def __getitem__(self, index):\n        \"\"\"Get axis by index (0=x, 1=y, 2=z).\"\"\"\n        if index == 0:\n            return self._x_axis\n        elif index == 1:\n            return self._y_axis\n        elif index == 2:\n            return self._z_axis\n        raise IndexError(\"Plane index out of range (0-2)\")\n\n    ###########################################################################################\n    # No-copy Operators\n    ###########################################################################################\n\n    def __iadd__(self, other):\n        \"\"\"Translate plane by vector (in-place).\"\"\"\n        if isinstance(other, Vector):",
          "file": "plane.py"
        }
      }
    },
    {
      "name": "Plane.__getitem__",
      "implementations": {
        "python": {
          "sig": "__getitem__(index)",
          "code": "def __getitem__(self, index):\n\n        \"\"\"Get axis by index (0=x, 1=y, 2=z).\"\"\"\n        if index == 0:\n            return self._x_axis\n        elif index == 1:\n            return self._y_axis\n        elif index == 2:\n            return self._z_axis\n        raise IndexError(\"Plane index out of range (0-2)\")\n\n    ###########################################################################################\n    # No-copy Operators\n    ###########################################################################################\n\n    def __iadd__(self, other):\n        \"\"\"Translate plane by vector (in-place).\"\"\"\n        if isinstance(other, Vector):\n            self._origin += other\n            self._update_equation()\n        return self",
          "file": "plane.py"
        }
      }
    },
    {
      "name": "Plane.__iadd__",
      "implementations": {
        "python": {
          "sig": "__iadd__(other)",
          "code": "def __iadd__(self, other):\n\n        \"\"\"Translate plane by vector (in-place).\"\"\"\n        if isinstance(other, Vector):\n            self._origin += other\n            self._update_equation()\n        return self\n\n    def __isub__(self, other):\n        \"\"\"Translate plane by negative vector (in-place).\"\"\"\n        if isinstance(other, Vector):\n            self._origin -= other\n            self._update_equation()\n        return self\n\n    ###########################################################################################\n    # Copy Operators\n    ###########################################################################################\n\n    def __add__(self, other):\n        \"\"\"Translate plane by vector (copy).\"\"\"",
          "file": "plane.py"
        }
      }
    },
    {
      "name": "Plane.__isub__",
      "implementations": {
        "python": {
          "sig": "__isub__(other)",
          "code": "def __isub__(self, other):\n\n        \"\"\"Translate plane by negative vector (in-place).\"\"\"\n        if isinstance(other, Vector):\n            self._origin -= other\n            self._update_equation()\n        return self\n\n    ###########################################################################################\n    # Copy Operators\n    ###########################################################################################\n\n    def __add__(self, other):\n        \"\"\"Translate plane by vector (copy).\"\"\"\n        if isinstance(other, Vector):\n            result = Plane.__new__(Plane)\n            result.guid = self.guid\n            result.name = self.name\n            result.width = self.width\n            result.xform = Xform.identity()\n            result._origin = self._origin + other",
          "file": "plane.py"
        }
      }
    },
    {
      "name": "Plane.__add__",
      "implementations": {
        "python": {
          "sig": "__add__(other)",
          "code": "def __add__(self, other):\n\n        \"\"\"Translate plane by vector (copy).\"\"\"\n        if isinstance(other, Vector):\n            result = Plane.__new__(Plane)\n            result.guid = self.guid\n            result.name = self.name\n            result.width = self.width\n            result.xform = Xform.identity()\n            result._origin = self._origin + other\n            result._x_axis = Vector(self._x_axis[0], self._x_axis[1], self._x_axis[2])\n            result._y_axis = Vector(self._y_axis[0], self._y_axis[1], self._y_axis[2])\n            result._z_axis = Vector(self._z_axis[0], self._z_axis[1], self._z_axis[2])\n            result._update_equation()\n            return result\n        return NotImplemented\n\n    def __sub__(self, other):\n        \"\"\"Translate plane by negative vector (copy).\"\"\"\n        if isinstance(other, Vector):\n            result = Plane.__new__(Plane)",
          "file": "plane.py"
        }
      }
    },
    {
      "name": "Plane.__sub__",
      "implementations": {
        "python": {
          "sig": "__sub__(other)",
          "code": "def __sub__(self, other):\n\n        \"\"\"Translate plane by negative vector (copy).\"\"\"\n        if isinstance(other, Vector):\n            result = Plane.__new__(Plane)\n            result.guid = self.guid\n            result.name = self.name\n            result.width = self.width\n            result.xform = Xform.identity()\n            result._origin = self._origin - other\n            result._x_axis = Vector(self._x_axis[0], self._x_axis[1], self._x_axis[2])\n            result._y_axis = Vector(self._y_axis[0], self._y_axis[1], self._y_axis[2])\n            result._z_axis = Vector(self._z_axis[0], self._z_axis[1], self._z_axis[2])\n            result._update_equation()\n            return result\n        return NotImplemented\n\n    ###########################################################################################\n    # Details\n    ###########################################################################################",
          "file": "plane.py"
        }
      }
    },
    {
      "name": "Plane.reverse",
      "implementations": {
        "python": {
          "sig": "reverse()",
          "code": "def reverse(self):\n\n        \"\"\"Reverse the plane's normal direction.\"\"\"\n        temp = self._x_axis\n        self._x_axis = self._y_axis\n        self._y_axis = temp\n        self._z_axis.reverse()\n        self._update_equation()\n\n    def rotate(self, angles_in_radians):\n        \"\"\"Rotate the plane around its normal.\n\n        Parameters\n        ----------\n        angles_in_radians : float\n            Rotation angle in radians.\n        \"\"\"\n        cos_angle = math.cos(angles_in_radians)\n        sin_angle = math.sin(angles_in_radians)\n\n        new_x = self._x_axis * cos_angle + self._y_axis * sin_angle",
          "file": "plane.py"
        },
        "cpp": {
          "sig": "void reverse()",
          "code": "void Plane::reverse() {\n    Vector temp = _x_axis;\n    _x_axis = _y_axis;\n    _y_axis = temp;\n    _z_axis.reverse();\n    \n    _a = _z_axis[0];\n    _b = _z_axis[1];\n    _c = _z_axis[2];\n    _d = -(_a * _origin[0] + _b * _origin[1] + _c * _origin[2]);\n}",
          "file": "plane.cpp"
        },
        "rust": {
          "sig": "reverse()",
          "code": "pub fn reverse(&mut self) {\n        std::mem::swap(&mut self._x_axis, &mut self._y_axis);\n        self._z_axis.reverse();\n\n        self._a = self._z_axis[0];\n        self._b = self._z_axis[1];\n        self._c = self._z_axis[2];\n        self._d =\n            -(self._a * self._origin[0] + self._b * self._origin[1] + self._c * self._origin[2]);\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.rotate",
      "implementations": {
        "python": {
          "sig": "rotate(angles_in_radians)",
          "code": "def rotate(self, angles_in_radians):\n\n        \"\"\"Rotate the plane around its normal.\n\n        Parameters\n        ----------\n        angles_in_radians : float\n            Rotation angle in radians.\n        \"\"\"\n        cos_angle = math.cos(angles_in_radians)\n        sin_angle = math.sin(angles_in_radians)\n\n        new_x = self._x_axis * cos_angle + self._y_axis * sin_angle\n        new_y = self._y_axis * cos_angle - self._x_axis * sin_angle\n\n        self._x_axis = new_x\n        self._y_axis = new_y\n        self._update_equation()\n\n    def is_right_hand(self):\n        \"\"\"Check if the plane follows the right-hand rule.",
          "file": "plane.py"
        },
        "cpp": {
          "sig": "void rotate(double angles_in_radians)",
          "code": "void Plane::rotate(double angles_in_radians) {\n    double cos_angle = std::cos(angles_in_radians);\n    double sin_angle = std::sin(angles_in_radians);\n    \n    Vector new_x = _x_axis * cos_angle + _y_axis * sin_angle;\n    Vector new_y = _y_axis * cos_angle - _x_axis * sin_angle;\n    \n    _x_axis = new_x;\n    _y_axis = new_y;\n    \n    _a = _z_axis[0];\n    _b = _z_axis[1];\n    _c = _z_axis[2];\n    _d = -(_a * _origin[0] + _b * _origin[1] + _c * _origin[2]);\n}",
          "file": "plane.cpp"
        },
        "rust": {
          "sig": "rotate(angles_in_radians: f64)",
          "code": "pub fn rotate(&mut self, angles_in_radians: f64) {\n        let cos_angle = angles_in_radians.cos();\n        let sin_angle = angles_in_radians.sin();\n\n        let new_x = self._x_axis.clone() * cos_angle + self._y_axis.clone() * sin_angle;\n        let new_y = self._y_axis.clone() * cos_angle - self._x_axis.clone() * sin_angle;\n\n        self._x_axis = new_x;\n        self._y_axis = new_y;\n\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.is_right_hand",
      "implementations": {
        "python": {
          "sig": "is_right_hand()",
          "code": "def is_right_hand(self):\n\n        \"\"\"Check if the plane follows the right-hand rule.\n\n        Returns\n        -------\n        bool\n            True if x_axis \u00d7 y_axis = z_axis (right-handed).\n        \"\"\"\n        cross = self._x_axis.cross(self._y_axis)\n        dot_product = cross.dot(self._z_axis)\n        return dot_product > 0.999\n\n    @staticmethod\n    def is_same_direction(plane0, plane1, can_be_flipped=True):\n        \"\"\"Check if two planes have the same or flipped normal.\n\n        Parameters\n        ----------\n        plane0 : Plane\n            First plane.",
          "file": "plane.py"
        },
        "cpp": {
          "sig": "bool is_right_hand()",
          "code": "bool Plane::is_right_hand() const {\n    Vector x_copy = _x_axis;\n    Vector y_copy = _y_axis;\n    Vector z_copy = _z_axis;\n    Vector cross = x_copy.cross(y_copy);\n    double dot_product = cross.dot(z_copy);\n    return dot_product > 0.999;\n}",
          "file": "plane.cpp"
        },
        "rust": {
          "sig": "is_right_hand() -> bool",
          "code": "pub fn is_right_hand(&self) -> bool {\n        let cross = self._x_axis.cross(&self._y_axis);\n        cross.dot(&self._z_axis) > 0.0\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.is_same_direction",
      "implementations": {
        "python": {
          "sig": "is_same_direction(plane0, plane1, can_be_flipped=True)",
          "code": "def is_same_direction(plane0, plane1, can_be_flipped=True):\n\n        \"\"\"Check if two planes have the same or flipped normal.\n\n        Parameters\n        ----------\n        plane0 : Plane\n            First plane.\n        plane1 : Plane\n            Second plane.\n        can_be_flipped : bool, optional\n            Allow flipped normals. Defaults to True.\n\n        Returns\n        -------\n        bool\n            True if normals are parallel or antiparallel.\n        \"\"\"\n        n0 = plane0._z_axis\n        n1 = plane1._z_axis",
          "file": "plane.py"
        },
        "cpp": {
          "sig": "bool is_same_direction(const Plane &plane0, const Plane &plane1, bool can_be_flipped)",
          "code": "bool Plane::is_same_direction(const Plane &plane0, const Plane &plane1, bool can_be_flipped) {\n    Vector n0 = plane0._z_axis;\n    Vector n1 = plane1._z_axis;\n    \n    int parallel = n0.is_parallel_to(n1);\n    \n    if (can_be_flipped) {\n        return parallel != 0;\n    }",
          "file": "plane.cpp"
        },
        "rust": {
          "sig": "is_same_direction(plane0: &Plane, plane1: &Plane, can_be_flipped: bool) -> bool",
          "code": "pub fn is_same_direction(plane0: &Plane, plane1: &Plane, can_be_flipped: bool) -> bool {\n        let n0 = plane0._z_axis.clone();\n        let n1 = plane1._z_axis.clone();\n\n        let parallel = n0.is_parallel_to(&n1);\n\n        if can_be_flipped {\n            parallel != 0\n        } else {\n            parallel == 1\n        }\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.is_same_position",
      "implementations": {
        "python": {
          "sig": "is_same_position(plane0, plane1)",
          "code": "def is_same_position(plane0, plane1):\n\n        \"\"\"Check if two planes are in the same position.\n\n        Parameters\n        ----------\n        plane0 : Plane\n            First plane.\n        plane1 : Plane\n            Second plane.\n\n        Returns\n        -------\n        bool\n            True if origins are very close.\n        \"\"\"\n        dist0 = abs(\n            plane0._a * plane1._origin[0]\n            + plane0._b * plane1._origin[1]\n            + plane0._c * plane1._origin[2]\n            + plane0._d",
          "file": "plane.py"
        },
        "cpp": {
          "sig": "bool is_same_position(const Plane &plane0, const Plane &plane1)",
          "code": "bool Plane::is_same_position(const Plane &plane0, const Plane &plane1) {\n    double dist0 = std::abs(plane0._a * plane1._origin[0] + \n                           plane0._b * plane1._origin[1] + \n                           plane0._c * plane1._origin[2] + \n                           plane0._d);\n    \n    double dist1 = std::abs(plane1._a * plane0._origin[0] + \n                           plane1._b * plane0._origin[1] + \n                           plane1._c * plane0._origin[2] + \n                           plane1._d);\n    \n    double tolerance = static_cast<double>(session_cpp::Tolerance::ZERO_TOLERANCE);\n    return dist0 < tolerance && dist1 < tolerance;\n}",
          "file": "plane.cpp"
        },
        "rust": {
          "sig": "is_same_position(plane0: &Plane, plane1: &Plane) -> bool",
          "code": "pub fn is_same_position(plane0: &Plane, plane1: &Plane) -> bool {\n        let dist0 = (plane0._a * plane1._origin[0]\n            + plane0._b * plane1._origin[1]\n            + plane0._c * plane1._origin[2]\n            + plane0._d)\n            .abs();\n\n        let dist1 = (plane1._a * plane0._origin[0]\n            + plane1._b * plane0._origin[1]\n            + plane1._c * plane0._origin[2]\n            + plane1._d)\n            .abs();\n\n        let tolerance = crate::tolerance::Tolerance::ZERO_TOLERANCE;\n        dist0 < tolerance && dist1 < tolerance\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.is_coplanar",
      "implementations": {
        "python": {
          "sig": "is_coplanar(plane0, plane1, can_be_flipped=True)",
          "code": "def is_coplanar(plane0, plane1, can_be_flipped=True):\n\n        \"\"\"Check if two planes are coplanar.\n\n        Parameters\n        ----------\n        plane0 : Plane\n            First plane.\n        plane1 : Plane\n            Second plane.\n        can_be_flipped : bool, optional\n            Allow flipped normals. Defaults to True.\n\n        Returns\n        -------\n        bool\n            True if planes are coplanar.\n        \"\"\"\n        return Plane.is_same_direction(\n            plane0, plane1, can_be_flipped\n        ) and Plane.is_same_position(plane0, plane1)",
          "file": "plane.py"
        },
        "cpp": {
          "sig": "bool is_coplanar(const Plane &plane0, const Plane plane1, bool can_be_flipped)",
          "code": "bool Plane::is_coplanar(const Plane &plane0, const Plane plane1, bool can_be_flipped) {\n    return is_same_direction(plane0, plane1, can_be_flipped) && \n           is_same_position(plane0, plane1);\n}",
          "file": "plane.cpp"
        },
        "rust": {
          "sig": "is_coplanar(plane0: &Plane, plane1: &Plane, can_be_flipped: bool) -> bool",
          "code": "pub fn is_coplanar(plane0: &Plane, plane1: &Plane, can_be_flipped: bool) -> bool {\n        Self::is_same_direction(plane0, plane1, can_be_flipped)\n            && Self::is_same_position(plane0, plane1)\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.translate_by_normal",
      "implementations": {
        "python": {
          "sig": "translate_by_normal(distance)",
          "code": "def translate_by_normal(self, distance):\n\n        \"\"\"Translate (move) a plane along its normal direction by a specified distance.\n\n        Parameters\n        ----------\n        distance : float\n            Distance to move the plane along its normal (positive = normal direction, negative = opposite).\n\n        Returns\n        -------\n        Plane\n            New plane translated by the specified distance.\n        \"\"\"\n        normal = Vector(self._z_axis[0], self._z_axis[1], self._z_axis[2])\n        normal.normalize_self()\n\n        new_origin = self._origin + (normal * distance)\n\n        return Plane(new_origin, self._x_axis, self._y_axis)",
          "file": "plane.py"
        },
        "cpp": {
          "sig": "Plane translate_by_normal(double distance)",
          "code": "Plane Plane::translate_by_normal(double distance) const {\n    // Get normalized normal vector (z_axis)\n    Vector normal = _z_axis;\n    normal.normalize_self();\n    \n    // Move origin along the normal\n    Point new_origin = _origin + (normal * distance);\n    \n    // Create new plane with same orientation but new origin\n    Vector x_copy = _x_axis;\n    Vector y_copy = _y_axis;\n    return Plane(new_origin, x_copy, y_copy, name);\n}",
          "file": "plane.cpp"
        },
        "rust": {
          "sig": "translate_by_normal(distance: f64) -> Plane",
          "code": "pub fn translate_by_normal(&self, distance: f64) -> Plane {\n        let mut normal = self._z_axis.clone();\n        normal.normalize();\n\n        let new_origin = self._origin.clone() + (normal * distance);\n\n        Plane::new(new_origin, self._x_axis.clone(), self._y_axis.clone())\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.__jsondump__",
      "implementations": {
        "python": {
          "sig": "__jsondump__()",
          "code": "def __jsondump__(self):\n\n        \"\"\"Serialize to polymorphic JSON format with type field.\n\n        Returns\n        -------\n        dict\n            Dictionary with 'type', 'guid', 'name', and object fields.\n            Uses single flat array of 12 numbers for frame:\n            [ox, oy, oz, xx, xy, xz, yx, yy, yz, zx, zy, zz]\n            Plane equation coefficients (a, b, c, d) are computed on load.\n\n        \"\"\"\n        # Alphabetical order to match Rust's serde_json\n        return {\n            \"frame\": [\n                self._origin[0], self._origin[1], self._origin[2],\n                self._x_axis[0], self._x_axis[1], self._x_axis[2],\n                self._y_axis[0], self._y_axis[1], self._y_axis[2],\n                self._z_axis[0], self._z_axis[1], self._z_axis[2],\n            ],",
          "file": "plane.py"
        }
      }
    },
    {
      "name": "Plane.__jsonload__",
      "implementations": {
        "python": {
          "sig": "__jsonload__(cls, data, guid=None, name=None)",
          "code": "def __jsonload__(cls, data, guid=None, name=None):\n\n        \"\"\"Deserialize from polymorphic JSON format.\n\n        Parameters\n        ----------\n        data : dict\n            Dictionary containing plane data.\n        guid : str, optional\n            GUID for the plane.\n        name : str, optional\n            Name for the plane.\n\n        Returns\n        -------\n        :class:`Plane`\n            Reconstructed plane instance.\n\n        \"\"\"\n        from .encoders import decode_node",
          "file": "plane.py"
        }
      }
    },
    {
      "name": "Plane.json_dump",
      "implementations": {
        "python": {
          "sig": "json_dump(filepath)",
          "code": "def json_dump(self, filepath):\n\n        \"\"\"Write JSON to file.\n\n        Parameters\n        ----------\n        filepath : str or Path\n            Path to the output file.\n\n        \"\"\"\n        import json\n        with open(filepath, 'w') as f:\n            json.dump(self.__jsondump__(), f, indent=2)\n\n    @classmethod\n    def json_load(cls, filepath):\n        \"\"\"Read JSON from file.\n\n        Parameters\n        ----------\n        filepath : str or Path",
          "file": "plane.py"
        },
        "cpp": {
          "sig": "void json_dump(const std::string& filename)",
          "code": "void Plane::json_dump(const std::string& filename) const {\n    std::ofstream ofs(filename);\n    ofs << jsondump().dump(2);\n    ofs.close();\n}",
          "file": "plane.cpp"
        },
        "rust": {
          "sig": "json_dump(filepath: &str) -> Result<(), Box<dyn std::error::Error>>",
          "code": "pub fn json_dump(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {\n        let json = self.jsondump()?;\n        std::fs::write(filepath, json)?;\n        Ok(())\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.json_load",
      "implementations": {
        "python": {
          "sig": "json_load(cls, filepath)",
          "code": "def json_load(cls, filepath):\n\n        \"\"\"Read JSON from file.\n\n        Parameters\n        ----------\n        filepath : str or Path\n            Path to the JSON file.\n\n        Returns\n        -------\n        :class:`Plane`\n            The deserialized Plane.\n\n        \"\"\"\n        import json\n        with open(filepath, 'r') as f:\n            data = json.load(f)\n        return cls.__jsonload__(data)\n\n    ###########################################################################################",
          "file": "plane.py"
        },
        "cpp": {
          "sig": "Plane json_load(const std::string& filename)",
          "code": "Plane Plane::json_load(const std::string& filename) {\n    std::ifstream ifs(filename);\n    nlohmann::json data;\n    ifs >> data;\n    ifs.close();\n    return jsonload(data);\n}",
          "file": "plane.cpp"
        },
        "rust": {
          "sig": "json_load(filepath: &str) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn json_load(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {\n        let json = std::fs::read_to_string(filepath)?;\n        Self::jsonload(&json)\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.to_protobuf",
      "implementations": {
        "python": {
          "sig": "to_protobuf()",
          "code": "def to_protobuf(self):\n\n        \"\"\"Convert to protobuf binary format.\n\n        Returns\n        -------\n        bytes\n            Serialized protobuf data.\n\n        \"\"\"\n        from .proto import plane_pb2\n\n        proto = plane_pb2.Plane()\n        proto.guid = self.guid\n        proto.name = self.name\n        proto.width = self.width\n\n        # Set frame as flat array of 12 numbers:\n        # [ox, oy, oz, xx, xy, xz, yx, yy, yz, zx, zy, zz]\n        proto.frame.extend([\n            self._origin[0], self._origin[1], self._origin[2],",
          "file": "plane.py"
        },
        "cpp": {
          "sig": "std::string to_protobuf()",
          "code": "std::string Plane::to_protobuf() const {\n    throw std::runtime_error(\"Protobuf support not enabled\");\n}",
          "file": "plane.cpp"
        },
        "rust": {
          "sig": "to_protobuf() -> Vec<u8>",
          "code": "pub fn to_protobuf(&self) -> Vec<u8> {\n        use prost::Message;\n        // Use single flat frame array of 12 numbers\n        let proto = crate::proto::Plane {\n            guid: self.guid.clone(),\n            name: self.name.clone(),\n            frame: vec![\n                self._origin[0], self._origin[1], self._origin[2],\n                self._x_axis[0], self._x_axis[1], self._x_axis[2],\n                self._y_axis[0], self._y_axis[1], self._y_axis[2],\n                self._z_axis[0], self._z_axis[1], self._z_axis[2],\n            ],\n            width: self.width,\n            xform: Some(crate::proto::Xform {\n                guid: self.xform.guid.clone(),\n                name: self.xform.name.clone(),\n                matrix: self.xform.m.to_vec(),\n            }),\n        };",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.from_protobuf",
      "implementations": {
        "python": {
          "sig": "from_protobuf(cls, data)",
          "code": "def from_protobuf(cls, data):\n\n        \"\"\"Create Plane from protobuf binary data.\n\n        Parameters\n        ----------\n        data : bytes\n            Protobuf-encoded plane data.\n\n        Returns\n        -------\n        :class:`Plane`\n            The deserialized Plane.\n\n        \"\"\"\n        from .proto import plane_pb2\n\n        proto = plane_pb2.Plane()\n        proto.ParseFromString(data)\n\n        # Load frame as flat array of 12 numbers",
          "file": "plane.py"
        },
        "cpp": {
          "sig": "Plane from_protobuf(const std::string& data)",
          "code": "Plane Plane::from_protobuf(const std::string& data) {\n    (void)data;\n    throw std::runtime_error(\"Protobuf support not enabled\");\n}",
          "file": "plane.cpp"
        },
        "rust": {
          "sig": "from_protobuf(data: &[u8]) -> Result<Self, prost::DecodeError>",
          "code": "pub fn from_protobuf(data: &[u8]) -> Result<Self, prost::DecodeError> {\n        use prost::Message;\n        let proto = crate::proto::Plane::decode(data)?;\n\n        // Parse frame array\n        let origin = Point::new(proto.frame[0], proto.frame[1], proto.frame[2]);\n        let x_axis = Vector::new(proto.frame[3], proto.frame[4], proto.frame[5]);\n        let y_axis = Vector::new(proto.frame[6], proto.frame[7], proto.frame[8]);\n        let z_axis = Vector::new(proto.frame[9], proto.frame[10], proto.frame[11]);\n\n        // Compute plane equation coefficients\n        let a = z_axis[0];\n        let b = z_axis[1];\n        let c = z_axis[2];\n        let d = -(a * origin[0] + b * origin[1] + c * origin[2]);\n\n        // Load xform if present\n        let xform = if let Some(proto_xform) = proto",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.protobuf_dump",
      "implementations": {
        "python": {
          "sig": "protobuf_dump(filepath)",
          "code": "def protobuf_dump(self, filepath):\n\n        \"\"\"Write protobuf to file.\n\n        Parameters\n        ----------\n        filepath : str or Path\n            Path to the output file.\n\n        \"\"\"\n        data = self.to_protobuf()\n        with open(filepath, 'wb') as f:\n            f.write(data)\n\n    @classmethod\n    def protobuf_load(cls, filepath):\n        \"\"\"Read protobuf from file.\n\n        Parameters\n        ----------\n        filepath : str or Path",
          "file": "plane.py"
        },
        "cpp": {
          "sig": "void protobuf_dump(const std::string& filename)",
          "code": "void Plane::protobuf_dump(const std::string& filename) const {\n    (void)filename;\n    throw std::runtime_error(\"Protobuf support not enabled\");\n}",
          "file": "plane.cpp"
        },
        "rust": {
          "sig": "protobuf_dump(filepath: &str)",
          "code": "pub fn protobuf_dump(&self, filepath: &str) {\n        let data = self.to_protobuf();\n        std::fs::write(filepath, data).expect(\"Failed to write protobuf file\");\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.protobuf_load",
      "implementations": {
        "python": {
          "sig": "protobuf_load(cls, filepath)",
          "code": "def protobuf_load(cls, filepath):\n\n        \"\"\"Read protobuf from file.\n\n        Parameters\n        ----------\n        filepath : str or Path\n            Path to the protobuf file.\n\n        Returns\n        -------\n        :class:`Plane`\n            The deserialized Plane.\n\n        \"\"\"\n        with open(filepath, 'rb') as f:\n            data = f.read()\n        return cls.from_protobuf(data)",
          "file": "plane.py"
        },
        "cpp": {
          "sig": "Plane protobuf_load(const std::string& filename)",
          "code": "Plane Plane::protobuf_load(const std::string& filename) {\n    (void)filename;\n    throw std::runtime_error(\"Protobuf support not enabled\");\n}",
          "file": "plane.cpp"
        },
        "rust": {
          "sig": "protobuf_load(filepath: &str) -> Self",
          "code": "pub fn protobuf_load(filepath: &str) -> Self {\n        let data = std::fs::read(filepath).expect(\"Failed to read protobuf file\");\n        Self::from_protobuf(&data).expect(\"Failed to parse protobuf\")\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Point.__init__",
      "implementations": {
        "python": {
          "sig": "__init__(x=0.0, y=0.0, z=0.0, name=\"my_point\")",
          "code": "def __init__(self, x=0.0, y=0.0, z=0.0, name=\"my_point\"):\n\n        self.guid = str(uuid.uuid4())\n        self.name = name\n        self._x = x\n        self._y = y\n        self._z = z\n        self.width = 1.0\n        self.pointcolor = Color.blue()\n        self.xform = Xform.identity()\n\n    ###########################################################################################\n    # Operators\n    ###########################################################################################\n\n    def __deepcopy__(self, memo):\n\n        cls = self.__class__\n        result = cls.__new__(cls)\n        memo[id(self)] = result",
          "file": "point.py"
        }
      }
    },
    {
      "name": "Point.__deepcopy__",
      "implementations": {
        "python": {
          "sig": "__deepcopy__(memo)",
          "code": "def __deepcopy__(self, memo):\n\n\n        cls = self.__class__\n        result = cls.__new__(cls)\n        memo[id(self)] = result\n\n        # New guid\n        result.guid = str(uuid.uuid4())\n\n        # Copy remaining fields\n        result.name = copy.deepcopy(self.name, memo)\n        result._x = self._x\n        result._y = self._y\n        result._z = self._z\n        result.width = self.width\n        result.pointcolor = copy.deepcopy(self.pointcolor, memo)\n        result.xform = copy.deepcopy(self.xform, memo)\n        return result\n\n    def duplicate(self):",
          "file": "point.py"
        }
      }
    },
    {
      "name": "Point.duplicate",
      "implementations": {
        "python": {
          "sig": "duplicate()",
          "code": "def duplicate(self):\n\n        \"\"\"Create a deep copy of this point with a new GUID.\n\n        Returns\n        -------\n        :class:`Point`\n            A new Point with identical values but a different GUID.\n\n        \"\"\"\n        return copy.deepcopy(self)\n\n    @staticmethod\n    def sum(p0, p1):\n        \"\"\"Returns a new point that is the sum of two points.\n\n        Parameters\n        ----------\n        p0 : :class:`Point`\n            First point.\n        p1 : :class:`Point`",
          "file": "point.py"
        },
        "rust": {
          "sig": "duplicate() -> Self",
          "code": "pub fn duplicate(&self) -> Self {\n        let mut copy = self.clone();\n        copy.guid = Uuid::new_v4().to_string();\n        copy\n    }",
          "file": "point.rs"
        }
      }
    },
    {
      "name": "Point.sum",
      "implementations": {
        "python": {
          "sig": "sum(p0, p1)",
          "code": "def sum(p0, p1):\n\n        \"\"\"Returns a new point that is the sum of two points.\n\n        Parameters\n        ----------\n        p0 : :class:`Point`\n            First point.\n        p1 : :class:`Point`\n            Second point.\n\n        Returns\n        -------\n        :class:`Point`\n            A new Point with coordinates (p0.x + p1.x, p0.y + p1.y, p0.z + p1.z).\n\n        \"\"\"\n        return Point(p0[0] + p1[0], p0[1] + p1[1], p0[2] + p1[2])\n\n    @staticmethod\n    def sub(p0, p1):",
          "file": "point.py"
        },
        "cpp": {
          "sig": "Point sum(const Point& p0, const Point& p1)",
          "code": "Point Point::sum(const Point& p0, const Point& p1) {\n  return Point(p0._x + p1._x, p0._y + p1._y, p0._z + p1._z);\n}",
          "file": "point.cpp"
        },
        "rust": {
          "sig": "sum(p0: &Point, p1: &Point) -> Self",
          "code": "pub fn sum(p0: &Point, p1: &Point) -> Self {\n        Point::new(p0._x + p1._x, p0._y + p1._y, p0._z + p1._z)\n    }",
          "file": "point.rs"
        }
      }
    },
    {
      "name": "Point.sub",
      "implementations": {
        "python": {
          "sig": "sub(p0, p1)",
          "code": "def sub(p0, p1):\n\n        \"\"\"Returns a new point that is the difference of two points.\n\n        Parameters\n        ----------\n        p0 : :class:`Point`\n            First point.\n        p1 : :class:`Point`\n            Second point.\n\n        Returns\n        -------\n        :class:`Point`\n            A new Point with coordinates (p0.x - p1.x, p0.y - p1.y, p0.z - p1.z).\n\n        \"\"\"\n        return Point(p0[0] - p1[0], p0[1] - p1[1], p0[2] - p1[2])\n\n    ###########################################################################################\n    # Coordinate Properties",
          "file": "point.py"
        },
        "cpp": {
          "sig": "Point sub(const Point& p0, const Point& p1)",
          "code": "Point Point::sub(const Point& p0, const Point& p1) {\n  return Point(p0._x - p1._x, p0._y - p1._y, p0._z - p1._z);\n}",
          "file": "point.cpp"
        },
        "rust": {
          "sig": "sub(p0: &Point, p1: &Point) -> Self",
          "code": "pub fn sub(p0: &Point, p1: &Point) -> Self {\n        Point::new(p0._x - p1._x, p0._y - p1._y, p0._z - p1._z)\n    }",
          "file": "point.rs"
        }
      }
    },
    {
      "name": "Point.x",
      "implementations": {
        "python": {
          "sig": "x(value)",
          "code": "def x(self, value):\n\n        \"\"\"Set the X coordinate.\"\"\"\n        self._x = value\n\n    @property\n    def y(self):\n        \"\"\"Get the Y coordinate.\"\"\"\n        return self._y\n\n    @y.setter\n    def y(self, value):\n        \"\"\"Set the Y coordinate.\"\"\"\n        self._y = value\n\n    @property\n    def z(self):\n        \"\"\"Get the Z coordinate.\"\"\"\n        return self._z\n\n    @z.setter",
          "file": "point.py"
        }
      }
    },
    {
      "name": "Point.y",
      "implementations": {
        "python": {
          "sig": "y(value)",
          "code": "def y(self, value):\n\n        \"\"\"Set the Y coordinate.\"\"\"\n        self._y = value\n\n    @property\n    def z(self):\n        \"\"\"Get the Z coordinate.\"\"\"\n        return self._z\n\n    @z.setter\n    def z(self, value):\n        \"\"\"Set the Z coordinate.\"\"\"\n        self._z = value\n\n    ###########################################################################################\n    # No-copy Operators\n    ###########################################################################################\n\n    def __getitem__(self, index):\n        if index == 0:",
          "file": "point.py"
        }
      }
    },
    {
      "name": "Point.z",
      "implementations": {
        "python": {
          "sig": "z(value)",
          "code": "def z(self, value):\n\n        \"\"\"Set the Z coordinate.\"\"\"\n        self._z = value\n\n    ###########################################################################################\n    # No-copy Operators\n    ###########################################################################################\n\n    def __getitem__(self, index):\n        if index == 0:\n            return self._x\n        elif index == 1:\n            return self._y\n        elif index == 2:\n            return self._z\n        else:\n            raise IndexError(\"Index out of range\")\n\n    def __setitem__(self, index, value):\n        if index == 0:",
          "file": "point.py"
        }
      }
    },
    {
      "name": "Point.__getitem__",
      "implementations": {
        "python": {
          "sig": "__getitem__(index)",
          "code": "def __getitem__(self, index):\n\n        if index == 0:\n            return self._x\n        elif index == 1:\n            return self._y\n        elif index == 2:\n            return self._z\n        else:\n            raise IndexError(\"Index out of range\")\n\n    def __setitem__(self, index, value):\n        if index == 0:\n            self._x = value\n        elif index == 1:\n            self._y = value\n        elif index == 2:\n            self._z = value\n        else:\n            raise IndexError(\"Index out of range\")",
          "file": "point.py"
        }
      }
    },
    {
      "name": "Point.__setitem__",
      "implementations": {
        "python": {
          "sig": "__setitem__(index, value)",
          "code": "def __setitem__(self, index, value):\n\n        if index == 0:\n            self._x = value\n        elif index == 1:\n            self._y = value\n        elif index == 2:\n            self._z = value\n        else:\n            raise IndexError(\"Index out of range\")\n\n    def __imul__(self, other):\n        self._x *= other\n        self._y *= other\n        self._z *= other\n        return self\n\n    def __itruediv__(self, other):\n        self._x /= other\n        self._y /= other\n        self._z /= other",
          "file": "point.py"
        }
      }
    },
    {
      "name": "Point.__imul__",
      "implementations": {
        "python": {
          "sig": "__imul__(other)",
          "code": "def __imul__(self, other):\n\n        self._x *= other\n        self._y *= other\n        self._z *= other\n        return self\n\n    def __itruediv__(self, other):\n        self._x /= other\n        self._y /= other\n        self._z /= other\n        return self\n\n    def __iadd__(self, other):\n        if isinstance(other, Vector):\n            self._x += other[0]\n            self._y += other[1]\n            self._z += other[2]\n        else:\n            raise TypeError(\"Point can only be added with Vector\")\n        return self",
          "file": "point.py"
        }
      }
    },
    {
      "name": "Point.__itruediv__",
      "implementations": {
        "python": {
          "sig": "__itruediv__(other)",
          "code": "def __itruediv__(self, other):\n\n        self._x /= other\n        self._y /= other\n        self._z /= other\n        return self\n\n    def __iadd__(self, other):\n        if isinstance(other, Vector):\n            self._x += other[0]\n            self._y += other[1]\n            self._z += other[2]\n        else:\n            raise TypeError(\"Point can only be added with Vector\")\n        return self\n\n    def __isub__(self, other):\n        if isinstance(other, Vector):\n            self._x -= other[0]\n            self._y -= other[1]\n            self._z -= other[2]",
          "file": "point.py"
        }
      }
    },
    {
      "name": "Point.__iadd__",
      "implementations": {
        "python": {
          "sig": "__iadd__(other)",
          "code": "def __iadd__(self, other):\n\n        if isinstance(other, Vector):\n            self._x += other[0]\n            self._y += other[1]\n            self._z += other[2]\n        else:\n            raise TypeError(\"Point can only be added with Vector\")\n        return self\n\n    def __isub__(self, other):\n        if isinstance(other, Vector):\n            self._x -= other[0]\n            self._y -= other[1]\n            self._z -= other[2]\n        else:\n            raise TypeError(\"Point can only be subtracted with Vector\")\n        return self\n\n    ###########################################################################################\n    # Copy Operators",
          "file": "point.py"
        }
      }
    },
    {
      "name": "Point.__isub__",
      "implementations": {
        "python": {
          "sig": "__isub__(other)",
          "code": "def __isub__(self, other):\n\n        if isinstance(other, Vector):\n            self._x -= other[0]\n            self._y -= other[1]\n            self._z -= other[2]\n        else:\n            raise TypeError(\"Point can only be subtracted with Vector\")\n        return self\n\n    ###########################################################################################\n    # Copy Operators\n    ###########################################################################################\n\n    def __mul__(self, other):\n        return Point(self[0] * other, self[1] * other, self[2] * other)\n\n    def __truediv__(self, other):\n        return Point(self[0] / other, self[1] / other, self[2] / other)\n\n    def __add__(self, other):",
          "file": "point.py"
        }
      }
    },
    {
      "name": "Point.__mul__",
      "implementations": {
        "python": {
          "sig": "__mul__(other)",
          "code": "def __mul__(self, other):\n\n        return Point(self[0] * other, self[1] * other, self[2] * other)\n\n    def __truediv__(self, other):\n        return Point(self[0] / other, self[1] / other, self[2] / other)\n\n    def __add__(self, other):\n        return Point(self[0] + other[0], self[1] + other[1], self[2] + other[2])\n\n    def __sub__(self, other):\n        return Vector(self[0] - other[0], self[1] - other[1], self[2] - other[2])\n\n    ###########################################################################################\n    # Transformation\n    ###########################################################################################\n\n    def transform(self):\n        \"\"\"Apply the stored xform transformation to the point coordinates.\n\n        Transforms the point in-place and resets xform to identity.",
          "file": "point.py"
        }
      }
    },
    {
      "name": "Point.__truediv__",
      "implementations": {
        "python": {
          "sig": "__truediv__(other)",
          "code": "def __truediv__(self, other):\n\n        return Point(self[0] / other, self[1] / other, self[2] / other)\n\n    def __add__(self, other):\n        return Point(self[0] + other[0], self[1] + other[1], self[2] + other[2])\n\n    def __sub__(self, other):\n        return Vector(self[0] - other[0], self[1] - other[1], self[2] - other[2])\n\n    ###########################################################################################\n    # Transformation\n    ###########################################################################################\n\n    def transform(self):\n        \"\"\"Apply the stored xform transformation to the point coordinates.\n\n        Transforms the point in-place and resets xform to identity.\n        \"\"\"\n        self.xform.transform_point(self)\n        self.xform = Xform.identity()",
          "file": "point.py"
        }
      }
    },
    {
      "name": "Point.__add__",
      "implementations": {
        "python": {
          "sig": "__add__(other)",
          "code": "def __add__(self, other):\n\n        return Point(self[0] + other[0], self[1] + other[1], self[2] + other[2])\n\n    def __sub__(self, other):\n        return Vector(self[0] - other[0], self[1] - other[1], self[2] - other[2])\n\n    ###########################################################################################\n    # Transformation\n    ###########################################################################################\n\n    def transform(self):\n        \"\"\"Apply the stored xform transformation to the point coordinates.\n\n        Transforms the point in-place and resets xform to identity.\n        \"\"\"\n        self.xform.transform_point(self)\n        self.xform = Xform.identity()\n\n    def transformed(self):\n        \"\"\"Return a transformed copy of the point.",
          "file": "point.py"
        }
      }
    },
    {
      "name": "Point.__sub__",
      "implementations": {
        "python": {
          "sig": "__sub__(other)",
          "code": "def __sub__(self, other):\n\n        return Vector(self[0] - other[0], self[1] - other[1], self[2] - other[2])\n\n    ###########################################################################################\n    # Transformation\n    ###########################################################################################\n\n    def transform(self):\n        \"\"\"Apply the stored xform transformation to the point coordinates.\n\n        Transforms the point in-place and resets xform to identity.\n        \"\"\"\n        self.xform.transform_point(self)\n        self.xform = Xform.identity()\n\n    def transformed(self):\n        \"\"\"Return a transformed copy of the point.\n\n        Returns a new point with the transformation applied.\n        The original point and its xform remain unchanged.",
          "file": "point.py"
        }
      }
    },
    {
      "name": "Point.transform",
      "implementations": {
        "python": {
          "sig": "transform()",
          "code": "def transform(self):\n\n        \"\"\"Apply the stored xform transformation to the point coordinates.\n\n        Transforms the point in-place and resets xform to identity.\n        \"\"\"\n        self.xform.transform_point(self)\n        self.xform = Xform.identity()\n\n    def transformed(self):\n        \"\"\"Return a transformed copy of the point.\n\n        Returns a new point with the transformation applied.\n        The original point and its xform remain unchanged.\n\n        Returns\n        -------\n        Point\n            A new transformed point.\n        \"\"\"",
          "file": "point.py"
        },
        "cpp": {
          "sig": "void transform()",
          "code": "void Point::transform() {\n  xform.transform_point(*this);\n  xform = Xform::identity();\n}",
          "file": "point.cpp"
        },
        "rust": {
          "sig": "transform()",
          "code": "pub fn transform(&mut self) {\n        let xform = self.xform.clone();\n        xform.transform_point(self);\n        self.xform = Xform::identity();\n    }",
          "file": "point.rs"
        }
      }
    },
    {
      "name": "Point.transformed",
      "implementations": {
        "python": {
          "sig": "transformed()",
          "code": "def transformed(self):\n\n        \"\"\"Return a transformed copy of the point.\n\n        Returns a new point with the transformation applied.\n        The original point and its xform remain unchanged.\n\n        Returns\n        -------\n        Point\n            A new transformed point.\n        \"\"\"\n\n        result = copy.deepcopy(self)\n        result.transform()\n        return result\n\n    ###########################################################################################\n    # Details\n    ###########################################################################################",
          "file": "point.py"
        },
        "cpp": {
          "sig": "Point transformed()",
          "code": "Point Point::transformed() const {\n  Point result = *this;\n  result.transform();\n  return result;\n}",
          "file": "point.cpp"
        },
        "rust": {
          "sig": "transformed() -> Self",
          "code": "pub fn transformed(&self) -> Self {\n        let mut result = self.clone();\n        result.transform();\n        result\n    }",
          "file": "point.rs"
        }
      }
    },
    {
      "name": "Point.is_ccw",
      "implementations": {
        "python": {
          "sig": "is_ccw(a, b, c)",
          "code": "def is_ccw(a, b, c):\n\n        \"\"\"Check if the points are in counter-clockwise order on xy plane.\n\n        Parameters\n        ----------\n        a : :class:`Point`\n            First point.\n        b : :class:`Point`\n            Second point.\n        c : :class:`Point`\n            Third point.\n\n        Returns\n        -------\n        bool\n            True if the points are in counter-clockwise order, False otherwise.\n\n        \"\"\"\n\n        return (c[1] - a[1]) * (b[0] - a[0]) > (b[1] - a[1]) * (c[0] - a[0])",
          "file": "point.py"
        },
        "cpp": {
          "sig": "bool is_ccw(const Point& a, const Point& b, const Point& c)",
          "code": "bool Point::is_ccw(const Point& a, const Point& b, const Point& c) {\n    return ccw(a, b, c);\n}",
          "file": "point.cpp"
        },
        "rust": {
          "sig": "is_ccw(a: &Point, b: &Point, c: &Point) -> bool",
          "code": "pub fn is_ccw(a: &Point, b: &Point, c: &Point) -> bool {\n        Self::ccw(a, b, c)\n    }",
          "file": "point.rs"
        }
      }
    },
    {
      "name": "Point.mid_point",
      "implementations": {
        "python": {
          "sig": "mid_point(p)",
          "code": "def mid_point(self, p):\n\n        \"\"\"Calculate the mid point between this point and another point.\n\n        Parameters\n        ----------\n        p : :class:`Point`\n            The other point.\n\n        Returns\n        -------\n        :class:`Point`\n            The mid point between this point and the other point.\n\n        \"\"\"\n\n        return Point((self[0] + p[0]) / 2, (self[1] + p[1]) / 2, (self[2] + p[2]) / 2)\n\n    def distance(self, p, double_min=1e-12):\n        \"\"\"Calculate the distance between this point and another point.",
          "file": "point.py"
        },
        "cpp": {
          "sig": "Point mid_point(const Point& a, const Point& b)",
          "code": "Point Point::mid_point(const Point& a, const Point& b) {\n    return a.mid_point(b);\n}",
          "file": "point.cpp"
        },
        "rust": {
          "sig": "mid_point(a: &Point, b: &Point) -> Point",
          "code": "pub fn mid_point(a: &Point, b: &Point) -> Point {\n        Point::new(\n            (a._x + b._x) / 2.0,\n            (a._y + b._y) / 2.0,\n            (a._z + b._z) / 2.0,\n        )\n    }",
          "file": "point.rs"
        }
      }
    },
    {
      "name": "Point.distance",
      "implementations": {
        "python": {
          "sig": "distance(p, double_min=1e-12)",
          "code": "def distance(self, p, double_min=1e-12):\n\n        \"\"\"Calculate the distance between this point and another point.\n\n        Parameters\n        ----------\n        p : :class:`Point`\n            The other point.\n        double_min : float, optional\n            The minimum value for the distance. Defaults to 1e-12.\n\n        Returns\n        -------\n        float\n            The distance between this point and the other point.\n\n        \"\"\"\n\n        x = abs(self[0] - p[0])\n        y = abs(self[1] - p[1])\n        z = abs(self[2] - p[2])",
          "file": "point.py"
        },
        "cpp": {
          "sig": "double distance(const Point& a, const Point& b, double float_min)",
          "code": "double Point::distance(const Point& a, const Point& b, double float_min) {\n    return a.distance(b, float_min);\n}",
          "file": "point.cpp"
        },
        "rust": {
          "sig": "distance(p: &Point, double_min: Option<f64>) -> f64",
          "code": "pub fn distance(&self, p: &Point, double_min: Option<f64>) -> f64 {\n        let double_min = double_min.unwrap_or(1e-12);\n        let mut dx = (self[0] - p[0]).abs();\n        let mut dy = (self[1] - p[1]).abs();\n        let mut dz = (self[2] - p[2]).abs();\n\n        // Reorder coordinates to put largest in dx\n        if dy >= dx && dy >= dz {\n            std::mem::swap(&mut dx, &mut dy);\n        } else if dz >= dx && dz >= dy {\n            std::mem::swap(&mut dx, &mut dz);\n        }\n\n        if dx > double_min {\n            dy /= dx;\n            dz /= dx;\n            dx * (1.0 + dy * dy + dz * dz).sqrt()\n        } else if dx > 0.0 && dx.is_finite() {\n            dx\n        } else {\n            0.0\n        }\n    }",
          "file": "point.rs"
        }
      }
    },
    {
      "name": "Point.squared_distance",
      "implementations": {
        "python": {
          "sig": "squared_distance(p, double_min=1e-12)",
          "code": "def squared_distance(self, p, double_min=1e-12):\n\n        \"\"\"Calculate the squared distance between this point and another point.\n\n        Parameters\n        ----------\n        p : :class:`Point`\n            The other point.\n        double_min : float, optional\n            The minimum value for the distance. Defaults to 1e-12.\n\n        Returns\n        -------\n        float\n            The distance between this point and the other point.\n\n        \"\"\"\n\n        x = abs(self[0] - p[0])\n        y = abs(self[1] - p[1])\n        z = abs(self[2] - p[2])",
          "file": "point.py"
        },
        "cpp": {
          "sig": "double squared_distance(const Point& a, const Point& b, double float_min)",
          "code": "double Point::squared_distance(const Point& a, const Point& b, double float_min) {\n    return a.squared_distance(b, float_min);\n}",
          "file": "point.cpp"
        },
        "rust": {
          "sig": "squared_distance(p: &Point, double_min: Option<f64>) -> f64",
          "code": "pub fn squared_distance(&self, p: &Point, double_min: Option<f64>) -> f64 {\n        let double_min = double_min.unwrap_or(1e-12);\n        let mut dx = (self[0] - p[0]).abs();\n        let mut dy = (self[1] - p[1]).abs();\n        let mut dz = (self[2] - p[2]).abs();\n\n        if dy >= dx && dy >= dz {\n            std::mem::swap(&mut dx, &mut dy);\n        } else if dz >= dx && dz >= dy {\n            std::mem::swap(&mut dx, &mut dz);\n        }\n\n        if dx > double_min {\n            dy /= dx;\n            dz /= dx;\n            dx * dx * (1.0 + dy * dy + dz * dz)\n        } else if dx > 0.0 && dx.is_finite() {\n            dx * dx\n        } else {\n            0.0\n        }\n    }",
          "file": "point.rs"
        }
      }
    },
    {
      "name": "Point.area",
      "implementations": {
        "python": {
          "sig": "area(points)",
          "code": "def area(points):\n\n        \"\"\"Calculate the area of a 2d polygon.\n\n        Parameters\n        ----------\n        points : list of :class:`Point`\n            The points of the polygon.\n\n        Returns\n        -------\n        float\n            The area of the polygon.\n\n        \"\"\"\n\n        n = len(points)\n        area = 0.0\n        for i in range(n):\n            j = (i + 1) % n\n            area += points[i][0] * points[j][1]",
          "file": "point.py"
        },
        "cpp": {
          "sig": "double area(const std::vector<Point>& points)",
          "code": "double Point::area(const std::vector<Point>& points) {\n    size_t n = points.size();\n    double area = 0.0;\n    \n    for (size_t i = 0; i < n; ++i) {\n        size_t j = (i + 1) % n;\n        area += points[i][0] * points[j][1];\n        area -= points[j][0] * points[i][1];\n    }",
          "file": "point.cpp"
        },
        "rust": {
          "sig": "area(points: &[Point]) -> f64",
          "code": "pub fn area(points: &[Point]) -> f64 {\n        let n = points.len();\n        let mut area = 0.0;\n\n        for i in 0..n {\n            let j = (i + 1) % n;\n            area += points[i][0] * points[j][1];\n            area -= points[j][0] * points[i][1];\n        }\n\n        area.abs() / 2.0\n    }",
          "file": "point.rs"
        }
      }
    },
    {
      "name": "Point.centroid_quad",
      "implementations": {
        "python": {
          "sig": "centroid_quad(vertices)",
          "code": "def centroid_quad(vertices):\n\n        \"\"\"Calculate the centroid of a quadrilateral.\n\n        Parameters\n        ----------\n        vertices : list of :class:`Point`\n            The vertices of the quadrilateral.\n\n        Returns\n        -------\n        :class:`Point`\n            The centroid of the quadrilateral.\n\n        \"\"\"\n\n        if len(vertices) != 4:\n            raise ValueError(\"Polygon must have exactly 4 vertices.\")\n\n        total_area = 0.0\n        centroid_sum = Vector(0, 0, 0)",
          "file": "point.py"
        },
        "cpp": {
          "sig": "Point centroid_quad(const std::vector<Point>& vertices)",
          "code": "Point Point::centroid_quad(const std::vector<Point>& vertices) {\n    if (vertices.size() != 4) {\n        throw std::invalid_argument(\"Polygon must have exactly 4 vertices.\");\n    }",
          "file": "point.cpp"
        },
        "rust": {
          "sig": "centroid_quad(vertices: &[Point]) -> Result<Point, &'static str>",
          "code": "pub fn centroid_quad(vertices: &[Point]) -> Result<Point, &'static str> {\n        if vertices.len() != 4 {\n            return Err(\"Polygon must have exactly 4 vertices.\");\n        }\n\n        let mut total_area = 0.0;\n        let mut centroid_sum = Vector::new(0.0, 0.0, 0.0);\n\n        for i in 0..4 {\n            let p0 = &vertices[i];\n            let p1 = &vertices[(i + 1) % 4];\n            let p2 = &vertices[(i + 2) % 4];\n\n            let tri_area =\n                ((p0[0] * (p1[1] - p2[1]) + p1[0] * (p2[1] - p0[1]) + p2[0] * (p0[1] - p1[1]))\n                    .abs())\n                    / 2.0;\n            total_area += tri_area;\n\n            let tri_centroid = Vector::new(\n                (p0[0] + p1[0] + p2[0]) / 3.0,\n                (p0[1] + p1[1] + p2[1]) / 3.0,\n                (",
          "file": "point.rs"
        }
      }
    },
    {
      "name": "Point.__jsondump__",
      "implementations": {
        "python": {
          "sig": "__jsondump__()",
          "code": "def __jsondump__(self):\n\n        \"\"\"Serialize to polymorphic JSON format with type field.\n\n        Returns\n        -------\n        dict\n            Dictionary with 'type', 'guid', 'name', and object fields.\n\n        \"\"\"\n        # Alphabetical order to match Rust's serde_json\n        return {\n            \"guid\": self.guid,\n            \"name\": self.name,\n            \"pointcolor\": self.pointcolor.__jsondump__(),\n            \"type\": f\"{self.__class__.__name__}\",\n            \"width\": self.width,\n            \"x\": self[0],\n            \"xform\": self.xform.__jsondump__(),\n            \"y\": self[1],\n            \"z\": self[2],",
          "file": "point.py"
        }
      }
    },
    {
      "name": "Point.__jsonload__",
      "implementations": {
        "python": {
          "sig": "__jsonload__(cls, data, guid=None, name=None)",
          "code": "def __jsonload__(cls, data, guid=None, name=None):\n\n        \"\"\"Deserialize from polymorphic JSON format.\"\"\"\n        from .encoders import decode_node\n\n        pt = cls(data[\"x\"], data[\"y\"], data[\"z\"])\n        pt.width = data.get(\"width\", 1.0)\n\n        # Decode nested color (supports polymorphic dicts and plain values)\n        pt.pointcolor = decode_node(data.get(\"pointcolor\"))\n\n        # Always assign metadata (per project convention)\n        pt.guid = guid if guid is not None else data.get(\"guid\", pt.guid)\n        pt.name = name if name is not None else data.get(\"name\", pt.name)\n\n        if \"xform\" in data:\n            pt.xform = decode_node(data[\"xform\"])\n\n        return pt\n\n    def json_dump(self, filepath):",
          "file": "point.py"
        }
      }
    },
    {
      "name": "Point.json_dump",
      "implementations": {
        "python": {
          "sig": "json_dump(filepath)",
          "code": "def json_dump(self, filepath):\n\n        \"\"\"Write JSON to file.\n\n        Parameters\n        ----------\n        filepath : str or Path\n            Path to the output file.\n\n        \"\"\"\n        import json\n        with open(filepath, 'w') as f:\n            json.dump(self.__jsondump__(), f, indent=2)\n\n    @classmethod\n    def json_load(cls, filepath):\n        \"\"\"Read JSON from file.\n\n        Parameters\n        ----------\n        filepath : str or Path",
          "file": "point.py"
        },
        "cpp": {
          "sig": "void json_dump(const std::string& filename)",
          "code": "void Point::json_dump(const std::string& filename) const {\n  std::ofstream file(filename);\n  file << jsondump().dump(4);\n}",
          "file": "point.cpp"
        }
      }
    },
    {
      "name": "Point.json_load",
      "implementations": {
        "python": {
          "sig": "json_load(cls, filepath)",
          "code": "def json_load(cls, filepath):\n\n        \"\"\"Read JSON from file.\n\n        Parameters\n        ----------\n        filepath : str or Path\n            Path to the JSON file.\n\n        Returns\n        -------\n        :class:`Point`\n            The deserialized Point.\n\n        \"\"\"\n        import json\n        with open(filepath, 'r') as f:\n            data = json.load(f)\n        return cls.__jsonload__(data)\n\n    ###########################################################################################",
          "file": "point.py"
        },
        "cpp": {
          "sig": "Point json_load(const std::string& filename)",
          "code": "Point Point::json_load(const std::string& filename) {\n  std::ifstream file(filename);\n  nlohmann::json data = nlohmann::json::parse(file);\n  return jsonload(data);\n}",
          "file": "point.cpp"
        }
      }
    },
    {
      "name": "Point.to_protobuf",
      "implementations": {
        "python": {
          "sig": "to_protobuf()",
          "code": "def to_protobuf(self):\n\n        \"\"\"Convert to protobuf binary format.\n\n        Returns\n        -------\n        bytes\n            Serialized protobuf data.\n\n        \"\"\"\n        from .proto import point_pb2\n        \n        proto = point_pb2.Point()\n        proto.guid = self.guid\n        proto.name = self.name\n        proto.x = self[0]\n        proto.y = self[1]\n        proto.z = self[2]\n        proto.width = self.width\n        \n        # Set color (no guid in proto schema)",
          "file": "point.py"
        },
        "cpp": {
          "sig": "std::string to_protobuf()",
          "code": "std::string Point::to_protobuf() const {\n  session_proto::Point proto;\n  proto.set_guid(guid);\n  proto.set_name(name);\n  proto.set_x(_x);\n  proto.set_y(_y);\n  proto.set_z(_z);\n  proto.set_width(width);\n  \n  // Set color (no guid in proto schema)\n  auto* color_proto = proto.mutable_pointcolor();\n  color_proto->set_name(pointcolor.name);\n  color_proto->set_r(pointcolor.r);\n  color_proto->set_g(pointcolor.g);\n  color_proto->set_b(pointcolor.b);\n  color_proto->set_a(pointcolor.a);\n  \n  // Set xform\n  auto* xform_proto = proto.mutable_xform();\n  xform_proto->set_guid(xform.guid);\n  xform_proto->set_name(xform.name);\n  for (int i = 0; i < 16; ++i) {\n    xform_proto->add_matrix(xform.m[i]);\n  }",
          "file": "point.cpp"
        },
        "rust": {
          "sig": "to_protobuf() -> Vec<u8>",
          "code": "pub fn to_protobuf(&self) -> Vec<u8> {\n        use prost::Message;\n        \n        let proto = crate::proto::Point {\n            guid: self.guid.clone(),\n            name: self.name.clone(),\n            x: self._x,\n            y: self._y,\n            z: self._z,\n            width: self.width,\n            pointcolor: Some(crate::proto::Color {\n                guid: self.pointcolor.guid.clone(),\n                name: self.pointcolor.name.clone(),\n                r: self.pointcolor.r as i32,\n                g: self.pointcolor.g as i32,\n                b: self.pointcolor.b as i32,\n                a: self.pointcolor.a as i32,\n            }),\n            xform: Some(crate::proto::Xform {\n                guid: self.xform.guid.clone(),\n                name: self.xform.name.clone(),",
          "file": "point.rs"
        }
      }
    },
    {
      "name": "Point.from_protobuf",
      "implementations": {
        "python": {
          "sig": "from_protobuf(cls, data)",
          "code": "def from_protobuf(cls, data):\n\n        \"\"\"Create Point from protobuf binary data.\n\n        Parameters\n        ----------\n        data : bytes\n            Protobuf-encoded point data.\n\n        Returns\n        -------\n        :class:`Point`\n            The deserialized Point.\n\n        \"\"\"\n        from .proto import point_pb2\n        from .color import Color\n        from .xform import Xform\n        \n        proto = point_pb2.Point()\n        proto.ParseFromString(data)",
          "file": "point.py"
        },
        "cpp": {
          "sig": "Point from_protobuf(const std::string& data)",
          "code": "Point Point::from_protobuf(const std::string& data) {\n  session_proto::Point proto;\n  proto.ParseFromString(data);\n  \n  Point point(proto.x(), proto.y(), proto.z());\n  point.guid = proto.guid();\n  point.name = proto.name();\n  point.width = proto.width();\n  \n  // Load color (no guid in proto schema)\n  const auto& color_proto = proto.pointcolor();\n  point.pointcolor.name = color_proto.name();\n  point.pointcolor.r = color_proto.r();\n  point.pointcolor.g = color_proto.g();\n  point.pointcolor.b = color_proto.b();\n  point.pointcolor.a = color_proto.a();\n  \n  // Load xform\n  const auto& xform_proto = proto.xform();\n  point.xform.guid = xform_proto.guid();\n  point.xform.name = xform_proto.name();\n  for (int i = 0; i < 16 && i < xform_proto.matrix_size(); ++i) {\n    point.xform.m[i] = xform_proto.",
          "file": "point.cpp"
        },
        "rust": {
          "sig": "from_protobuf(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn from_protobuf(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {\n        use prost::Message;\n        \n        let proto = crate::proto::Point::decode(data)?;\n        \n        let mut pt = Self::new(proto.x, proto.y, proto.z);\n        pt.guid = proto.guid;\n        pt.name = proto.name;\n        pt.width = proto.width;\n        \n        if let Some(color) = proto.pointcolor {\n            pt.pointcolor.name = color.name;\n            pt.pointcolor.r = color.r as u8;\n            pt.pointcolor.g = color.g as u8;\n            pt.pointcolor.b = color.b as u8;\n            pt.pointcolor.a = color.a as u8;\n        }\n        \n        if let Some(xform) = proto.xform {\n            pt.xform.guid = xform.guid;\n            pt.xform.name = xform.name;\n            for (i, val) in xform.matri",
          "file": "point.rs"
        }
      }
    },
    {
      "name": "Point.protobuf_dump",
      "implementations": {
        "python": {
          "sig": "protobuf_dump(filepath)",
          "code": "def protobuf_dump(self, filepath):\n\n        \"\"\"Write protobuf to file.\n\n        Parameters\n        ----------\n        filepath : str\n            Path to the output file.\n\n        \"\"\"\n        data = self.to_protobuf()\n        with open(filepath, 'wb') as f:\n            f.write(data)\n\n    @classmethod\n    def protobuf_load(cls, filepath):\n        \"\"\"Read protobuf from file.\n\n        Parameters\n        ----------\n        filepath : str",
          "file": "point.py"
        },
        "cpp": {
          "sig": "void protobuf_dump(const std::string& filename)",
          "code": "void Point::protobuf_dump(const std::string& filename) const {\n  std::string data = to_protobuf();\n  std::ofstream file(filename, std::ios::binary);\n  file.write(data.data(), data.size());\n}",
          "file": "point.cpp"
        },
        "rust": {
          "sig": "protobuf_dump(filepath: &str)",
          "code": "pub fn protobuf_dump(&self, filepath: &str) {\n        let data = self.to_protobuf();\n        std::fs::write(filepath, data).expect(\"Failed to write protobuf file\");\n    }",
          "file": "point.rs"
        }
      }
    },
    {
      "name": "Point.protobuf_load",
      "implementations": {
        "python": {
          "sig": "protobuf_load(cls, filepath)",
          "code": "def protobuf_load(cls, filepath):\n\n        \"\"\"Read protobuf from file.\n\n        Parameters\n        ----------\n        filepath : str\n            Path to the protobuf file.\n\n        Returns\n        -------\n        :class:`Point`\n            The deserialized Point.\n\n        \"\"\"\n        with open(filepath, 'rb') as f:\n            data = f.read()\n        return cls.from_protobuf(data)\n\n    def __str__(self):\n        return f\"{self[0]}, {self[1]}, {self[2]}\"",
          "file": "point.py"
        },
        "cpp": {
          "sig": "Point protobuf_load(const std::string& filename)",
          "code": "Point Point::protobuf_load(const std::string& filename) {\n  std::ifstream file(filename, std::ios::binary);\n  std::string data((std::istreambuf_iterator<char>(file)),\n                    std::istreambuf_iterator<char>());\n  return from_protobuf(data);\n}",
          "file": "point.cpp"
        },
        "rust": {
          "sig": "protobuf_load(filepath: &str) -> Self",
          "code": "pub fn protobuf_load(filepath: &str) -> Self {\n        let data = std::fs::read(filepath).expect(\"Failed to read protobuf file\");\n        Self::from_protobuf(&data).expect(\"Failed to parse protobuf\")\n    }",
          "file": "point.rs"
        }
      }
    },
    {
      "name": "Point.__str__",
      "implementations": {
        "python": {
          "sig": "__str__()",
          "code": "def __str__(self):\n\n        return f\"{self[0]}, {self[1]}, {self[2]}\"\n\n    def __repr__(self):\n        return f\"Point({self.name}, {self[0]}, {self[1]}, {self[2]}, {repr(self.pointcolor)}, {self.width})\"\n\n    def __eq__(self, other):\n        return (\n            self.name == other.name\n            and round(self[0], Tolerance.ROUNDING) == round(other[0], Tolerance.ROUNDING)\n            and round(self[1], Tolerance.ROUNDING) == round(other[1], Tolerance.ROUNDING)\n            and round(self[2], Tolerance.ROUNDING) == round(other[2], Tolerance.ROUNDING)\n            and round(self.width, Tolerance.ROUNDING) == round(other.width, Tolerance.ROUNDING)\n            and self.pointcolor == other.pointcolor\n            and self.xform == other.xform\n        )\n\n    def __ne__(self, other):\n        return not self == other",
          "file": "point.py"
        }
      }
    },
    {
      "name": "Point.__repr__",
      "implementations": {
        "python": {
          "sig": "__repr__()",
          "code": "def __repr__(self):\n\n        return f\"Point({self.name}, {self[0]}, {self[1]}, {self[2]}, {repr(self.pointcolor)}, {self.width})\"\n\n    def __eq__(self, other):\n        return (\n            self.name == other.name\n            and round(self[0], Tolerance.ROUNDING) == round(other[0], Tolerance.ROUNDING)\n            and round(self[1], Tolerance.ROUNDING) == round(other[1], Tolerance.ROUNDING)\n            and round(self[2], Tolerance.ROUNDING) == round(other[2], Tolerance.ROUNDING)\n            and round(self.width, Tolerance.ROUNDING) == round(other.width, Tolerance.ROUNDING)\n            and self.pointcolor == other.pointcolor\n            and self.xform == other.xform\n        )\n\n    def __ne__(self, other):\n        return not self == other",
          "file": "point.py"
        }
      }
    },
    {
      "name": "Point.__eq__",
      "implementations": {
        "python": {
          "sig": "__eq__(other)",
          "code": "def __eq__(self, other):\n\n        return (\n            self.name == other.name\n            and round(self[0], Tolerance.ROUNDING) == round(other[0], Tolerance.ROUNDING)\n            and round(self[1], Tolerance.ROUNDING) == round(other[1], Tolerance.ROUNDING)\n            and round(self[2], Tolerance.ROUNDING) == round(other[2], Tolerance.ROUNDING)\n            and round(self.width, Tolerance.ROUNDING) == round(other.width, Tolerance.ROUNDING)\n            and self.pointcolor == other.pointcolor\n            and self.xform == other.xform\n        )\n\n    def __ne__(self, other):\n        return not self == other",
          "file": "point.py"
        }
      }
    },
    {
      "name": "Point.__ne__",
      "implementations": {
        "python": {
          "sig": "__ne__(other)",
          "code": "def __ne__(self, other):\n\n        return not self == other",
          "file": "point.py"
        }
      }
    },
    {
      "name": "PointCloud.__init__",
      "implementations": {
        "python": {
          "sig": "__init__(points: Optional[List[Point]] = None,\n                 normals: Optional[List[Vector]] = None,\n                 colors: Optional[List[Color]] = None)",
          "code": "def __init__(self, points: Optional[List[Point]] = None,\n                 normals: Optional[List[Vector]] = None,\n                 colors: Optional[List[Color]] = None):\n\n        \"\"\"Creates a new PointCloud with default guid and name.\n\n        Args:\n            points: Collection of points (converted to flat coords internally).\n            normals: Collection of normals (converted to flat array internally).\n            colors: Collection of colors (converted to flat array internally).\n        \"\"\"\n        self.guid = str(uuid.uuid4())\n        self.name = \"my_pointcloud\"\n        self.point_size = 1.0\n        self.xform = Xform.identity()\n\n        # Store as flat arrays\n        self._coords: List[float] = []\n        self._colors: List[int] = []\n        self._normals: List[float] = []\n\n        if points is not None:\n            for p in points:",
          "file": "pointcloud.py"
        }
      }
    },
    {
      "name": "PointCloud.from_coords",
      "implementations": {
        "python": {
          "sig": "from_coords(cls, coords: List[float],\n                    colors: Optional[List[int]] = None,\n                    normals: Optional[List[float]] = None) -> \"PointCloud\"",
          "code": "def from_coords(cls, coords: List[float],\n                    colors: Optional[List[int]] = None,\n                    normals: Optional[List[float]] = None) -> \"PointCloud\":\n\n        \"\"\"Create a PointCloud from flat arrays.\n\n        Args:\n            coords: Flat array [x0, y0, z0, x1, y1, z1, ...]\n            colors: Flat array [r0, g0, b0, a0, r1, g1, b1, a1, ...]\n            normals: Flat array [nx0, ny0, nz0, nx1, ny1, nz1, ...]\n\n        Returns:\n            New PointCloud instance.\n        \"\"\"\n        pc = cls()\n        pc._coords = list(coords)\n        if colors is not None:\n            pc._colors = list(colors)\n        if normals is not None:\n            pc._normals = list(normals)\n        return pc\n\n    ###########################################################################################",
          "file": "pointcloud.py"
        },
        "cpp": {
          "sig": "PointCloud from_coords(const std::vector<double>& coords,\n                                   const std::vector<int>& colors,\n                                   const std::vector<double>& normals)",
          "code": "PointCloud PointCloud::from_coords(const std::vector<double>& coords,\n                                   const std::vector<int>& colors,\n                                   const std::vector<double>& normals) {\n    PointCloud pc;\n    pc._coords = coords;\n    pc._colors = colors;\n    pc._normals = normals;\n    return pc;\n}",
          "file": "pointcloud.cpp"
        },
        "rust": {
          "sig": "from_coords(coords: Vec<f64>, colors: Vec<i32>, normals: Vec<f64>) -> Self",
          "code": "pub fn from_coords(coords: Vec<f64>, colors: Vec<i32>, normals: Vec<f64>) -> Self {\n        Self {\n            guid: Uuid::new_v4().to_string(),\n            name: \"my_pointcloud\".to_string(),\n            point_size: 1.0,\n            xform: Xform::identity(),\n            _coords: coords,\n            _colors: colors,\n            _normals: normals,\n        }\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.point_count",
      "implementations": {
        "python": {
          "sig": "point_count() -> int",
          "code": "def point_count(self) -> int:\n\n        \"\"\"Returns the number of points.\"\"\"\n        return len(self._coords) // 3\n\n    def __len__(self) -> int:\n        \"\"\"Returns the number of points.\"\"\"\n        return self.point_count()\n\n    def is_empty(self) -> bool:\n        \"\"\"Returns true if the point cloud has no points.\"\"\"\n        return self.point_count() == 0\n\n    def get_point(self, index: int) -> Point:\n        \"\"\"Get point at index as Point object.\"\"\"\n        idx = index * 3\n        return Point(self._coords[idx], self._coords[idx + 1], self._coords[idx + 2])\n\n    def set_point(self, index: int, point: Point) -> None:\n        \"\"\"Set point at index from Point object.\"\"\"\n        idx = index * 3",
          "file": "pointcloud.py"
        },
        "rust": {
          "sig": "point_count() -> usize",
          "code": "pub fn point_count(&self) -> usize {\n        self._coords.len() / 3\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.__len__",
      "implementations": {
        "python": {
          "sig": "__len__() -> int",
          "code": "def __len__(self) -> int:\n\n        \"\"\"Returns the number of points.\"\"\"\n        return self.point_count()\n\n    def is_empty(self) -> bool:\n        \"\"\"Returns true if the point cloud has no points.\"\"\"\n        return self.point_count() == 0\n\n    def get_point(self, index: int) -> Point:\n        \"\"\"Get point at index as Point object.\"\"\"\n        idx = index * 3\n        return Point(self._coords[idx], self._coords[idx + 1], self._coords[idx + 2])\n\n    def set_point(self, index: int, point: Point) -> None:\n        \"\"\"Set point at index from Point object.\"\"\"\n        idx = index * 3\n        self._coords[idx] = point[0]\n        self._coords[idx + 1] = point[1]\n        self._coords[idx + 2] = point[2]",
          "file": "pointcloud.py"
        }
      }
    },
    {
      "name": "PointCloud.is_empty",
      "implementations": {
        "python": {
          "sig": "is_empty() -> bool",
          "code": "def is_empty(self) -> bool:\n\n        \"\"\"Returns true if the point cloud has no points.\"\"\"\n        return self.point_count() == 0\n\n    def get_point(self, index: int) -> Point:\n        \"\"\"Get point at index as Point object.\"\"\"\n        idx = index * 3\n        return Point(self._coords[idx], self._coords[idx + 1], self._coords[idx + 2])\n\n    def set_point(self, index: int, point: Point) -> None:\n        \"\"\"Set point at index from Point object.\"\"\"\n        idx = index * 3\n        self._coords[idx] = point[0]\n        self._coords[idx + 1] = point[1]\n        self._coords[idx + 2] = point[2]\n\n    def add_point(self, point: Point) -> None:\n        \"\"\"Add a point to the cloud.\"\"\"\n        self._coords.extend([point[0], point[1], point[2]])",
          "file": "pointcloud.py"
        },
        "rust": {
          "sig": "is_empty() -> bool",
          "code": "pub fn is_empty(&self) -> bool {\n        self._coords.is_empty()\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.get_point",
      "implementations": {
        "python": {
          "sig": "get_point(index: int) -> Point",
          "code": "def get_point(self, index: int) -> Point:\n\n        \"\"\"Get point at index as Point object.\"\"\"\n        idx = index * 3\n        return Point(self._coords[idx], self._coords[idx + 1], self._coords[idx + 2])\n\n    def set_point(self, index: int, point: Point) -> None:\n        \"\"\"Set point at index from Point object.\"\"\"\n        idx = index * 3\n        self._coords[idx] = point[0]\n        self._coords[idx + 1] = point[1]\n        self._coords[idx + 2] = point[2]\n\n    def add_point(self, point: Point) -> None:\n        \"\"\"Add a point to the cloud.\"\"\"\n        self._coords.extend([point[0], point[1], point[2]])\n\n    def get_points(self) -> List[Point]:\n        \"\"\"Returns all points as Point objects.\"\"\"\n        points = []\n        for i in range(self.point_count()):",
          "file": "pointcloud.py"
        },
        "cpp": {
          "sig": "Point get_point(size_t index)",
          "code": "Point PointCloud::get_point(size_t index) const {\n    size_t idx = index * 3;\n    return Point(_coords[idx], _coords[idx + 1], _coords[idx + 2]);\n}",
          "file": "pointcloud.cpp"
        },
        "rust": {
          "sig": "get_point(index: usize) -> Point",
          "code": "pub fn get_point(&self, index: usize) -> Point {\n        let idx = index * 3;\n        Point::new(self._coords[idx], self._coords[idx + 1], self._coords[idx + 2])\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.set_point",
      "implementations": {
        "python": {
          "sig": "set_point(index: int, point: Point) -> None",
          "code": "def set_point(self, index: int, point: Point) -> None:\n\n        \"\"\"Set point at index from Point object.\"\"\"\n        idx = index * 3\n        self._coords[idx] = point[0]\n        self._coords[idx + 1] = point[1]\n        self._coords[idx + 2] = point[2]\n\n    def add_point(self, point: Point) -> None:\n        \"\"\"Add a point to the cloud.\"\"\"\n        self._coords.extend([point[0], point[1], point[2]])\n\n    def get_points(self) -> List[Point]:\n        \"\"\"Returns all points as Point objects.\"\"\"\n        points = []\n        for i in range(self.point_count()):\n            idx = i * 3\n            points.append(Point(self._coords[idx], self._coords[idx + 1], self._coords[idx + 2]))\n        return points\n\n    @property",
          "file": "pointcloud.py"
        },
        "cpp": {
          "sig": "void set_point(size_t index, const Point& point)",
          "code": "void PointCloud::set_point(size_t index, const Point& point) {\n    size_t idx = index * 3;\n    _coords[idx] = point[0];\n    _coords[idx + 1] = point[1];\n    _coords[idx + 2] = point[2];\n}",
          "file": "pointcloud.cpp"
        },
        "rust": {
          "sig": "set_point(index: usize, point: &Point)",
          "code": "pub fn set_point(&mut self, index: usize, point: &Point) {\n        let idx = index * 3;\n        self._coords[idx] = point[0];\n        self._coords[idx + 1] = point[1];\n        self._coords[idx + 2] = point[2];\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.add_point",
      "implementations": {
        "python": {
          "sig": "add_point(point: Point) -> None",
          "code": "def add_point(self, point: Point) -> None:\n\n        \"\"\"Add a point to the cloud.\"\"\"\n        self._coords.extend([point[0], point[1], point[2]])\n\n    def get_points(self) -> List[Point]:\n        \"\"\"Returns all points as Point objects.\"\"\"\n        points = []\n        for i in range(self.point_count()):\n            idx = i * 3\n            points.append(Point(self._coords[idx], self._coords[idx + 1], self._coords[idx + 2]))\n        return points\n\n    @property\n    def points(self) -> List[Point]:\n        \"\"\"Property for backward compatibility - returns list of Point objects.\"\"\"\n        return self.get_points()\n\n    @points.setter\n    def points(self, value: List[Point]) -> None:\n        \"\"\"Set points from a list of Point objects.\"\"\"",
          "file": "pointcloud.py"
        },
        "cpp": {
          "sig": "void add_point(const Point& point)",
          "code": "void PointCloud::add_point(const Point& point) {\n    _coords.push_back(point[0]);\n    _coords.push_back(point[1]);\n    _coords.push_back(point[2]);\n}",
          "file": "pointcloud.cpp"
        },
        "rust": {
          "sig": "add_point(point: &Point)",
          "code": "pub fn add_point(&mut self, point: &Point) {\n        self._coords.push(point[0]);\n        self._coords.push(point[1]);\n        self._coords.push(point[2]);\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.get_points",
      "implementations": {
        "python": {
          "sig": "get_points() -> List[Point]",
          "code": "def get_points(self) -> List[Point]:\n\n        \"\"\"Returns all points as Point objects.\"\"\"\n        points = []\n        for i in range(self.point_count()):\n            idx = i * 3\n            points.append(Point(self._coords[idx], self._coords[idx + 1], self._coords[idx + 2]))\n        return points\n\n    @property\n    def points(self) -> List[Point]:\n        \"\"\"Property for backward compatibility - returns list of Point objects.\"\"\"\n        return self.get_points()\n\n    @points.setter\n    def points(self, value: List[Point]) -> None:\n        \"\"\"Set points from a list of Point objects.\"\"\"\n        self._coords = []\n        for p in value:\n            self._coords.extend([p[0], p[1], p[2]])",
          "file": "pointcloud.py"
        },
        "cpp": {
          "sig": "std::vector<Point> get_points()",
          "code": "std::vector<Point> PointCloud::get_points() const {\n    std::vector<Point> points;\n    points.reserve(point_count());\n    for (size_t i = 0; i < point_count(); ++i) {\n        size_t idx = i * 3;\n        points.emplace_back(_coords[idx], _coords[idx + 1], _coords[idx + 2]);\n    }",
          "file": "pointcloud.cpp"
        },
        "rust": {
          "sig": "get_points() -> Vec<Point>",
          "code": "pub fn get_points(&self) -> Vec<Point> {\n        let mut points = Vec::with_capacity(self.point_count());\n        for i in 0..self.point_count() {\n            let idx = i * 3;\n            points.push(Point::new(self._coords[idx], self._coords[idx + 1], self._coords[idx + 2]));\n        }\n        points\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.points",
      "implementations": {
        "python": {
          "sig": "points(value: List[Point]) -> None",
          "code": "def points(self, value: List[Point]) -> None:\n\n        \"\"\"Set points from a list of Point objects.\"\"\"\n        self._coords = []\n        for p in value:\n            self._coords.extend([p[0], p[1], p[2]])\n\n    ###########################################################################################\n    # Color Access\n    ###########################################################################################\n\n    def color_count(self) -> int:\n        \"\"\"Returns the number of colors.\"\"\"\n        return len(self._colors) // 4\n\n    def get_color(self, index: int) -> Color:\n        \"\"\"Get color at index as Color object.\"\"\"\n        idx = index * 4\n        return Color(self._colors[idx], self._colors[idx + 1],\n                     self._colors[idx + 2], self._colors[idx + 3])",
          "file": "pointcloud.py"
        }
      }
    },
    {
      "name": "PointCloud.color_count",
      "implementations": {
        "python": {
          "sig": "color_count() -> int",
          "code": "def color_count(self) -> int:\n\n        \"\"\"Returns the number of colors.\"\"\"\n        return len(self._colors) // 4\n\n    def get_color(self, index: int) -> Color:\n        \"\"\"Get color at index as Color object.\"\"\"\n        idx = index * 4\n        return Color(self._colors[idx], self._colors[idx + 1],\n                     self._colors[idx + 2], self._colors[idx + 3])\n\n    def set_color(self, index: int, color: Color) -> None:\n        \"\"\"Set color at index from Color object.\"\"\"\n        idx = index * 4\n        self._colors[idx] = color[0]\n        self._colors[idx + 1] = color[1]\n        self._colors[idx + 2] = color[2]\n        self._colors[idx + 3] = color[3]\n\n    def add_color(self, color: Color) -> None:\n        \"\"\"Add a color to the cloud.\"\"\"",
          "file": "pointcloud.py"
        },
        "rust": {
          "sig": "color_count() -> usize",
          "code": "pub fn color_count(&self) -> usize {\n        self._colors.len() / 4\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.get_color",
      "implementations": {
        "python": {
          "sig": "get_color(index: int) -> Color",
          "code": "def get_color(self, index: int) -> Color:\n\n        \"\"\"Get color at index as Color object.\"\"\"\n        idx = index * 4\n        return Color(self._colors[idx], self._colors[idx + 1],\n                     self._colors[idx + 2], self._colors[idx + 3])\n\n    def set_color(self, index: int, color: Color) -> None:\n        \"\"\"Set color at index from Color object.\"\"\"\n        idx = index * 4\n        self._colors[idx] = color[0]\n        self._colors[idx + 1] = color[1]\n        self._colors[idx + 2] = color[2]\n        self._colors[idx + 3] = color[3]\n\n    def add_color(self, color: Color) -> None:\n        \"\"\"Add a color to the cloud.\"\"\"\n        self._colors.extend([color[0], color[1], color[2], color[3]])\n\n    def get_colors(self) -> List[Color]:\n        \"\"\"Returns all colors as Color objects.\"\"\"",
          "file": "pointcloud.py"
        },
        "cpp": {
          "sig": "Color get_color(size_t index)",
          "code": "Color PointCloud::get_color(size_t index) const {\n    size_t idx = index * 4;\n    return Color(_colors[idx], _colors[idx + 1], _colors[idx + 2], _colors[idx + 3]);\n}",
          "file": "pointcloud.cpp"
        },
        "rust": {
          "sig": "get_color(index: usize) -> Color",
          "code": "pub fn get_color(&self, index: usize) -> Color {\n        let idx = index * 4;\n        Color::new(\n            self._colors[idx] as u8,\n            self._colors[idx + 1] as u8,\n            self._colors[idx + 2] as u8,\n            self._colors[idx + 3] as u8,\n        )\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.set_color",
      "implementations": {
        "python": {
          "sig": "set_color(index: int, color: Color) -> None",
          "code": "def set_color(self, index: int, color: Color) -> None:\n\n        \"\"\"Set color at index from Color object.\"\"\"\n        idx = index * 4\n        self._colors[idx] = color[0]\n        self._colors[idx + 1] = color[1]\n        self._colors[idx + 2] = color[2]\n        self._colors[idx + 3] = color[3]\n\n    def add_color(self, color: Color) -> None:\n        \"\"\"Add a color to the cloud.\"\"\"\n        self._colors.extend([color[0], color[1], color[2], color[3]])\n\n    def get_colors(self) -> List[Color]:\n        \"\"\"Returns all colors as Color objects.\"\"\"\n        colors = []\n        for i in range(self.color_count()):\n            idx = i * 4\n            colors.append(Color(self._colors[idx], self._colors[idx + 1],\n                                self._colors[idx + 2], self._colors[idx + 3]))\n        return colors",
          "file": "pointcloud.py"
        },
        "cpp": {
          "sig": "void set_color(size_t index, const Color& color)",
          "code": "void PointCloud::set_color(size_t index, const Color& color) {\n    size_t idx = index * 4;\n    _colors[idx] = color.r;\n    _colors[idx + 1] = color.g;\n    _colors[idx + 2] = color.b;\n    _colors[idx + 3] = color.a;\n}",
          "file": "pointcloud.cpp"
        },
        "rust": {
          "sig": "set_color(index: usize, color: &Color)",
          "code": "pub fn set_color(&mut self, index: usize, color: &Color) {\n        let idx = index * 4;\n        self._colors[idx] = color.r as i32;\n        self._colors[idx + 1] = color.g as i32;\n        self._colors[idx + 2] = color.b as i32;\n        self._colors[idx + 3] = color.a as i32;\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.add_color",
      "implementations": {
        "python": {
          "sig": "add_color(color: Color) -> None",
          "code": "def add_color(self, color: Color) -> None:\n\n        \"\"\"Add a color to the cloud.\"\"\"\n        self._colors.extend([color[0], color[1], color[2], color[3]])\n\n    def get_colors(self) -> List[Color]:\n        \"\"\"Returns all colors as Color objects.\"\"\"\n        colors = []\n        for i in range(self.color_count()):\n            idx = i * 4\n            colors.append(Color(self._colors[idx], self._colors[idx + 1],\n                                self._colors[idx + 2], self._colors[idx + 3]))\n        return colors\n\n    @property\n    def colors(self) -> List[Color]:\n        \"\"\"Property for backward compatibility.\"\"\"\n        return self.get_colors()\n\n    @colors.setter\n    def colors(self, value: List[Color]) -> None:",
          "file": "pointcloud.py"
        },
        "cpp": {
          "sig": "void add_color(const Color& color)",
          "code": "void PointCloud::add_color(const Color& color) {\n    _colors.push_back(color.r);\n    _colors.push_back(color.g);\n    _colors.push_back(color.b);\n    _colors.push_back(color.a);\n}",
          "file": "pointcloud.cpp"
        },
        "rust": {
          "sig": "add_color(color: &Color)",
          "code": "pub fn add_color(&mut self, color: &Color) {\n        self._colors.push(color.r as i32);\n        self._colors.push(color.g as i32);\n        self._colors.push(color.b as i32);\n        self._colors.push(color.a as i32);\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.get_colors",
      "implementations": {
        "python": {
          "sig": "get_colors() -> List[Color]",
          "code": "def get_colors(self) -> List[Color]:\n\n        \"\"\"Returns all colors as Color objects.\"\"\"\n        colors = []\n        for i in range(self.color_count()):\n            idx = i * 4\n            colors.append(Color(self._colors[idx], self._colors[idx + 1],\n                                self._colors[idx + 2], self._colors[idx + 3]))\n        return colors\n\n    @property\n    def colors(self) -> List[Color]:\n        \"\"\"Property for backward compatibility.\"\"\"\n        return self.get_colors()\n\n    @colors.setter\n    def colors(self, value: List[Color]) -> None:\n        \"\"\"Set colors from a list of Color objects.\"\"\"\n        self._colors = []\n        for c in value:\n            self._colors.extend([c.r, c.g, c.b, c.a])",
          "file": "pointcloud.py"
        },
        "cpp": {
          "sig": "std::vector<Color> get_colors()",
          "code": "std::vector<Color> PointCloud::get_colors() const {\n    std::vector<Color> colors;\n    colors.reserve(color_count());\n    for (size_t i = 0; i < color_count(); ++i) {\n        size_t idx = i * 4;\n        colors.emplace_back(_colors[idx], _colors[idx + 1], _colors[idx + 2], _colors[idx + 3]);\n    }",
          "file": "pointcloud.cpp"
        },
        "rust": {
          "sig": "get_colors() -> Vec<Color>",
          "code": "pub fn get_colors(&self) -> Vec<Color> {\n        let mut colors = Vec::with_capacity(self.color_count());\n        for i in 0..self.color_count() {\n            let idx = i * 4;\n            colors.push(Color::new(\n                self._colors[idx] as u8,\n                self._colors[idx + 1] as u8,\n                self._colors[idx + 2] as u8,\n                self._colors[idx + 3] as u8,\n            ));\n        }\n        colors\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.colors",
      "implementations": {
        "python": {
          "sig": "colors(value: List[Color]) -> None",
          "code": "def colors(self, value: List[Color]) -> None:\n\n        \"\"\"Set colors from a list of Color objects.\"\"\"\n        self._colors = []\n        for c in value:\n            self._colors.extend([c.r, c.g, c.b, c.a])\n\n    ###########################################################################################\n    # Normal Access\n    ###########################################################################################\n\n    def normal_count(self) -> int:\n        \"\"\"Returns the number of normals.\"\"\"\n        return len(self._normals) // 3\n\n    def get_normal(self, index: int) -> Vector:\n        \"\"\"Get normal at index as Vector object.\"\"\"\n        idx = index * 3\n        return Vector(self._normals[idx], self._normals[idx + 1], self._normals[idx + 2])\n\n    def set_normal(self, index: int, normal: Vector) -> None:",
          "file": "pointcloud.py"
        }
      }
    },
    {
      "name": "PointCloud.normal_count",
      "implementations": {
        "python": {
          "sig": "normal_count() -> int",
          "code": "def normal_count(self) -> int:\n\n        \"\"\"Returns the number of normals.\"\"\"\n        return len(self._normals) // 3\n\n    def get_normal(self, index: int) -> Vector:\n        \"\"\"Get normal at index as Vector object.\"\"\"\n        idx = index * 3\n        return Vector(self._normals[idx], self._normals[idx + 1], self._normals[idx + 2])\n\n    def set_normal(self, index: int, normal: Vector) -> None:\n        \"\"\"Set normal at index from Vector object.\"\"\"\n        idx = index * 3\n        self._normals[idx] = normal[0]\n        self._normals[idx + 1] = normal[1]\n        self._normals[idx + 2] = normal[2]\n\n    def add_normal(self, normal: Vector) -> None:\n        \"\"\"Add a normal to the cloud.\"\"\"\n        self._normals.extend([normal[0], normal[1], normal[2]])",
          "file": "pointcloud.py"
        },
        "rust": {
          "sig": "normal_count() -> usize",
          "code": "pub fn normal_count(&self) -> usize {\n        self._normals.len() / 3\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.get_normal",
      "implementations": {
        "python": {
          "sig": "get_normal(index: int) -> Vector",
          "code": "def get_normal(self, index: int) -> Vector:\n\n        \"\"\"Get normal at index as Vector object.\"\"\"\n        idx = index * 3\n        return Vector(self._normals[idx], self._normals[idx + 1], self._normals[idx + 2])\n\n    def set_normal(self, index: int, normal: Vector) -> None:\n        \"\"\"Set normal at index from Vector object.\"\"\"\n        idx = index * 3\n        self._normals[idx] = normal[0]\n        self._normals[idx + 1] = normal[1]\n        self._normals[idx + 2] = normal[2]\n\n    def add_normal(self, normal: Vector) -> None:\n        \"\"\"Add a normal to the cloud.\"\"\"\n        self._normals.extend([normal[0], normal[1], normal[2]])\n\n    def get_normals(self) -> List[Vector]:\n        \"\"\"Returns all normals as Vector objects.\"\"\"\n        normals = []\n        for i in range(self.normal_count()):",
          "file": "pointcloud.py"
        },
        "cpp": {
          "sig": "Vector get_normal(size_t index)",
          "code": "Vector PointCloud::get_normal(size_t index) const {\n    size_t idx = index * 3;\n    return Vector(_normals[idx], _normals[idx + 1], _normals[idx + 2]);\n}",
          "file": "pointcloud.cpp"
        },
        "rust": {
          "sig": "get_normal(index: usize) -> Vector",
          "code": "pub fn get_normal(&self, index: usize) -> Vector {\n        let idx = index * 3;\n        Vector::new(self._normals[idx], self._normals[idx + 1], self._normals[idx + 2])\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.set_normal",
      "implementations": {
        "python": {
          "sig": "set_normal(index: int, normal: Vector) -> None",
          "code": "def set_normal(self, index: int, normal: Vector) -> None:\n\n        \"\"\"Set normal at index from Vector object.\"\"\"\n        idx = index * 3\n        self._normals[idx] = normal[0]\n        self._normals[idx + 1] = normal[1]\n        self._normals[idx + 2] = normal[2]\n\n    def add_normal(self, normal: Vector) -> None:\n        \"\"\"Add a normal to the cloud.\"\"\"\n        self._normals.extend([normal[0], normal[1], normal[2]])\n\n    def get_normals(self) -> List[Vector]:\n        \"\"\"Returns all normals as Vector objects.\"\"\"\n        normals = []\n        for i in range(self.normal_count()):\n            idx = i * 3\n            normals.append(Vector(self._normals[idx], self._normals[idx + 1], self._normals[idx + 2]))\n        return normals\n\n    @property",
          "file": "pointcloud.py"
        },
        "cpp": {
          "sig": "void set_normal(size_t index, const Vector& normal)",
          "code": "void PointCloud::set_normal(size_t index, const Vector& normal) {\n    size_t idx = index * 3;\n    _normals[idx] = normal[0];\n    _normals[idx + 1] = normal[1];\n    _normals[idx + 2] = normal[2];\n}",
          "file": "pointcloud.cpp"
        },
        "rust": {
          "sig": "set_normal(index: usize, normal: &Vector)",
          "code": "pub fn set_normal(&mut self, index: usize, normal: &Vector) {\n        let idx = index * 3;\n        self._normals[idx] = normal[0];\n        self._normals[idx + 1] = normal[1];\n        self._normals[idx + 2] = normal[2];\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.add_normal",
      "implementations": {
        "python": {
          "sig": "add_normal(normal: Vector) -> None",
          "code": "def add_normal(self, normal: Vector) -> None:\n\n        \"\"\"Add a normal to the cloud.\"\"\"\n        self._normals.extend([normal[0], normal[1], normal[2]])\n\n    def get_normals(self) -> List[Vector]:\n        \"\"\"Returns all normals as Vector objects.\"\"\"\n        normals = []\n        for i in range(self.normal_count()):\n            idx = i * 3\n            normals.append(Vector(self._normals[idx], self._normals[idx + 1], self._normals[idx + 2]))\n        return normals\n\n    @property\n    def normals(self) -> List[Vector]:\n        \"\"\"Property for backward compatibility.\"\"\"\n        return self.get_normals()\n\n    @normals.setter\n    def normals(self, value: List[Vector]) -> None:\n        \"\"\"Set normals from a list of Vector objects.\"\"\"",
          "file": "pointcloud.py"
        },
        "cpp": {
          "sig": "void add_normal(const Vector& normal)",
          "code": "void PointCloud::add_normal(const Vector& normal) {\n    _normals.push_back(normal[0]);\n    _normals.push_back(normal[1]);\n    _normals.push_back(normal[2]);\n}",
          "file": "pointcloud.cpp"
        },
        "rust": {
          "sig": "add_normal(normal: &Vector)",
          "code": "pub fn add_normal(&mut self, normal: &Vector) {\n        self._normals.push(normal[0]);\n        self._normals.push(normal[1]);\n        self._normals.push(normal[2]);\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.get_normals",
      "implementations": {
        "python": {
          "sig": "get_normals() -> List[Vector]",
          "code": "def get_normals(self) -> List[Vector]:\n\n        \"\"\"Returns all normals as Vector objects.\"\"\"\n        normals = []\n        for i in range(self.normal_count()):\n            idx = i * 3\n            normals.append(Vector(self._normals[idx], self._normals[idx + 1], self._normals[idx + 2]))\n        return normals\n\n    @property\n    def normals(self) -> List[Vector]:\n        \"\"\"Property for backward compatibility.\"\"\"\n        return self.get_normals()\n\n    @normals.setter\n    def normals(self, value: List[Vector]) -> None:\n        \"\"\"Set normals from a list of Vector objects.\"\"\"\n        self._normals = []\n        for n in value:\n            self._normals.extend([n[0], n[1], n[2]])",
          "file": "pointcloud.py"
        },
        "cpp": {
          "sig": "std::vector<Vector> get_normals()",
          "code": "std::vector<Vector> PointCloud::get_normals() const {\n    std::vector<Vector> normals;\n    normals.reserve(normal_count());\n    for (size_t i = 0; i < normal_count(); ++i) {\n        size_t idx = i * 3;\n        normals.emplace_back(_normals[idx], _normals[idx + 1], _normals[idx + 2]);\n    }",
          "file": "pointcloud.cpp"
        },
        "rust": {
          "sig": "get_normals() -> Vec<Vector>",
          "code": "pub fn get_normals(&self) -> Vec<Vector> {\n        let mut normals = Vec::with_capacity(self.normal_count());\n        for i in 0..self.normal_count() {\n            let idx = i * 3;\n            normals.push(Vector::new(self._normals[idx], self._normals[idx + 1], self._normals[idx + 2]));\n        }\n        normals\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.normals",
      "implementations": {
        "python": {
          "sig": "normals(value: List[Vector]) -> None",
          "code": "def normals(self, value: List[Vector]) -> None:\n\n        \"\"\"Set normals from a list of Vector objects.\"\"\"\n        self._normals = []\n        for n in value:\n            self._normals.extend([n[0], n[1], n[2]])\n\n    ###########################################################################################\n    # String Representations\n    ###########################################################################################\n\n    def __str__(self) -> str:\n        \"\"\"Minimal string representation.\"\"\"\n        return f\"{self.point_count()} points\"\n\n    def __repr__(self) -> str:\n        \"\"\"Full string representation.\"\"\"\n        return f\"PointCloud({self.name}, {self.point_count()} points, {self.color_count()} colors, {self.normal_count()} normals)\"\n\n    def str(self) -> str:\n        \"\"\"Minimal string representation.\"\"\"",
          "file": "pointcloud.py"
        }
      }
    },
    {
      "name": "PointCloud.__str__",
      "implementations": {
        "python": {
          "sig": "__str__() -> str",
          "code": "def __str__(self) -> str:\n\n        \"\"\"Minimal string representation.\"\"\"\n        return f\"{self.point_count()} points\"\n\n    def __repr__(self) -> str:\n        \"\"\"Full string representation.\"\"\"\n        return f\"PointCloud({self.name}, {self.point_count()} points, {self.color_count()} colors, {self.normal_count()} normals)\"\n\n    def str(self) -> str:\n        \"\"\"Minimal string representation.\"\"\"\n        return self.__str__()\n\n    def repr(self) -> str:\n        \"\"\"Full string representation.\"\"\"\n        return self.__repr__()\n\n    ###########################################################################################\n    # Duplicate and Equality\n    ###########################################################################################",
          "file": "pointcloud.py"
        }
      }
    },
    {
      "name": "PointCloud.__repr__",
      "implementations": {
        "python": {
          "sig": "__repr__() -> str",
          "code": "def __repr__(self) -> str:\n\n        \"\"\"Full string representation.\"\"\"\n        return f\"PointCloud({self.name}, {self.point_count()} points, {self.color_count()} colors, {self.normal_count()} normals)\"\n\n    def str(self) -> str:\n        \"\"\"Minimal string representation.\"\"\"\n        return self.__str__()\n\n    def repr(self) -> str:\n        \"\"\"Full string representation.\"\"\"\n        return self.__repr__()\n\n    ###########################################################################################\n    # Duplicate and Equality\n    ###########################################################################################\n\n    def duplicate(self) -> \"PointCloud\":\n        \"\"\"Create a deep copy with a new GUID.\"\"\"\n        result = copy.deepcopy(self)\n        result.guid = str(uuid.uuid4())",
          "file": "pointcloud.py"
        }
      }
    },
    {
      "name": "PointCloud.str",
      "implementations": {
        "python": {
          "sig": "str() -> str",
          "code": "def str(self) -> str:\n\n        \"\"\"Minimal string representation.\"\"\"\n        return self.__str__()\n\n    def repr(self) -> str:\n        \"\"\"Full string representation.\"\"\"\n        return self.__repr__()\n\n    ###########################################################################################\n    # Duplicate and Equality\n    ###########################################################################################\n\n    def duplicate(self) -> \"PointCloud\":\n        \"\"\"Create a deep copy with a new GUID.\"\"\"\n        result = copy.deepcopy(self)\n        result.guid = str(uuid.uuid4())\n        return result\n\n    def __eq__(self, other) -> bool:\n        \"\"\"Equality comparison (ignores guid).\"\"\"",
          "file": "pointcloud.py"
        },
        "cpp": {
          "sig": "std::string str()",
          "code": "std::string PointCloud::str() const {\n    return fmt::format(\"{}",
          "file": "pointcloud.cpp"
        },
        "rust": {
          "sig": "str() -> String",
          "code": "pub fn str(&self) -> String {\n        format!(\"{} points\", self.point_count())\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.repr",
      "implementations": {
        "python": {
          "sig": "repr() -> str",
          "code": "def repr(self) -> str:\n\n        \"\"\"Full string representation.\"\"\"\n        return self.__repr__()\n\n    ###########################################################################################\n    # Duplicate and Equality\n    ###########################################################################################\n\n    def duplicate(self) -> \"PointCloud\":\n        \"\"\"Create a deep copy with a new GUID.\"\"\"\n        result = copy.deepcopy(self)\n        result.guid = str(uuid.uuid4())\n        return result\n\n    def __eq__(self, other) -> bool:\n        \"\"\"Equality comparison (ignores guid).\"\"\"\n        if not isinstance(other, PointCloud):\n            return False\n        return (self.name == other.name and\n                self._coords == other._coords and",
          "file": "pointcloud.py"
        },
        "cpp": {
          "sig": "std::string repr()",
          "code": "std::string PointCloud::repr() const {\n    return fmt::format(\"PointCloud({}",
          "file": "pointcloud.cpp"
        },
        "rust": {
          "sig": "repr() -> String",
          "code": "pub fn repr(&self) -> String {\n        format!(\n            \"PointCloud({}, {} points, {} colors, {} normals)\",\n            self.name,\n            self.point_count(),\n            self.color_count(),\n            self.normal_count()\n        )\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.duplicate",
      "implementations": {
        "python": {
          "sig": "duplicate() -> \"PointCloud\"",
          "code": "def duplicate(self) -> \"PointCloud\":\n\n        \"\"\"Create a deep copy with a new GUID.\"\"\"\n        result = copy.deepcopy(self)\n        result.guid = str(uuid.uuid4())\n        return result\n\n    def __eq__(self, other) -> bool:\n        \"\"\"Equality comparison (ignores guid).\"\"\"\n        if not isinstance(other, PointCloud):\n            return False\n        return (self.name == other.name and\n                self._coords == other._coords and\n                self._colors == other._colors and\n                self._normals == other._normals)\n\n    ###########################################################################################\n    # Transform\n    ###########################################################################################\n\n    def transform(self) -> None:",
          "file": "pointcloud.py"
        },
        "rust": {
          "sig": "duplicate() -> Self",
          "code": "pub fn duplicate(&self) -> Self {\n        let mut result = self.clone();\n        result.guid = Uuid::new_v4().to_string();\n        result\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.__eq__",
      "implementations": {
        "python": {
          "sig": "__eq__(other) -> bool",
          "code": "def __eq__(self, other) -> bool:\n\n        \"\"\"Equality comparison (ignores guid).\"\"\"\n        if not isinstance(other, PointCloud):\n            return False\n        return (self.name == other.name and\n                self._coords == other._coords and\n                self._colors == other._colors and\n                self._normals == other._normals)\n\n    ###########################################################################################\n    # Transform\n    ###########################################################################################\n\n    def transform(self) -> None:\n        \"\"\"Apply the stored xform transformation to the point cloud in-place.\"\"\"\n        for i in range(self.point_count()):\n            idx = i * 3\n            pt = Point(self._coords[idx], self._coords[idx + 1], self._coords[idx + 2])\n            self.xform.transform_point(pt)\n            self._coords[idx] = pt[0]",
          "file": "pointcloud.py"
        }
      }
    },
    {
      "name": "PointCloud.transform",
      "implementations": {
        "python": {
          "sig": "transform() -> None",
          "code": "def transform(self) -> None:\n\n        \"\"\"Apply the stored xform transformation to the point cloud in-place.\"\"\"\n        for i in range(self.point_count()):\n            idx = i * 3\n            pt = Point(self._coords[idx], self._coords[idx + 1], self._coords[idx + 2])\n            self.xform.transform_point(pt)\n            self._coords[idx] = pt[0]\n            self._coords[idx + 1] = pt[1]\n            self._coords[idx + 2] = pt[2]\n\n        for i in range(self.normal_count()):\n            idx = i * 3\n            n = Vector(self._normals[idx], self._normals[idx + 1], self._normals[idx + 2])\n            self.xform.transform_vector(n)\n            self._normals[idx] = n[0]\n            self._normals[idx + 1] = n[1]\n            self._normals[idx + 2] = n[2]\n\n        self.xform = Xform.identity()",
          "file": "pointcloud.py"
        },
        "cpp": {
          "sig": "void transform()",
          "code": "void PointCloud::transform() {\n    for (size_t i = 0; i < point_count(); ++i) {\n        size_t idx = i * 3;\n        Point pt(_coords[idx], _coords[idx + 1], _coords[idx + 2]);\n        xform.transform_point(pt);\n        _coords[idx] = pt[0];\n        _coords[idx + 1] = pt[1];\n        _coords[idx + 2] = pt[2];\n    }",
          "file": "pointcloud.cpp"
        },
        "rust": {
          "sig": "transform()",
          "code": "pub fn transform(&mut self) {\n        for i in 0..self.point_count() {\n            let idx = i * 3;\n            let mut pt = Point::new(self._coords[idx], self._coords[idx + 1], self._coords[idx + 2]);\n            self.xform.transform_point(&mut pt);\n            self._coords[idx] = pt[0];\n            self._coords[idx + 1] = pt[1];\n            self._coords[idx + 2] = pt[2];\n        }\n\n        for i in 0..self.normal_count() {\n            let idx = i * 3;\n            let mut n = Vector::new(self._normals[idx], self._normals[idx + 1], self._normals[idx + 2]);\n            self.xform.transform_vector(&mut n);\n            self._normals[idx] = n[0];\n            self._normals[idx + 1] = n[1];\n            self._normals[idx + 2] = n[2];\n        }\n\n        self.xform = Xform::identity();\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.transformed",
      "implementations": {
        "python": {
          "sig": "transformed() -> \"PointCloud\"",
          "code": "def transformed(self) -> \"PointCloud\":\n\n        \"\"\"Return a transformed copy of the point cloud.\"\"\"\n        result = copy.deepcopy(self)\n        result.transform()\n        return result\n\n    ###########################################################################################\n    # No-copy Operators\n    ###########################################################################################\n\n    def __iadd__(self, other: Vector) -> \"PointCloud\":\n        \"\"\"Translate point cloud by vector (in-place).\"\"\"\n        for i in range(self.point_count()):\n            idx = i * 3\n            self._coords[idx] += other[0]\n            self._coords[idx + 1] += other[1]\n            self._coords[idx + 2] += other[2]\n        return self\n\n    def __isub__(self, other: Vector) -> \"PointCloud\":",
          "file": "pointcloud.py"
        },
        "cpp": {
          "sig": "PointCloud transformed()",
          "code": "PointCloud PointCloud::transformed() const {\n    PointCloud result = *this;\n    result.transform();\n    return result;\n}",
          "file": "pointcloud.cpp"
        },
        "rust": {
          "sig": "transformed() -> Self",
          "code": "pub fn transformed(&self) -> Self {\n        let mut result = self.clone();\n        result.transform();\n        result\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.__iadd__",
      "implementations": {
        "python": {
          "sig": "__iadd__(other: Vector) -> \"PointCloud\"",
          "code": "def __iadd__(self, other: Vector) -> \"PointCloud\":\n\n        \"\"\"Translate point cloud by vector (in-place).\"\"\"\n        for i in range(self.point_count()):\n            idx = i * 3\n            self._coords[idx] += other[0]\n            self._coords[idx + 1] += other[1]\n            self._coords[idx + 2] += other[2]\n        return self\n\n    def __isub__(self, other: Vector) -> \"PointCloud\":\n        \"\"\"Translate point cloud by negative vector (in-place).\"\"\"\n        for i in range(self.point_count()):\n            idx = i * 3\n            self._coords[idx] -= other[0]\n            self._coords[idx + 1] -= other[1]\n            self._coords[idx + 2] -= other[2]\n        return self\n\n    ###########################################################################################\n    # Copy Operators",
          "file": "pointcloud.py"
        }
      }
    },
    {
      "name": "PointCloud.__isub__",
      "implementations": {
        "python": {
          "sig": "__isub__(other: Vector) -> \"PointCloud\"",
          "code": "def __isub__(self, other: Vector) -> \"PointCloud\":\n\n        \"\"\"Translate point cloud by negative vector (in-place).\"\"\"\n        for i in range(self.point_count()):\n            idx = i * 3\n            self._coords[idx] -= other[0]\n            self._coords[idx + 1] -= other[1]\n            self._coords[idx + 2] -= other[2]\n        return self\n\n    ###########################################################################################\n    # Copy Operators\n    ###########################################################################################\n\n    def __add__(self, other: Vector) -> \"PointCloud\":\n        \"\"\"Translate point cloud by vector (copy).\"\"\"\n        result = self.duplicate()\n        result.guid = self.guid  # Keep same guid for copy operators\n        result += other\n        return result",
          "file": "pointcloud.py"
        }
      }
    },
    {
      "name": "PointCloud.__add__",
      "implementations": {
        "python": {
          "sig": "__add__(other: Vector) -> \"PointCloud\"",
          "code": "def __add__(self, other: Vector) -> \"PointCloud\":\n\n        \"\"\"Translate point cloud by vector (copy).\"\"\"\n        result = self.duplicate()\n        result.guid = self.guid  # Keep same guid for copy operators\n        result += other\n        return result\n\n    def __sub__(self, other: Vector) -> \"PointCloud\":\n        \"\"\"Translate point cloud by negative vector (copy).\"\"\"\n        result = self.duplicate()\n        result.guid = self.guid  # Keep same guid for copy operators\n        result -= other\n        return result\n\n    ###########################################################################################\n    # JSON Serialization\n    ###########################################################################################\n\n    def __jsondump__(self):\n        \"\"\"Serialize to polymorphic JSON format with type field.\"\"\"",
          "file": "pointcloud.py"
        }
      }
    },
    {
      "name": "PointCloud.__sub__",
      "implementations": {
        "python": {
          "sig": "__sub__(other: Vector) -> \"PointCloud\"",
          "code": "def __sub__(self, other: Vector) -> \"PointCloud\":\n\n        \"\"\"Translate point cloud by negative vector (copy).\"\"\"\n        result = self.duplicate()\n        result.guid = self.guid  # Keep same guid for copy operators\n        result -= other\n        return result\n\n    ###########################################################################################\n    # JSON Serialization\n    ###########################################################################################\n\n    def __jsondump__(self):\n        \"\"\"Serialize to polymorphic JSON format with type field.\"\"\"\n        # Alphabetical order to match Rust's serde_json\n        return {\n            \"colors\": self._colors,\n            \"coords\": self._coords,\n            \"guid\": self.guid,\n            \"name\": self.name,\n            \"normals\": self._normals,",
          "file": "pointcloud.py"
        }
      }
    },
    {
      "name": "PointCloud.__jsondump__",
      "implementations": {
        "python": {
          "sig": "__jsondump__()",
          "code": "def __jsondump__(self):\n\n        \"\"\"Serialize to polymorphic JSON format with type field.\"\"\"\n        # Alphabetical order to match Rust's serde_json\n        return {\n            \"colors\": self._colors,\n            \"coords\": self._coords,\n            \"guid\": self.guid,\n            \"name\": self.name,\n            \"normals\": self._normals,\n            \"point_size\": self.point_size,\n            \"type\": f\"{self.__class__.__name__}\",\n            \"xform\": self.xform.__jsondump__(),\n        }\n\n    @classmethod\n    def __jsonload__(cls, data, guid=None, name=None):\n        \"\"\"Deserialize from polymorphic JSON format.\"\"\"\n        from .encoders import decode_node\n\n        pc = cls.from_coords(",
          "file": "pointcloud.py"
        }
      }
    },
    {
      "name": "PointCloud.__jsonload__",
      "implementations": {
        "python": {
          "sig": "__jsonload__(cls, data, guid=None, name=None)",
          "code": "def __jsonload__(cls, data, guid=None, name=None):\n\n        \"\"\"Deserialize from polymorphic JSON format.\"\"\"\n        from .encoders import decode_node\n\n        pc = cls.from_coords(\n            data.get(\"coords\", []),\n            data.get(\"colors\", []),\n            data.get(\"normals\", [])\n        )\n        pc.guid = guid if guid is not None else data.get(\"guid\", pc.guid)\n        pc.name = name if name is not None else data.get(\"name\", pc.name)\n\n        if \"point_size\" in data:\n            pc.point_size = data[\"point_size\"]\n        if \"xform\" in data:\n            pc.xform = decode_node(data[\"xform\"])\n\n        return pc\n\n    def json_dump(self, filepath) -> None:",
          "file": "pointcloud.py"
        }
      }
    },
    {
      "name": "PointCloud.json_dump",
      "implementations": {
        "python": {
          "sig": "json_dump(filepath) -> None",
          "code": "def json_dump(self, filepath) -> None:\n\n        \"\"\"Write JSON to file.\"\"\"\n        import json\n        with open(filepath, 'w') as f:\n            json.dump(self.__jsondump__(), f, indent=2)\n\n    @classmethod\n    def json_load(cls, filepath) -> \"PointCloud\":\n        \"\"\"Read JSON from file.\"\"\"\n        import json\n        with open(filepath, 'r') as f:\n            data = json.load(f)\n        return cls.__jsonload__(data)\n\n    ###########################################################################################\n    # Protobuf Serialization\n    ###########################################################################################\n\n    def to_protobuf(self) -> bytes:\n        \"\"\"Convert to protobuf binary format.\"\"\"",
          "file": "pointcloud.py"
        },
        "cpp": {
          "sig": "void json_dump(const std::string& filename)",
          "code": "void PointCloud::json_dump(const std::string& filename) const {\n    std::ofstream ofs(filename);\n    ofs << jsondump().dump(2);\n    ofs.close();\n}",
          "file": "pointcloud.cpp"
        },
        "rust": {
          "sig": "json_dump(filepath: &str) -> Result<(), Box<dyn std::error::Error>>",
          "code": "pub fn json_dump(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {\n        let json_str = self.jsondump()?;\n        std::fs::write(filepath, json_str)?;\n        Ok(())\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.json_load",
      "implementations": {
        "python": {
          "sig": "json_load(cls, filepath) -> \"PointCloud\"",
          "code": "def json_load(cls, filepath) -> \"PointCloud\":\n\n        \"\"\"Read JSON from file.\"\"\"\n        import json\n        with open(filepath, 'r') as f:\n            data = json.load(f)\n        return cls.__jsonload__(data)\n\n    ###########################################################################################\n    # Protobuf Serialization\n    ###########################################################################################\n\n    def to_protobuf(self) -> bytes:\n        \"\"\"Convert to protobuf binary format.\"\"\"\n        from .proto import pointcloud_pb2\n\n        proto = pointcloud_pb2.PointCloud()\n        proto.guid = self.guid\n        proto.name = self.name\n        proto.coords.extend(self._coords)\n        proto.colors.extend(self._colors)",
          "file": "pointcloud.py"
        },
        "cpp": {
          "sig": "PointCloud json_load(const std::string& filename)",
          "code": "PointCloud PointCloud::json_load(const std::string& filename) {\n    std::ifstream ifs(filename);\n    nlohmann::json data;\n    ifs >> data;\n    ifs.close();\n    return jsonload(data);\n}",
          "file": "pointcloud.cpp"
        },
        "rust": {
          "sig": "json_load(filepath: &str) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn json_load(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {\n        let json_str = std::fs::read_to_string(filepath)?;\n        Self::jsonload(&json_str)\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.to_protobuf",
      "implementations": {
        "python": {
          "sig": "to_protobuf() -> bytes",
          "code": "def to_protobuf(self) -> bytes:\n\n        \"\"\"Convert to protobuf binary format.\"\"\"\n        from .proto import pointcloud_pb2\n\n        proto = pointcloud_pb2.PointCloud()\n        proto.guid = self.guid\n        proto.name = self.name\n        proto.coords.extend(self._coords)\n        proto.colors.extend(self._colors)\n        proto.normals.extend(self._normals)\n        proto.point_size = self.point_size\n\n        # Serialize xform\n        proto.xform.name = self.xform.name\n        proto.xform.matrix.extend(self.xform.m)\n\n        return proto.SerializeToString()\n\n    @classmethod\n    def from_protobuf(cls, data: bytes) -> \"PointCloud\":",
          "file": "pointcloud.py"
        },
        "cpp": {
          "sig": "std::string to_protobuf()",
          "code": "std::string PointCloud::to_protobuf() const {\n    throw std::runtime_error(\"Protobuf support not enabled\");\n}",
          "file": "pointcloud.cpp"
        },
        "rust": {
          "sig": "to_protobuf() -> Vec<u8>",
          "code": "pub fn to_protobuf(&self) -> Vec<u8> {\n        panic!(\"Protobuf support not enabled\")\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.from_protobuf",
      "implementations": {
        "python": {
          "sig": "from_protobuf(cls, data: bytes) -> \"PointCloud\"",
          "code": "def from_protobuf(cls, data: bytes) -> \"PointCloud\":\n\n        \"\"\"Create from protobuf binary format.\"\"\"\n        from .proto import pointcloud_pb2\n\n        proto = pointcloud_pb2.PointCloud()\n        proto.ParseFromString(data)\n\n        pc = cls.from_coords(\n            list(proto.coords),\n            list(proto.colors),\n            list(proto.normals)\n        )\n        pc.guid = proto.guid\n        pc.name = proto.name\n        pc.point_size = proto.point_size if proto.point_size > 0 else 1.0\n\n        # Deserialize xform\n        if proto.HasField(\"xform\"):\n            pc.xform.name = proto.xform.name\n            for i, val in enumerate(proto.xform.matrix):",
          "file": "pointcloud.py"
        },
        "cpp": {
          "sig": "PointCloud from_protobuf(const std::string& data)",
          "code": "PointCloud PointCloud::from_protobuf(const std::string& data) {\n    (void)data;\n    throw std::runtime_error(\"Protobuf support not enabled\");\n}",
          "file": "pointcloud.cpp"
        },
        "rust": {
          "sig": "from_protobuf(_data: &[u8]) -> Self",
          "code": "pub fn from_protobuf(_data: &[u8]) -> Self {\n        panic!(\"Protobuf support not enabled\")\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.protobuf_dump",
      "implementations": {
        "python": {
          "sig": "protobuf_dump(filepath) -> None",
          "code": "def protobuf_dump(self, filepath) -> None:\n\n        \"\"\"Write protobuf to file.\"\"\"\n        with open(filepath, 'wb') as f:\n            f.write(self.to_protobuf())\n\n    @classmethod\n    def protobuf_load(cls, filepath) -> \"PointCloud\":\n        \"\"\"Read protobuf from file.\"\"\"\n        with open(filepath, 'rb') as f:\n            data = f.read()\n        return cls.from_protobuf(data)",
          "file": "pointcloud.py"
        },
        "cpp": {
          "sig": "void protobuf_dump(const std::string& filename)",
          "code": "void PointCloud::protobuf_dump(const std::string& filename) const {\n    (void)filename;\n    throw std::runtime_error(\"Protobuf support not enabled\");\n}",
          "file": "pointcloud.cpp"
        },
        "rust": {
          "sig": "protobuf_dump(_filepath: &str)",
          "code": "pub fn protobuf_dump(&self, _filepath: &str) {\n        panic!(\"Protobuf support not enabled\")\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.protobuf_load",
      "implementations": {
        "python": {
          "sig": "protobuf_load(cls, filepath) -> \"PointCloud\"",
          "code": "def protobuf_load(cls, filepath) -> \"PointCloud\":\n\n        \"\"\"Read protobuf from file.\"\"\"\n        with open(filepath, 'rb') as f:\n            data = f.read()\n        return cls.from_protobuf(data)",
          "file": "pointcloud.py"
        },
        "cpp": {
          "sig": "PointCloud protobuf_load(const std::string& filename)",
          "code": "PointCloud PointCloud::protobuf_load(const std::string& filename) {\n    (void)filename;\n    throw std::runtime_error(\"Protobuf support not enabled\");\n}",
          "file": "pointcloud.cpp"
        },
        "rust": {
          "sig": "protobuf_load(_filepath: &str) -> Self",
          "code": "pub fn protobuf_load(_filepath: &str) -> Self {\n        panic!(\"Protobuf support not enabled\")\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "Polyline.__init__",
      "implementations": {
        "python": {
          "sig": "__init__(points: Optional[List[Point]] = None)",
          "code": "def __init__(self, points: Optional[List[Point]] = None):\n\n        \"\"\"Creates a new Polyline with default guid and name.\n\n        Args:\n            points: The collection of points (converted to flat coords internally).\n        \"\"\"\n        self.guid = str(uuid.uuid4())\n        self.name = \"my_polyline\"\n        self.width = 1.0\n        self.linecolor = Color.white()\n        self.xform = Xform.identity()\n\n        # Store coordinates as flat array [x0, y0, z0, x1, y1, z1, ...]\n        self._coords: List[float] = []\n        if points is not None:\n            for p in points:\n                self._coords.extend([p[0], p[1], p[2]])\n\n        # Delegate plane computation to Plane.from_points\n        if self.point_count() >= 3:",
          "file": "polyline.py"
        }
      }
    },
    {
      "name": "Polyline.from_coords",
      "implementations": {
        "python": {
          "sig": "from_coords(cls, coords: List[float]) -> \"Polyline\"",
          "code": "def from_coords(cls, coords: List[float]) -> \"Polyline\":\n\n        \"\"\"Create a Polyline from a flat coordinate array.\n\n        Args:\n            coords: Flat array [x0, y0, z0, x1, y1, z1, ...]\n\n        Returns:\n            New Polyline instance.\n        \"\"\"\n        pl = cls()\n        pl._coords = list(coords)\n        if pl.point_count() >= 3:\n            pl.plane = Plane.from_points(pl.get_points())\n        return pl\n\n    ###########################################################################################\n    # Point Access (compatibility layer)\n    ###########################################################################################\n\n    def point_count(self) -> int:",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "Polyline from_coords(const std::vector<double>& coords)",
          "code": "Polyline Polyline::from_coords(const std::vector<double>& coords) {\n    Polyline pl;\n    pl._coords = coords;\n    pl.recompute_plane_if_needed();\n    return pl;\n}",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "from_coords(coords: Vec<f64>) -> Self",
          "code": "pub fn from_coords(coords: Vec<f64>) -> Self {\n        let mut pl = Self {\n            guid: Uuid::new_v4().to_string(),\n            name: \"my_polyline\".to_string(),\n            coords,\n            plane: Plane::default(),\n            width: 1.0,\n            linecolor: Color::white(),\n            xform: Xform::identity(),\n        };\n        pl.recompute_plane_if_needed();\n        pl\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.point_count",
      "implementations": {
        "python": {
          "sig": "point_count() -> int",
          "code": "def point_count(self) -> int:\n\n        \"\"\"Returns the number of points.\"\"\"\n        return len(self._coords) // 3\n\n    def get_points(self) -> List[Point]:\n        \"\"\"Returns all points as Point objects.\"\"\"\n        points = []\n        for i in range(self.point_count()):\n            idx = i * 3\n            points.append(Point(self._coords[idx], self._coords[idx + 1], self._coords[idx + 2]))\n        return points\n\n    @property\n    def points(self) -> List[Point]:\n        \"\"\"Property for backward compatibility - returns list of Point objects.\"\"\"\n        return self.get_points()\n\n    @points.setter\n    def points(self, value: List[Point]) -> None:\n        \"\"\"Set points from a list of Point objects.\"\"\"",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "size_t point_count()",
          "code": "size_t Polyline::point_count() const {\n    return _coords.size() / 3;\n}",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "point_count() -> usize",
          "code": "pub fn point_count(&self) -> usize {\n        self.coords.len() / 3\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.get_points",
      "implementations": {
        "python": {
          "sig": "get_points() -> List[Point]",
          "code": "def get_points(self) -> List[Point]:\n\n        \"\"\"Returns all points as Point objects.\"\"\"\n        points = []\n        for i in range(self.point_count()):\n            idx = i * 3\n            points.append(Point(self._coords[idx], self._coords[idx + 1], self._coords[idx + 2]))\n        return points\n\n    @property\n    def points(self) -> List[Point]:\n        \"\"\"Property for backward compatibility - returns list of Point objects.\"\"\"\n        return self.get_points()\n\n    @points.setter\n    def points(self, value: List[Point]) -> None:\n        \"\"\"Set points from a list of Point objects.\"\"\"\n        self._coords = []\n        for p in value:\n            self._coords.extend([p[0], p[1], p[2]])",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "std::vector<Point> get_points()",
          "code": "std::vector<Point> Polyline::get_points() const {\n    std::vector<Point> points;\n    points.reserve(point_count());\n    for (size_t i = 0; i < point_count(); i++) {\n        size_t idx = i * 3;\n        points.emplace_back(_coords[idx], _coords[idx + 1], _coords[idx + 2]);\n    }",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "get_points() -> Vec<Point>",
          "code": "pub fn get_points(&self) -> Vec<Point> {\n        let mut points = Vec::with_capacity(self.point_count());\n        for i in 0..self.point_count() {\n            let idx = i * 3;\n            points.push(Point::new(\n                self.coords[idx],\n                self.coords[idx + 1],\n                self.coords[idx + 2],\n            ));\n        }\n        points\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.points",
      "implementations": {
        "python": {
          "sig": "points(value: List[Point]) -> None",
          "code": "def points(self, value: List[Point]) -> None:\n\n        \"\"\"Set points from a list of Point objects.\"\"\"\n        self._coords = []\n        for p in value:\n            self._coords.extend([p[0], p[1], p[2]])\n\n    def __len__(self) -> int:\n        \"\"\"Returns the number of points in the polyline.\"\"\"\n        return self.point_count()\n\n    def is_empty(self) -> bool:\n        \"\"\"Returns true if the polyline has no points.\"\"\"\n        return self.point_count() == 0\n\n    def segment_count(self) -> int:\n        \"\"\"Returns the number of segments (n-1 for n points).\"\"\"\n        n = self.point_count()\n        return n - 1 if n > 1 else 0\n\n    def length(self) -> float:",
          "file": "polyline.py"
        }
      }
    },
    {
      "name": "Polyline.__len__",
      "implementations": {
        "python": {
          "sig": "__len__() -> int",
          "code": "def __len__(self) -> int:\n\n        \"\"\"Returns the number of points in the polyline.\"\"\"\n        return self.point_count()\n\n    def is_empty(self) -> bool:\n        \"\"\"Returns true if the polyline has no points.\"\"\"\n        return self.point_count() == 0\n\n    def segment_count(self) -> int:\n        \"\"\"Returns the number of segments (n-1 for n points).\"\"\"\n        n = self.point_count()\n        return n - 1 if n > 1 else 0\n\n    def length(self) -> float:\n        \"\"\"Calculates the total length of the polyline.\"\"\"\n        total_length = 0.0\n        for i in range(self.segment_count()):\n            idx0 = i * 3\n            idx1 = (i + 1) * 3\n            dx = self._coords[idx1] - self._coords[idx0]",
          "file": "polyline.py"
        }
      }
    },
    {
      "name": "Polyline.is_empty",
      "implementations": {
        "python": {
          "sig": "is_empty() -> bool",
          "code": "def is_empty(self) -> bool:\n\n        \"\"\"Returns true if the polyline has no points.\"\"\"\n        return self.point_count() == 0\n\n    def segment_count(self) -> int:\n        \"\"\"Returns the number of segments (n-1 for n points).\"\"\"\n        n = self.point_count()\n        return n - 1 if n > 1 else 0\n\n    def length(self) -> float:\n        \"\"\"Calculates the total length of the polyline.\"\"\"\n        total_length = 0.0\n        for i in range(self.segment_count()):\n            idx0 = i * 3\n            idx1 = (i + 1) * 3\n            dx = self._coords[idx1] - self._coords[idx0]\n            dy = self._coords[idx1 + 1] - self._coords[idx0 + 1]\n            dz = self._coords[idx1 + 2] - self._coords[idx0 + 2]\n            total_length += (dx * dx + dy * dy + dz * dz) ** 0.5\n        return total_length",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "bool is_empty()",
          "code": "bool Polyline::is_empty() const {\n    return _coords.empty();\n}",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "is_empty() -> bool",
          "code": "pub fn is_empty(&self) -> bool {\n        self.coords.is_empty()\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.segment_count",
      "implementations": {
        "python": {
          "sig": "segment_count() -> int",
          "code": "def segment_count(self) -> int:\n\n        \"\"\"Returns the number of segments (n-1 for n points).\"\"\"\n        n = self.point_count()\n        return n - 1 if n > 1 else 0\n\n    def length(self) -> float:\n        \"\"\"Calculates the total length of the polyline.\"\"\"\n        total_length = 0.0\n        for i in range(self.segment_count()):\n            idx0 = i * 3\n            idx1 = (i + 1) * 3\n            dx = self._coords[idx1] - self._coords[idx0]\n            dy = self._coords[idx1 + 1] - self._coords[idx0 + 1]\n            dz = self._coords[idx1 + 2] - self._coords[idx0 + 2]\n            total_length += (dx * dx + dy * dy + dz * dz) ** 0.5\n        return total_length\n\n    def get_point(self, index: int) -> Optional[Point]:\n        \"\"\"Returns the point at the given index, or None if out of bounds.\"\"\"\n        if 0 <= index < self.point_count():",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "size_t segment_count()",
          "code": "size_t Polyline::segment_count() const {\n    size_t n = point_count();\n    return n > 1 ? n - 1 : 0;\n}",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "segment_count() -> usize",
          "code": "pub fn segment_count(&self) -> usize {\n        let n = self.point_count();\n        if n > 1 { n - 1 } else { 0 }\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.length",
      "implementations": {
        "python": {
          "sig": "length() -> float",
          "code": "def length(self) -> float:\n\n        \"\"\"Calculates the total length of the polyline.\"\"\"\n        total_length = 0.0\n        for i in range(self.segment_count()):\n            idx0 = i * 3\n            idx1 = (i + 1) * 3\n            dx = self._coords[idx1] - self._coords[idx0]\n            dy = self._coords[idx1 + 1] - self._coords[idx0 + 1]\n            dz = self._coords[idx1 + 2] - self._coords[idx0 + 2]\n            total_length += (dx * dx + dy * dy + dz * dz) ** 0.5\n        return total_length\n\n    def get_point(self, index: int) -> Optional[Point]:\n        \"\"\"Returns the point at the given index, or None if out of bounds.\"\"\"\n        if 0 <= index < self.point_count():\n            idx = index * 3\n            return Point(self._coords[idx], self._coords[idx + 1], self._coords[idx + 2])\n        return None\n\n    def set_point(self, index: int, point: Point) -> None:",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "double length()",
          "code": "double Polyline::length() const {\n    double total_length = 0.0;\n    for (size_t i = 0; i < segment_count(); i++) {\n        size_t idx0 = i * 3;\n        size_t idx1 = (i + 1) * 3;\n        double dx = _coords[idx1] - _coords[idx0];\n        double dy = _coords[idx1 + 1] - _coords[idx0 + 1];\n        double dz = _coords[idx1 + 2] - _coords[idx0 + 2];\n        total_length += std::sqrt(dx * dx + dy * dy + dz * dz);\n    }",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "length() -> f64",
          "code": "pub fn length(&self) -> f64 {\n        let mut total_length = 0.0;\n        for i in 0..self.segment_count() {\n            let idx0 = i * 3;\n            let idx1 = (i + 1) * 3;\n            let dx = self.coords[idx1] - self.coords[idx0];\n            let dy = self.coords[idx1 + 1] - self.coords[idx0 + 1];\n            let dz = self.coords[idx1 + 2] - self.coords[idx0 + 2];\n            total_length += (dx * dx + dy * dy + dz * dz).sqrt();\n        }\n        total_length\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.get_point",
      "implementations": {
        "python": {
          "sig": "get_point(index: int) -> Optional[Point]",
          "code": "def get_point(self, index: int) -> Optional[Point]:\n\n        \"\"\"Returns the point at the given index, or None if out of bounds.\"\"\"\n        if 0 <= index < self.point_count():\n            idx = index * 3\n            return Point(self._coords[idx], self._coords[idx + 1], self._coords[idx + 2])\n        return None\n\n    def set_point(self, index: int, point: Point) -> None:\n        \"\"\"Sets the point at the given index.\"\"\"\n        if 0 <= index < self.point_count():\n            idx = index * 3\n            self._coords[idx] = point[0]\n            self._coords[idx + 1] = point[1]\n            self._coords[idx + 2] = point[2]\n\n    def add_point(self, point: Point) -> None:\n        \"\"\"Adds a point to the end of the polyline.\"\"\"\n        self._coords.extend([point[0], point[1], point[2]])\n        if self.point_count() == 3:\n            self._recompute_plane()",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "Point get_point(size_t index)",
          "code": "Point Polyline::get_point(size_t index) const {\n    if (index < point_count()) {\n        size_t idx = index * 3;\n        return Point(_coords[idx], _coords[idx + 1], _coords[idx + 2]);\n    }",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "get_point(index: usize) -> Option<Point>",
          "code": "pub fn get_point(&self, index: usize) -> Option<Point> {\n        if index < self.point_count() {\n            let idx = index * 3;\n            Some(Point::new(\n                self.coords[idx],\n                self.coords[idx + 1],\n                self.coords[idx + 2],\n            ))\n        } else {\n            None\n        }\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.set_point",
      "implementations": {
        "python": {
          "sig": "set_point(index: int, point: Point) -> None",
          "code": "def set_point(self, index: int, point: Point) -> None:\n\n        \"\"\"Sets the point at the given index.\"\"\"\n        if 0 <= index < self.point_count():\n            idx = index * 3\n            self._coords[idx] = point[0]\n            self._coords[idx + 1] = point[1]\n            self._coords[idx + 2] = point[2]\n\n    def add_point(self, point: Point) -> None:\n        \"\"\"Adds a point to the end of the polyline.\"\"\"\n        self._coords.extend([point[0], point[1], point[2]])\n        if self.point_count() == 3:\n            self._recompute_plane()\n\n    def insert_point(self, index: int, point: Point) -> None:\n        \"\"\"Inserts a point at the specified index.\"\"\"\n        idx = index * 3\n        self._coords[idx:idx] = [point[0], point[1], point[2]]\n        if self.point_count() == 3:\n            self._recompute_plane()",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "void set_point(size_t index, const Point& point)",
          "code": "void Polyline::set_point(size_t index, const Point& point) {\n    if (index < point_count()) {\n        size_t idx = index * 3;\n        _coords[idx] = point[0];\n        _coords[idx + 1] = point[1];\n        _coords[idx + 2] = point[2];\n    }",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "set_point(index: usize, point: &Point)",
          "code": "pub fn set_point(&mut self, index: usize, point: &Point) {\n        if index < self.point_count() {\n            let idx = index * 3;\n            self.coords[idx] = point[0];\n            self.coords[idx + 1] = point[1];\n            self.coords[idx + 2] = point[2];\n        }\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.add_point",
      "implementations": {
        "python": {
          "sig": "add_point(point: Point) -> None",
          "code": "def add_point(self, point: Point) -> None:\n\n        \"\"\"Adds a point to the end of the polyline.\"\"\"\n        self._coords.extend([point[0], point[1], point[2]])\n        if self.point_count() == 3:\n            self._recompute_plane()\n\n    def insert_point(self, index: int, point: Point) -> None:\n        \"\"\"Inserts a point at the specified index.\"\"\"\n        idx = index * 3\n        self._coords[idx:idx] = [point[0], point[1], point[2]]\n        if self.point_count() == 3:\n            self._recompute_plane()\n\n    def remove_point(self, index: int) -> Optional[Point]:\n        \"\"\"Removes and returns the point at the specified index.\"\"\"\n        if 0 <= index < self.point_count():\n            idx = index * 3\n            point = Point(self._coords[idx], self._coords[idx + 1], self._coords[idx + 2])\n            del self._coords[idx:idx + 3]\n            if self.point_count() == 3:",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "void add_point(const Point& point)",
          "code": "void Polyline::add_point(const Point& point) {\n    _coords.push_back(point[0]);\n    _coords.push_back(point[1]);\n    _coords.push_back(point[2]);\n    if (point_count() == 3) {\n        recompute_plane_if_needed();\n    }",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "add_point(point: Point)",
          "code": "pub fn add_point(&mut self, point: Point) {\n        self.coords.push(point[0]);\n        self.coords.push(point[1]);\n        self.coords.push(point[2]);\n        // Recompute plane if we have at least 3 points\n        if self.point_count() == 3 {\n            self.recompute_plane_if_needed();\n        }\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.insert_point",
      "implementations": {
        "python": {
          "sig": "insert_point(index: int, point: Point) -> None",
          "code": "def insert_point(self, index: int, point: Point) -> None:\n\n        \"\"\"Inserts a point at the specified index.\"\"\"\n        idx = index * 3\n        self._coords[idx:idx] = [point[0], point[1], point[2]]\n        if self.point_count() == 3:\n            self._recompute_plane()\n\n    def remove_point(self, index: int) -> Optional[Point]:\n        \"\"\"Removes and returns the point at the specified index.\"\"\"\n        if 0 <= index < self.point_count():\n            idx = index * 3\n            point = Point(self._coords[idx], self._coords[idx + 1], self._coords[idx + 2])\n            del self._coords[idx:idx + 3]\n            if self.point_count() == 3:\n                self._recompute_plane()\n            return point\n        return None\n\n    def reverse(self) -> None:\n        \"\"\"Reverses the order of points in the polyline.\"\"\"",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "void insert_point(size_t index, const Point& point)",
          "code": "void Polyline::insert_point(size_t index, const Point& point) {\n    if (index <= point_count()) {\n        size_t idx = index * 3;\n        _coords.insert(_coords.begin() + idx, {point[0], point[1], point[2]}",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "insert_point(index: usize, point: Point)",
          "code": "pub fn insert_point(&mut self, index: usize, point: Point) {\n        let idx = index * 3;\n        if idx <= self.coords.len() {\n            self.coords.insert(idx, point[2]);\n            self.coords.insert(idx, point[1]);\n            self.coords.insert(idx, point[0]);\n            // Recompute plane if we have at least 3 points\n            if self.point_count() == 3 {\n                self.recompute_plane_if_needed();\n            }\n        }\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.remove_point",
      "implementations": {
        "python": {
          "sig": "remove_point(index: int) -> Optional[Point]",
          "code": "def remove_point(self, index: int) -> Optional[Point]:\n\n        \"\"\"Removes and returns the point at the specified index.\"\"\"\n        if 0 <= index < self.point_count():\n            idx = index * 3\n            point = Point(self._coords[idx], self._coords[idx + 1], self._coords[idx + 2])\n            del self._coords[idx:idx + 3]\n            if self.point_count() == 3:\n                self._recompute_plane()\n            return point\n        return None\n\n    def reverse(self) -> None:\n        \"\"\"Reverses the order of points in the polyline.\"\"\"\n        # Reverse coords in groups of 3\n        n = self.point_count()\n        new_coords = []\n        for i in range(n - 1, -1, -1):\n            idx = i * 3\n            new_coords.extend([self._coords[idx], self._coords[idx + 1], self._coords[idx + 2]])\n        self._coords = new_coords",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "bool remove_point(size_t index, Point& out_point)",
          "code": "bool Polyline::remove_point(size_t index, Point& out_point) {\n    if (index < point_count()) {\n        size_t idx = index * 3;\n        out_point = Point(_coords[idx], _coords[idx + 1], _coords[idx + 2]);\n        _coords.erase(_coords.begin() + idx, _coords.begin() + idx + 3);\n        if (point_count() == 3) {\n            recompute_plane_if_needed();\n        }",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "remove_point(index: usize) -> Option<Point>",
          "code": "pub fn remove_point(&mut self, index: usize) -> Option<Point> {\n        if index < self.point_count() {\n            let idx = index * 3;\n            let z = self.coords.remove(idx + 2);\n            let y = self.coords.remove(idx + 1);\n            let x = self.coords.remove(idx);\n            // Recompute plane if we still have at least 3 points\n            if self.point_count() == 3 {\n                self.recompute_plane_if_needed();\n            }\n            Some(Point::new(x, y, z))\n        } else {\n            None\n        }\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.reverse",
      "implementations": {
        "python": {
          "sig": "reverse() -> None",
          "code": "def reverse(self) -> None:\n\n        \"\"\"Reverses the order of points in the polyline.\"\"\"\n        # Reverse coords in groups of 3\n        n = self.point_count()\n        new_coords = []\n        for i in range(n - 1, -1, -1):\n            idx = i * 3\n            new_coords.extend([self._coords[idx], self._coords[idx + 1], self._coords[idx + 2]])\n        self._coords = new_coords\n        self.plane.reverse()\n\n    def reversed(self) -> \"Polyline\":\n        \"\"\"Returns a new polyline with reversed point order.\"\"\"\n        result = Polyline.from_coords(self._coords[:])\n        result.guid = self.guid\n        result.name = self.name\n        result.width = self.width\n        result.linecolor = copy.deepcopy(self.linecolor)\n        result.xform = copy.deepcopy(self.xform)\n        result.plane = copy.deepcopy(self.plane)",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "void reverse()",
          "code": "void Polyline::reverse() {\n    // Reverse coords in groups of 3\n    size_t n = point_count();\n    std::vector<double> new_coords;\n    new_coords.reserve(_coords.size());\n    for (size_t i = n; i > 0; i--) {\n        size_t idx = (i - 1) * 3;\n        new_coords.push_back(_coords[idx]);\n        new_coords.push_back(_coords[idx + 1]);\n        new_coords.push_back(_coords[idx + 2]);\n    }",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "reverse()",
          "code": "pub fn reverse(&mut self) {\n        let n = self.point_count();\n        if n <= 1 {\n            return;\n        }\n        // Reverse in groups of 3\n        let mut new_coords = Vec::with_capacity(self.coords.len());\n        for i in (0..n).rev() {\n            let idx = i * 3;\n            new_coords.push(self.coords[idx]);\n            new_coords.push(self.coords[idx + 1]);\n            new_coords.push(self.coords[idx + 2]);\n        }\n        self.coords = new_coords;\n        self.plane.reverse();\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.reversed",
      "implementations": {
        "python": {
          "sig": "reversed() -> \"Polyline\"",
          "code": "def reversed(self) -> \"Polyline\":\n\n        \"\"\"Returns a new polyline with reversed point order.\"\"\"\n        result = Polyline.from_coords(self._coords[:])\n        result.guid = self.guid\n        result.name = self.name\n        result.width = self.width\n        result.linecolor = copy.deepcopy(self.linecolor)\n        result.xform = copy.deepcopy(self.xform)\n        result.plane = copy.deepcopy(self.plane)\n        result.reverse()\n        return result\n\n    def _recompute_plane(self) -> None:\n        \"\"\"Helper to recompute plane when points change.\"\"\"\n        if self.point_count() >= 3:\n            self.plane = Plane.from_points(self.get_points())\n\n    ###########################################################################################\n    # Core Methods\n    ###########################################################################################",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "Polyline reversed()",
          "code": "Polyline Polyline::reversed() const {\n    Polyline result = *this;\n    result.reverse();\n    return result;\n}",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "reversed() -> Self",
          "code": "pub fn reversed(&self) -> Self {\n        let mut reversed = self.clone();\n        reversed.reverse();\n        reversed\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline._recompute_plane",
      "implementations": {
        "python": {
          "sig": "_recompute_plane() -> None",
          "code": "def _recompute_plane(self) -> None:\n\n        \"\"\"Helper to recompute plane when points change.\"\"\"\n        if self.point_count() >= 3:\n            self.plane = Plane.from_points(self.get_points())\n\n    ###########################################################################################\n    # Core Methods\n    ###########################################################################################\n\n    def duplicate(self) -> \"Polyline\":\n        \"\"\"Create a deep copy with a new GUID.\"\"\"\n        result = copy.deepcopy(self)\n        result.guid = str(uuid.uuid4())\n        return result\n\n    def __iadd__(self, vector: Vector) -> \"Polyline\":\n        \"\"\"Translates all points in the polyline by a vector (+=).\"\"\"\n        for i in range(self.point_count()):\n            idx = i * 3\n            self._coords[idx] += vector[0]",
          "file": "polyline.py"
        }
      }
    },
    {
      "name": "Polyline.duplicate",
      "implementations": {
        "python": {
          "sig": "duplicate() -> \"Polyline\"",
          "code": "def duplicate(self) -> \"Polyline\":\n\n        \"\"\"Create a deep copy with a new GUID.\"\"\"\n        result = copy.deepcopy(self)\n        result.guid = str(uuid.uuid4())\n        return result\n\n    def __iadd__(self, vector: Vector) -> \"Polyline\":\n        \"\"\"Translates all points in the polyline by a vector (+=).\"\"\"\n        for i in range(self.point_count()):\n            idx = i * 3\n            self._coords[idx] += vector[0]\n            self._coords[idx + 1] += vector[1]\n            self._coords[idx + 2] += vector[2]\n        # Update plane origin\n        self.plane = Plane(\n            self.plane.origin + vector, self.plane.x_axis, self.plane.y_axis\n        )\n        return self\n\n    def __add__(self, vector: Vector) -> \"Polyline\":",
          "file": "polyline.py"
        },
        "rust": {
          "sig": "duplicate() -> Self",
          "code": "pub fn duplicate(&self) -> Self {\n        Self {\n            guid: Uuid::new_v4().to_string(),\n            name: self.name.clone(),\n            coords: self.coords.clone(),\n            plane: self.plane.clone(),\n            width: self.width,\n            linecolor: self.linecolor.clone(),\n            xform: self.xform.clone(),\n        }\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.__iadd__",
      "implementations": {
        "python": {
          "sig": "__iadd__(vector: Vector) -> \"Polyline\"",
          "code": "def __iadd__(self, vector: Vector) -> \"Polyline\":\n\n        \"\"\"Translates all points in the polyline by a vector (+=).\"\"\"\n        for i in range(self.point_count()):\n            idx = i * 3\n            self._coords[idx] += vector[0]\n            self._coords[idx + 1] += vector[1]\n            self._coords[idx + 2] += vector[2]\n        # Update plane origin\n        self.plane = Plane(\n            self.plane.origin + vector, self.plane.x_axis, self.plane.y_axis\n        )\n        return self\n\n    def __add__(self, vector: Vector) -> \"Polyline\":\n        \"\"\"Translates the polyline by a vector and returns a new polyline (+).\"\"\"\n        result = Polyline.from_coords(self._coords[:])\n        result.guid = self.guid\n        result.name = self.name\n        result.width = self.width\n        result.linecolor = copy.deepcopy(self.linecolor)",
          "file": "polyline.py"
        }
      }
    },
    {
      "name": "Polyline.__add__",
      "implementations": {
        "python": {
          "sig": "__add__(vector: Vector) -> \"Polyline\"",
          "code": "def __add__(self, vector: Vector) -> \"Polyline\":\n\n        \"\"\"Translates the polyline by a vector and returns a new polyline (+).\"\"\"\n        result = Polyline.from_coords(self._coords[:])\n        result.guid = self.guid\n        result.name = self.name\n        result.width = self.width\n        result.linecolor = copy.deepcopy(self.linecolor)\n        result.xform = copy.deepcopy(self.xform)\n        result.plane = copy.deepcopy(self.plane)\n        result += vector\n        return result\n\n    def __isub__(self, vector: Vector) -> \"Polyline\":\n        \"\"\"Translates all points by the negative of a vector (-=).\"\"\"\n        for i in range(self.point_count()):\n            idx = i * 3\n            self._coords[idx] -= vector[0]\n            self._coords[idx + 1] -= vector[1]\n            self._coords[idx + 2] -= vector[2]\n        # Update plane origin",
          "file": "polyline.py"
        }
      }
    },
    {
      "name": "Polyline.__isub__",
      "implementations": {
        "python": {
          "sig": "__isub__(vector: Vector) -> \"Polyline\"",
          "code": "def __isub__(self, vector: Vector) -> \"Polyline\":\n\n        \"\"\"Translates all points by the negative of a vector (-=).\"\"\"\n        for i in range(self.point_count()):\n            idx = i * 3\n            self._coords[idx] -= vector[0]\n            self._coords[idx + 1] -= vector[1]\n            self._coords[idx + 2] -= vector[2]\n        # Update plane origin\n        self.plane = Plane(\n            self.plane.origin - vector, self.plane.x_axis, self.plane.y_axis\n        )\n        return self\n\n    def __sub__(self, vector: Vector) -> \"Polyline\":\n        \"\"\"Translates the polyline by the negative of a vector and returns a new polyline (-).\"\"\"\n        result = Polyline.from_coords(self._coords[:])\n        result.guid = self.guid\n        result.name = self.name\n        result.width = self.width\n        result.linecolor = copy.deepcopy(self.linecolor)",
          "file": "polyline.py"
        }
      }
    },
    {
      "name": "Polyline.__sub__",
      "implementations": {
        "python": {
          "sig": "__sub__(vector: Vector) -> \"Polyline\"",
          "code": "def __sub__(self, vector: Vector) -> \"Polyline\":\n\n        \"\"\"Translates the polyline by the negative of a vector and returns a new polyline (-).\"\"\"\n        result = Polyline.from_coords(self._coords[:])\n        result.guid = self.guid\n        result.name = self.name\n        result.width = self.width\n        result.linecolor = copy.deepcopy(self.linecolor)\n        result.xform = copy.deepcopy(self.xform)\n        result.plane = copy.deepcopy(self.plane)\n        result -= vector\n        return result\n\n    def __imul__(self, factor: float) -> \"Polyline\":\n        \"\"\"Multiply all coordinates by scalar in place (*=).\"\"\"\n        for i in range(len(self._coords)):\n            self._coords[i] *= factor\n        return self\n\n    def __mul__(self, factor: float) -> \"Polyline\":\n        \"\"\"Multiply polyline by scalar and return new polyline (*).\"\"\"",
          "file": "polyline.py"
        }
      }
    },
    {
      "name": "Polyline.__imul__",
      "implementations": {
        "python": {
          "sig": "__imul__(factor: float) -> \"Polyline\"",
          "code": "def __imul__(self, factor: float) -> \"Polyline\":\n\n        \"\"\"Multiply all coordinates by scalar in place (*=).\"\"\"\n        for i in range(len(self._coords)):\n            self._coords[i] *= factor\n        return self\n\n    def __mul__(self, factor: float) -> \"Polyline\":\n        \"\"\"Multiply polyline by scalar and return new polyline (*).\"\"\"\n        result = Polyline.from_coords([c * factor for c in self._coords])\n        result.name = self.name\n        result.width = self.width\n        result.linecolor = copy.deepcopy(self.linecolor)\n        result.xform = copy.deepcopy(self.xform)\n        result.plane = copy.deepcopy(self.plane)\n        return result\n\n    def __itruediv__(self, factor: float) -> \"Polyline\":\n        \"\"\"Divide all coordinates by scalar in place (/=).\"\"\"\n        for i in range(len(self._coords)):\n            self._coords[i] /= factor",
          "file": "polyline.py"
        }
      }
    },
    {
      "name": "Polyline.__mul__",
      "implementations": {
        "python": {
          "sig": "__mul__(factor: float) -> \"Polyline\"",
          "code": "def __mul__(self, factor: float) -> \"Polyline\":\n\n        \"\"\"Multiply polyline by scalar and return new polyline (*).\"\"\"\n        result = Polyline.from_coords([c * factor for c in self._coords])\n        result.name = self.name\n        result.width = self.width\n        result.linecolor = copy.deepcopy(self.linecolor)\n        result.xform = copy.deepcopy(self.xform)\n        result.plane = copy.deepcopy(self.plane)\n        return result\n\n    def __itruediv__(self, factor: float) -> \"Polyline\":\n        \"\"\"Divide all coordinates by scalar in place (/=).\"\"\"\n        for i in range(len(self._coords)):\n            self._coords[i] /= factor\n        return self\n\n    def __truediv__(self, factor: float) -> \"Polyline\":\n        \"\"\"Divide polyline by scalar and return new polyline (/).\"\"\"\n        result = Polyline.from_coords([c / factor for c in self._coords])\n        result.name = self.name",
          "file": "polyline.py"
        }
      }
    },
    {
      "name": "Polyline.__itruediv__",
      "implementations": {
        "python": {
          "sig": "__itruediv__(factor: float) -> \"Polyline\"",
          "code": "def __itruediv__(self, factor: float) -> \"Polyline\":\n\n        \"\"\"Divide all coordinates by scalar in place (/=).\"\"\"\n        for i in range(len(self._coords)):\n            self._coords[i] /= factor\n        return self\n\n    def __truediv__(self, factor: float) -> \"Polyline\":\n        \"\"\"Divide polyline by scalar and return new polyline (/).\"\"\"\n        result = Polyline.from_coords([c / factor for c in self._coords])\n        result.name = self.name\n        result.width = self.width\n        result.linecolor = copy.deepcopy(self.linecolor)\n        result.xform = copy.deepcopy(self.xform)\n        result.plane = copy.deepcopy(self.plane)\n        return result\n\n    def __neg__(self) -> \"Polyline\":\n        \"\"\"Negate polyline (reverse point order).\"\"\"\n        return self.reversed()",
          "file": "polyline.py"
        }
      }
    },
    {
      "name": "Polyline.__truediv__",
      "implementations": {
        "python": {
          "sig": "__truediv__(factor: float) -> \"Polyline\"",
          "code": "def __truediv__(self, factor: float) -> \"Polyline\":\n\n        \"\"\"Divide polyline by scalar and return new polyline (/).\"\"\"\n        result = Polyline.from_coords([c / factor for c in self._coords])\n        result.name = self.name\n        result.width = self.width\n        result.linecolor = copy.deepcopy(self.linecolor)\n        result.xform = copy.deepcopy(self.xform)\n        result.plane = copy.deepcopy(self.plane)\n        return result\n\n    def __neg__(self) -> \"Polyline\":\n        \"\"\"Negate polyline (reverse point order).\"\"\"\n        return self.reversed()\n\n    def transform(self):\n        \"\"\"Apply the stored xform transformation to the polyline.\n\n        Transforms all points in-place and resets xform to identity.\n        \"\"\"\n        for i in range(self.point_count()):",
          "file": "polyline.py"
        }
      }
    },
    {
      "name": "Polyline.__neg__",
      "implementations": {
        "python": {
          "sig": "__neg__() -> \"Polyline\"",
          "code": "def __neg__(self) -> \"Polyline\":\n\n        \"\"\"Negate polyline (reverse point order).\"\"\"\n        return self.reversed()\n\n    def transform(self):\n        \"\"\"Apply the stored xform transformation to the polyline.\n\n        Transforms all points in-place and resets xform to identity.\n        \"\"\"\n        for i in range(self.point_count()):\n            idx = i * 3\n            pt = Point(self._coords[idx], self._coords[idx + 1], self._coords[idx + 2])\n            self.xform.transform_point(pt)\n            self._coords[idx] = pt[0]\n            self._coords[idx + 1] = pt[1]\n            self._coords[idx + 2] = pt[2]\n        self.xform = Xform.identity()\n\n    def transformed(self):\n        \"\"\"Return a transformed copy of the polyline.\"\"\"",
          "file": "polyline.py"
        }
      }
    },
    {
      "name": "Polyline.transform",
      "implementations": {
        "python": {
          "sig": "transform()",
          "code": "def transform(self):\n\n        \"\"\"Apply the stored xform transformation to the polyline.\n\n        Transforms all points in-place and resets xform to identity.\n        \"\"\"\n        for i in range(self.point_count()):\n            idx = i * 3\n            pt = Point(self._coords[idx], self._coords[idx + 1], self._coords[idx + 2])\n            self.xform.transform_point(pt)\n            self._coords[idx] = pt[0]\n            self._coords[idx + 1] = pt[1]\n            self._coords[idx + 2] = pt[2]\n        self.xform = Xform.identity()\n\n    def transformed(self):\n        \"\"\"Return a transformed copy of the polyline.\"\"\"\n        result = copy.deepcopy(self)\n        result.transform()\n        return result",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "void transform()",
          "code": "void Polyline::transform() {\n    for (size_t i = 0; i < point_count(); i++) {\n        size_t idx = i * 3;\n        Point pt(_coords[idx], _coords[idx + 1], _coords[idx + 2]);\n        xform.transform_point(pt);\n        _coords[idx] = pt[0];\n        _coords[idx + 1] = pt[1];\n        _coords[idx + 2] = pt[2];\n    }",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "transform()",
          "code": "pub fn transform(&mut self) {\n        // Transform coordinates in-place without creating Point objects\n        for i in 0..self.point_count() {\n            let idx = i * 3;\n            let mut pt = Point::new(self.coords[idx], self.coords[idx + 1], self.coords[idx + 2]);\n            self.xform.transform_point(&mut pt);\n            self.coords[idx] = pt[0];\n            self.coords[idx + 1] = pt[1];\n            self.coords[idx + 2] = pt[2];\n        }\n        self.xform = Xform::identity();\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.transformed",
      "implementations": {
        "python": {
          "sig": "transformed()",
          "code": "def transformed(self):\n\n        \"\"\"Return a transformed copy of the polyline.\"\"\"\n        result = copy.deepcopy(self)\n        result.transform()\n        return result\n\n    # ===========================================================================================\n    # Geometric Utilities\n    # ===========================================================================================\n\n    def shift(self, times: int) -> None:\n        \"\"\"Shift polyline points by specified number of positions.\"\"\"\n        if not self.points:\n            return\n        n = len(self.points)\n        shift_amount = times % n\n        self.points = self.points[shift_amount:] + self.points[:shift_amount]\n\n    def magnitude_squared(self) -> float:\n        \"\"\"Calculate squared magnitude of polyline (faster, no sqrt).\"\"\"",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "Polyline transformed()",
          "code": "Polyline Polyline::transformed() const {\n    Polyline result = *this;\n    result.transform();\n    return result;\n}",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "transformed() -> Self",
          "code": "pub fn transformed(&self) -> Self {\n        let mut result = self.clone();\n        result.transform();\n        result\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.shift",
      "implementations": {
        "python": {
          "sig": "shift(times: int) -> None",
          "code": "def shift(self, times: int) -> None:\n\n        \"\"\"Shift polyline points by specified number of positions.\"\"\"\n        if not self.points:\n            return\n        n = len(self.points)\n        shift_amount = times % n\n        self.points = self.points[shift_amount:] + self.points[:shift_amount]\n\n    def magnitude_squared(self) -> float:\n        \"\"\"Calculate squared magnitude of polyline (faster, no sqrt).\"\"\"\n        mag = 0.0\n        for i in range(self.segment_count()):\n            segment = self.points[i + 1] - self.points[i]\n            mag += segment.magnitude_squared()\n        return mag\n\n    @staticmethod\n    def point_at(start: Point, end: Point, t: float) -> Point:\n        \"\"\"Get point at parameter t along a line segment (t=0 is start, t=1 is end).\"\"\"\n        s = 1.0 - t",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "void shift(int times)",
          "code": "void Polyline::shift(int times) {\n    if (_coords.empty()) return;\n\n    // Remove last point if closed\n    bool was_closed = is_closed();\n    if (was_closed && point_count() > 0) {\n        _coords.resize(_coords.size() - 3);\n    }",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "shift(times: i32)",
          "code": "pub fn shift(&mut self, times: i32) {\n        if self.coords.is_empty() {\n            return;\n        }\n        let n = self.point_count();\n        let shift_amount = ((times % n as i32) + n as i32) % n as i32;\n        // Rotate coords in groups of 3\n        let mut new_coords = Vec::with_capacity(self.coords.len());\n        for i in 0..n {\n            let src_idx = ((i + shift_amount as usize) % n) * 3;\n            new_coords.push(self.coords[src_idx]);\n            new_coords.push(self.coords[src_idx + 1]);\n            new_coords.push(self.coords[src_idx + 2]);\n        }\n        self.coords = new_coords;\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.magnitude_squared",
      "implementations": {
        "python": {
          "sig": "magnitude_squared() -> float",
          "code": "def magnitude_squared(self) -> float:\n\n        \"\"\"Calculate squared magnitude of polyline (faster, no sqrt).\"\"\"\n        mag = 0.0\n        for i in range(self.segment_count()):\n            segment = self.points[i + 1] - self.points[i]\n            mag += segment.magnitude_squared()\n        return mag\n\n    @staticmethod\n    def point_at(start: Point, end: Point, t: float) -> Point:\n        \"\"\"Get point at parameter t along a line segment (t=0 is start, t=1 is end).\"\"\"\n        s = 1.0 - t\n        return Point(\n            start.x if start.x == end.x else s * start.x + t * end.x,\n            start.y if start.y == end.y else s * start.y + t * end.y,\n            start.z if start.z == end.z else s * start.z + t * end.z,\n        )\n\n    @staticmethod\n    def closest_point_to_line(",
          "file": "polyline.py"
        },
        "rust": {
          "sig": "magnitude_squared() -> f64",
          "code": "pub fn magnitude_squared(&self) -> f64 {\n        let mut length = 0.0f64;\n        for i in 0..self.segment_count() {\n            let idx0 = i * 3;\n            let idx1 = (i + 1) * 3;\n            let dx = self.coords[idx1] - self.coords[idx0];\n            let dy = self.coords[idx1 + 1] - self.coords[idx0 + 1];\n            let dz = self.coords[idx1 + 2] - self.coords[idx0 + 2];\n            length += dx * dx + dy * dy + dz * dz;\n        }\n        length\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.point_at",
      "implementations": {
        "python": {
          "sig": "point_at(start: Point, end: Point, t: float) -> Point",
          "code": "def point_at(start: Point, end: Point, t: float) -> Point:\n\n        \"\"\"Get point at parameter t along a line segment (t=0 is start, t=1 is end).\"\"\"\n        s = 1.0 - t\n        return Point(\n            start.x if start.x == end.x else s * start.x + t * end.x,\n            start.y if start.y == end.y else s * start.y + t * end.y,\n            start.z if start.z == end.z else s * start.z + t * end.z,\n        )\n\n    @staticmethod\n    def closest_point_to_line(\n        point: Point, line_start: Point, line_end: Point\n    ) -> float:\n        \"\"\"Find closest point on line segment to given point, returns parameter t.\"\"\"\n        d = line_end - line_start\n        dod = d.magnitude_squared()\n\n        if dod > 0.0:\n            if (point - line_start).magnitude_squared() <= (\n                point - line_end",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "Point point_at(const Point& start, const Point& end, double t)",
          "code": "Point Polyline::point_at(const Point& start, const Point& end, double t) {\n    const double s = 1.0 - t;\n    return Point(\n        (start[0] == end[0]) ? start[0] : static_cast<double>(s * start[0] + t * end[0]),\n        (start[1] == end[1]) ? start[1] : static_cast<double>(s * start[1] + t * end[1]),\n        (start[2] == end[2]) ? start[2] : static_cast<double>(s * start[2] + t * end[2])\n    );\n}",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "point_at(start: &Point, end: &Point, t: f64) -> Point",
          "code": "pub fn point_at(start: &Point, end: &Point, t: f64) -> Point {\n        let s = 1.0 - t;\n        let t_f32 = t;\n        let s_f32 = s;\n        Point::new(\n            if start[0] == end[0] {\n                start[0]\n            } else {\n                s_f32 * start[0] + t_f32 * end[0]\n            },\n            if start[1] == end[1] {\n                start[1]\n            } else {\n                s_f32 * start[1] + t_f32 * end[1]\n            },\n            if start[2] == end[2] {\n                start[2]\n            } else {\n                s_f32 * start[2] + t_f32 * end[2]\n            },\n        )\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.closest_point_to_line",
      "implementations": {
        "python": {
          "sig": "closest_point_to_line(\n        point: Point, line_start: Point, line_end: Point\n    ) -> float",
          "code": "def closest_point_to_line(\n        point: Point, line_start: Point, line_end: Point\n    ) -> float:\n\n        \"\"\"Find closest point on line segment to given point, returns parameter t.\"\"\"\n        d = line_end - line_start\n        dod = d.magnitude_squared()\n\n        if dod > 0.0:\n            if (point - line_start).magnitude_squared() <= (\n                point - line_end\n            ).magnitude_squared():\n                t = (point - line_start).dot(d) / dod\n            else:\n                t = 1.0 + (point - line_end).dot(d) / dod\n            return t\n        else:\n            return 0.0\n\n    @staticmethod\n    def line_line_overlap(\n        line0_start: Point,\n        line0_end: Point,",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "void closest_point_to_line(const Point& point, const Point& line_start, \n                                    const Point& line_end, double& t)",
          "code": "void Polyline::closest_point_to_line(const Point& point, const Point& line_start, \n                                    const Point& line_end, double& t) {\n    Vector D = line_end - line_start;\n    double DoD = D.magnitude_squared();\n\n    if (DoD > 0.0) {\n        Vector to_point_start = point - line_start;\n        Vector to_point_end = point - line_end;\n        \n        if (to_point_start.magnitude_squared() <= to_point_end.magnitude_squared()) {\n            t = to_point_start.dot(D) / DoD;\n        }",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "closest_point_to_line(point: &Point, line_start: &Point, line_end: &Point) -> f64",
          "code": "pub fn closest_point_to_line(point: &Point, line_start: &Point, line_end: &Point) -> f64 {\n        // Direction vector (no clone needed - use direct coordinate access)\n        let dx = line_end[0] - line_start[0];\n        let dy = line_end[1] - line_start[1];\n        let dz = line_end[2] - line_start[2];\n        let dod = dx * dx + dy * dy + dz * dz;\n\n        if dod > 0.0 {\n            // Vector from line_start to point\n            let px = point[0] - line_start[0];\n            let py = point[1] - line_start[1];\n            let pz = point[2] - line_start[2];\n            \n            // Vector from line_end to point\n            let qx = point[0] - line_end[0];\n            let qy = point[1] - line_end[1];\n            let qz = point[2] - line_end[2];\n            \n            let dist_star",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.line_line_overlap",
      "implementations": {
        "python": {
          "sig": "line_line_overlap(\n        line0_start: Point,\n        line0_end: Point,\n        line1_start: Point,\n        line1_end: Point,\n    ) -> Optional[Tuple[Point, Point]]",
          "code": "def line_line_overlap(\n        line0_start: Point,\n        line0_end: Point,\n        line1_start: Point,\n        line1_end: Point,\n    ) -> Optional[Tuple[Point, Point]]:\n\n        \"\"\"Check if two line segments overlap and return the overlapping segment.\"\"\"\n        t = [0.0, 1.0, 0.0, 0.0]\n        t[2] = Polyline.closest_point_to_line(line1_start, line0_start, line0_end)\n        t[3] = Polyline.closest_point_to_line(line1_end, line0_start, line0_end)\n\n        do_overlap = not ((t[2] < 0.0 and t[3] < 0.0) or (t[2] > 1.0 and t[3] > 1.0))\n        t.sort()\n\n        overlap_valid = abs(t[2] - t[1]) > Tolerance.ZERO_TOLERANCE\n\n        if do_overlap and overlap_valid:\n            return (\n                Polyline.point_at(line0_start, line0_end, t[1]),\n                Polyline.point_at(line0_start, line0_end, t[2]),\n            )\n        else:\n            return None\n\n    @staticmethod",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "bool line_line_overlap(const Point& line0_start, const Point& line0_end,\n                                const Point& line1_start, const Point& line1_end,\n                                Point& overlap_start, Point& overlap_end)",
          "code": "bool Polyline::line_line_overlap(const Point& line0_start, const Point& line0_end,\n                                const Point& line1_start, const Point& line1_end,\n                                Point& overlap_start, Point& overlap_end) {\n    double t[4];\n    t[0] = 0.0;\n    t[1] = 1.0;\n\n    closest_point_to_line(line1_start, line0_start, line0_end, t[2]);\n    closest_point_to_line(line1_end, line0_start, line0_end, t[3]);\n\n    // Check if there is an overlap\n    bool do_overlap = !((t[2] < 0 && t[3] < 0) || (t[2] > 1 && t[3] > 1));\n\n    // Sort parameters\n    std::sort(t, t + 4);\n\n    // Check if the overlap is not just a point\n    do_overlap = do_overlap && (std::abs(t[2] - t[1]) > Tolerance::ZERO_TOLERANCE);\n\n    // Get overlap points\n    overlap_start = point_at(line0_start, line0_e",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "line_line_overlap(\n        line0_start: &Point,\n        line0_end: &Point,\n        line1_start: &Point,\n        line1_end: &Point,\n    ) -> Option<(Point, Point)>",
          "code": "pub fn line_line_overlap(\n        line0_start: &Point,\n        line0_end: &Point,\n        line1_start: &Point,\n        line1_end: &Point,\n    ) -> Option<(Point, Point)> {\n        let mut t = [0.0, 1.0, 0.0, 0.0];\n        t[2] = Self::closest_point_to_line(line1_start, line0_start, line0_end);\n        t[3] = Self::closest_point_to_line(line1_end, line0_start, line0_end);\n\n        let do_overlap = !((t[2] < 0.0 && t[3] < 0.0) || (t[2] > 1.0 && t[3] > 1.0));\n        t.sort_by(|a, b| a.partial_cmp(b).unwrap());\n\n        let overlap_valid = (t[2] - t[1]).abs() > Tolerance::ZERO_TOLERANCE;\n\n        if do_overlap && overlap_valid {\n            Some((\n                Self::point_at(line0_start, line0_end, t[1]),\n                Self::point_at(line0_start, line0_end, t[2]),\n            ))",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.line_line_average",
      "implementations": {
        "python": {
          "sig": "line_line_average(\n        line0_start: Point,\n        line0_end: Point,\n        line1_start: Point,\n        line1_end: Point,\n    ) -> Tuple[Point, Point]",
          "code": "def line_line_average(\n        line0_start: Point,\n        line0_end: Point,\n        line1_start: Point,\n        line1_end: Point,\n    ) -> Tuple[Point, Point]:\n\n        \"\"\"Calculate average of two line segments.\"\"\"\n        output_start = Point(\n            (line0_start.x + line1_start.x) * 0.5,\n            (line0_start.y + line1_start.y) * 0.5,\n            (line0_start.z + line1_start.z) * 0.5,\n        )\n        output_end = Point(\n            (line0_end.x + line1_end.x) * 0.5,\n            (line0_end.y + line1_end.y) * 0.5,\n            (line0_end.z + line1_end.z) * 0.5,\n        )\n        return output_start, output_end\n\n    @staticmethod\n    def line_line_overlap_average(\n        line0_start: Point,\n        line0_end: Point,\n        line1_start: Point,\n        line1_end: Point,",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "void line_line_average(const Point& line0_start, const Point& line0_end,\n                                const Point& line1_start, const Point& line1_end,\n                                Point& output_start, Point& output_end)",
          "code": "void Polyline::line_line_average(const Point& line0_start, const Point& line0_end,\n                                const Point& line1_start, const Point& line1_end,\n                                Point& output_start, Point& output_end) {\n    output_start = Point(\n        (line0_start[0] + line1_start[0]) * 0.5,\n        (line0_start[1] + line1_start[1]) * 0.5,\n        (line0_start[2] + line1_start[2]) * 0.5\n    );\n    \n    output_end = Point(\n        (line0_end[0] + line1_end[0]) * 0.5,\n        (line0_end[1] + line1_end[1]) * 0.5,\n        (line0_end[2] + line1_end[2]) * 0.5\n    );\n}",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "line_line_average(\n        line0_start: &Point,\n        line0_end: &Point,\n        line1_start: &Point,\n        line1_end: &Point,\n    ) -> (Point, Point)",
          "code": "pub fn line_line_average(\n        line0_start: &Point,\n        line0_end: &Point,\n        line1_start: &Point,\n        line1_end: &Point,\n    ) -> (Point, Point) {\n        let output_start = Point::new(\n            (line0_start[0] + line1_start[0]) * 0.5,\n            (line0_start[1] + line1_start[1]) * 0.5,\n            (line0_start[2] + line1_start[2]) * 0.5,\n        );\n        let output_end = Point::new(\n            (line0_end[0] + line1_end[0]) * 0.5,\n            (line0_end[1] + line1_end[1]) * 0.5,\n            (line0_end[2] + line1_end[2]) * 0.5,\n        );\n        (output_start, output_end)\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.line_line_overlap_average",
      "implementations": {
        "python": {
          "sig": "line_line_overlap_average(\n        line0_start: Point,\n        line0_end: Point,\n        line1_start: Point,\n        line1_end: Point,\n    ) -> Tuple[Point, Point]",
          "code": "def line_line_overlap_average(\n        line0_start: Point,\n        line0_end: Point,\n        line1_start: Point,\n        line1_end: Point,\n    ) -> Tuple[Point, Point]:\n\n        \"\"\"Calculate overlap average of two line segments.\"\"\"\n        line_a = Polyline.line_line_overlap(\n            line0_start, line0_end, line1_start, line1_end\n        )\n        line_b = Polyline.line_line_overlap(\n            line1_start, line1_end, line0_start, line0_end\n        )\n\n        if line_a and line_b:\n            line_a_start, line_a_end = line_a\n            line_b_start, line_b_end = line_b\n\n            mid_line0_start = Point(\n                (line_a_start.x + line_b_start.x) * 0.5,\n                (line_a_start.y + line_b_start.y) * 0.5,\n                (line_a_start.z + line_b_start.z) * 0.5,\n            )\n            mid_line0_end = Point(\n                (line_a_end.x + line_b_end.x) * 0.5,",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "void line_line_overlap_average(const Point& line0_start, const Point& line0_end,\n                                        const Point& line1_start, const Point& line1_end,\n                                        Point& output_start, Point& output_end)",
          "code": "void Polyline::line_line_overlap_average(const Point& line0_start, const Point& line0_end,\n                                        const Point& line1_start, const Point& line1_end,\n                                        Point& output_start, Point& output_end) {\n    // Get two overlaps\n    Point lineA_start, lineA_end;\n    line_line_overlap(line0_start, line0_end, line1_start, line1_end, lineA_start, lineA_end);\n    \n    Point lineB_start, lineB_end;\n    line_line_overlap(line1_start, line1_end, line0_start, line0_end, lineB_start, lineB_end);\n\n    // Construct middle lines, in case the first one is flipped\n    Point mid_line0_start(\n        (lineA_start[0] + lineB_start[0]) * 0.5,\n        (lineA_start[1] + lineB_start[1]) * 0.5,\n        (lineA_start[2] + lineB_start[2]) * 0.5\n    );",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "line_line_overlap_average(\n        line0_start: &Point,\n        line0_end: &Point,\n        line1_start: &Point,\n        line1_end: &Point,\n    ) -> (Point, Point)",
          "code": "pub fn line_line_overlap_average(\n        line0_start: &Point,\n        line0_end: &Point,\n        line1_start: &Point,\n        line1_end: &Point,\n    ) -> (Point, Point) {\n        let line_a = Self::line_line_overlap(line0_start, line0_end, line1_start, line1_end);\n        let line_b = Self::line_line_overlap(line1_start, line1_end, line0_start, line0_end);\n\n        if let (Some((line_a_start, line_a_end)), Some((line_b_start, line_b_end))) =\n            (line_a, line_b)\n        {\n            let mid_line0_start = Point::new(\n                (line_a_start[0] + line_b_start[0]) * 0.5,\n                (line_a_start[1] + line_b_start[1]) * 0.5,\n                (line_a_start[2] + line_b_start[2]) * 0.5,\n            );\n            let mid_line0_end = Point::new(\n                (line_a_end[",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.line_from_projected_points",
      "implementations": {
        "python": {
          "sig": "line_from_projected_points(\n        line_start: Point,\n        line_end: Point,\n        points: List[Point],\n    ) -> Optional[Tuple[Point, Point]]",
          "code": "def line_from_projected_points(\n        line_start: Point,\n        line_end: Point,\n        points: List[Point],\n    ) -> Optional[Tuple[Point, Point]]:\n\n        \"\"\"Create line from projected points onto a base line.\"\"\"\n        if not points:\n            return None\n\n        t_values = [\n            Polyline.closest_point_to_line(p, line_start, line_end) for p in points\n        ]\n        t_values.sort()\n\n        output_start = Polyline.point_at(line_start, line_end, t_values[0])\n        output_end = Polyline.point_at(line_start, line_end, t_values[-1])\n\n        if abs(t_values[0] - t_values[-1]) > Tolerance.ZERO_TOLERANCE:\n            return output_start, output_end\n        else:\n            return None\n\n    def closest_distance_and_point(self, point: Point) -> Tuple[float, int, Point]:\n        \"\"\"Find closest distance and point from a point to this polyline.\"\"\"",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "bool line_from_projected_points(const Point& line_start, const Point& line_end,\n                                         const std::vector<Point>& points,\n                                         Point& output_start, Point& output_end)",
          "code": "bool Polyline::line_from_projected_points(const Point& line_start, const Point& line_end,\n                                         const std::vector<Point>& points,\n                                         Point& output_start, Point& output_end) {\n    if (points.empty()) return false;\n    \n    std::vector<double> t_values;\n    t_values.reserve(points.size());\n\n    // Project all points to the line\n    for (const auto& point : points) {\n        double t;\n        closest_point_to_line(point, line_start, line_end, t);\n        t_values.push_back(t);\n    }",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "line_from_projected_points(\n        line_start: &Point,\n        line_end: &Point,\n        points: &[Point],\n    ) -> Option<(Point, Point)>",
          "code": "pub fn line_from_projected_points(\n        line_start: &Point,\n        line_end: &Point,\n        points: &[Point],\n    ) -> Option<(Point, Point)> {\n        if points.is_empty() {\n            return None;\n        }\n\n        let mut t_values: Vec<f64> = points\n            .iter()\n            .map(|p| Self::closest_point_to_line(p, line_start, line_end))\n            .collect();\n\n        t_values.sort_by(|a, b| a.partial_cmp(b).unwrap());\n\n        let output_start = Self::point_at(line_start, line_end, t_values[0]);\n        let output_end =\n            Self::point_at(line_start, line_end, t_values[t_values.len() - 1]);\n\n        if (t_values[0] - t_values[t_values.len() - 1]).abs() > Tolerance::ZERO_TOLERANCE {\n            Some((output_start, output_end))\n        } else {\n            None",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.closest_distance_and_point",
      "implementations": {
        "python": {
          "sig": "closest_distance_and_point(point: Point) -> Tuple[float, int, Point]",
          "code": "def closest_distance_and_point(self, point: Point) -> Tuple[float, int, Point]:\n\n        \"\"\"Find closest distance and point from a point to this polyline.\"\"\"\n        edge_id = 0\n        closest_distance = float(\"inf\")\n        best_t = 0.0\n\n        for i in range(self.segment_count()):\n            t = self.closest_point_to_line(point, self.points[i], self.points[i + 1])\n            point_on_segment = Polyline.point_at(\n                self.points[i], self.points[i + 1], t\n            )\n            distance = point.distance(point_on_segment)\n\n            if distance < closest_distance:\n                closest_distance = distance\n                edge_id = i\n                best_t = t\n\n            if closest_distance < Tolerance.ZERO_TOLERANCE:\n                break",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "double closest_distance_and_point(const Point& point, size_t& edge_id, Point& closest_point)",
          "code": "double Polyline::closest_distance_and_point(const Point& point, size_t& edge_id, Point& closest_point) const {\n    edge_id = 0;\n    double closest_distance = std::numeric_limits<double>::max();\n    double best_t = 0.0;\n\n    for (size_t i = 0; i < segment_count(); i++) {\n        double t;\n        Point pi = get_point(i);\n        Point pi1 = get_point(i + 1);\n        closest_point_to_line(point, pi, pi1, t);\n\n        Point point_on_segment = point_at(pi, pi1, t);\n        double distance = point.distance(point_on_segment);\n\n        if (distance < closest_distance) {\n            closest_distance = distance;\n            edge_id = i;\n            best_t = t;\n        }",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "closest_distance_and_point(point: &Point) -> (f64, usize, Point)",
          "code": "pub fn closest_distance_and_point(&self, point: &Point) -> (f64, usize, Point) {\n        let mut edge_id = 0;\n        let mut closest_distance = f64::MAX;\n        let mut best_t = 0.0;\n        let points = self.get_points();\n\n        for i in 0..self.segment_count() {\n            let t = Self::closest_point_to_line(point, &points[i], &points[i + 1]);\n            let point_on_segment = Self::point_at(&points[i], &points[i + 1], t);\n            let distance = point.distance(&point_on_segment, None);\n\n            if distance < closest_distance {\n                closest_distance = distance;\n                edge_id = i;\n                best_t = t;\n            }\n\n            if closest_distance < Tolerance::ZERO_TOLERANCE {\n                break;\n            }\n        }\n\n        let closest_",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.is_closed",
      "implementations": {
        "python": {
          "sig": "is_closed() -> bool",
          "code": "def is_closed(self) -> bool:\n\n        \"\"\"Check if polyline is closed (first and last points are the same).\"\"\"\n        if len(self.points) < 2:\n            return False\n        return self.points[0].distance(self.points[-1]) < Tolerance.ZERO_TOLERANCE\n\n    def center(self) -> Point:\n        \"\"\"Calculate center point of polyline.\"\"\"\n        if not self.points:\n            return Point(0.0, 0.0, 0.0)\n\n        n = (\n            len(self.points) - 1\n            if self.is_closed() and len(self.points) > 1\n            else len(self.points)\n        )\n\n        sum_x = sum(self.points[i].x for i in range(n))\n        sum_y = sum(self.points[i].y for i in range(n))\n        sum_z = sum(self.points[i].z for i in range(n))",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "bool is_closed()",
          "code": "bool Polyline::is_closed() const {\n    if (point_count() < 2) return false;\n    return get_point(0).distance(get_point(point_count() - 1)) < static_cast<double>(Tolerance::ZERO_TOLERANCE);\n}",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "is_closed() -> bool",
          "code": "pub fn is_closed(&self) -> bool {\n        let n = self.point_count();\n        if n < 2 {\n            return false;\n        }\n        let first = self.get_point(0).unwrap();\n        let last = self.get_point(n - 1).unwrap();\n        first.distance(&last, None) < Tolerance::ZERO_TOLERANCE\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.center",
      "implementations": {
        "python": {
          "sig": "center() -> Point",
          "code": "def center(self) -> Point:\n\n        \"\"\"Calculate center point of polyline.\"\"\"\n        if not self.points:\n            return Point(0.0, 0.0, 0.0)\n\n        n = (\n            len(self.points) - 1\n            if self.is_closed() and len(self.points) > 1\n            else len(self.points)\n        )\n\n        sum_x = sum(self.points[i].x for i in range(n))\n        sum_y = sum(self.points[i].y for i in range(n))\n        sum_z = sum(self.points[i].z for i in range(n))\n\n        return Point(sum_x / n, sum_y / n, sum_z / n)\n\n    def get_average_plane(self) -> Tuple[Point, Vector, Vector, Vector]:\n        \"\"\"Get average plane from polyline points.\"\"\"\n        origin = self.center()",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "Point center()",
          "code": "Point Polyline::center() const {\n    if (_coords.empty()) return Point(0, 0, 0);\n\n    double x = 0, y = 0, z = 0;\n    size_t n = is_closed() ? point_count() - 1 : point_count();\n\n    for (size_t i = 0; i < n; i++) {\n        size_t idx = i * 3;\n        x += _coords[idx];\n        y += _coords[idx + 1];\n        z += _coords[idx + 2];\n    }",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "center() -> Point",
          "code": "pub fn center(&self) -> Point {\n        if self.coords.is_empty() {\n            return Point::new(0.0, 0.0, 0.0);\n        }\n\n        let total = self.point_count();\n        let n = if self.is_closed() && total > 1 { total - 1 } else { total };\n\n        let mut sum_x = 0.0;\n        let mut sum_y = 0.0;\n        let mut sum_z = 0.0;\n\n        for i in 0..n {\n            let idx = i * 3;\n            sum_x += self.coords[idx];\n            sum_y += self.coords[idx + 1];\n            sum_z += self.coords[idx + 2];\n        }\n\n        Point::new(sum_x / n as f64, sum_y / n as f64, sum_z / n as f64)\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.get_average_plane",
      "implementations": {
        "python": {
          "sig": "get_average_plane() -> Tuple[Point, Vector, Vector, Vector]",
          "code": "def get_average_plane(self) -> Tuple[Point, Vector, Vector, Vector]:\n\n        \"\"\"Get average plane from polyline points.\"\"\"\n        origin = self.center()\n\n        if len(self.points) >= 2:\n            x_axis = (self.points[1] - self.points[0]).normalize()\n        else:\n            x_axis = Vector(1.0, 0.0, 0.0)\n\n        z_axis = self._average_normal()\n        y_axis = z_axis.cross(x_axis).normalize()\n\n        return origin, x_axis, y_axis, z_axis\n\n    def get_fast_plane(self) -> Tuple[Point, Plane]:\n        \"\"\"Get fast plane calculation from polyline.\"\"\"\n        origin = self.points[0] if self.points else Point(0.0, 0.0, 0.0)\n        average_normal = self._average_normal()\n        plane = Plane.from_point_normal(origin, average_normal)\n        return origin, plane",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "void get_average_plane(Point& origin, Vector& x_axis, Vector& y_axis, Vector& z_axis)",
          "code": "void Polyline::get_average_plane(Point& origin, Vector& x_axis, Vector& y_axis, Vector& z_axis) const {\n    // Origin\n    origin = center();\n\n    // X-axis (first segment direction)\n    if (point_count() >= 2) {\n        x_axis = get_point(1) - get_point(0);\n        x_axis.normalize_self();\n    }",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "get_average_plane() -> (Point, Vector, Vector, Vector)",
          "code": "pub fn get_average_plane(&self) -> (Point, Vector, Vector, Vector) {\n        let origin = self.center();\n        let points = self.get_points();\n\n        let x_axis = if points.len() >= 2 {\n            let mut x = points[1].clone() - points[0].clone();\n            x.normalize();\n            x\n        } else {\n            Vector::new(1.0, 0.0, 0.0)\n        };\n\n        let z_axis = self.average_normal();\n        let mut y_axis = z_axis.cross(&x_axis);\n        y_axis.normalize();\n\n        (origin, x_axis, y_axis, z_axis)\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.get_fast_plane",
      "implementations": {
        "python": {
          "sig": "get_fast_plane() -> Tuple[Point, Plane]",
          "code": "def get_fast_plane(self) -> Tuple[Point, Plane]:\n\n        \"\"\"Get fast plane calculation from polyline.\"\"\"\n        origin = self.points[0] if self.points else Point(0.0, 0.0, 0.0)\n        average_normal = self._average_normal()\n        plane = Plane.from_point_normal(origin, average_normal)\n        return origin, plane\n\n    def extend_segment(\n        self,\n        segment_id: int,\n        dist0: float,\n        dist1: float,\n        proportion0: float = 0.0,\n        proportion1: float = 0.0,\n    ) -> None:\n        \"\"\"Extend polyline segment.\"\"\"\n        if segment_id < 0 or segment_id >= self.segment_count():\n            return\n\n        p0 = self.get_point(segment_id)",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "void get_fast_plane(Point& origin, Plane& pln)",
          "code": "void Polyline::get_fast_plane(Point& origin, Plane& pln) const {\n    if (_coords.empty()) {\n        origin = Point(0, 0, 0);\n        pln = Plane();\n        return;\n    }",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "get_fast_plane() -> (Point, Plane)",
          "code": "pub fn get_fast_plane(&self) -> (Point, Plane) {\n        let origin = if !self.coords.is_empty() {\n            self.get_point(0).unwrap()\n        } else {\n            Point::new(0.0, 0.0, 0.0)\n        };\n\n        let average_normal = self.average_normal();\n        let plane = Plane::from_point_normal(origin.clone(), average_normal);\n        (origin, plane)\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.extend_segment",
      "implementations": {
        "python": {
          "sig": "extend_segment(\n        ,\n        segment_id: int,\n        dist0: float,\n        dist1: float,\n        proportion0: float = 0.0,\n        proportion1: float = 0.0,\n    ) -> None",
          "code": "def extend_segment(\n        self,\n        segment_id: int,\n        dist0: float,\n        dist1: float,\n        proportion0: float = 0.0,\n        proportion1: float = 0.0,\n    ) -> None:\n\n        \"\"\"Extend polyline segment.\"\"\"\n        if segment_id < 0 or segment_id >= self.segment_count():\n            return\n\n        p0 = self.get_point(segment_id)\n        p1 = self.get_point(segment_id + 1)\n        v = p1 - p0\n\n        if proportion0 != 0.0 or proportion1 != 0.0:\n            p0 -= v * proportion0\n            p1 += v * proportion1\n        else:\n            v_norm = v.normalize()\n            p0 -= v_norm * dist0\n            p1 += v_norm * dist1\n\n        self.set_point(segment_id, p0)\n        self.set_point(segment_id + 1, p1)",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "void extend_segment(int segment_id, double dist0, double dist1,\n                             double proportion0, double proportion1)",
          "code": "void Polyline::extend_segment(int segment_id, double dist0, double dist1,\n                             double proportion0, double proportion1) {\n    if (segment_id < 0 || segment_id >= static_cast<int>(segment_count())) return;\n    if (dist0 == 0 && dist1 == 0 && proportion0 == 0 && proportion1 == 0) return;\n\n    Point p0 = get_point(segment_id);\n    Point p1 = get_point(segment_id + 1);\n    Vector v = p1 - p0;\n\n    if (proportion0 != 0 || proportion1 != 0) {\n        p0 = p0 - v * static_cast<double>(proportion0);\n        p1 = p1 + v * static_cast<double>(proportion1);\n    }",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "extend_segment(\n        ,\n        segment_id: usize,\n        dist0: f64,\n        dist1: f64,\n        proportion0: f64,\n        proportion1: f64,\n    )",
          "code": "pub fn extend_segment(\n        &mut self,\n        segment_id: usize,\n        dist0: f64,\n        dist1: f64,\n        proportion0: f64,\n        proportion1: f64,\n    ) {\n        if segment_id >= self.segment_count() {\n            return;\n        }\n\n        let mut p0 = self.get_point(segment_id).unwrap();\n        let mut p1 = self.get_point(segment_id + 1).unwrap();\n        let v = p1.clone() - p0.clone();\n\n        if proportion0 != 0.0 || proportion1 != 0.0 {\n            p0 -= v.clone() * proportion0;\n            p1 += v * proportion1;\n        } else {\n            let v_norm = v.normalized();\n            p0 -= v_norm.clone() * dist0;\n            p1 += v_norm * dist1;\n        }\n\n        self.set_point(segment_id, &p0);\n        self.set_point(segment_id + 1, &p1);\n\n        if self.is_clo",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.extend_segment_equally_static",
      "implementations": {
        "python": {
          "sig": "extend_segment_equally_static(\n        segment_start: Point, segment_end: Point, dist: float, proportion: float = 0.0\n    ) -> None",
          "code": "def extend_segment_equally_static(\n        segment_start: Point, segment_end: Point, dist: float, proportion: float = 0.0\n    ) -> None:\n\n        \"\"\"Extend segment equally on both ends (static utility).\"\"\"\n        if dist == 0.0 and proportion == 0.0:\n            return\n\n        v = segment_end - segment_start\n\n        if proportion != 0.0:\n            segment_start -= v * proportion\n            segment_end += v * proportion\n        else:\n            v_norm = v.normalize()\n            segment_start -= v_norm * dist\n            segment_end += v_norm * dist\n\n    def extend_segment_equally(\n        self, segment_id: int, dist: float, proportion: float = 0.0\n    ) -> None:\n        \"\"\"Extend polyline segment equally.\"\"\"\n        if segment_id < 0 or segment_id >= self.segment_count():",
          "file": "polyline.py"
        },
        "rust": {
          "sig": "extend_segment_equally_static(\n        segment_start: &mut Point,\n        segment_end: &mut Point,\n        dist: f64,\n        proportion: f64,\n    )",
          "code": "pub fn extend_segment_equally_static(\n        segment_start: &mut Point,\n        segment_end: &mut Point,\n        dist: f64,\n        proportion: f64,\n    ) {\n        if dist == 0.0 && proportion == 0.0 {\n            return;\n        }\n\n        let v = segment_end.clone() - segment_start.clone();\n\n        if proportion != 0.0 {\n            *segment_start = segment_start.clone() - (v.clone() * proportion);\n            *segment_end = segment_end.clone() + (v * proportion);\n        } else {\n            let mut v_norm = v;\n            v_norm.normalize();\n            *segment_start = segment_start.clone() - (v_norm.clone() * dist);\n            *segment_end = segment_end.clone() + (v_norm * dist);\n        }\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.extend_segment_equally",
      "implementations": {
        "python": {
          "sig": "extend_segment_equally(\n        segment_id: int, dist: float, proportion: float = 0.0\n    ) -> None",
          "code": "def extend_segment_equally(\n        self, segment_id: int, dist: float, proportion: float = 0.0\n    ) -> None:\n\n        \"\"\"Extend polyline segment equally.\"\"\"\n        if segment_id < 0 or segment_id >= self.segment_count():\n            return\n\n        start = self.get_point(segment_id)\n        end = self.get_point(segment_id + 1)\n        self.extend_segment_equally_static(start, end, dist, proportion)\n        self.set_point(segment_id, start)\n        self.set_point(segment_id + 1, end)\n\n        if self.point_count() > 2 and self.is_closed():\n            if segment_id == 0:\n                self.set_point(self.point_count() - 1, self.get_point(0))\n            elif segment_id + 1 == self.point_count() - 1:\n                self.set_point(0, self.get_point(self.point_count() - 1))\n\n    def is_clockwise(self, plane: Plane) -> bool:\n        \"\"\"Check if polyline is clockwise oriented.\"\"\"\n        if len(self.points) < 3:",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "void extend_segment_equally(int segment_id, double dist, double proportion)",
          "code": "void Polyline::extend_segment_equally(int segment_id, double dist, double proportion) {\n    if (segment_id < 0 || segment_id >= static_cast<int>(segment_count())) return;\n\n    Point p0 = get_point(segment_id);\n    Point p1 = get_point(segment_id + 1);\n    extend_segment_equally(p0, p1, dist, proportion);\n    set_point(segment_id, p0);\n    set_point(segment_id + 1, p1);\n\n    // Handle closed polylines\n    if (point_count() > 2 && is_closed()) {\n        if (segment_id == 0) {\n            set_point(point_count() - 1, get_point(0));\n        }",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "extend_segment_equally(segment_id: usize, dist: f64, proportion: f64)",
          "code": "pub fn extend_segment_equally(&mut self, segment_id: usize, dist: f64, proportion: f64) {\n        if segment_id >= self.segment_count() {\n            return;\n        }\n\n        let mut start = self.get_point(segment_id).unwrap();\n        let mut end = self.get_point(segment_id + 1).unwrap();\n        Self::extend_segment_equally_static(&mut start, &mut end, dist, proportion);\n        self.set_point(segment_id, &start);\n        self.set_point(segment_id + 1, &end);\n\n        if self.point_count() > 2 && self.is_closed() {\n            let len = self.point_count();\n            if segment_id == 0 {\n                let first = self.get_point(0).unwrap();\n                self.set_point(len - 1, &first);\n            } else if segment_id + 1 == len - 1 {\n                let last = self.get_point",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.is_clockwise",
      "implementations": {
        "python": {
          "sig": "is_clockwise(plane: Plane) -> bool",
          "code": "def is_clockwise(self, plane: Plane) -> bool:\n\n        \"\"\"Check if polyline is clockwise oriented.\"\"\"\n        if len(self.points) < 3:\n            return False\n\n        sum_val = 0.0\n        n = len(self.points) - 1 if self.is_closed() else len(self.points)\n\n        for i in range(n):\n            current = self.points[i]\n            next_pt = self.points[(i + 1) % n]\n            sum_val += (next_pt.x - current.x) * (next_pt.y + current.y)\n\n        return sum_val > 0.0\n\n    def get_convex_corners(self) -> List[bool]:\n        \"\"\"Get convex/concave corners of polyline.\"\"\"\n        if len(self.points) < 3:\n            return []",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "bool is_clockwise(const Plane& pln)",
          "code": "bool Polyline::is_clockwise(const Plane& pln) const {\n    (void)pln;  // Reserved for future use - may project to plane\n    if (point_count() < 3) return false;\n\n    // Create a copy for transformation\n    Polyline cp = *this;\n\n    // Ensure closed for winding calculation\n    if (!cp.is_closed()) {\n        cp.add_point(cp.get_point(0));\n    }",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "is_clockwise(_plane: &Plane) -> bool",
          "code": "pub fn is_clockwise(&self, _plane: &Plane) -> bool {\n        let total = self.point_count();\n        if total < 3 {\n            return false;\n        }\n\n        let mut sum = 0.0;\n        let n = if self.is_closed() { total - 1 } else { total };\n\n        for i in 0..n {\n            let idx_curr = i * 3;\n            let idx_next = ((i + 1) % n) * 3;\n            sum += (self.coords[idx_next] - self.coords[idx_curr])\n                * (self.coords[idx_next + 1] + self.coords[idx_curr + 1]);\n        }\n\n        sum > 0.0\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.get_convex_corners",
      "implementations": {
        "python": {
          "sig": "get_convex_corners() -> List[bool]",
          "code": "def get_convex_corners(self) -> List[bool]:\n\n        \"\"\"Get convex/concave corners of polyline.\"\"\"\n        if len(self.points) < 3:\n            return []\n\n        closed = self.is_closed()\n        normal = self._average_normal()\n        n = len(self.points) - 1 if closed else len(self.points)\n        convex_corners = []\n\n        for current in range(n):\n            prev = n - 1 if current == 0 else current - 1\n            next_pt = 0 if current == n - 1 else current + 1\n\n            dir0 = (self.points[current] - self.points[prev]).normalize()\n            dir1 = (self.points[next_pt] - self.points[current]).normalize()\n\n            cross = dir0.cross(dir1).normalize()\n            dot = cross.dot(normal)\n            is_convex = not (dot < 0.0)",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "void get_convex_corners(std::vector<bool>& convex_or_concave)",
          "code": "void Polyline::get_convex_corners(std::vector<bool>& convex_or_concave) const {\n    if (point_count() < 3) return;\n\n    bool closed = is_closed();\n    size_t n = closed ? point_count() - 1 : point_count();\n\n    Vector normal;\n    average_normal(normal);\n\n    convex_or_concave.clear();\n    convex_or_concave.reserve(n);\n\n    for (size_t current = 0; current < n; current++) {\n        size_t prev = (current == 0) ? n - 1 : current - 1;\n        size_t next = (current == n - 1) ? 0 : current + 1;\n\n        Vector dir0 = get_point(current) - get_point(prev);\n        dir0.normalize_self();\n\n        Vector dir1 = get_point(next) - get_point(current);\n        dir1.normalize_self();\n\n        Vector cross = dir0.cross(dir1);\n        cross.normalize_self();\n\n        double dot = cross.dot(normal);",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "get_convex_corners() -> Vec<bool>",
          "code": "pub fn get_convex_corners(&self) -> Vec<bool> {\n        let total = self.point_count();\n        if total < 3 {\n            return Vec::new();\n        }\n\n        let closed = self.is_closed();\n        let normal = self.average_normal();\n        let n = if closed { total - 1 } else { total };\n        let mut convex_corners = Vec::with_capacity(n);\n        let points = self.get_points();\n\n        for current in 0..n {\n            let prev = if current == 0 { n - 1 } else { current - 1 };\n            let next = if current == n - 1 { 0 } else { current + 1 };\n\n            let mut dir0 = points[current].clone() - points[prev].clone();\n            dir0.normalize();\n\n            let mut dir1 = points[next].clone() - points[current].clone();\n            dir1.normalize();\n\n            let mut cr",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.tween_two_polylines",
      "implementations": {
        "python": {
          "sig": "tween_two_polylines(\n        polyline0: \"Polyline\", polyline1: \"Polyline\", weight: float\n    ) -> \"Polyline\"",
          "code": "def tween_two_polylines(\n        polyline0: \"Polyline\", polyline1: \"Polyline\", weight: float\n    ) -> \"Polyline\":\n\n        \"\"\"Interpolate between two polylines.\"\"\"\n        if len(polyline0.points) != len(polyline1.points):\n            return Polyline(polyline0.points[:])\n\n        result_points = []\n        for i in range(len(polyline0.points)):\n            diff = polyline1.points[i] - polyline0.points[i]\n            interpolated = polyline0.points[i] + diff * weight\n            result_points.append(interpolated)\n\n        return Polyline(result_points)\n\n    def _average_normal(self) -> Vector:\n        \"\"\"Calculate average normal from polyline points.\"\"\"\n        if len(self.points) < 3:\n            return Vector(0.0, 0.0, 1.0)\n\n        closed = self.is_closed()\n        n = (",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "Polyline tween_two_polylines(const Polyline& polyline0, const Polyline& polyline1, double weight)",
          "code": "Polyline Polyline::tween_two_polylines(const Polyline& polyline0, const Polyline& polyline1, double weight) {\n    if (polyline0.point_count() != polyline1.point_count()) {\n        // Return first polyline if sizes don't match\n        return polyline0;\n    }",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "tween_two_polylines(\n        polyline0: &Polyline,\n        polyline1: &Polyline,\n        weight: f64,\n    ) -> Polyline",
          "code": "pub fn tween_two_polylines(\n        polyline0: &Polyline,\n        polyline1: &Polyline,\n        weight: f64,\n    ) -> Polyline {\n        if polyline0.point_count() != polyline1.point_count() {\n            return polyline0.clone();\n        }\n\n        let mut result = Polyline::default();\n        result.coords.reserve(polyline0.coords.len());\n\n        for i in 0..polyline0.point_count() {\n            let idx = i * 3;\n            let x = polyline0.coords[idx] + (polyline1.coords[idx] - polyline0.coords[idx]) * weight;\n            let y = polyline0.coords[idx + 1] + (polyline1.coords[idx + 1] - polyline0.coords[idx + 1]) * weight;\n            let z = polyline0.coords[idx + 2] + (polyline1.coords[idx + 2] - polyline0.coords[idx + 2]) * weight;\n            result.coords.push(x);",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline._average_normal",
      "implementations": {
        "python": {
          "sig": "_average_normal() -> Vector",
          "code": "def _average_normal(self) -> Vector:\n\n        \"\"\"Calculate average normal from polyline points.\"\"\"\n        if len(self.points) < 3:\n            return Vector(0.0, 0.0, 1.0)\n\n        closed = self.is_closed()\n        n = (\n            len(self.points) - 1\n            if closed and len(self.points) > 1\n            else len(self.points)\n        )\n\n        average_normal = Vector(0.0, 0.0, 0.0)\n\n        for i in range(n):\n            prev = n - 1 if i == 0 else i - 1\n            next_pt = (i + 1) % n\n\n            v1 = self.points[prev] - self.points[i]\n            v2 = self.points[i] - self.points[next_pt]",
          "file": "polyline.py"
        }
      }
    },
    {
      "name": "Polyline.__jsondump__",
      "implementations": {
        "python": {
          "sig": "__jsondump__()",
          "code": "def __jsondump__(self):\n\n        \"\"\"Serialize to polymorphic JSON format with type field.\n\n        Uses compact coords array format: [x0, y0, z0, x1, y1, z1, ...]\n\n        Returns\n        -------\n        dict\n            Dictionary with 'type', 'guid', 'name', and object fields.\n\n        \"\"\"\n        # Alphabetical order to match Rust's serde_json\n        return {\n            \"coords\": self._coords,\n            \"guid\": self.guid,\n            \"linecolor\": self.linecolor.__jsondump__(),\n            \"name\": self.name,\n            \"type\": f\"{self.__class__.__name__}\",\n            \"width\": self.width,\n            \"xform\": self.xform.__jsondump__(),",
          "file": "polyline.py"
        }
      }
    },
    {
      "name": "Polyline.__jsonload__",
      "implementations": {
        "python": {
          "sig": "__jsonload__(cls, data, guid=None, name=None)",
          "code": "def __jsonload__(cls, data, guid=None, name=None):\n\n        \"\"\"Deserialize from polymorphic JSON format.\n\n        Supports both compact coords format and legacy points format.\n\n        Parameters\n        ----------\n        data : dict\n            Dictionary containing polyline data.\n        guid : str, optional\n            GUID for the polyline.\n        name : str, optional\n            Name for the polyline.\n\n        Returns\n        -------\n        :class:`Polyline`\n            Reconstructed polyline instance.\n\n        \"\"\"",
          "file": "polyline.py"
        }
      }
    },
    {
      "name": "Polyline.json_dump",
      "implementations": {
        "python": {
          "sig": "json_dump(filepath)",
          "code": "def json_dump(self, filepath):\n\n        \"\"\"Write JSON to file.\n\n        Parameters\n        ----------\n        filepath : str or Path\n            Path to the output file.\n\n        \"\"\"\n        import json\n        with open(filepath, 'w') as f:\n            json.dump(self.__jsondump__(), f, indent=2)\n\n    @classmethod\n    def json_load(cls, filepath):\n        \"\"\"Read JSON from file.\n\n        Parameters\n        ----------\n        filepath : str or Path",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "void json_dump(const std::string& filename)",
          "code": "void Polyline::json_dump(const std::string& filename) const {\n    std::ofstream file(filename);\n    file << jsondump().dump(2);\n}",
          "file": "polyline.cpp"
        }
      }
    },
    {
      "name": "Polyline.json_load",
      "implementations": {
        "python": {
          "sig": "json_load(cls, filepath)",
          "code": "def json_load(cls, filepath):\n\n        \"\"\"Read JSON from file.\n\n        Parameters\n        ----------\n        filepath : str or Path\n            Path to the JSON file.\n\n        Returns\n        -------\n        :class:`Polyline`\n            The deserialized Polyline.\n\n        \"\"\"\n        import json\n        with open(filepath, 'r') as f:\n            data = json.load(f)\n        return cls.__jsonload__(data)\n\n    ###########################################################################################",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "Polyline json_load(const std::string& filename)",
          "code": "Polyline Polyline::json_load(const std::string& filename) {\n    std::ifstream file(filename);\n    nlohmann::json data;\n    file >> data;\n    return jsonload(data);\n}",
          "file": "polyline.cpp"
        }
      }
    },
    {
      "name": "Polyline.to_protobuf",
      "implementations": {
        "python": {
          "sig": "to_protobuf()",
          "code": "def to_protobuf(self):\n\n        \"\"\"Convert to protobuf binary format.\n\n        Returns\n        -------\n        bytes\n            Serialized protobuf data.\n\n        \"\"\"\n        from .proto import polyline_pb2\n\n        proto = polyline_pb2.Polyline()\n        proto.guid = self.guid\n        proto.name = self.name\n        proto.coords.extend(self._coords)\n        proto.width = self.width\n\n        # Set linecolor\n        proto.linecolor.name = self.linecolor.name\n        proto.linecolor.r = self.linecolor[0]",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "std::string to_protobuf()",
          "code": "std::string Polyline::to_protobuf() const {\n    session_proto::Polyline proto;\n    proto.set_guid(this->guid);\n    proto.set_name(this->name);\n    proto.set_width(this->width);\n\n    // Add coords as flat array [x0, y0, z0, x1, y1, z1, ...]\n    for (double c : _coords) {\n        proto.add_coords(c);\n    }",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "to_protobuf() -> Vec<u8>",
          "code": "pub fn to_protobuf(&self) -> Vec<u8> {\n        use prost::Message;\n\n        let proto = crate::proto::Polyline {\n            guid: self.guid.clone(),\n            name: self.name.clone(),\n            coords: self.coords.clone(),\n            width: self.width,\n            linecolor: Some(crate::proto::Color {\n                guid: self.linecolor.guid.clone(),\n                name: self.linecolor.name.clone(),\n                r: self.linecolor.r as i32,\n                g: self.linecolor.g as i32,\n                b: self.linecolor.b as i32,\n                a: self.linecolor.a as i32,\n            }),\n            xform: Some(crate::proto::Xform {\n                guid: self.xform.guid.clone(),\n                name: self.xform.name.clone(),\n                matrix: self.xform.m.to_vec(),",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.from_protobuf",
      "implementations": {
        "python": {
          "sig": "from_protobuf(cls, data)",
          "code": "def from_protobuf(cls, data):\n\n        \"\"\"Create Polyline from protobuf binary data.\n\n        Parameters\n        ----------\n        data : bytes\n            Protobuf-encoded polyline data.\n\n        Returns\n        -------\n        :class:`Polyline`\n            The deserialized Polyline.\n\n        \"\"\"\n        from .proto import polyline_pb2\n\n        proto = polyline_pb2.Polyline()\n        proto.ParseFromString(data)\n\n        polyline = cls.from_coords(list(proto.coords))",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "Polyline from_protobuf(const std::string& data)",
          "code": "Polyline Polyline::from_protobuf(const std::string& data) {\n    session_proto::Polyline proto;\n    proto.ParseFromString(data);\n\n    // Read coords as flat array [x0, y0, z0, x1, y1, z1, ...]\n    std::vector<double> coords(proto.coords().begin(), proto.coords().end());\n\n    Polyline pl = Polyline::from_coords(coords);\n    pl.guid = proto.guid();\n    pl.name = proto.name();\n    pl.width = proto.width();\n\n    // Load linecolor\n    if (proto.has_linecolor()) {\n        const auto& c = proto.linecolor();\n        pl.linecolor = Color(c.r(), c.g(), c.b(), c.a(), c.name());\n    }",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "from_protobuf(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn from_protobuf(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {\n        use prost::Message;\n\n        let proto = crate::proto::Polyline::decode(data)?;\n\n        let mut pl = Self::from_coords(proto.coords);\n        pl.guid = proto.guid;\n        pl.name = proto.name;\n        pl.width = proto.width;\n\n        if let Some(color) = proto.linecolor {\n            pl.linecolor.guid = color.guid;\n            pl.linecolor.name = color.name;\n            pl.linecolor.r = color.r as u8;\n            pl.linecolor.g = color.g as u8;\n            pl.linecolor.b = color.b as u8;\n            pl.linecolor.a = color.a as u8;\n        }\n\n        if let Some(xform) = proto.xform {\n            pl.xform.guid = xform.guid;\n            pl.xform.name = xform.name;\n            for (i, val) in xform.m",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.protobuf_dump",
      "implementations": {
        "python": {
          "sig": "protobuf_dump(filepath)",
          "code": "def protobuf_dump(self, filepath):\n\n        \"\"\"Write protobuf to file.\n\n        Parameters\n        ----------\n        filepath : str\n            Path to the output file.\n\n        \"\"\"\n        data = self.to_protobuf()\n        with open(filepath, 'wb') as f:\n            f.write(data)\n\n    @classmethod\n    def protobuf_load(cls, filepath):\n        \"\"\"Read protobuf from file.\n\n        Parameters\n        ----------\n        filepath : str",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "void protobuf_dump(const std::string& filename)",
          "code": "void Polyline::protobuf_dump(const std::string& filename) const {\n    std::string data = to_protobuf();\n    std::ofstream file(filename, std::ios::binary);\n    file.write(data.data(), data.size());\n}",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "protobuf_dump(filepath: &str)",
          "code": "pub fn protobuf_dump(&self, filepath: &str) {\n        let data = self.to_protobuf();\n        std::fs::write(filepath, data).expect(\"Failed to write protobuf file\");\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.protobuf_load",
      "implementations": {
        "python": {
          "sig": "protobuf_load(cls, filepath)",
          "code": "def protobuf_load(cls, filepath):\n\n        \"\"\"Read protobuf from file.\n\n        Parameters\n        ----------\n        filepath : str\n            Path to the protobuf file.\n\n        Returns\n        -------\n        :class:`Polyline`\n            The deserialized Polyline.\n\n        \"\"\"\n        with open(filepath, 'rb') as f:\n            data = f.read()\n        return cls.from_protobuf(data)\n\n    def __str__(self) -> str:\n        \"\"\"Returns a minimal string representation of the polyline.\"\"\"",
          "file": "polyline.py"
        },
        "cpp": {
          "sig": "Polyline protobuf_load(const std::string& filename)",
          "code": "Polyline Polyline::protobuf_load(const std::string& filename) {\n    std::ifstream file(filename, std::ios::binary);\n    std::string data((std::istreambuf_iterator<char>(file)),\n                      std::istreambuf_iterator<char>());\n    return from_protobuf(data);\n}",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "protobuf_load(filepath: &str) -> Self",
          "code": "pub fn protobuf_load(filepath: &str) -> Self {\n        let data = std::fs::read(filepath).expect(\"Failed to read protobuf file\");\n        Self::from_protobuf(&data).expect(\"Failed to parse protobuf\")\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.__str__",
      "implementations": {
        "python": {
          "sig": "__str__() -> str",
          "code": "def __str__(self) -> str:\n\n        \"\"\"Returns a minimal string representation of the polyline.\"\"\"\n        pts = []\n        for i in range(self.point_count()):\n            idx = i * 3\n            pts.append(f\"({self._coords[idx]}, {self._coords[idx + 1]}, {self._coords[idx + 2]})\")\n        return \"[\" + \", \".join(pts) + \"]\"\n\n    def __repr__(self) -> str:\n        \"\"\"Returns a detailed string representation.\"\"\"\n        return f\"Polyline({self.name}, {self.point_count()} points)\"\n\n    def __eq__(self, other) -> bool:\n        \"\"\"Compare polylines by value (ignoring GUIDs).\"\"\"\n        if not isinstance(other, Polyline):\n            return False\n        if self.name != other.name:\n            return False\n        if self.point_count() != other.point_count():\n            return False",
          "file": "polyline.py"
        }
      }
    },
    {
      "name": "Polyline.__repr__",
      "implementations": {
        "python": {
          "sig": "__repr__() -> str",
          "code": "def __repr__(self) -> str:\n\n        \"\"\"Returns a detailed string representation.\"\"\"\n        return f\"Polyline({self.name}, {self.point_count()} points)\"\n\n    def __eq__(self, other) -> bool:\n        \"\"\"Compare polylines by value (ignoring GUIDs).\"\"\"\n        if not isinstance(other, Polyline):\n            return False\n        if self.name != other.name:\n            return False\n        if self.point_count() != other.point_count():\n            return False\n        for i in range(len(self._coords)):\n            if round(self._coords[i], Tolerance.ROUNDING) != round(other._coords[i], Tolerance.ROUNDING):\n                return False\n        if round(self.width, Tolerance.ROUNDING) != round(other.width, Tolerance.ROUNDING):\n            return False\n        if self.linecolor != other.linecolor:\n            return False\n        return True",
          "file": "polyline.py"
        }
      }
    },
    {
      "name": "Polyline.__eq__",
      "implementations": {
        "python": {
          "sig": "__eq__(other) -> bool",
          "code": "def __eq__(self, other) -> bool:\n\n        \"\"\"Compare polylines by value (ignoring GUIDs).\"\"\"\n        if not isinstance(other, Polyline):\n            return False\n        if self.name != other.name:\n            return False\n        if self.point_count() != other.point_count():\n            return False\n        for i in range(len(self._coords)):\n            if round(self._coords[i], Tolerance.ROUNDING) != round(other._coords[i], Tolerance.ROUNDING):\n                return False\n        if round(self.width, Tolerance.ROUNDING) != round(other.width, Tolerance.ROUNDING):\n            return False\n        if self.linecolor != other.linecolor:\n            return False\n        return True\n\n    def __ne__(self, other) -> bool:\n        return not self == other",
          "file": "polyline.py"
        }
      }
    },
    {
      "name": "Polyline.__ne__",
      "implementations": {
        "python": {
          "sig": "__ne__(other) -> bool",
          "code": "def __ne__(self, other) -> bool:\n\n        return not self == other",
          "file": "polyline.py"
        }
      }
    },
    {
      "name": "Tolerance.__new__",
      "implementations": {
        "python": {
          "sig": "__new__(cls, *args, **kwargs)",
          "code": "def __new__(cls, *args, **kwargs):\n\n        if not cls._instance:\n            cls._instance = object.__new__(cls)\n            cls._is_inited = False\n        return cls._instance\n\n    def __init__(\n        self,\n        unit=\"M\",\n        absolute=None,\n        relative=None,\n        angular=None,\n        approximation=None,\n        precision=None,\n        lineardeflection=None,\n        angulardeflection=None,\n        name=None,\n    ):\n        if not self._is_inited:\n            self._unit = None",
          "file": "tolerance.py"
        }
      }
    },
    {
      "name": "Tolerance.__init__",
      "implementations": {
        "python": {
          "sig": "__init__(\n        ,\n        unit=\"M\",\n        absolute=None,\n        relative=None,\n        angular=None,\n        approximation=None,\n        precision=None,\n        lineardeflection=None,\n        angulardeflection=None,\n        name=None,\n    )",
          "code": "def __init__(\n        self,\n        unit=\"M\",\n        absolute=None,\n        relative=None,\n        angular=None,\n        approximation=None,\n        precision=None,\n        lineardeflection=None,\n        angulardeflection=None,\n        name=None,\n    ):\n\n        if not self._is_inited:\n            self._unit = None\n            self._absolute = None\n            self._relative = None\n            self._angular = None\n            self._approximation = None\n            self._precision = None\n            self._lineardeflection = None\n            self._angulardeflection = None\n\n        self._is_inited = True\n\n        if unit is not None:\n            self.unit = unit\n        if absolute is not None:\n            self.absolute = absolute\n        if relative is not None:\n            self.relative = relative\n        if angular is not None:",
          "file": "tolerance.py"
        }
      }
    },
    {
      "name": "Tolerance.reset",
      "implementations": {
        "python": {
          "sig": "reset()",
          "code": "def reset(self):\n\n        \"\"\"Reset all precision settings to their default values.\"\"\"\n        self._absolute = None\n        self._relative = None\n        self._angular = None\n        self._approximation = None\n        self._precision = None\n        self._lineardeflection = None\n        self._angulardeflection = None\n\n    @property\n    def unit(self):\n        return self._unit or \"M\"\n\n    @unit.setter\n    def unit(self, value):\n        if value not in [\"M\", \"MM\"]:\n            raise ValueError(f\"Invalid unit: {value}\")\n        self._unit = value",
          "file": "tolerance.py"
        },
        "cpp": {
          "sig": "void reset()",
          "code": "void Tolerance::reset() {\n    _has_absolute = false;\n    _has_relative = false;\n    _has_angular = false;\n    _has_approximation = false;\n    _has_precision = false;\n    _has_lineardeflection = false;\n    _has_angulardeflection = false;\n}",
          "file": "tolerance.cpp"
        },
        "rust": {
          "sig": "reset()",
          "code": "pub fn reset(&mut self) {\n        self.absolute = None;\n        self.relative = None;\n        self.angular = None;\n        self.approximation = None;\n        self.precision = None;\n        self.lineardeflection = None;\n        self.angulardeflection = None;\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.unit",
      "implementations": {
        "python": {
          "sig": "unit(value)",
          "code": "def unit(self, value):\n\n        if value not in [\"M\", \"MM\"]:\n            raise ValueError(f\"Invalid unit: {value}\")\n        self._unit = value\n\n    @property\n    def units(self):\n        return self._unit or \"M\"\n\n    @units.setter\n    def units(self, value):\n        if value not in [\"M\", \"MM\"]:\n            raise ValueError(f\"Invalid unit: {value}\")\n        self._unit = value\n\n    @property\n    def absolute(self):\n        return self._absolute if self._absolute is not None else self.ABSOLUTE\n\n    @absolute.setter",
          "file": "tolerance.py"
        }
      }
    },
    {
      "name": "Tolerance.units",
      "implementations": {
        "python": {
          "sig": "units(value)",
          "code": "def units(self, value):\n\n        if value not in [\"M\", \"MM\"]:\n            raise ValueError(f\"Invalid unit: {value}\")\n        self._unit = value\n\n    @property\n    def absolute(self):\n        return self._absolute if self._absolute is not None else self.ABSOLUTE\n\n    @absolute.setter\n    def absolute(self, value):\n        self._absolute = value\n\n    @property\n    def relative(self):\n        return self._relative if self._relative is not None else self.RELATIVE\n\n    @relative.setter\n    def relative(self, value):\n        self._relative = value",
          "file": "tolerance.py"
        }
      }
    },
    {
      "name": "Tolerance.absolute",
      "implementations": {
        "python": {
          "sig": "absolute(value)",
          "code": "def absolute(self, value):\n\n        self._absolute = value\n\n    @property\n    def relative(self):\n        return self._relative if self._relative is not None else self.RELATIVE\n\n    @relative.setter\n    def relative(self, value):\n        self._relative = value\n\n    @property\n    def angular(self):\n        return self._angular if self._angular is not None else self.ANGULAR\n\n    @angular.setter\n    def angular(self, value):\n        self._angular = value\n\n    @property",
          "file": "tolerance.py"
        },
        "rust": {
          "sig": "absolute() -> f64",
          "code": "pub fn absolute(&self) -> f64 {\n        self.absolute.unwrap_or(Self::ABSOLUTE)\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.relative",
      "implementations": {
        "python": {
          "sig": "relative(value)",
          "code": "def relative(self, value):\n\n        self._relative = value\n\n    @property\n    def angular(self):\n        return self._angular if self._angular is not None else self.ANGULAR\n\n    @angular.setter\n    def angular(self, value):\n        self._angular = value\n\n    @property\n    def approximation(self):\n        return (\n            self._approximation\n            if self._approximation is not None\n            else self.APPROXIMATION\n        )\n\n    @approximation.setter",
          "file": "tolerance.py"
        },
        "rust": {
          "sig": "relative() -> f64",
          "code": "pub fn relative(&self) -> f64 {\n        self.relative.unwrap_or(Self::RELATIVE)\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.angular",
      "implementations": {
        "python": {
          "sig": "angular(value)",
          "code": "def angular(self, value):\n\n        self._angular = value\n\n    @property\n    def approximation(self):\n        return (\n            self._approximation\n            if self._approximation is not None\n            else self.APPROXIMATION\n        )\n\n    @approximation.setter\n    def approximation(self, value):\n        self._approximation = value\n\n    @property\n    def precision(self):\n        return self._precision if self._precision is not None else self.PRECISION\n\n    @precision.setter",
          "file": "tolerance.py"
        },
        "rust": {
          "sig": "angular() -> f64",
          "code": "pub fn angular(&self) -> f64 {\n        self.angular.unwrap_or(Self::ANGULAR)\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.approximation",
      "implementations": {
        "python": {
          "sig": "approximation(value)",
          "code": "def approximation(self, value):\n\n        self._approximation = value\n\n    @property\n    def precision(self):\n        return self._precision if self._precision is not None else self.PRECISION\n\n    @precision.setter\n    def precision(self, value):\n        if value == 0:\n            raise ValueError(\"Precision cannot be zero.\")\n        self._precision = value\n\n    @property\n    def lineardeflection(self):\n        return (\n            self._lineardeflection\n            if self._lineardeflection is not None\n            else self.LINEARDEFLECTION\n        )",
          "file": "tolerance.py"
        },
        "rust": {
          "sig": "approximation() -> f64",
          "code": "pub fn approximation(&self) -> f64 {\n        self.approximation.unwrap_or(Self::APPROXIMATION)\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.precision",
      "implementations": {
        "python": {
          "sig": "precision(value)",
          "code": "def precision(self, value):\n\n        if value == 0:\n            raise ValueError(\"Precision cannot be zero.\")\n        self._precision = value\n\n    @property\n    def lineardeflection(self):\n        return (\n            self._lineardeflection\n            if self._lineardeflection is not None\n            else self.LINEARDEFLECTION\n        )\n\n    @lineardeflection.setter\n    def lineardeflection(self, value):\n        self._lineardeflection = value\n\n    @property\n    def angulardeflection(self):\n        return (",
          "file": "tolerance.py"
        },
        "rust": {
          "sig": "precision() -> i32",
          "code": "pub fn precision(&self) -> i32 {\n        self.precision.unwrap_or(Self::PRECISION)\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.lineardeflection",
      "implementations": {
        "python": {
          "sig": "lineardeflection(value)",
          "code": "def lineardeflection(self, value):\n\n        self._lineardeflection = value\n\n    @property\n    def angulardeflection(self):\n        return (\n            self._angulardeflection\n            if self._angulardeflection is not None\n            else self.ANGULARDEFLECTION\n        )\n\n    @angulardeflection.setter\n    def angulardeflection(self, value):\n        self._angulardeflection = value\n\n    def tolerance(self, truevalue, rtol, atol):\n        \"\"\"Compute the tolerance for a comparison.\"\"\"\n        return rtol * abs(truevalue) + atol\n\n    def compare(self, a, b, rtol, atol):",
          "file": "tolerance.py"
        },
        "rust": {
          "sig": "lineardeflection() -> f64",
          "code": "pub fn lineardeflection(&self) -> f64 {\n        self.lineardeflection.unwrap_or(Self::LINEARDEFLECTION)\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.angulardeflection",
      "implementations": {
        "python": {
          "sig": "angulardeflection(value)",
          "code": "def angulardeflection(self, value):\n\n        self._angulardeflection = value\n\n    def tolerance(self, truevalue, rtol, atol):\n        \"\"\"Compute the tolerance for a comparison.\"\"\"\n        return rtol * abs(truevalue) + atol\n\n    def compare(self, a, b, rtol, atol):\n        \"\"\"Compare two values.\"\"\"\n        return abs(a - b) <= self.tolerance(b, rtol, atol)\n\n    def is_zero(self, a):\n        \"\"\"Check if a value is close enough to zero to be considered zero.\"\"\"\n        return abs(a) <= self.absolute\n\n    def is_positive(self, a):\n        \"\"\"Check if a value can be considered a strictly positive number.\"\"\"\n        return a > self.absolute\n\n    def is_negative(self, a):",
          "file": "tolerance.py"
        },
        "rust": {
          "sig": "angulardeflection() -> f64",
          "code": "pub fn angulardeflection(&self) -> f64 {\n        self.angulardeflection.unwrap_or(Self::ANGULARDEFLECTION)\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.tolerance",
      "implementations": {
        "python": {
          "sig": "tolerance(truevalue, rtol, atol)",
          "code": "def tolerance(self, truevalue, rtol, atol):\n\n        \"\"\"Compute the tolerance for a comparison.\"\"\"\n        return rtol * abs(truevalue) + atol\n\n    def compare(self, a, b, rtol, atol):\n        \"\"\"Compare two values.\"\"\"\n        return abs(a - b) <= self.tolerance(b, rtol, atol)\n\n    def is_zero(self, a):\n        \"\"\"Check if a value is close enough to zero to be considered zero.\"\"\"\n        return abs(a) <= self.absolute\n\n    def is_positive(self, a):\n        \"\"\"Check if a value can be considered a strictly positive number.\"\"\"\n        return a > self.absolute\n\n    def is_negative(self, a):\n        \"\"\"Check if a value can be considered a strictly negative number.\"\"\"\n        return a < -self.absolute",
          "file": "tolerance.py"
        },
        "cpp": {
          "sig": "double tolerance(double truevalue, double rtol, double atol)",
          "code": "double Tolerance::tolerance(double truevalue, double rtol, double atol) const {\n    return rtol * std::abs(truevalue) + atol;\n}",
          "file": "tolerance.cpp"
        },
        "rust": {
          "sig": "tolerance(truevalue: f64, rtol: f64, atol: f64) -> f64",
          "code": "pub fn tolerance(&self, truevalue: f64, rtol: f64, atol: f64) -> f64 {\n        rtol * truevalue.abs() + atol\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.compare",
      "implementations": {
        "python": {
          "sig": "compare(a, b, rtol, atol)",
          "code": "def compare(self, a, b, rtol, atol):\n\n        \"\"\"Compare two values.\"\"\"\n        return abs(a - b) <= self.tolerance(b, rtol, atol)\n\n    def is_zero(self, a):\n        \"\"\"Check if a value is close enough to zero to be considered zero.\"\"\"\n        return abs(a) <= self.absolute\n\n    def is_positive(self, a):\n        \"\"\"Check if a value can be considered a strictly positive number.\"\"\"\n        return a > self.absolute\n\n    def is_negative(self, a):\n        \"\"\"Check if a value can be considered a strictly negative number.\"\"\"\n        return a < -self.absolute\n\n    def is_between(self, value, minval, maxval):\n        \"\"\"Check if a value is between two other values.\"\"\"\n        atol = self.absolute\n        return minval - atol <= value <= maxval + atol",
          "file": "tolerance.py"
        },
        "cpp": {
          "sig": "bool compare(double a, double b, double rtol, double atol)",
          "code": "bool Tolerance::compare(double a, double b, double rtol, double atol) const {\n    return std::abs(a - b) <= tolerance(b, rtol, atol);\n}",
          "file": "tolerance.cpp"
        },
        "rust": {
          "sig": "compare(a: f64, b: f64, rtol: f64, atol: f64) -> bool",
          "code": "pub fn compare(&self, a: f64, b: f64, rtol: f64, atol: f64) -> bool {\n        (a - b).abs() <= self.tolerance(b, rtol, atol)\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.is_zero",
      "implementations": {
        "python": {
          "sig": "is_zero(a)",
          "code": "def is_zero(self, a):\n\n        \"\"\"Check if a value is close enough to zero to be considered zero.\"\"\"\n        return abs(a) <= self.absolute\n\n    def is_positive(self, a):\n        \"\"\"Check if a value can be considered a strictly positive number.\"\"\"\n        return a > self.absolute\n\n    def is_negative(self, a):\n        \"\"\"Check if a value can be considered a strictly negative number.\"\"\"\n        return a < -self.absolute\n\n    def is_between(self, value, minval, maxval):\n        \"\"\"Check if a value is between two other values.\"\"\"\n        atol = self.absolute\n        return minval - atol <= value <= maxval + atol\n\n    def is_close(self, a, b):\n        \"\"\"Check if two values are close enough to be considered equal.\"\"\"\n        return self.compare(a, b, self.relative, self.absolute)",
          "file": "tolerance.py"
        },
        "cpp": {
          "sig": "bool is_zero(double a)",
          "code": "bool Tolerance::is_zero(double a) const {\n    return std::abs(a) <= absolute();\n}",
          "file": "tolerance.cpp"
        },
        "rust": {
          "sig": "is_zero(a: f64) -> bool",
          "code": "pub fn is_zero(&self, a: f64) -> bool {\n        a.abs() <= self.absolute()\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.is_positive",
      "implementations": {
        "python": {
          "sig": "is_positive(a)",
          "code": "def is_positive(self, a):\n\n        \"\"\"Check if a value can be considered a strictly positive number.\"\"\"\n        return a > self.absolute\n\n    def is_negative(self, a):\n        \"\"\"Check if a value can be considered a strictly negative number.\"\"\"\n        return a < -self.absolute\n\n    def is_between(self, value, minval, maxval):\n        \"\"\"Check if a value is between two other values.\"\"\"\n        atol = self.absolute\n        return minval - atol <= value <= maxval + atol\n\n    def is_close(self, a, b):\n        \"\"\"Check if two values are close enough to be considered equal.\"\"\"\n        return self.compare(a, b, self.relative, self.absolute)\n\n    def is_allclose(self, A, B):\n        \"\"\"Check if two lists of values are element-wise close enough to be considered equal.\"\"\"\n        rtol = self.relative",
          "file": "tolerance.py"
        },
        "cpp": {
          "sig": "bool is_positive(double a)",
          "code": "bool Tolerance::is_positive(double a) const {\n    return a > absolute();\n}",
          "file": "tolerance.cpp"
        },
        "rust": {
          "sig": "is_positive(a: f64) -> bool",
          "code": "pub fn is_positive(&self, a: f64) -> bool {\n        a > self.absolute()\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.is_negative",
      "implementations": {
        "python": {
          "sig": "is_negative(a)",
          "code": "def is_negative(self, a):\n\n        \"\"\"Check if a value can be considered a strictly negative number.\"\"\"\n        return a < -self.absolute\n\n    def is_between(self, value, minval, maxval):\n        \"\"\"Check if a value is between two other values.\"\"\"\n        atol = self.absolute\n        return minval - atol <= value <= maxval + atol\n\n    def is_close(self, a, b):\n        \"\"\"Check if two values are close enough to be considered equal.\"\"\"\n        return self.compare(a, b, self.relative, self.absolute)\n\n    def is_allclose(self, A, B):\n        \"\"\"Check if two lists of values are element-wise close enough to be considered equal.\"\"\"\n        rtol = self.relative\n        atol = self.absolute\n        return all(\n            (\n                self.is_allclose(a, b)",
          "file": "tolerance.py"
        },
        "cpp": {
          "sig": "bool is_negative(double a)",
          "code": "bool Tolerance::is_negative(double a) const {\n    return a < -absolute();\n}",
          "file": "tolerance.cpp"
        },
        "rust": {
          "sig": "is_negative(a: f64) -> bool",
          "code": "pub fn is_negative(&self, a: f64) -> bool {\n        a < -self.absolute()\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.is_between",
      "implementations": {
        "python": {
          "sig": "is_between(value, minval, maxval)",
          "code": "def is_between(self, value, minval, maxval):\n\n        \"\"\"Check if a value is between two other values.\"\"\"\n        atol = self.absolute\n        return minval - atol <= value <= maxval + atol\n\n    def is_close(self, a, b):\n        \"\"\"Check if two values are close enough to be considered equal.\"\"\"\n        return self.compare(a, b, self.relative, self.absolute)\n\n    def is_allclose(self, A, B):\n        \"\"\"Check if two lists of values are element-wise close enough to be considered equal.\"\"\"\n        rtol = self.relative\n        atol = self.absolute\n        return all(\n            (\n                self.is_allclose(a, b)\n                if hasattr(a, \"__iter__\")\n                else self.compare(a, b, rtol, atol)\n            )\n            for a, b in zip(A, B)",
          "file": "tolerance.py"
        },
        "cpp": {
          "sig": "bool is_between(double value, double minval, double maxval)",
          "code": "bool Tolerance::is_between(double value, double minval, double maxval) const {\n    double atol = absolute();\n    return minval - atol <= value && value <= maxval + atol;\n}",
          "file": "tolerance.cpp"
        },
        "rust": {
          "sig": "is_between(value: f64, minval: f64, maxval: f64) -> bool",
          "code": "pub fn is_between(&self, value: f64, minval: f64, maxval: f64) -> bool {\n        let atol = self.absolute();\n        minval - atol <= value && value <= maxval + atol\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.is_close",
      "implementations": {
        "python": {
          "sig": "is_close(a, b)",
          "code": "def is_close(self, a, b):\n\n        \"\"\"Check if two values are close enough to be considered equal.\"\"\"\n        return self.compare(a, b, self.relative, self.absolute)\n\n    def is_allclose(self, A, B):\n        \"\"\"Check if two lists of values are element-wise close enough to be considered equal.\"\"\"\n        rtol = self.relative\n        atol = self.absolute\n        return all(\n            (\n                self.is_allclose(a, b)\n                if hasattr(a, \"__iter__\")\n                else self.compare(a, b, rtol, atol)\n            )\n            for a, b in zip(A, B)\n        )\n\n    def is_angle_zero(self, a):\n        \"\"\"Check if an angle is close enough to zero to be considered zero.\"\"\"\n        return abs(a) <= self.angular",
          "file": "tolerance.py"
        },
        "cpp": {
          "sig": "bool is_close(double a, double b)",
          "code": "bool Tolerance::is_close(double a, double b) const {\n    return compare(a, b, relative(), absolute());\n}",
          "file": "tolerance.cpp"
        },
        "rust": {
          "sig": "is_close(a: f64, b: f64) -> bool",
          "code": "pub fn is_close(&self, a: f64, b: f64) -> bool {\n        self.compare(a, b, self.relative(), self.absolute())\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.is_allclose",
      "implementations": {
        "python": {
          "sig": "is_allclose(A, B)",
          "code": "def is_allclose(self, A, B):\n\n        \"\"\"Check if two lists of values are element-wise close enough to be considered equal.\"\"\"\n        rtol = self.relative\n        atol = self.absolute\n        return all(\n            (\n                self.is_allclose(a, b)\n                if hasattr(a, \"__iter__\")\n                else self.compare(a, b, rtol, atol)\n            )\n            for a, b in zip(A, B)\n        )\n\n    def is_angle_zero(self, a):\n        \"\"\"Check if an angle is close enough to zero to be considered zero.\"\"\"\n        return abs(a) <= self.angular\n\n    def is_angles_close(self, a, b):\n        \"\"\"Check if two angles are close enough to be considered equal.\"\"\"\n        return abs(a - b) <= self.angular",
          "file": "tolerance.py"
        },
        "rust": {
          "sig": "is_allclose(a: &[f64], b: &[f64]) -> bool",
          "code": "pub fn is_allclose(&self, a: &[f64], b: &[f64]) -> bool {\n        let rtol = self.relative();\n        let atol = self.absolute();\n        a.iter()\n            .zip(b.iter())\n            .all(|(x, y)| self.compare(*x, *y, rtol, atol))\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.is_angle_zero",
      "implementations": {
        "python": {
          "sig": "is_angle_zero(a)",
          "code": "def is_angle_zero(self, a):\n\n        \"\"\"Check if an angle is close enough to zero to be considered zero.\"\"\"\n        return abs(a) <= self.angular\n\n    def is_angles_close(self, a, b):\n        \"\"\"Check if two angles are close enough to be considered equal.\"\"\"\n        return abs(a - b) <= self.angular\n\n    def key(self, xyz, precision=None, sanitize=True):\n        \"\"\"Compute the geometric key of a point.\"\"\"\n        x, y, z = xyz\n        if not precision:\n            precision = self.precision\n\n        if precision == 0:\n            raise ValueError(\"Precision cannot be zero.\")\n\n        if precision == -1:\n            return f\"{int(x)},{int(y)},{int(z)}\"",
          "file": "tolerance.py"
        },
        "cpp": {
          "sig": "bool is_angle_zero(double a)",
          "code": "bool Tolerance::is_angle_zero(double a) const {\n    return std::abs(a) <= angular();\n}",
          "file": "tolerance.cpp"
        },
        "rust": {
          "sig": "is_angle_zero(a: f64) -> bool",
          "code": "pub fn is_angle_zero(&self, a: f64) -> bool {\n        a.abs() <= self.angular()\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.is_angles_close",
      "implementations": {
        "python": {
          "sig": "is_angles_close(a, b)",
          "code": "def is_angles_close(self, a, b):\n\n        \"\"\"Check if two angles are close enough to be considered equal.\"\"\"\n        return abs(a - b) <= self.angular\n\n    def key(self, xyz, precision=None, sanitize=True):\n        \"\"\"Compute the geometric key of a point.\"\"\"\n        x, y, z = xyz\n        if not precision:\n            precision = self.precision\n\n        if precision == 0:\n            raise ValueError(\"Precision cannot be zero.\")\n\n        if precision == -1:\n            return f\"{int(x)},{int(y)},{int(z)}\"\n\n        if precision < -1:\n            precision = -precision - 1\n            factor = 10**precision\n            return f\"{int(round(x / factor) * factor)},{int(round(y / factor) * factor)},{int(round(z / factor) * factor)}\"",
          "file": "tolerance.py"
        },
        "cpp": {
          "sig": "bool is_angles_close(double a, double b)",
          "code": "bool Tolerance::is_angles_close(double a, double b) const {\n    return std::abs(a - b) <= angular();\n}",
          "file": "tolerance.cpp"
        },
        "rust": {
          "sig": "is_angles_close(a: f64, b: f64) -> bool",
          "code": "pub fn is_angles_close(&self, a: f64, b: f64) -> bool {\n        (a - b).abs() <= self.angular()\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.key",
      "implementations": {
        "python": {
          "sig": "key(xyz, precision=None, sanitize=True)",
          "code": "def key(self, xyz, precision=None, sanitize=True):\n\n        \"\"\"Compute the geometric key of a point.\"\"\"\n        x, y, z = xyz\n        if not precision:\n            precision = self.precision\n\n        if precision == 0:\n            raise ValueError(\"Precision cannot be zero.\")\n\n        if precision == -1:\n            return f\"{int(x)},{int(y)},{int(z)}\"\n\n        if precision < -1:\n            precision = -precision - 1\n            factor = 10**precision\n            return f\"{int(round(x / factor) * factor)},{int(round(y / factor) * factor)},{int(round(z / factor) * factor)}\"\n\n        if sanitize:\n            minzero = f\"-{0.0:.{precision}f}\"\n            if f\"{x:.{precision}f}\" == minzero:",
          "file": "tolerance.py"
        },
        "cpp": {
          "sig": "std::string key(double x, double y, double z, int precision)",
          "code": "std::string Tolerance::key(double x, double y, double z, int precision) const {\n    int prec = (precision != -999) ? precision : this->precision();\n    \n    if (prec == 0) {\n        throw std::invalid_argument(\"Precision cannot be zero.\");\n    }",
          "file": "tolerance.cpp"
        },
        "rust": {
          "sig": "key(xyz: [f64; 3], precision: i32) -> String",
          "code": "pub fn key(&self, xyz: [f64; 3], precision: i32) -> String {\n        let precision = if precision == -999 { self.precision() } else { precision };\n        let [mut x, mut y, mut z] = xyz;\n\n        if precision == -1 {\n            return format!(\"{},{},{}\", x as i64, y as i64, z as i64);\n        }\n\n        if precision < -1 {\n            let p = (-precision - 1) as u32;\n            let factor = 10_f64.powi(p as i32);\n            return format!(\n                \"{},{},{}\",\n                ((x / factor).round() * factor) as i64,\n                ((y / factor).round() * factor) as i64,\n                ((z / factor).round() * factor) as i64\n            );\n        }\n\n        let minzero = format!(\"-{:.prec$}\", 0.0, prec = precision as usize);\n        if format!(\"{:.prec$}\", x, prec = precisi",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.key_xy",
      "implementations": {
        "python": {
          "sig": "key_xy(xy, precision=None, sanitize=True)",
          "code": "def key_xy(self, xy, precision=None, sanitize=True):\n\n        \"\"\"Compute the geometric key of a point in the XY plane.\"\"\"\n        x, y = xy\n        if not precision:\n            precision = self.precision\n\n        if precision == 0:\n            raise ValueError(\"Precision cannot be zero.\")\n\n        if precision == -1:\n            return f\"{int(x)},{int(y)}\"\n\n        if precision < -1:\n            precision = -precision - 1\n            factor = 10**precision\n            return (\n                f\"{int(round(x / factor) * factor)},{int(round(y / factor) * factor)}\"\n            )\n\n        if sanitize:",
          "file": "tolerance.py"
        },
        "cpp": {
          "sig": "std::string key_xy(double x, double y, int precision)",
          "code": "std::string Tolerance::key_xy(double x, double y, int precision) const {\n    int prec = (precision != -999) ? precision : this->precision();\n    \n    if (prec == 0) {\n        throw std::invalid_argument(\"Precision cannot be zero.\");\n    }",
          "file": "tolerance.cpp"
        },
        "rust": {
          "sig": "key_xy(xy: [f64; 2], precision: i32) -> String",
          "code": "pub fn key_xy(&self, xy: [f64; 2], precision: i32) -> String {\n        let precision = if precision == -999 { self.precision() } else { precision };\n        let [mut x, mut y] = xy;\n\n        if precision == -1 {\n            return format!(\"{},{}\", x as i64, y as i64);\n        }\n\n        if precision < -1 {\n            let p = (-precision - 1) as u32;\n            let factor = 10_f64.powi(p as i32);\n            return format!(\n                \"{},{}\",\n                ((x / factor).round() * factor) as i64,\n                ((y / factor).round() * factor) as i64\n            );\n        }\n\n        let minzero = format!(\"-{:.prec$}\", 0.0, prec = precision as usize);\n        if format!(\"{:.prec$}\", x, prec = precision as usize) == minzero {\n            x = 0.0;\n        }\n        if format!(\"{",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.format_number",
      "implementations": {
        "python": {
          "sig": "format_number(number, precision=None)",
          "code": "def format_number(self, number, precision=None):\n\n        \"\"\"Format a number as a string.\"\"\"\n        if not precision:\n            precision = self.precision\n\n        if precision == 0:\n            raise ValueError(\"Precision cannot be zero.\")\n\n        if precision == -1:\n            return f\"{int(round(number))}\"\n\n        if precision < -1:\n            precision = -precision - 1\n            factor = 10**precision\n            return f\"{int(round(number / factor) * factor)}\"\n\n        return f\"{number:.{precision}f}\"\n\n    def precision_from_tolerance(self, tol=None):\n        \"\"\"Compute the precision from a given tolerance.\"\"\"",
          "file": "tolerance.py"
        },
        "cpp": {
          "sig": "std::string format_number(double number, int precision)",
          "code": "std::string Tolerance::format_number(double number, int precision) const {\n    int prec = (precision != -999) ? precision : this->precision();\n    \n    if (prec == 0) {\n        throw std::invalid_argument(\"Precision cannot be zero.\");\n    }",
          "file": "tolerance.cpp"
        },
        "rust": {
          "sig": "format_number(number: f64, precision: i32) -> String",
          "code": "pub fn format_number(&self, number: f64, precision: i32) -> String {\n        let precision = if precision == -999 { self.precision() } else { precision };\n\n        if precision == -1 {\n            return format!(\"{}\", number.round() as i64);\n        }\n\n        if precision < -1 {\n            let p = (-precision - 1) as u32;\n            let factor = 10_f64.powi(p as i32);\n            return format!(\"{}\", ((number / factor).round() * factor) as i64);\n        }\n\n        format!(\"{:.prec$}\", number, prec = precision as usize)\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.precision_from_tolerance",
      "implementations": {
        "python": {
          "sig": "precision_from_tolerance(tol=None)",
          "code": "def precision_from_tolerance(self, tol=None):\n\n        \"\"\"Compute the precision from a given tolerance.\"\"\"\n        tol = tol or self.absolute\n        if tol < 1:\n            import decimal\n\n            return abs(int(decimal.Decimal(str(tol)).as_tuple().exponent))\n        raise NotImplementedError\n\n    def __repr__(self):\n        return f\"Tolerance(unit='{self.unit}', absolute={self.absolute}, relative={self.relative}, angular={self.angular}, approximation={self.approximation}, precision={self.precision}, lineardeflection={self.lineardeflection}, angulardeflection={self.angulardeflection})\"\n\n\ndef is_finite(x):\n    \"\"\"Test if a number is finite (equivalent to C++ IS_FINITE function).\"\"\"\n    return math.isfinite(x)\n\n\n# Global tolerance instance\nTOLERANCE = Tolerance()",
          "file": "tolerance.py"
        },
        "cpp": {
          "sig": "int precision_from_tolerance(double tol)",
          "code": "int Tolerance::precision_from_tolerance(double tol) const {\n    double tolerance_val = (tol >= 0) ? tol : absolute();\n    if (tolerance_val < 1.0) {\n        std::ostringstream oss;\n        oss << std::scientific << tolerance_val;\n        std::string s = oss.str();\n        size_t pos = s.find(\"e-\");\n        if (pos != std::string::npos) {\n            return std::stoi(s.substr(pos + 2));\n        }",
          "file": "tolerance.cpp"
        },
        "rust": {
          "sig": "precision_from_tolerance(tol: Option<f64>) -> i32",
          "code": "pub fn precision_from_tolerance(&self, tol: Option<f64>) -> i32 {\n        let tol = tol.unwrap_or_else(|| self.absolute());\n        if tol < 1.0 {\n            let s = format!(\"{tol:e}\");\n            if let Some(exp_pos) = s.find(\"e-\") {\n                if let Ok(exp) = s[exp_pos + 2..].parse::<i32>() {\n                    return exp;\n                }\n            }\n        }\n        0\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.__repr__",
      "implementations": {
        "python": {
          "sig": "__repr__()",
          "code": "def __repr__(self):\n\n        return f\"Tolerance(unit='{self.unit}', absolute={self.absolute}, relative={self.relative}, angular={self.angular}, approximation={self.approximation}, precision={self.precision}, lineardeflection={self.lineardeflection}, angulardeflection={self.angulardeflection})\"\n\n\ndef is_finite(x):\n    \"\"\"Test if a number is finite (equivalent to C++ IS_FINITE function).\"\"\"\n    return math.isfinite(x)\n\n\n# Global tolerance instance\nTOLERANCE = Tolerance()",
          "file": "tolerance.py"
        }
      }
    },
    {
      "name": "Tolerance.is_finite",
      "implementations": {
        "python": {
          "sig": "is_finite(x)",
          "code": "def is_finite(x):\n\n    \"\"\"Test if a number is finite (equivalent to C++ IS_FINITE function).\"\"\"\n    return math.isfinite(x)\n\n\n# Global tolerance instance\nTOLERANCE = Tolerance()",
          "file": "tolerance.py"
        },
        "cpp": {
          "sig": "bool is_finite(double x)",
          "code": "bool is_finite(double x);",
          "file": "tolerance.h"
        }
      }
    },
    {
      "name": "Vector.__init__",
      "implementations": {
        "python": {
          "sig": "__init__(x=0.0, y=0.0, z=0.0)",
          "code": "def __init__(self, x=0.0, y=0.0, z=0.0):\n\n        self.guid = str(uuid.uuid4())\n        self.name = \"my_vector\"\n        self._x = x\n        self._y = y\n        self._z = z\n        self._magnitude = 0.0\n        self._has_magnitude = False\n\n    def __deepcopy__(self, memo):\n        cls = self.__class__\n        result = cls.__new__(cls)\n        memo[id(self)] = result\n\n        # New guid\n        result.guid = str(uuid.uuid4())\n\n        # Copy remaining fields\n        result.name = self.name\n        result._x = self._x",
          "file": "vector.py"
        }
      }
    },
    {
      "name": "Vector.__deepcopy__",
      "implementations": {
        "python": {
          "sig": "__deepcopy__(memo)",
          "code": "def __deepcopy__(self, memo):\n\n        cls = self.__class__\n        result = cls.__new__(cls)\n        memo[id(self)] = result\n\n        # New guid\n        result.guid = str(uuid.uuid4())\n\n        # Copy remaining fields\n        result.name = self.name\n        result._x = self._x\n        result._y = self._y\n        result._z = self._z\n        result._magnitude = self._magnitude\n        result._has_magnitude = self._has_magnitude\n        return result\n\n    def duplicate(self):\n        \"\"\"Create a deep copy of this vector with a new GUID.",
          "file": "vector.py"
        }
      }
    },
    {
      "name": "Vector.duplicate",
      "implementations": {
        "python": {
          "sig": "duplicate()",
          "code": "def duplicate(self):\n\n        \"\"\"Create a deep copy of this vector with a new GUID.\n\n        Returns\n        -------\n        :class:`Vector`\n            A new Vector with identical values but a different GUID.\n\n        \"\"\"\n        import copy\n        return copy.deepcopy(self)\n\n    def __str__(self):\n        return f\"Vector({self[0]}, {self[1]}, {self[2]})\"\n\n    def __repr__(self):\n        return f\"Vector({self.guid}, {self.name}, {self[0]}, {self[1]}, {self[2]})\"\n\n    def str(self):\n        \"\"\"Simple string form: just coordinates formatted to 6 decimals.\"\"\"",
          "file": "vector.py"
        },
        "rust": {
          "sig": "duplicate() -> Self",
          "code": "pub fn duplicate(&self) -> Self {\n        let mut copy = self.clone();\n        copy.guid = Uuid::new_v4().to_string();\n        copy\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.__str__",
      "implementations": {
        "python": {
          "sig": "__str__()",
          "code": "def __str__(self):\n\n        return f\"Vector({self[0]}, {self[1]}, {self[2]})\"\n\n    def __repr__(self):\n        return f\"Vector({self.guid}, {self.name}, {self[0]}, {self[1]}, {self[2]})\"\n\n    def str(self):\n        \"\"\"Simple string form: just coordinates formatted to 6 decimals.\"\"\"\n        from .tolerance import Tolerance\n        return f\"{round(self[0], Tolerance.ROUNDING):.6f}, {round(self[1], Tolerance.ROUNDING):.6f}, {round(self[2], Tolerance.ROUNDING):.6f}\"\n\n    def repr(self):\n        \"\"\"Detailed representation with name, coordinates, and magnitude.\"\"\"\n        from .tolerance import Tolerance\n        mag = self.magnitude()\n        return f\"Vector({self.name}, {round(self[0], Tolerance.ROUNDING):.6f}, {round(self[1], Tolerance.ROUNDING):.6f}, {round(self[2], Tolerance.ROUNDING):.6f}, {round(mag, Tolerance.ROUNDING):.6f})\"\n\n    def __eq__(self, other):\n        return (\n            self.name == other.name",
          "file": "vector.py"
        }
      }
    },
    {
      "name": "Vector.__repr__",
      "implementations": {
        "python": {
          "sig": "__repr__()",
          "code": "def __repr__(self):\n\n        return f\"Vector({self.guid}, {self.name}, {self[0]}, {self[1]}, {self[2]})\"\n\n    def str(self):\n        \"\"\"Simple string form: just coordinates formatted to 6 decimals.\"\"\"\n        from .tolerance import Tolerance\n        return f\"{round(self[0], Tolerance.ROUNDING):.6f}, {round(self[1], Tolerance.ROUNDING):.6f}, {round(self[2], Tolerance.ROUNDING):.6f}\"\n\n    def repr(self):\n        \"\"\"Detailed representation with name, coordinates, and magnitude.\"\"\"\n        from .tolerance import Tolerance\n        mag = self.magnitude()\n        return f\"Vector({self.name}, {round(self[0], Tolerance.ROUNDING):.6f}, {round(self[1], Tolerance.ROUNDING):.6f}, {round(self[2], Tolerance.ROUNDING):.6f}, {round(mag, Tolerance.ROUNDING):.6f})\"\n\n    def __eq__(self, other):\n        return (\n            self.name == other.name\n            and round(self[0], 6) == round(other[0], 6)\n            and round(self[1], 6) == round(other[1], 6)\n            and round(self[2], 6) == round(other[2], 6)",
          "file": "vector.py"
        }
      }
    },
    {
      "name": "Vector.str",
      "implementations": {
        "python": {
          "sig": "str()",
          "code": "def str(self):\n\n        \"\"\"Simple string form: just coordinates formatted to 6 decimals.\"\"\"\n        from .tolerance import Tolerance\n        return f\"{round(self[0], Tolerance.ROUNDING):.6f}, {round(self[1], Tolerance.ROUNDING):.6f}, {round(self[2], Tolerance.ROUNDING):.6f}\"\n\n    def repr(self):\n        \"\"\"Detailed representation with name, coordinates, and magnitude.\"\"\"\n        from .tolerance import Tolerance\n        mag = self.magnitude()\n        return f\"Vector({self.name}, {round(self[0], Tolerance.ROUNDING):.6f}, {round(self[1], Tolerance.ROUNDING):.6f}, {round(self[2], Tolerance.ROUNDING):.6f}, {round(mag, Tolerance.ROUNDING):.6f})\"\n\n    def __eq__(self, other):\n        return (\n            self.name == other.name\n            and round(self[0], 6) == round(other[0], 6)\n            and round(self[1], 6) == round(other[1], 6)\n            and round(self[2], 6) == round(other[2], 6)\n        )\n\n    def __ne__(self, other):",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "std::string str()",
          "code": "std::string Vector::str() const {\n  int prec = static_cast<int>(Tolerance::ROUNDING);\n  return fmt::format(\n      \"{}",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "str() -> String",
          "code": "pub fn str(&self) -> String {\n        use crate::tolerance::TOLERANCE;\n        let prec = crate::tolerance::Tolerance::ROUNDING;\n        format!(\n            \"{}, {}, {}\",\n            TOLERANCE.format_number(self._x, prec),\n            TOLERANCE.format_number(self._y, prec),\n            TOLERANCE.format_number(self._z, prec),\n        )\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.repr",
      "implementations": {
        "python": {
          "sig": "repr()",
          "code": "def repr(self):\n\n        \"\"\"Detailed representation with name, coordinates, and magnitude.\"\"\"\n        from .tolerance import Tolerance\n        mag = self.magnitude()\n        return f\"Vector({self.name}, {round(self[0], Tolerance.ROUNDING):.6f}, {round(self[1], Tolerance.ROUNDING):.6f}, {round(self[2], Tolerance.ROUNDING):.6f}, {round(mag, Tolerance.ROUNDING):.6f})\"\n\n    def __eq__(self, other):\n        return (\n            self.name == other.name\n            and round(self[0], 6) == round(other[0], 6)\n            and round(self[1], 6) == round(other[1], 6)\n            and round(self[2], 6) == round(other[2], 6)\n        )\n\n    def __ne__(self, other):\n        return not self == other\n\n    ###########################################################################################\n    # No-copy Operators\n    ###########################################################################################",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "std::string repr()",
          "code": "std::string Vector::repr() {\n  int prec = static_cast<int>(Tolerance::ROUNDING);\n  return fmt::format(\n      \"Vector({}",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "repr() -> String",
          "code": "pub fn repr(&mut self) -> String {\n        use crate::tolerance::TOLERANCE;\n        let prec = crate::tolerance::Tolerance::ROUNDING;\n        let mag = self.magnitude(); // compute first to avoid borrow conflict\n        format!(\n            \"Vector({}, {}, {}, {}, {})\",\n            self.name,\n            TOLERANCE.format_number(self._x, prec),\n            TOLERANCE.format_number(self._y, prec),\n            TOLERANCE.format_number(self._z, prec),\n            TOLERANCE.format_number(mag, prec),\n        )\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.__eq__",
      "implementations": {
        "python": {
          "sig": "__eq__(other)",
          "code": "def __eq__(self, other):\n\n        return (\n            self.name == other.name\n            and round(self[0], 6) == round(other[0], 6)\n            and round(self[1], 6) == round(other[1], 6)\n            and round(self[2], 6) == round(other[2], 6)\n        )\n\n    def __ne__(self, other):\n        return not self == other\n\n    ###########################################################################################\n    # No-copy Operators\n    ###########################################################################################\n\n    def __getitem__(self, index):\n        \"\"\"Access coordinate by index (0=x, 1=y, 2=z).\"\"\"\n        if index == 0:\n            return self._x\n        elif index == 1:",
          "file": "vector.py"
        }
      }
    },
    {
      "name": "Vector.__ne__",
      "implementations": {
        "python": {
          "sig": "__ne__(other)",
          "code": "def __ne__(self, other):\n\n        return not self == other\n\n    ###########################################################################################\n    # No-copy Operators\n    ###########################################################################################\n\n    def __getitem__(self, index):\n        \"\"\"Access coordinate by index (0=x, 1=y, 2=z).\"\"\"\n        if index == 0:\n            return self._x\n        elif index == 1:\n            return self._y\n        elif index == 2:\n            return self._z\n        else:\n            raise IndexError(\"Index out of range\")\n\n    def __setitem__(self, index, value):\n        \"\"\"Set coordinate by index (0=x, 1=y, 2=z). Invalidates length cache.\"\"\"",
          "file": "vector.py"
        }
      }
    },
    {
      "name": "Vector.__getitem__",
      "implementations": {
        "python": {
          "sig": "__getitem__(index)",
          "code": "def __getitem__(self, index):\n\n        \"\"\"Access coordinate by index (0=x, 1=y, 2=z).\"\"\"\n        if index == 0:\n            return self._x\n        elif index == 1:\n            return self._y\n        elif index == 2:\n            return self._z\n        else:\n            raise IndexError(\"Index out of range\")\n\n    def __setitem__(self, index, value):\n        \"\"\"Set coordinate by index (0=x, 1=y, 2=z). Invalidates length cache.\"\"\"\n        if index == 0:\n            self._x = value\n        elif index == 1:\n            self._y = value\n        elif index == 2:\n            self._z = value\n        else:",
          "file": "vector.py"
        }
      }
    },
    {
      "name": "Vector.__setitem__",
      "implementations": {
        "python": {
          "sig": "__setitem__(index, value)",
          "code": "def __setitem__(self, index, value):\n\n        \"\"\"Set coordinate by index (0=x, 1=y, 2=z). Invalidates length cache.\"\"\"\n        if index == 0:\n            self._x = value\n        elif index == 1:\n            self._y = value\n        elif index == 2:\n            self._z = value\n        else:\n            raise IndexError(\"Index out of range\")\n        self._has_magnitude = False\n\n    def __imul__(self, other):\n        self._x *= other\n        self._y *= other\n        self._z *= other\n        self._has_magnitude = False\n        return self\n\n    def __itruediv__(self, other):",
          "file": "vector.py"
        }
      }
    },
    {
      "name": "Vector.__imul__",
      "implementations": {
        "python": {
          "sig": "__imul__(other)",
          "code": "def __imul__(self, other):\n\n        self._x *= other\n        self._y *= other\n        self._z *= other\n        self._has_magnitude = False\n        return self\n\n    def __itruediv__(self, other):\n        self._x /= other\n        self._y /= other\n        self._z /= other\n        self._has_magnitude = False\n        return self\n\n    def __iadd__(self, other):\n        self._x += other._x\n        self._y += other._y\n        self._z += other._z\n        self._has_magnitude = False\n        return self",
          "file": "vector.py"
        }
      }
    },
    {
      "name": "Vector.__itruediv__",
      "implementations": {
        "python": {
          "sig": "__itruediv__(other)",
          "code": "def __itruediv__(self, other):\n\n        self._x /= other\n        self._y /= other\n        self._z /= other\n        self._has_magnitude = False\n        return self\n\n    def __iadd__(self, other):\n        self._x += other._x\n        self._y += other._y\n        self._z += other._z\n        self._has_magnitude = False\n        return self\n\n    def __isub__(self, other):\n        self._x -= other._x\n        self._y -= other._y\n        self._z -= other._z\n        self._has_magnitude = False\n        return self",
          "file": "vector.py"
        }
      }
    },
    {
      "name": "Vector.__iadd__",
      "implementations": {
        "python": {
          "sig": "__iadd__(other)",
          "code": "def __iadd__(self, other):\n\n        self._x += other._x\n        self._y += other._y\n        self._z += other._z\n        self._has_magnitude = False\n        return self\n\n    def __isub__(self, other):\n        self._x -= other._x\n        self._y -= other._y\n        self._z -= other._z\n        self._has_magnitude = False\n        return self\n\n    ###########################################################################################\n    # Copy Operators\n    ###########################################################################################\n\n    def __mul__(self, other):\n        return Vector(self._x * other, self._y * other, self._z * other)",
          "file": "vector.py"
        }
      }
    },
    {
      "name": "Vector.__isub__",
      "implementations": {
        "python": {
          "sig": "__isub__(other)",
          "code": "def __isub__(self, other):\n\n        self._x -= other._x\n        self._y -= other._y\n        self._z -= other._z\n        self._has_magnitude = False\n        return self\n\n    ###########################################################################################\n    # Copy Operators\n    ###########################################################################################\n\n    def __mul__(self, other):\n        return Vector(self._x * other, self._y * other, self._z * other)\n\n    def __truediv__(self, other):\n        return Vector(self._x / other, self._y / other, self._z / other)\n\n    def __add__(self, other):\n        return Vector(self._x + other._x, self._y + other._y, self._z + other._z)",
          "file": "vector.py"
        }
      }
    },
    {
      "name": "Vector.__mul__",
      "implementations": {
        "python": {
          "sig": "__mul__(other)",
          "code": "def __mul__(self, other):\n\n        return Vector(self._x * other, self._y * other, self._z * other)\n\n    def __truediv__(self, other):\n        return Vector(self._x / other, self._y / other, self._z / other)\n\n    def __add__(self, other):\n        return Vector(self._x + other._x, self._y + other._y, self._z + other._z)\n\n    def __sub__(self, other):\n        return Vector(self._x - other._x, self._y - other._y, self._z - other._z)\n\n    ###########################################################################################\n    # Static Methods\n    ###########################################################################################\n\n    @staticmethod\n    def zero():\n        \"\"\"Get a zero vector (0, 0, 0).",
          "file": "vector.py"
        }
      }
    },
    {
      "name": "Vector.__truediv__",
      "implementations": {
        "python": {
          "sig": "__truediv__(other)",
          "code": "def __truediv__(self, other):\n\n        return Vector(self._x / other, self._y / other, self._z / other)\n\n    def __add__(self, other):\n        return Vector(self._x + other._x, self._y + other._y, self._z + other._z)\n\n    def __sub__(self, other):\n        return Vector(self._x - other._x, self._y - other._y, self._z - other._z)\n\n    ###########################################################################################\n    # Static Methods\n    ###########################################################################################\n\n    @staticmethod\n    def zero():\n        \"\"\"Get a zero vector (0, 0, 0).\n\n        Returns\n        -------\n        :class:`Vector`",
          "file": "vector.py"
        }
      }
    },
    {
      "name": "Vector.__add__",
      "implementations": {
        "python": {
          "sig": "__add__(other)",
          "code": "def __add__(self, other):\n\n        return Vector(self._x + other._x, self._y + other._y, self._z + other._z)\n\n    def __sub__(self, other):\n        return Vector(self._x - other._x, self._y - other._y, self._z - other._z)\n\n    ###########################################################################################\n    # Static Methods\n    ###########################################################################################\n\n    @staticmethod\n    def zero():\n        \"\"\"Get a zero vector (0, 0, 0).\n\n        Returns\n        -------\n        :class:`Vector`\n            Zero vector (0, 0, 0).\n\n        \"\"\"",
          "file": "vector.py"
        }
      }
    },
    {
      "name": "Vector.__sub__",
      "implementations": {
        "python": {
          "sig": "__sub__(other)",
          "code": "def __sub__(self, other):\n\n        return Vector(self._x - other._x, self._y - other._y, self._z - other._z)\n\n    ###########################################################################################\n    # Static Methods\n    ###########################################################################################\n\n    @staticmethod\n    def zero():\n        \"\"\"Get a zero vector (0, 0, 0).\n\n        Returns\n        -------\n        :class:`Vector`\n            Zero vector (0, 0, 0).\n\n        \"\"\"\n        return Vector(0.0, 0.0, 0.0)\n\n    @staticmethod",
          "file": "vector.py"
        }
      }
    },
    {
      "name": "Vector.zero",
      "implementations": {
        "python": {
          "sig": "zero()",
          "code": "def zero():\n\n        \"\"\"Get a zero vector (0, 0, 0).\n\n        Returns\n        -------\n        :class:`Vector`\n            Zero vector (0, 0, 0).\n\n        \"\"\"\n        return Vector(0.0, 0.0, 0.0)\n\n    @staticmethod\n    def x_axis():\n        \"\"\"Get unit vector along the x-axis.\n\n        Returns\n        -------\n        :class:`Vector`\n            Unit vector (1, 0, 0).",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "return zero()",
          "code": "return Vector::zero();\n  }",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "zero() -> Self",
          "code": "pub fn zero() -> Self {\n        Self::new(0.0, 0.0, 0.0)\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.x_axis",
      "implementations": {
        "python": {
          "sig": "x_axis()",
          "code": "def x_axis():\n\n        \"\"\"Get unit vector along the x-axis.\n\n        Returns\n        -------\n        :class:`Vector`\n            Unit vector (1, 0, 0).\n\n        \"\"\"\n        return Vector(1.0, 0.0, 0.0)\n\n    @staticmethod\n    def y_axis():\n        \"\"\"Get unit vector along the y-axis.\n\n        Returns\n        -------\n        :class:`Vector`\n            Unit vector (0, 1, 0).",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "Vector x_axis()",
          "code": "Vector Vector::x_axis() { return Vector(1.0, 0.0, 0.0); }",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "x_axis() -> Self",
          "code": "pub fn x_axis() -> Self {\n        Self::new(1.0, 0.0, 0.0)\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.y_axis",
      "implementations": {
        "python": {
          "sig": "y_axis()",
          "code": "def y_axis():\n\n        \"\"\"Get unit vector along the y-axis.\n\n        Returns\n        -------\n        :class:`Vector`\n            Unit vector (0, 1, 0).\n\n        \"\"\"\n        return Vector(0.0, 1.0, 0.0)\n\n    @staticmethod\n    def z_axis():\n        \"\"\"Get unit vector along the z-axis.\n\n        Returns\n        -------\n        :class:`Vector`\n            Unit vector (0, 0, 1).",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "Vector y_axis()",
          "code": "Vector Vector::y_axis() { return Vector(0.0, 1.0, 0.0); }",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "y_axis() -> Self",
          "code": "pub fn y_axis() -> Self {\n        Self::new(0.0, 1.0, 0.0)\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.z_axis",
      "implementations": {
        "python": {
          "sig": "z_axis()",
          "code": "def z_axis():\n\n        \"\"\"Get unit vector along the z-axis.\n\n        Returns\n        -------\n        :class:`Vector`\n            Unit vector (0, 0, 1).\n\n        \"\"\"\n        return Vector(0.0, 0.0, 1.0)\n\n    @staticmethod\n    def from_points(p0, p1):\n        \"\"\"Vector from p0 to p1 (p1 - p0).\n\n        Parameters\n        ----------\n        p0 : :class:`Point`\n            Start point.\n        p1 : :class:`Point`",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "Vector z_axis()",
          "code": "Vector Vector::z_axis() { return Vector(0.0, 0.0, 1.0); }",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "z_axis() -> Self",
          "code": "pub fn z_axis() -> Self {\n        Self::new(0.0, 0.0, 1.0)\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.from_points",
      "implementations": {
        "python": {
          "sig": "from_points(p0, p1)",
          "code": "def from_points(p0, p1):\n\n        \"\"\"Vector from p0 to p1 (p1 - p0).\n\n        Parameters\n        ----------\n        p0 : :class:`Point`\n            Start point.\n        p1 : :class:`Point`\n            End point.\n\n        Returns\n        -------\n        :class:`Vector`\n            The vector from p0 to p1.\n\n        \"\"\"\n        return Vector(p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2])\n\n    ###########################################################################################\n    # Details",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "Vector from_points(const Point &p0, const Point &p1)",
          "code": "Vector Vector::from_points(const Point &p0, const Point &p1) {\n  return Vector(p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]);\n}",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "from_points(p0: &Point, p1: &Point) -> Self",
          "code": "pub fn from_points(p0: &Point, p1: &Point) -> Self {\n        Self::new(p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2])\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.scale",
      "implementations": {
        "python": {
          "sig": "scale(factor)",
          "code": "def scale(self, factor):\n\n        \"\"\"Scale the vector by a factor.\n\n        Parameters\n        ----------\n        factor : float\n            The scaling factor to apply to all components.\n\n        \"\"\"\n        self._x *= factor\n        self._y *= factor\n        self._z *= factor\n        self._has_magnitude = False\n\n    def scale_up(self):\n        \"\"\"Scale the vector up by the global scale factor (SCALE).\"\"\"\n        from .tolerance import SCALE\n        self.scale(SCALE)\n\n    def scale_down(self):",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "void scale(double factor)",
          "code": "void Vector::scale(double factor) {\n  (*this)[0] = _x * factor;\n  (*this)[1] = _y * factor;\n  (*this)[2] = _z * factor;\n}",
          "file": "vector.cpp"
        }
      }
    },
    {
      "name": "Vector.scale_up",
      "implementations": {
        "python": {
          "sig": "scale_up()",
          "code": "def scale_up(self):\n\n        \"\"\"Scale the vector up by the global scale factor (SCALE).\"\"\"\n        from .tolerance import SCALE\n        self.scale(SCALE)\n\n    def scale_down(self):\n        \"\"\"Scale the vector down by the global scale factor (1/SCALE).\"\"\"\n        from .tolerance import SCALE\n        self.scale(1.0 / SCALE)\n\n    def reverse(self):\n        \"\"\"Reverse the vector (negate all components).\n\n        Returns\n        -------\n        :class:`Vector`\n            Self.\n\n        \"\"\"\n        self._x = -self._x",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "void scale_up()",
          "code": "void Vector::scale_up() { scale(static_cast<double>(session_cpp::SCALE)); }",
          "file": "vector.cpp"
        }
      }
    },
    {
      "name": "Vector.scale_down",
      "implementations": {
        "python": {
          "sig": "scale_down()",
          "code": "def scale_down(self):\n\n        \"\"\"Scale the vector down by the global scale factor (1/SCALE).\"\"\"\n        from .tolerance import SCALE\n        self.scale(1.0 / SCALE)\n\n    def reverse(self):\n        \"\"\"Reverse the vector (negate all components).\n\n        Returns\n        -------\n        :class:`Vector`\n            Self.\n\n        \"\"\"\n        self._x = -self._x\n        self._y = -self._y\n        self._z = -self._z\n        self._has_magnitude = False\n        return self",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "void scale_down()",
          "code": "void Vector::scale_down() { scale(1.0 / static_cast<double>(session_cpp::SCALE)); }",
          "file": "vector.cpp"
        }
      }
    },
    {
      "name": "Vector.reverse",
      "implementations": {
        "python": {
          "sig": "reverse()",
          "code": "def reverse(self):\n\n        \"\"\"Reverse the vector (negate all components).\n\n        Returns\n        -------\n        :class:`Vector`\n            Self.\n\n        \"\"\"\n        self._x = -self._x\n        self._y = -self._y\n        self._z = -self._z\n        self._has_magnitude = False\n        return self\n\n    def _compute_magnitude(self):\n        \"\"\"Compute the magnitude of the vector without caching.\n\n        Use magnitude() for cached version.\n        \"\"\"",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "void reverse()",
          "code": "void Vector::reverse() {\n  _x = -_x;\n  _y = -_y;\n  _z = -_z;\n  // Length magnitude stays the same, no need to invalidate cache\n}",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "reverse()",
          "code": "pub fn reverse(&mut self) {\n        self._x = -self._x;\n        self._y = -self._y;\n        self._z = -self._z;\n        // Length magnitude stays the same, no need to invalidate cache\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector._compute_magnitude",
      "implementations": {
        "python": {
          "sig": "_compute_magnitude()",
          "code": "def _compute_magnitude(self):\n\n        \"\"\"Compute the magnitude of the vector without caching.\n\n        Use magnitude() for cached version.\n        \"\"\"\n        mag = 0.0\n\n        x = abs(self._x)\n        y = abs(self._y)\n        z = abs(self._z)\n\n        # Handle two zero case:\n        x_zero = x < Tolerance.ZERO_TOLERANCE\n        y_zero = y < Tolerance.ZERO_TOLERANCE\n        z_zero = z < Tolerance.ZERO_TOLERANCE\n\n        if x_zero and y_zero and z_zero:\n            mag = 0.0\n            return mag\n        elif x_zero and y_zero:",
          "file": "vector.py"
        }
      }
    },
    {
      "name": "Vector.magnitude",
      "implementations": {
        "python": {
          "sig": "magnitude()",
          "code": "def magnitude(self):\n\n        \"\"\"Get the cached magnitude of the vector, computing it if necessary.\n\n        Returns\n        -------\n        float\n            The magnitude of the vector.\n        \"\"\"\n        if not self._has_magnitude:\n            self._magnitude = self._compute_magnitude()\n            self._has_magnitude = True\n\n        return self._magnitude\n\n    def magnitude_squared(self):\n        \"\"\"Get the squared magnitude of the vector (avoids sqrt for performance).\n\n        Returns\n        -------\n        float",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "double magnitude()",
          "code": "double Vector::magnitude() const { return cached_magnitude(); }",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "magnitude() -> f64",
          "code": "pub fn magnitude(&self) -> f64 {\n        if !self._has_magnitude.get() {\n            self._magnitude.set(self.compute_magnitude());\n            self._has_magnitude.set(true);\n        }\n        self._magnitude.get()\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.magnitude_squared",
      "implementations": {
        "python": {
          "sig": "magnitude_squared()",
          "code": "def magnitude_squared(self):\n\n        \"\"\"Get the squared magnitude of the vector (avoids sqrt for performance).\n\n        Returns\n        -------\n        float\n            The squared magnitude of the vector.\n        \"\"\"\n        return self._x * self._x + self._y * self._y + self._z * self._z\n\n    def normalize_self(self):\n        \"\"\"Normalize the vector in place (make it unit magnitude).\n\n        Returns\n        -------\n        bool\n            True if successful, False if vector has zero magnitude.\n        \"\"\"\n        d = self.magnitude()\n        if d > 0.0:",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "double magnitude_squared()",
          "code": "double Vector::magnitude_squared() const {\n  return _x * _x + _y * _y + _z * _z;\n}",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "magnitude_squared() -> f64",
          "code": "pub fn magnitude_squared(&self) -> f64 {\n        self._x * self._x + self._y * self._y + self._z * self._z\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.normalize_self",
      "implementations": {
        "python": {
          "sig": "normalize_self()",
          "code": "def normalize_self(self):\n\n        \"\"\"Normalize the vector in place (make it unit magnitude).\n\n        Returns\n        -------\n        bool\n            True if successful, False if vector has zero magnitude.\n        \"\"\"\n        d = self.magnitude()\n        if d > 0.0:\n            self._x /= d\n            self._y /= d\n            self._z /= d\n            self._magnitude = 1.0\n            self._has_magnitude = True\n            return True\n        return False\n\n    def normalize(self):\n        \"\"\"Return a normalized copy of the vector.",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "bool normalize_self()",
          "code": "bool Vector::normalize_self() {\n  double d = compute_magnitude();\n  if (d > 0.0) {\n    (*this)[0] = _x / d;\n    (*this)[1] = _y / d;\n    (*this)[2] = _z / d;\n    return true;\n  }",
          "file": "vector.cpp"
        }
      }
    },
    {
      "name": "Vector.normalize",
      "implementations": {
        "python": {
          "sig": "normalize()",
          "code": "def normalize(self):\n\n        \"\"\"Return a normalized copy of the vector.\n\n        Returns\n        -------\n        Vector\n            A new vector that is the unit vector of this vector.\n        \"\"\"\n        normalized_vector = Vector(self._x, self._y, self._z)\n        normalized_vector.normalize_self()\n        return normalized_vector\n\n    def dot(self, other):\n        \"\"\"Calculate dot product with another vector.\n\n        Parameters\n        ----------\n        other : :class:`Vector`\n            Other vector.",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "Vector normalize()",
          "code": "Vector Vector::normalize() const {\n  Vector result(_x, _y, _z);\n  result.normalize_self();\n  return result;\n}",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "normalize()",
          "code": "pub fn normalize(&mut self) {\n        let len = self.compute_magnitude();\n        if len > Tolerance::ZERO_TOLERANCE {\n            self._x /= len;\n            self._y /= len;\n            self._z /= len;\n            self.invalidate_magnitude_cache();\n        }\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.dot",
      "implementations": {
        "python": {
          "sig": "dot(other)",
          "code": "def dot(self, other):\n\n        \"\"\"Calculate dot product with another vector.\n\n        Parameters\n        ----------\n        other : :class:`Vector`\n            Other vector.\n\n        Returns\n        -------\n        float\n            Dot product value.\n\n        \"\"\"\n        return self._x * other._x + self._y * other._y + self._z * other._z\n\n    def cross(self, other):\n        \"\"\"Calculate cross product with another vector.\n\n        Parameters",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "double dot(const Vector &other)",
          "code": "double Vector::dot(const Vector &other) const {\n  double result = 0.0;\n  for (int i = 0; i < 3; ++i) {\n    result += (*this)[i] * other[i];\n  }",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "dot(other: &Vector) -> f64",
          "code": "pub fn dot(&self, other: &Vector) -> f64 {\n        self._x * other._x + self._y * other._y + self._z * other._z\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.cross",
      "implementations": {
        "python": {
          "sig": "cross(other)",
          "code": "def cross(self, other):\n\n        \"\"\"Calculate cross product with another vector.\n\n        Parameters\n        ----------\n        other : :class:`Vector`\n            Other vector.\n\n        Returns\n        -------\n        :class:`Vector`\n            Cross product vector (orthogonal to inputs).\n\n        \"\"\"\n        x = self._y * other._z - self._z * other._y\n        y = self._z * other._x - self._x * other._z\n        z = self._x * other._y - self._y * other._x\n        return Vector(x, y, z)\n\n    def is_parallel_to(self, v):",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "Vector cross(const Vector &other)",
          "code": "Vector Vector::cross(const Vector &other) const {\n  double cx = (*this)[1] * other[2] - (*this)[2] * other[1];\n  double cy = (*this)[2] * other[0] - (*this)[0] * other[2];\n  double cz = (*this)[0] * other[1] - (*this)[1] * other[0];\n  return Vector(cx, cy, cz);\n}",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "cross(other: &Vector) -> Vector",
          "code": "pub fn cross(&self, other: &Vector) -> Vector {\n        Vector::new(\n            self._y * other._z - self._z * other._y,\n            self._z * other._x - self._x * other._z,\n            self._x * other._y - self._y * other._x,\n        )\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.is_parallel_to",
      "implementations": {
        "python": {
          "sig": "is_parallel_to(v)",
          "code": "def is_parallel_to(self, v):\n\n        \"\"\"Check if this vector is parallel/antiparallel to another.\n\n        Parameters\n        ----------\n        v : :class:`Vector`\n            Other vector.\n\n        Returns\n        -------\n        int\n            1 if parallel, -1 if antiparallel, 0 otherwise.\n\n        \"\"\"\n        ll = self.magnitude() * v.magnitude()\n\n        if ll > 0.0:\n            cos_angle = self.dot(v) / ll\n            angle_in_radians = Tolerance.ANGLE_TOLERANCE_DEGREES * TO_RADIANS\n            cos_tol = math.cos(angle_in_radians)",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "int is_parallel_to(const Vector &other)",
          "code": "int Vector::is_parallel_to(const Vector &other) {\n  double ll = cached_magnitude() * other.cached_magnitude();\n  int result;\n  \n  if (ll > 0.0) {\n    const double cos_angle = ((*this)[0] * other[0] + (*this)[1] * other[1] + (*this)[2] * other[2]) / ll;\n    const double angle_in_radians = static_cast<double>(Tolerance::ANGLE_TOLERANCE_DEGREES) * static_cast<double>(Tolerance::TO_RADIANS);\n    const double cos_tol = std::cos(angle_in_radians);\n    if (cos_angle >= cos_tol)\n      result = 1;  // Parallel\n    else if (cos_angle <= -cos_tol)\n      result = -1; // Antiparallel\n    else\n      result = 0;  // Not parallel\n  }",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "is_parallel_to(other: &Vector) -> i32",
          "code": "pub fn is_parallel_to(&self, other: &Vector) -> i32 {\n        let len_product = self.compute_magnitude() * other.compute_magnitude();\n\n        if len_product <= 0.0 {\n            return 0;\n        }\n\n        let cos_angle = self.dot(other) / len_product;\n        let angle_in_radians = Tolerance::ANGLE_TOLERANCE_DEGREES * TO_RADIANS;\n        let cos_tolerance = angle_in_radians.cos();\n\n        if cos_angle >= cos_tolerance {\n            1 // Parallel\n        } else if cos_angle <= -cos_tolerance {\n            -1 // Antiparallel\n        } else {\n            0 // Not parallel\n        }\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.angle",
      "implementations": {
        "python": {
          "sig": "angle(other, sign_by_cross_product=False, degrees=True, tolerance=1e-12)",
          "code": "def angle(self, other, sign_by_cross_product=False, degrees=True, tolerance=1e-12):\n\n        \"\"\"Angle between this vector and another.\n\n        Parameters\n        ----------\n        other : :class:`Vector`\n            The other vector.\n        sign_by_cross_product : bool, optional\n            If True, sign the angle using the z-component of the cross product.\n        degrees : bool, optional\n            If True (default), return angle in degrees; otherwise radians.\n        tolerance : float, optional\n            Denominator tolerance to treat near-zero lengths as zero.\n\n        Returns\n        -------\n        float\n            The angle value (degrees if `degrees` else radians).\n\n        \"\"\"",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "double angle(const Vector &other, bool sign_by_cross_product, bool degrees,\n                     double tolerance)",
          "code": "double Vector::angle(const Vector &other, bool sign_by_cross_product, bool degrees,\n                     double tolerance) {\n  double dotp = this->dot(other);\n  double len0 = this->cached_magnitude();\n  double len1 = other.cached_magnitude();\n  double denom = len0 * len1;\n  if (denom < tolerance) {\n    return 0.0;\n  }",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "angle(other: &Vector, sign_by_cross_product: bool) -> f64",
          "code": "pub fn angle(&self, other: &Vector, sign_by_cross_product: bool) -> f64 {\n        let dotp = self.dot(other);\n        let len_product = self.compute_magnitude() * other.compute_magnitude();\n\n        if len_product < Tolerance::ZERO_TOLERANCE {\n            return 0.0;\n        }\n\n        let cos_angle = (dotp / len_product).clamp(-1.0, 1.0);\n        let mut angle = cos_angle.acos() * TO_DEGREES;\n\n        if sign_by_cross_product {\n            let cp = self.cross(other);\n            if cp[2] < 0.0 {\n                angle = -angle;\n            }\n        }\n\n        angle\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.projection",
      "implementations": {
        "python": {
          "sig": "projection(projection_vector, tolerance=1e-12)",
          "code": "def projection(self, projection_vector, tolerance=1e-12):\n\n        \"\"\"Project this vector onto another vector.\n\n        Parameters\n        ----------\n        projection_vector : :class:`Vector`\n            Vector to project onto.\n        tolerance : float, optional\n            Treat `projection_vector` length below this as zero.\n\n        Returns\n        -------\n        tuple\n            (projection_vector, projected_length, perpendicular_vector, perpendicular_length),\n            where projection_vector is :class:`Vector`, projected_length is float,\n            perpendicular_vector is :class:`Vector`, and perpendicular_length is float.\n\n        \"\"\"\n        projection_vector_length = projection_vector.magnitude()",
          "file": "vector.py"
        },
        "rust": {
          "sig": "projection(onto: &Vector) -> (Vector, f64, Vector, f64)",
          "code": "pub fn projection(&self, onto: &Vector) -> (Vector, f64, Vector, f64) {\n        let onto_len_sq = onto.magnitude_squared();\n\n        if onto_len_sq < Tolerance::ZERO_TOLERANCE {\n            return (Vector::zero(), 0.0, Vector::zero(), 0.0);\n        }\n\n        // Unit vector along 'onto'\n        let onto_len = onto_len_sq.sqrt();\n        let onto_unit = Vector::new(onto._x / onto_len, onto._y / onto_len, onto._z / onto_len);\n\n        // Scalar projection and projected vector\n        let projected_len = self.dot(&onto_unit);\n        let projection_vec = Vector::new(\n            onto_unit._x * projected_len,\n            onto_unit._y * projected_len,\n            onto_unit._z * projected_len,\n        );\n\n        // Perpendicular component and its length\n        let perp_vec = Vector::new(",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.get_leveled_vector",
      "implementations": {
        "python": {
          "sig": "get_leveled_vector(vertical_height)",
          "code": "def get_leveled_vector(self, vertical_height):\n\n        \"\"\"Get a copy scaled by a vertical height along the Z-axis.\n\n        Parameters\n        ----------\n        vertical_height : float\n            Target vertical height.\n\n        Returns\n        -------\n        :class:`Vector`\n            Scaled copy matching the C++ implementation.\n\n        \"\"\"\n        copy = Vector(self._x, self._y, self._z)\n\n        if copy.normalize_self():\n            reference_vector = Vector(0, 0, 1)\n            angle_deg = copy.angle(\n                reference_vector, sign_by_cross_product=False, degrees=True",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "Vector get_leveled_vector(double &vertical_height)",
          "code": "Vector Vector::get_leveled_vector(double &vertical_height) {\n  Vector copy(_x, _y, _z);\n  if (copy.normalize_self()) {\n    Vector reference(0, 0, 1);\n    double angle_deg = copy.angle(reference, false); // returns degrees (unsigned)\n    double angle_rad = angle_deg * static_cast<double>(Tolerance::TO_RADIANS);\n    double inclined_offset_by_vertical_distance = vertical_height / std::cos(angle_rad);\n    copy *= inclined_offset_by_vertical_distance;\n  }",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "get_leveled_vector(vertical_height: f64) -> Vector",
          "code": "pub fn get_leveled_vector(&self, vertical_height: f64) -> Vector {\n        let mut copy = self.clone();\n        copy.normalize();\n\n        if vertical_height != 0.0 {\n            let reference = Vector::z_axis();\n            let angle_deg = copy.angle(&reference, false); // returns degrees (unsigned)\n            let angle_rad = angle_deg * TO_RADIANS;\n            let inclined_offset_by_vertical_distance = vertical_height / angle_rad.cos();\n            copy *= inclined_offset_by_vertical_distance;\n        }\n\n        copy\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.cosine_law",
      "implementations": {
        "python": {
          "sig": "cosine_law(\n        triangle_edge_length_a,\n        triangle_edge_length_b,\n        angle_in_between_edges,\n        degrees=True,\n    )",
          "code": "def cosine_law(\n        triangle_edge_length_a,\n        triangle_edge_length_b,\n        angle_in_between_edges,\n        degrees=True,\n    ):\n\n        \"\"\"Calculate third side of triangle using the cosine law.\n\n        Parameters\n        ----------\n        triangle_edge_length_a : float\n            Length of side a.\n        triangle_edge_length_b : float\n            Length of side b.\n        angle_in_between_edges : float\n            Angle between a and b.\n        degrees : bool, optional\n            If True, the angle is provided in degrees.\n\n        Returns\n        -------\n        float\n            Length of the third side.\n\n        \"\"\"",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "double cosine_law(double &a, double &b, double &ang_between, bool degrees)",
          "code": "double Vector::cosine_law(double &a, double &b, double &ang_between, bool degrees) {\n  double to_rad = degrees ? static_cast<double>(Tolerance::TO_RADIANS) : 1.0;\n  return std::sqrt(a * a + b * b - 2.0 * a * b * std::cos(ang_between * to_rad));\n}",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "cosine_law(\n        triangle_edge_length_a: f64,\n        triangle_edge_length_b: f64,\n        angle_in_degrees_between_edges: f64,\n        degrees: bool,\n    ) -> f64",
          "code": "pub fn cosine_law(\n        triangle_edge_length_a: f64,\n        triangle_edge_length_b: f64,\n        angle_in_degrees_between_edges: f64,\n        degrees: bool,\n    ) -> f64 {\n        let angle = if degrees {\n            angle_in_degrees_between_edges * TO_RADIANS\n        } else {\n            angle_in_degrees_between_edges\n        };\n\n        (triangle_edge_length_a.powi(2) + triangle_edge_length_b.powi(2)\n            - 2.0 * triangle_edge_length_a * triangle_edge_length_b * angle.cos())\n        .sqrt()\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.sine_law_angle",
      "implementations": {
        "python": {
          "sig": "sine_law_angle(\n        triangle_edge_length_a,\n        angle_in_front_of_a,\n        triangle_edge_length_b,\n        degrees=True,\n    )",
          "code": "def sine_law_angle(\n        triangle_edge_length_a,\n        angle_in_front_of_a,\n        triangle_edge_length_b,\n        degrees=True,\n    ):\n\n        \"\"\"Calculate angle using the sine law.\n\n        Parameters\n        ----------\n        triangle_edge_length_a : float\n            Length of side a.\n        angle_in_front_of_a : float\n            Angle opposite to side a.\n        triangle_edge_length_b : float\n            Length of side b.\n        degrees : bool, optional\n            If True, return angle in degrees.\n\n        Returns\n        -------\n        float\n            Angle opposite to side b (degrees if `degrees`).\n\n        \"\"\"",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "double sine_law_angle(double &a, double &A, double &b, bool degrees)",
          "code": "double Vector::sine_law_angle(double &a, double &A, double &b, bool degrees) {\n  double to_rad = degrees ? static_cast<double>(Tolerance::TO_RADIANS) : 1.0;\n  double to_deg = degrees ? static_cast<double>(Tolerance::TO_DEGREES) : 1.0;\n  return std::asin((b * std::sin(A * to_rad)) / a) * to_deg;\n}",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "sine_law_angle(\n        triangle_edge_length_a: f64,\n        angle_in_degrees_in_front_of_a: f64,\n        triangle_edge_length_b: f64,\n        degrees: bool,\n    ) -> f64",
          "code": "pub fn sine_law_angle(\n        triangle_edge_length_a: f64,\n        angle_in_degrees_in_front_of_a: f64,\n        triangle_edge_length_b: f64,\n        degrees: bool,\n    ) -> f64 {\n        let angle_a = if degrees {\n            angle_in_degrees_in_front_of_a * TO_RADIANS\n        } else {\n            angle_in_degrees_in_front_of_a\n        };\n\n        let sin_b = (triangle_edge_length_b * angle_a.sin()) / triangle_edge_length_a;\n        let angle_b = sin_b.asin();\n\n        if degrees {\n            angle_b * TO_DEGREES\n        } else {\n            angle_b\n        }\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.sine_law_length",
      "implementations": {
        "python": {
          "sig": "sine_law_length(\n        triangle_edge_length_a, angle_in_front_of_a, angle_in_front_of_b, degrees=True\n    )",
          "code": "def sine_law_length(\n        triangle_edge_length_a, angle_in_front_of_a, angle_in_front_of_b, degrees=True\n    ):\n\n        \"\"\"Calculate side length using the sine law.\n\n        Parameters\n        ----------\n        triangle_edge_length_a : float\n            Length of side a.\n        angle_in_front_of_a : float\n            Angle opposite to side a.\n        angle_in_front_of_b : float\n            Angle opposite to side b.\n        degrees : bool, optional\n            If True, angles are provided in degrees.\n\n        Returns\n        -------\n        float\n            Length of side b.\n\n        \"\"\"",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "double sine_law_length(double &a, double &A, double &B, bool degrees)",
          "code": "double Vector::sine_law_length(double &a, double &A, double &B, bool degrees) {\n  double to_rad = degrees ? static_cast<double>(Tolerance::TO_RADIANS) : 1.0;\n  return (a * std::sin(B * to_rad)) / std::sin(A * to_rad);\n}",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "sine_law_length(\n        triangle_edge_length_a: f64,\n        angle_in_degrees_in_front_of_a: f64,\n        angle_in_degrees_in_front_of_b: f64,\n        degrees: bool,\n    ) -> f64",
          "code": "pub fn sine_law_length(\n        triangle_edge_length_a: f64,\n        angle_in_degrees_in_front_of_a: f64,\n        angle_in_degrees_in_front_of_b: f64,\n        degrees: bool,\n    ) -> f64 {\n        let angle_a = if degrees {\n            angle_in_degrees_in_front_of_a * TO_RADIANS\n        } else {\n            angle_in_degrees_in_front_of_a\n        };\n\n        let angle_b = if degrees {\n            angle_in_degrees_in_front_of_b * TO_RADIANS\n        } else {\n            angle_in_degrees_in_front_of_b\n        };\n\n        (triangle_edge_length_a * angle_b.sin()) / angle_a.sin()\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.angle_from_cosine_law",
      "implementations": {
        "python": {
          "sig": "angle_from_cosine_law(\n        triangle_edge_length_a,\n        triangle_edge_length_b,\n        triangle_edge_length_c,\n        degrees=True,\n    )",
          "code": "def angle_from_cosine_law(\n        triangle_edge_length_a,\n        triangle_edge_length_b,\n        triangle_edge_length_c,\n        degrees=True,\n    ):\n\n        \"\"\"Calculate angle opposite to side c using the cosine law.\n\n        Parameters\n        ----------\n        triangle_edge_length_a : float\n            Length of side a (adjacent to angle C).\n        triangle_edge_length_b : float\n            Length of side b (adjacent to angle C).\n        triangle_edge_length_c : float\n            Length of side c (opposite to angle C).\n        degrees : bool, optional\n            If True, return degrees; otherwise radians.\n\n        Returns\n        -------\n        float\n            Angle opposite to side c.\n\n        \"\"\"",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "double angle_from_cosine_law(double a, double b, double c, bool degrees)",
          "code": "double Vector::angle_from_cosine_law(double a, double b, double c, bool degrees) {\n  // cos(C) = (a\u00b2 + b\u00b2 - c\u00b2) / (2ab)\n  double cos_c = (a * a + b * b - c * c) / (2.0 * a * b);\n  double angle_rad = std::acos(cos_c);\n  if (degrees) {\n    return angle_rad * static_cast<double>(Tolerance::TO_DEGREES);\n  }",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "angle_from_cosine_law(\n        triangle_edge_length_a: f64,\n        triangle_edge_length_b: f64,\n        triangle_edge_length_c: f64,\n        degrees: bool,\n    ) -> f64",
          "code": "pub fn angle_from_cosine_law(\n        triangle_edge_length_a: f64,\n        triangle_edge_length_b: f64,\n        triangle_edge_length_c: f64,\n        degrees: bool,\n    ) -> f64 {\n        let a = triangle_edge_length_a;\n        let b = triangle_edge_length_b;\n        let c = triangle_edge_length_c;\n\n        // cos(C) = (a\u00b2 + b\u00b2 - c\u00b2) / (2ab)\n        let cos_c = (a.powi(2) + b.powi(2) - c.powi(2)) / (2.0 * a * b);\n        let angle_rad = cos_c.acos();\n\n        if degrees {\n            angle_rad * TO_DEGREES\n        } else {\n            angle_rad\n        }\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.side_from_sine_law",
      "implementations": {
        "python": {
          "sig": "side_from_sine_law(\n        angle_in_front_of_result_side,\n        angle_in_front_of_known_side,\n        known_side_length,\n        degrees=True,\n    )",
          "code": "def side_from_sine_law(\n        angle_in_front_of_result_side,\n        angle_in_front_of_known_side,\n        known_side_length,\n        degrees=True,\n    ):\n\n        \"\"\"Calculate side length using the sine law.\n\n        Given two angles and the side opposite to one of them, calculates\n        the side opposite to the other angle: a/sin(A) = b/sin(B)\n\n        Parameters\n        ----------\n        angle_in_front_of_result_side : float\n            Angle opposite to the side we want to find.\n        angle_in_front_of_known_side : float\n            Angle opposite to the known side.\n        known_side_length : float\n            Length of the known side.\n        degrees : bool, optional\n            If True, angles are in degrees; otherwise radians.\n\n        Returns\n        -------\n        float",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "double side_from_sine_law(double angle_in_front_of_result_side,\n                                  double angle_in_front_of_known_side,\n                                  double known_side_length,\n                                  bool degrees)",
          "code": "double Vector::side_from_sine_law(double angle_in_front_of_result_side,\n                                  double angle_in_front_of_known_side,\n                                  double known_side_length,\n                                  bool degrees) {\n  double to_rad = degrees ? static_cast<double>(Tolerance::TO_RADIANS) : 1.0;\n  double angle_a = angle_in_front_of_result_side * to_rad;\n  double angle_b = angle_in_front_of_known_side * to_rad;\n  // a = b\u00b7sin(A)/sin(B)\n  return (known_side_length * std::sin(angle_a)) / std::sin(angle_b);\n}",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "side_from_sine_law(\n        angle_in_front_of_result_side: f64,\n        angle_in_front_of_known_side: f64,\n        known_side_length: f64,\n        degrees: bool,\n    ) -> f64",
          "code": "pub fn side_from_sine_law(\n        angle_in_front_of_result_side: f64,\n        angle_in_front_of_known_side: f64,\n        known_side_length: f64,\n        degrees: bool,\n    ) -> f64 {\n        let angle_a = if degrees {\n            angle_in_front_of_result_side * TO_RADIANS\n        } else {\n            angle_in_front_of_result_side\n        };\n\n        let angle_b = if degrees {\n            angle_in_front_of_known_side * TO_RADIANS\n        } else {\n            angle_in_front_of_known_side\n        };\n\n        // a = b\u00b7sin(A)/sin(B)\n        (known_side_length * angle_a.sin()) / angle_b.sin()\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.angle_between_vector_xy_components",
      "implementations": {
        "python": {
          "sig": "angle_between_vector_xy_components(vector, degrees=True)",
          "code": "def angle_between_vector_xy_components(vector, degrees=True):\n\n        \"\"\"Angle of vector's XY projection from +X axis (atan2).\n\n        Parameters\n        ----------\n        vector : :class:`Vector`\n            Input vector.\n        degrees : bool, optional\n            If True, return degrees; otherwise radians.\n\n        Returns\n        -------\n        float\n            Angle in the XY plane.\n\n        \"\"\"\n        to_degrees = TO_DEGREES if degrees else 1.0\n        return math.atan2(vector[1], vector[0]) * to_degrees\n\n    @staticmethod",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "double angle_between_vector_xy_components(Vector &vector)",
          "code": "double Vector::angle_between_vector_xy_components(Vector &vector) {\n  return std::atan2(vector[1], vector[0]) * static_cast<double>(Tolerance::TO_DEGREES);\n}",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "angle_between_vector_xy_components(vector: &Vector) -> f64",
          "code": "pub fn angle_between_vector_xy_components(vector: &Vector) -> f64 {\n        vector._y.atan2(vector._x) * TO_DEGREES\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.sum_of_vectors",
      "implementations": {
        "python": {
          "sig": "sum_of_vectors(vectors)",
          "code": "def sum_of_vectors(vectors):\n\n        \"\"\"Sum a list of vectors (component-wise).\n\n        Parameters\n        ----------\n        vectors : list[:class:`Vector`]\n            Vectors to sum.\n\n        Returns\n        -------\n        :class:`Vector`\n            The component-wise sum.\n\n        \"\"\"\n        x = y = z = 0.0\n        for vector in vectors:\n            x += vector._x\n            y += vector._y\n            z += vector._z\n        return Vector(x, y, z)",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "Vector sum_of_vectors(std::vector<Vector> &vectors)",
          "code": "Vector Vector::sum_of_vectors(std::vector<Vector> &vectors) {\n  double sx = 0, sy = 0, sz = 0;\n  for (const auto &v : vectors) {\n    sx += v[0];\n    sy += v[1];\n    sz += v[2];\n  }",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "sum_of_vectors(vectors: &[Vector]) -> Vector",
          "code": "pub fn sum_of_vectors(vectors: &[Vector]) -> Vector {\n        let mut result = Vector::zero();\n        for vector in vectors {\n            result._x += vector._x;\n            result._y += vector._y;\n            result._z += vector._z;\n        }\n        result\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.average",
      "implementations": {
        "python": {
          "sig": "average(vectors)",
          "code": "def average(vectors):\n\n        \"\"\"Compute the average of a list of vectors.\n\n        Parameters\n        ----------\n        vectors : list[:class:`Vector`]\n            Vectors to average.\n\n        Returns\n        -------\n        :class:`Vector`\n            The component-wise average, or zero vector if empty.\n\n        \"\"\"\n        if not vectors:\n            return Vector.zero()\n        s = Vector.sum_of_vectors(vectors)\n        count = len(vectors)\n        return Vector(s._x / count, s._y / count, s._z / count)",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "Vector average(std::vector<Vector> &vectors)",
          "code": "Vector Vector::average(std::vector<Vector> &vectors) {\n  if (vectors.empty()) {\n    return Vector::zero();\n  }",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "average(vectors: &[Vector]) -> Vector",
          "code": "pub fn average(vectors: &[Vector]) -> Vector {\n        if vectors.is_empty() {\n            return Vector::zero();\n        }\n        let sum = Self::sum_of_vectors(vectors);\n        let count = vectors.len() as f64;\n        Vector::new(sum._x / count, sum._y / count, sum._z / count)\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.is_perpendicular_to",
      "implementations": {
        "python": {
          "sig": "is_perpendicular_to(other)",
          "code": "def is_perpendicular_to(self, other):\n\n        \"\"\"Check if this vector is perpendicular to another.\n\n        Parameters\n        ----------\n        other : :class:`Vector`\n            The other vector.\n\n        Returns\n        -------\n        bool\n            True if perpendicular (dot product within tolerance of zero).\n\n        \"\"\"\n        return abs(self.dot(other)) < Tolerance.ZERO_TOLERANCE\n\n    def is_zero(self):\n        \"\"\"Check if this vector is a zero vector.\n\n        Returns",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "bool is_perpendicular_to(const Vector &other)",
          "code": "bool Vector::is_perpendicular_to(const Vector &other) const {\n  return std::fabs(dot(other)) < static_cast<double>(Tolerance::ZERO_TOLERANCE);\n}",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "is_perpendicular_to(other: &Vector) -> bool",
          "code": "pub fn is_perpendicular_to(&self, other: &Vector) -> bool {\n        self.dot(other).abs() < Tolerance::ZERO_TOLERANCE\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.is_zero",
      "implementations": {
        "python": {
          "sig": "is_zero()",
          "code": "def is_zero(self):\n\n        \"\"\"Check if this vector is a zero vector.\n\n        Returns\n        -------\n        bool\n            True if length is within tolerance of zero.\n\n        \"\"\"\n        return self._compute_magnitude() < Tolerance.ZERO_TOLERANCE\n\n    def coordinate_direction_3angles(self, degrees=True):\n        \"\"\"Compute coordinate direction angles (alpha, beta, gamma).\n\n        Parameters\n        ----------\n        degrees : bool, optional\n            Return angles in degrees if True, radians if False.\n\n        Returns",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "bool is_zero()",
          "code": "bool Vector::is_zero() const {\n  return compute_magnitude() < static_cast<double>(Tolerance::ZERO_TOLERANCE);\n}",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "is_zero() -> bool",
          "code": "pub fn is_zero(&self) -> bool {\n        self.compute_magnitude() < Tolerance::ZERO_TOLERANCE\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.coordinate_direction_3angles",
      "implementations": {
        "python": {
          "sig": "coordinate_direction_3angles(degrees=True)",
          "code": "def coordinate_direction_3angles(self, degrees=True):\n\n        \"\"\"Compute coordinate direction angles (alpha, beta, gamma).\n\n        Parameters\n        ----------\n        degrees : bool, optional\n            Return angles in degrees if True, radians if False.\n\n        Returns\n        -------\n        tuple\n            (alpha, beta, gamma)\n\n        \"\"\"\n        r = math.sqrt(self._x**2 + self._y**2 + self._z**2)\n\n        if r == 0:\n            return (0, 0, 0)\n\n        x_proportion = self._x / r",
          "file": "vector.py"
        },
        "rust": {
          "sig": "coordinate_direction_3angles(degrees: bool) -> [f64; 3]",
          "code": "pub fn coordinate_direction_3angles(&self, degrees: bool) -> [f64; 3] {\n        let length = self.compute_magnitude();\n        if length < Tolerance::ZERO_TOLERANCE {\n            return [0.0, 0.0, 0.0];\n        }\n\n        let cos_alpha = self._x / length;\n        let cos_beta = self._y / length;\n        let cos_gamma = self._z / length;\n\n        let alpha = cos_alpha.acos();\n        let beta = cos_beta.acos();\n        let gamma = cos_gamma.acos();\n\n        if degrees {\n            [alpha * TO_DEGREES, beta * TO_DEGREES, gamma * TO_DEGREES]\n        } else {\n            [alpha, beta, gamma]\n        }\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.coordinate_direction_2angles",
      "implementations": {
        "python": {
          "sig": "coordinate_direction_2angles(degrees=True)",
          "code": "def coordinate_direction_2angles(self, degrees=True):\n\n        \"\"\"Compute coordinate direction angles (phi, theta).\n\n        Parameters\n        ----------\n        degrees : bool, optional\n            Return angles in degrees if True, radians if False.\n\n        Returns\n        -------\n        tuple\n            (phi, theta)\n\n        \"\"\"\n        r = math.sqrt(self._x**2 + self._y**2 + self._z**2)\n\n        if r == 0:\n            return (0, 0)\n\n        phi = math.acos(self._z / r)",
          "file": "vector.py"
        },
        "rust": {
          "sig": "coordinate_direction_2angles(degrees: bool) -> [f64; 2]",
          "code": "pub fn coordinate_direction_2angles(&self, degrees: bool) -> [f64; 2] {\n        let length_xy = (self._x * self._x + self._y * self._y).sqrt();\n        let length = self.compute_magnitude();\n\n        if length < Tolerance::ZERO_TOLERANCE {\n            return [0.0, 0.0];\n        }\n\n        let phi = self._y.atan2(self._x);\n        let theta = length_xy.atan2(self._z);\n\n        if degrees {\n            [phi * TO_DEGREES, theta * TO_DEGREES]\n        } else {\n            [phi, theta]\n        }\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.perpendicular_to",
      "implementations": {
        "python": {
          "sig": "perpendicular_to(v)",
          "code": "def perpendicular_to(self, v):\n\n        \"\"\"Set this vector to be perpendicular to `v`.\n\n        Parameters\n        ----------\n        v : :class:`Vector`\n            Reference vector.\n\n        Returns\n        -------\n        bool\n            True on success, False otherwise.\n\n        \"\"\"\n        k = 2\n\n        if abs(v[1]) > abs(v[0]):\n            if abs(v[2]) > abs(v[1]):\n                # |v.z| > |v.y| > |v.x|\n                i, j, k = 2, 1, 0",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "bool perpendicular_to(Vector &v)",
          "code": "bool Vector::perpendicular_to(Vector &v) {\n  int i, j, k;\n  double a, b;\n  k = 2;\n  if (std::fabs(v[1]) > std::fabs(v[0])) {\n    if (std::fabs(v[2]) > std::fabs(v[1])) {\n      // |v[2]| > |v[1]| > |v[0]|\n      i = 2; j = 1; k = 0; a = v[2]; b = -v[1];\n    }",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "perpendicular_to(v: &Vector) -> bool",
          "code": "pub fn perpendicular_to(&mut self, v: &Vector) -> bool {\n        // Ported from Python implementation to ensure identical behavior\n        let i: usize;\n        let j: usize;\n        let k: usize;\n        let a: f64;\n        let b: f64;\n\n        if v[1].abs() > v[0].abs() {\n            if v[2].abs() > v[1].abs() {\n                // |v.z| > |v.y| > |v.x|\n                i = 2;\n                j = 1;\n                k = 0;\n                a = v[2];\n                b = -v[1];\n            } else if v[2].abs() >= v[0].abs() {\n                // |v.y| >= |v.z| >= |v.x|\n                i = 1;\n                j = 2;\n                k = 0;\n                a = v[1];\n                b = -v[2];\n            } else {\n                // |v.y| > |v.x| > |v.z|\n                i = 1;\n                j",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.__jsondump__",
      "implementations": {
        "python": {
          "sig": "__jsondump__()",
          "code": "def __jsondump__(self):\n\n        \"\"\"Serialize to polymorphic JSON format with type field.\"\"\"\n        # Alphabetical order to match Rust's serde_json\n        return {\n            \"guid\": self.guid,\n            \"name\": self.name,\n            \"type\": f\"{self.__class__.__name__}\",\n            \"x\": self[0],\n            \"y\": self[1],\n            \"z\": self[2],\n        }\n\n    @classmethod\n    def __jsonload__(cls, data, guid=None, name=None):\n        \"\"\"Deserialize from polymorphic JSON format.\"\"\"\n        vec = cls(data[\"x\"], data[\"y\"], data[\"z\"])\n        vec.guid = guid if guid is not None else data.get(\"guid\", vec.guid)\n        vec.name = name if name is not None else data.get(\"name\", vec.name)\n        return vec",
          "file": "vector.py"
        }
      }
    },
    {
      "name": "Vector.__jsonload__",
      "implementations": {
        "python": {
          "sig": "__jsonload__(cls, data, guid=None, name=None)",
          "code": "def __jsonload__(cls, data, guid=None, name=None):\n\n        \"\"\"Deserialize from polymorphic JSON format.\"\"\"\n        vec = cls(data[\"x\"], data[\"y\"], data[\"z\"])\n        vec.guid = guid if guid is not None else data.get(\"guid\", vec.guid)\n        vec.name = name if name is not None else data.get(\"name\", vec.name)\n        return vec\n\n    def json_dump(self, filepath):\n        \"\"\"Write JSON to file.\n\n        Parameters\n        ----------\n        filepath : str or Path\n            Path to the output file.\n\n        \"\"\"\n        import json\n        with open(filepath, 'w') as f:\n            json.dump(self.__jsondump__(), f, indent=2)",
          "file": "vector.py"
        }
      }
    },
    {
      "name": "Vector.json_dump",
      "implementations": {
        "python": {
          "sig": "json_dump(filepath)",
          "code": "def json_dump(self, filepath):\n\n        \"\"\"Write JSON to file.\n\n        Parameters\n        ----------\n        filepath : str or Path\n            Path to the output file.\n\n        \"\"\"\n        import json\n        with open(filepath, 'w') as f:\n            json.dump(self.__jsondump__(), f, indent=2)\n\n    @classmethod\n    def json_load(cls, filepath):\n        \"\"\"Read JSON from file.\n\n        Parameters\n        ----------\n        filepath : str or Path",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "void json_dump(const std::string& filename)",
          "code": "void Vector::json_dump(const std::string& filename) const {\n  std::ofstream ofs(filename);\n  ofs << jsondump().dump(4);\n  ofs.close();\n}",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "json_dump(filepath: &str) -> Result<(), Box<dyn std::error::Error>>",
          "code": "pub fn json_dump(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {\n        self.to_json(filepath)\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.json_load",
      "implementations": {
        "python": {
          "sig": "json_load(cls, filepath)",
          "code": "def json_load(cls, filepath):\n\n        \"\"\"Read JSON from file.\n\n        Parameters\n        ----------\n        filepath : str or Path\n            Path to the JSON file.\n\n        Returns\n        -------\n        :class:`Vector`\n            The deserialized Vector.\n\n        \"\"\"\n        import json\n        with open(filepath, 'r') as f:\n            data = json.load(f)\n        return cls.__jsonload__(data)\n\n    ###########################################################################################",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "Vector json_load(const std::string& filename)",
          "code": "Vector Vector::json_load(const std::string& filename) {\n  std::ifstream ifs(filename);\n  nlohmann::json data = nlohmann::json::parse(ifs);\n  ifs.close();\n  return jsonload(data);\n}",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "json_load(filepath: &str) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn json_load(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {\n        Self::from_json(filepath)\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.to_protobuf",
      "implementations": {
        "python": {
          "sig": "to_protobuf()",
          "code": "def to_protobuf(self):\n\n        \"\"\"Convert to protobuf binary format.\n\n        Returns\n        -------\n        bytes\n            Serialized protobuf data.\n\n        \"\"\"\n        from .proto import vector_pb2\n        \n        proto = vector_pb2.Vector()\n        proto.x = self._x\n        proto.y = self._y\n        proto.z = self._z\n        proto.name = self.name\n        \n        return proto.SerializeToString()\n\n    @classmethod",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "std::string to_protobuf()",
          "code": "std::string Vector::to_protobuf() const {\n  throw std::runtime_error(\"Protobuf support not enabled\");\n}",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "to_protobuf() -> Vec<u8>",
          "code": "pub fn to_protobuf(&self) -> Vec<u8> {\n        use prost::Message;\n        \n        let proto = crate::proto::Vector {\n            x: self._x,\n            y: self._y,\n            z: self._z,\n            name: self.name.clone(),\n        };\n        proto.encode_to_vec()\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.from_protobuf",
      "implementations": {
        "python": {
          "sig": "from_protobuf(cls, data)",
          "code": "def from_protobuf(cls, data):\n\n        \"\"\"Create Vector from protobuf binary data.\n\n        Parameters\n        ----------\n        data : bytes\n            Protobuf-encoded vector data.\n\n        Returns\n        -------\n        :class:`Vector`\n            The deserialized Vector.\n\n        \"\"\"\n        from .proto import vector_pb2\n        \n        proto = vector_pb2.Vector()\n        proto.ParseFromString(data)\n        \n        v = cls(proto.x, proto.y, proto.z)",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "Vector from_protobuf(const std::string& data)",
          "code": "Vector Vector::from_protobuf(const std::string& data) {\n  throw std::runtime_error(\"Protobuf support not enabled\");\n}",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "from_protobuf(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn from_protobuf(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {\n        use prost::Message;\n        \n        let proto = crate::proto::Vector::decode(data)?;\n        \n        let mut v = Self::new(proto.x, proto.y, proto.z);\n        v.name = proto.name;\n        \n        Ok(v)\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.protobuf_dump",
      "implementations": {
        "python": {
          "sig": "protobuf_dump(filepath)",
          "code": "def protobuf_dump(self, filepath):\n\n        \"\"\"Write protobuf to file.\n\n        Parameters\n        ----------\n        filepath : str\n            Path to the output file.\n\n        \"\"\"\n        data = self.to_protobuf()\n        with open(filepath, 'wb') as f:\n            f.write(data)\n\n    @classmethod\n    def protobuf_load(cls, filepath):\n        \"\"\"Read protobuf from file.\n\n        Parameters\n        ----------\n        filepath : str",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "void protobuf_dump(const std::string& filename)",
          "code": "void Vector::protobuf_dump(const std::string& filename) const {\n  throw std::runtime_error(\"Protobuf support not enabled\");\n}",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "protobuf_dump(filepath: &str)",
          "code": "pub fn protobuf_dump(&self, filepath: &str) {\n        let data = self.to_protobuf();\n        std::fs::write(filepath, data).expect(\"Failed to write protobuf file\");\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.protobuf_load",
      "implementations": {
        "python": {
          "sig": "protobuf_load(cls, filepath)",
          "code": "def protobuf_load(cls, filepath):\n\n        \"\"\"Read protobuf from file.\n\n        Parameters\n        ----------\n        filepath : str\n            Path to the protobuf file.\n\n        Returns\n        -------\n        :class:`Vector`\n            The deserialized Vector.\n\n        \"\"\"\n        with open(filepath, 'rb') as f:\n            data = f.read()\n        return cls.from_protobuf(data)",
          "file": "vector.py"
        },
        "cpp": {
          "sig": "Vector protobuf_load(const std::string& filename)",
          "code": "Vector Vector::protobuf_load(const std::string& filename) {\n  throw std::runtime_error(\"Protobuf support not enabled\");\n}",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "protobuf_load(filepath: &str) -> Self",
          "code": "pub fn protobuf_load(filepath: &str) -> Self {\n        let data = std::fs::read(filepath).expect(\"Failed to read protobuf file\");\n        Self::from_protobuf(&data).expect(\"Failed to parse protobuf\")\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Xform.__init__",
      "implementations": {
        "python": {
          "sig": "__init__(m=None)",
          "code": "def __init__(self, m=None):\n\n        self.guid = str(uuid.uuid4())\n        self.name = \"my_xform\"\n        if m is None:\n            self.m = [\n                1.0,\n                0.0,\n                0.0,\n                0.0,\n                0.0,\n                1.0,\n                0.0,\n                0.0,\n                0.0,\n                0.0,\n                1.0,\n                0.0,\n                0.0,\n                0.0,\n                0.0,",
          "file": "xform.py"
        }
      }
    },
    {
      "name": "Xform.__eq__",
      "implementations": {
        "python": {
          "sig": "__eq__(other)",
          "code": "def __eq__(self, other):\n\n        if not isinstance(other, Xform):\n            return False\n        return self.m == other.m\n\n    def __ne__(self, other):\n        return not self.__eq__(other)\n\n    def __str__(self):\n        rows = []\n        for i in range(4):\n            row = [self.m[j * 4 + i] for j in range(4)]\n            rows.append(f\"[{row[0]:.6f}, {row[1]:.6f}, {row[2]:.6f}, {row[3]:.6f}]\")\n        return \"\\n\".join(rows)\n\n    def __repr__(self):\n        return f\"Xform({self.name}, {self.guid[:8]})\"\n\n    def duplicate(self):\n        \"\"\"Create a copy with a new GUID.\"\"\"",
          "file": "xform.py"
        }
      }
    },
    {
      "name": "Xform.__ne__",
      "implementations": {
        "python": {
          "sig": "__ne__(other)",
          "code": "def __ne__(self, other):\n\n        return not self.__eq__(other)\n\n    def __str__(self):\n        rows = []\n        for i in range(4):\n            row = [self.m[j * 4 + i] for j in range(4)]\n            rows.append(f\"[{row[0]:.6f}, {row[1]:.6f}, {row[2]:.6f}, {row[3]:.6f}]\")\n        return \"\\n\".join(rows)\n\n    def __repr__(self):\n        return f\"Xform({self.name}, {self.guid[:8]})\"\n\n    def duplicate(self):\n        \"\"\"Create a copy with a new GUID.\"\"\"\n        copy = Xform(self.m)\n        copy.name = self.name\n        return copy\n\n    @staticmethod",
          "file": "xform.py"
        }
      }
    },
    {
      "name": "Xform.__str__",
      "implementations": {
        "python": {
          "sig": "__str__()",
          "code": "def __str__(self):\n\n        rows = []\n        for i in range(4):\n            row = [self.m[j * 4 + i] for j in range(4)]\n            rows.append(f\"[{row[0]:.6f}, {row[1]:.6f}, {row[2]:.6f}, {row[3]:.6f}]\")\n        return \"\\n\".join(rows)\n\n    def __repr__(self):\n        return f\"Xform({self.name}, {self.guid[:8]})\"\n\n    def duplicate(self):\n        \"\"\"Create a copy with a new GUID.\"\"\"\n        copy = Xform(self.m)\n        copy.name = self.name\n        return copy\n\n    @staticmethod\n    def identity():\n        return Xform()",
          "file": "xform.py"
        }
      }
    },
    {
      "name": "Xform.__repr__",
      "implementations": {
        "python": {
          "sig": "__repr__()",
          "code": "def __repr__(self):\n\n        return f\"Xform({self.name}, {self.guid[:8]})\"\n\n    def duplicate(self):\n        \"\"\"Create a copy with a new GUID.\"\"\"\n        copy = Xform(self.m)\n        copy.name = self.name\n        return copy\n\n    @staticmethod\n    def identity():\n        return Xform()\n\n    @staticmethod\n    def from_matrix(m):\n        return Xform(m)\n\n    @staticmethod\n    def translation(x, y, z):\n        xform = Xform()",
          "file": "xform.py"
        }
      }
    },
    {
      "name": "Xform.duplicate",
      "implementations": {
        "python": {
          "sig": "duplicate()",
          "code": "def duplicate(self):\n\n        \"\"\"Create a copy with a new GUID.\"\"\"\n        copy = Xform(self.m)\n        copy.name = self.name\n        return copy\n\n    @staticmethod\n    def identity():\n        return Xform()\n\n    @staticmethod\n    def from_matrix(m):\n        return Xform(m)\n\n    @staticmethod\n    def translation(x, y, z):\n        xform = Xform()\n        xform.m[12] = x\n        xform.m[13] = y\n        xform.m[14] = z",
          "file": "xform.py"
        },
        "rust": {
          "sig": "duplicate() -> Self",
          "code": "pub fn duplicate(&self) -> Self {\n        let mut copy = Self::from_matrix(self.m);\n        copy.name = self.name.clone();\n        copy\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.identity",
      "implementations": {
        "python": {
          "sig": "identity()",
          "code": "def identity():\n\n        return Xform()\n\n    @staticmethod\n    def from_matrix(m):\n        return Xform(m)\n\n    @staticmethod\n    def translation(x, y, z):\n        xform = Xform()\n        xform.m[12] = x\n        xform.m[13] = y\n        xform.m[14] = z\n        return xform\n\n    @staticmethod\n    def scaling(x, y, z):\n        xform = Xform()\n        xform.m[0] = x\n        xform.m[5] = y",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "Xform identity()",
          "code": "Xform Xform::identity() {\n    return Xform();\n}",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "identity() -> Self",
          "code": "pub fn identity() -> Self {\n        let mut xform = Xform {\n            typ: \"Xform\".to_string(),\n            guid: Uuid::new_v4().to_string(),\n            name: \"my_xform\".to_string(),\n            m: [0.0; 16],\n        };\n        xform.m[0] = 1.0;\n        xform.m[5] = 1.0;\n        xform.m[10] = 1.0;\n        xform.m[15] = 1.0;\n        xform\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.from_matrix",
      "implementations": {
        "python": {
          "sig": "from_matrix(m)",
          "code": "def from_matrix(m):\n\n        return Xform(m)\n\n    @staticmethod\n    def translation(x, y, z):\n        xform = Xform()\n        xform.m[12] = x\n        xform.m[13] = y\n        xform.m[14] = z\n        return xform\n\n    @staticmethod\n    def scaling(x, y, z):\n        xform = Xform()\n        xform.m[0] = x\n        xform.m[5] = y\n        xform.m[10] = z\n        return xform\n\n    @staticmethod",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "Xform from_matrix(const std::array<double, 16>& matrix)",
          "code": "Xform Xform::from_matrix(const std::array<double, 16>& matrix) {\n    return Xform(matrix);\n}",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "from_matrix(matrix: [f64; 16]) -> Self",
          "code": "pub fn from_matrix(matrix: [f64; 16]) -> Self {\n        Xform {\n            typ: \"Xform\".to_string(),\n            guid: Uuid::new_v4().to_string(),\n            name: \"my_xform\".to_string(),\n            m: matrix,\n        }\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.translation",
      "implementations": {
        "python": {
          "sig": "translation(x, y, z)",
          "code": "def translation(x, y, z):\n\n        xform = Xform()\n        xform.m[12] = x\n        xform.m[13] = y\n        xform.m[14] = z\n        return xform\n\n    @staticmethod\n    def scaling(x, y, z):\n        xform = Xform()\n        xform.m[0] = x\n        xform.m[5] = y\n        xform.m[10] = z\n        return xform\n\n    @staticmethod\n    def rotation_x(angle_radians):\n        xform = Xform()\n        cos_angle = math.cos(angle_radians)\n        sin_angle = math.sin(angle_radians)",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "Xform translation(double x, double y, double z)",
          "code": "Xform Xform::translation(double x, double y, double z) {\n    Xform xform;\n    xform.m[12] = x;\n    xform.m[13] = y;\n    xform.m[14] = z;\n    return xform;\n}",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "translation(x: f64, y: f64, z: f64) -> Self",
          "code": "pub fn translation(x: f64, y: f64, z: f64) -> Self {\n        let mut xform = Self::identity();\n        xform.m[12] = x;\n        xform.m[13] = y;\n        xform.m[14] = z;\n        xform\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.scaling",
      "implementations": {
        "python": {
          "sig": "scaling(x, y, z)",
          "code": "def scaling(x, y, z):\n\n        xform = Xform()\n        xform.m[0] = x\n        xform.m[5] = y\n        xform.m[10] = z\n        return xform\n\n    @staticmethod\n    def rotation_x(angle_radians):\n        xform = Xform()\n        cos_angle = math.cos(angle_radians)\n        sin_angle = math.sin(angle_radians)\n        xform.m[5] = cos_angle\n        xform.m[6] = sin_angle\n        xform.m[9] = -sin_angle\n        xform.m[10] = cos_angle\n        return xform\n\n    @staticmethod\n    def rotation_y(angle_radians):",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "Xform scaling(double x, double y, double z)",
          "code": "Xform Xform::scaling(double x, double y, double z) {\n    Xform xform;\n    xform.m[0] = x;\n    xform.m[5] = y;\n    xform.m[10] = z;\n    return xform;\n}",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "scaling(x: f64, y: f64, z: f64) -> Self",
          "code": "pub fn scaling(x: f64, y: f64, z: f64) -> Self {\n        let mut xform = Self::identity();\n        xform.m[0] = x;\n        xform.m[5] = y;\n        xform.m[10] = z;\n        xform\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.rotation_x",
      "implementations": {
        "python": {
          "sig": "rotation_x(angle_radians)",
          "code": "def rotation_x(angle_radians):\n\n        xform = Xform()\n        cos_angle = math.cos(angle_radians)\n        sin_angle = math.sin(angle_radians)\n        xform.m[5] = cos_angle\n        xform.m[6] = sin_angle\n        xform.m[9] = -sin_angle\n        xform.m[10] = cos_angle\n        return xform\n\n    @staticmethod\n    def rotation_y(angle_radians):\n        xform = Xform()\n        cos_angle = math.cos(angle_radians)\n        sin_angle = math.sin(angle_radians)\n        xform.m[0] = cos_angle\n        xform.m[2] = -sin_angle\n        xform.m[8] = sin_angle\n        xform.m[10] = cos_angle\n        return xform",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "Xform rotation_x(double angle_radians)",
          "code": "Xform Xform::rotation_x(double angle_radians) {\n    Xform xform;\n    double cos_angle = std::cos(angle_radians);\n    double sin_angle = std::sin(angle_radians);\n    xform.m[5] = cos_angle;\n    xform.m[6] = sin_angle;\n    xform.m[9] = -sin_angle;\n    xform.m[10] = cos_angle;\n    return xform;\n}",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "rotation_x(angle_radians: f64) -> Self",
          "code": "pub fn rotation_x(angle_radians: f64) -> Self {\n        let mut xform = Self::identity();\n\n        let cos_angle = angle_radians.cos();\n        let sin_angle = angle_radians.sin();\n\n        xform.m[5] = cos_angle;\n        xform.m[6] = sin_angle;\n        xform.m[9] = -sin_angle;\n        xform.m[10] = cos_angle;\n\n        xform\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.rotation_y",
      "implementations": {
        "python": {
          "sig": "rotation_y(angle_radians)",
          "code": "def rotation_y(angle_radians):\n\n        xform = Xform()\n        cos_angle = math.cos(angle_radians)\n        sin_angle = math.sin(angle_radians)\n        xform.m[0] = cos_angle\n        xform.m[2] = -sin_angle\n        xform.m[8] = sin_angle\n        xform.m[10] = cos_angle\n        return xform\n\n    @staticmethod\n    def rotation_z(angle_radians):\n        xform = Xform()\n        cos_angle = math.cos(angle_radians)\n        sin_angle = math.sin(angle_radians)\n        xform.m[0] = cos_angle\n        xform.m[1] = sin_angle\n        xform.m[4] = -sin_angle\n        xform.m[5] = cos_angle\n        return xform",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "Xform rotation_y(double angle_radians)",
          "code": "Xform Xform::rotation_y(double angle_radians) {\n    Xform xform;\n    double cos_angle = std::cos(angle_radians);\n    double sin_angle = std::sin(angle_radians);\n    xform.m[0] = cos_angle;\n    xform.m[2] = -sin_angle;\n    xform.m[8] = sin_angle;\n    xform.m[10] = cos_angle;\n    return xform;\n}",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "rotation_y(angle_radians: f64) -> Self",
          "code": "pub fn rotation_y(angle_radians: f64) -> Self {\n        let mut xform = Self::identity();\n\n        let cos_angle = angle_radians.cos();\n        let sin_angle = angle_radians.sin();\n\n        xform.m[0] = cos_angle;\n        xform.m[2] = -sin_angle;\n        xform.m[8] = sin_angle;\n        xform.m[10] = cos_angle;\n\n        xform\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.rotation_z",
      "implementations": {
        "python": {
          "sig": "rotation_z(angle_radians)",
          "code": "def rotation_z(angle_radians):\n\n        xform = Xform()\n        cos_angle = math.cos(angle_radians)\n        sin_angle = math.sin(angle_radians)\n        xform.m[0] = cos_angle\n        xform.m[1] = sin_angle\n        xform.m[4] = -sin_angle\n        xform.m[5] = cos_angle\n        return xform\n\n    @staticmethod\n    def rotation(axis, angle_radians):\n        xform = Xform()\n        axis = axis.normalize()\n        cos_angle = math.cos(angle_radians)\n        sin_angle = math.sin(angle_radians)\n        one_minus_cos = 1.0 - cos_angle\n        xx = axis[0] * axis[0]\n        xy = axis[0] * axis[1]\n        xz = axis[0] * axis[2]",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "Xform rotation_z(double angle_radians)",
          "code": "Xform Xform::rotation_z(double angle_radians) {\n    Xform xform;\n    double cos_angle = std::cos(angle_radians);\n    double sin_angle = std::sin(angle_radians);\n    xform.m[0] = cos_angle;\n    xform.m[1] = sin_angle;\n    xform.m[4] = -sin_angle;\n    xform.m[5] = cos_angle;\n    return xform;\n}",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "rotation_z(angle_radians: f64) -> Self",
          "code": "pub fn rotation_z(angle_radians: f64) -> Self {\n        let mut xform = Self::identity();\n        let cos_angle = angle_radians.cos();\n        let sin_angle = angle_radians.sin();\n\n        xform.m[0] = cos_angle;\n        xform.m[1] = sin_angle;\n        xform.m[4] = -sin_angle;\n        xform.m[5] = cos_angle;\n\n        xform\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.rotation",
      "implementations": {
        "python": {
          "sig": "rotation(axis, angle_radians)",
          "code": "def rotation(axis, angle_radians):\n\n        xform = Xform()\n        axis = axis.normalize()\n        cos_angle = math.cos(angle_radians)\n        sin_angle = math.sin(angle_radians)\n        one_minus_cos = 1.0 - cos_angle\n        xx = axis[0] * axis[0]\n        xy = axis[0] * axis[1]\n        xz = axis[0] * axis[2]\n        yy = axis[1] * axis[1]\n        yz = axis[1] * axis[2]\n        zz = axis[2] * axis[2]\n        xform.m[0] = cos_angle + xx * one_minus_cos\n        xform.m[1] = xy * one_minus_cos + axis[2] * sin_angle\n        xform.m[2] = xz * one_minus_cos - axis[1] * sin_angle\n        xform.m[4] = xy * one_minus_cos - axis[2] * sin_angle\n        xform.m[5] = cos_angle + yy * one_minus_cos\n        xform.m[6] = yz * one_minus_cos + axis[0] * sin_angle\n        xform.m[8] = xz * one_minus_cos + axis[1] * sin_angle\n        xform.m[9] = yz * one_minus_cos - axis[0] * sin_angle",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "Xform rotation(Vector& axis, double angle_radians)",
          "code": "Xform Xform::rotation(Vector& axis, double angle_radians) {\n    Xform xform;\n    axis.normalize_self();\n    \n    double cos_angle = std::cos(angle_radians);\n    double sin_angle = std::sin(angle_radians);\n    double one_minus_cos = 1.0 - cos_angle;\n\n    double xx = axis[0] * axis[0];\n    double xy = axis[0] * axis[1];\n    double xz = axis[0] * axis[2];\n    double yy = axis[1] * axis[1];\n    double yz = axis[1] * axis[2];\n    double zz = axis[2] * axis[2];\n\n    xform.m[0] = cos_angle + xx * one_minus_cos;\n    xform.m[1] = xy * one_minus_cos + axis[2] * sin_angle;\n    xform.m[2] = xz * one_minus_cos - axis[1] * sin_angle;\n\n    xform.m[4] = xy * one_minus_cos - axis[2] * sin_angle;\n    xform.m[5] = cos_angle + yy * one_minus_cos;\n    xform.m[6] = yz * one_minus_cos + axis[0] * sin_angle;",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "rotation(axis: &Vector, angle_radians: f64) -> Self",
          "code": "pub fn rotation(axis: &Vector, angle_radians: f64) -> Self {\n        let axis = axis.normalized();\n\n        let mut xform = Self::identity();\n        let cos_angle = angle_radians.cos();\n        let sin_angle = angle_radians.sin();\n        let one_minus_cos = 1.0 - cos_angle;\n\n        let xx = axis[0] * axis[0];\n        let xy = axis[0] * axis[1];\n        let xz = axis[0] * axis[2];\n        let yy = axis[1] * axis[1];\n        let yz = axis[1] * axis[2];\n        let zz = axis[2] * axis[2];\n\n        xform.m[0] = cos_angle + xx * one_minus_cos;\n        xform.m[1] = xy * one_minus_cos + axis[2] * sin_angle;\n        xform.m[2] = xz * one_minus_cos - axis[1] * sin_angle;\n\n        xform.m[4] = xy * one_minus_cos - axis[2] * sin_angle;\n        xform.m[5] = cos_angle + yy * one_minus_cos;",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.change_basis",
      "implementations": {
        "python": {
          "sig": "change_basis(\n        origin_1, x_axis_1, y_axis_1, z_axis_1, origin_0, x_axis_0, y_axis_0, z_axis_0\n    )",
          "code": "def change_basis(\n        origin_1, x_axis_1, y_axis_1, z_axis_1, origin_0, x_axis_0, y_axis_0, z_axis_0\n    ):\n\n        a = x_axis_1.dot(y_axis_1)\n        b = x_axis_1.dot(z_axis_1)\n        c = y_axis_1.dot(z_axis_1)\n        r = [\n            [\n                x_axis_1.dot(x_axis_1),\n                a,\n                b,\n                x_axis_1.dot(x_axis_0),\n                x_axis_1.dot(y_axis_0),\n                x_axis_1.dot(z_axis_0),\n            ],\n            [\n                a,\n                y_axis_1.dot(y_axis_1),\n                c,\n                y_axis_1.dot(x_axis_0),\n                y_axis_1.dot(y_axis_0),\n                y_axis_1.dot(z_axis_0),",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "Xform change_basis(Point& origin_1, Vector& x_axis_1, Vector& y_axis_1, Vector& z_axis_1,\n                           Point& origin_0, Vector& x_axis_0, Vector& y_axis_0, Vector& z_axis_0)",
          "code": "Xform Xform::change_basis(Point& origin_1, Vector& x_axis_1, Vector& y_axis_1, Vector& z_axis_1,\n                           Point& origin_0, Vector& x_axis_0, Vector& y_axis_0, Vector& z_axis_0) {\n    double a = x_axis_1.dot(y_axis_1);\n    double b = x_axis_1.dot(z_axis_1);\n    double c = y_axis_1.dot(z_axis_1);\n\n    double r[3][6] = {\n        {x_axis_1.dot(x_axis_1), a, b, x_axis_1.dot(x_axis_0), x_axis_1.dot(y_axis_0), x_axis_1.dot(z_axis_0)}",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "change_basis(\n        origin_1: &Point,\n        x_axis_1: &Vector,\n        y_axis_1: &Vector,\n        z_axis_1: &Vector,\n        origin_0: &Point,\n        x_axis_0: &Vector,\n        y_axis_0: &Vector,\n        z_axis_0: &Vector,\n    ) -> Self",
          "code": "pub fn change_basis(\n        origin_1: &Point,\n        x_axis_1: &Vector,\n        y_axis_1: &Vector,\n        z_axis_1: &Vector,\n        origin_0: &Point,\n        x_axis_0: &Vector,\n        y_axis_0: &Vector,\n        z_axis_0: &Vector,\n    ) -> Self {\n        let a = x_axis_1.dot(y_axis_1);\n        let b = x_axis_1.dot(z_axis_1);\n        let c = y_axis_1.dot(z_axis_1);\n\n        let mut r = [\n            [\n                x_axis_1.dot(x_axis_1),\n                a,\n                b,\n                x_axis_1.dot(x_axis_0),\n                x_axis_1.dot(y_axis_0),\n                x_axis_1.dot(z_axis_0),\n            ],\n            [\n                a,\n                y_axis_1.dot(y_axis_1),\n                c,\n                y_axis_1.dot(x_axis_0),\n                y_axis_1.dot(y_axis_0),",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.plane_to_plane",
      "implementations": {
        "python": {
          "sig": "plane_to_plane(plane_from, plane_to)",
          "code": "def plane_to_plane(plane_from, plane_to):\n\n        \"\"\"Create transformation from one plane to another.\n\n        Parameters\n        ----------\n        plane_from : Plane\n            Source plane.\n        plane_to : Plane\n            Target plane.\n\n        Returns\n        -------\n        :class:`Xform`\n            Transformation matrix.\n        \"\"\"\n        x0 = plane_from.x_axis.normalize()\n        y0 = plane_from.y_axis.normalize()\n        z0 = plane_from.z_axis.normalize()\n        x1 = plane_to.x_axis.normalize()\n        y1 = plane_to.y_axis.normalize()",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "Xform plane_to_plane(const Plane& plane_from, const Plane& plane_to)",
          "code": "Xform Xform::plane_to_plane(const Plane& plane_from, const Plane& plane_to) {\n    Vector x0 = plane_from.x_axis(), y0 = plane_from.y_axis(), z0 = plane_from.z_axis();\n    Vector x1 = plane_to.x_axis(), y1 = plane_to.y_axis(), z1 = plane_to.z_axis();\n    x0.normalize_self(); y0.normalize_self(); z0.normalize_self();\n    x1.normalize_self(); y1.normalize_self(); z1.normalize_self();\n\n    const Point& origin_0 = plane_from.origin();\n    const Point& origin_1 = plane_to.origin();\n\n    Xform t0 = translation(-origin_0[0], -origin_0[1], -origin_0[2]);\n\n    Xform f0;\n    f0.m[0] = x0[0]; f0.m[1] = x0[1]; f0.m[2] = x0[2];\n    f0.m[4] = y0[0]; f0.m[5] = y0[1]; f0.m[6] = y0[2];\n    f0.m[8] = z0[0]; f0.m[9] = z0[1]; f0.m[10] = z0[2];\n\n    Xform f1;\n    f1.m[0] = x1[0]; f1.m[4] = x1[1]; f1.m[8] = x1[",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "plane_to_plane(plane_from: &Plane, plane_to: &Plane) -> Self",
          "code": "pub fn plane_to_plane(plane_from: &Plane, plane_to: &Plane) -> Self {\n        let mut x0 = plane_from.x_axis();\n        let mut y0 = plane_from.y_axis();\n        let mut z0 = plane_from.z_axis();\n        let mut x1 = plane_to.x_axis();\n        let mut y1 = plane_to.y_axis();\n        let mut z1 = plane_to.z_axis();\n        x0.normalize();\n        y0.normalize();\n        z0.normalize();\n        x1.normalize();\n        y1.normalize();\n        z1.normalize();\n\n        let origin_0 = plane_from.origin();\n        let origin_1 = plane_to.origin();\n\n        let t0 = Self::translation(-origin_0[0], -origin_0[1], -origin_0[2]);\n\n        let mut f0 = Self::identity();\n        f0.m[0] = x0[0];\n        f0.m[1] = x0[1];\n        f0.m[2] = x0[2];\n        f0.m[4] = y0[0];\n        f0.m[5] = y0[1];",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.plane_to_xy",
      "implementations": {
        "python": {
          "sig": "plane_to_xy(origin, x_axis, y_axis, z_axis)",
          "code": "def plane_to_xy(origin, x_axis, y_axis, z_axis):\n\n        x = x_axis.normalize()\n        y = y_axis.normalize()\n        z = z_axis.normalize()\n        t = Xform.translation(-origin[0], -origin[1], -origin[2])\n        f = Xform()\n        f.m[0] = x[0]\n        f.m[1] = x[1]\n        f.m[2] = x[2]\n        f.m[4] = y[0]\n        f.m[5] = y[1]\n        f.m[6] = y[2]\n        f.m[8] = z[0]\n        f.m[9] = z[1]\n        f.m[10] = z[2]\n        return f * t\n\n    @staticmethod\n    def xy_to_plane(origin, x_axis, y_axis, z_axis):\n        x = x_axis.normalize()",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "Xform plane_to_xy(Point& origin, Vector& x_axis, Vector& y_axis, Vector& z_axis)",
          "code": "Xform Xform::plane_to_xy(Point& origin, Vector& x_axis, Vector& y_axis, Vector& z_axis) {\n    Vector x = x_axis, y = y_axis, z = z_axis;\n    x.normalize_self(); y.normalize_self(); z.normalize_self();\n\n    Xform t = translation(-origin[0], -origin[1], -origin[2]);\n    Xform f;\n    f.m[0] = x[0]; f.m[1] = x[1]; f.m[2] = x[2];\n    f.m[4] = y[0]; f.m[5] = y[1]; f.m[6] = y[2];\n    f.m[8] = z[0]; f.m[9] = z[1]; f.m[10] = z[2];\n    return f * t;\n}",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "plane_to_xy(origin: &Point, x_axis: &Vector, y_axis: &Vector, z_axis: &Vector) -> Self",
          "code": "pub fn plane_to_xy(origin: &Point, x_axis: &Vector, y_axis: &Vector, z_axis: &Vector) -> Self {\n        let mut x = x_axis.clone();\n        let mut y = y_axis.clone();\n        let mut z = z_axis.clone();\n        x.normalize();\n        y.normalize();\n        z.normalize();\n\n        let t = Self::translation(-origin[0], -origin[1], -origin[2]);\n        let mut f = Self::identity();\n        f.m[0] = x[0];\n        f.m[1] = x[1];\n        f.m[2] = x[2];\n        f.m[4] = y[0];\n        f.m[5] = y[1];\n        f.m[6] = y[2];\n        f.m[8] = z[0];\n        f.m[9] = z[1];\n        f.m[10] = z[2];\n        &f * &t\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.xy_to_plane",
      "implementations": {
        "python": {
          "sig": "xy_to_plane(origin, x_axis, y_axis, z_axis)",
          "code": "def xy_to_plane(origin, x_axis, y_axis, z_axis):\n\n        x = x_axis.normalize()\n        y = y_axis.normalize()\n        z = z_axis.normalize()\n        f = Xform()\n        f.m[0] = x[0]\n        f.m[4] = y[0]\n        f.m[8] = z[0]\n        f.m[1] = x[1]\n        f.m[5] = y[1]\n        f.m[9] = z[1]\n        f.m[2] = x[2]\n        f.m[6] = y[2]\n        f.m[10] = z[2]\n        t = Xform.translation(origin[0], origin[1], origin[2])\n        return t * f\n\n    @staticmethod\n    def scale_xyz(scale_x, scale_y, scale_z):\n        xform = Xform()",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "Xform xy_to_plane(Point& origin, Vector& x_axis, Vector& y_axis, Vector& z_axis)",
          "code": "Xform Xform::xy_to_plane(Point& origin, Vector& x_axis, Vector& y_axis, Vector& z_axis) {\n    Vector x = x_axis, y = y_axis, z = z_axis;\n    x.normalize_self(); y.normalize_self(); z.normalize_self();\n\n    Xform f;\n    f.m[0] = x[0]; f.m[4] = y[0]; f.m[8] = z[0];\n    f.m[1] = x[1]; f.m[5] = y[1]; f.m[9] = z[1];\n    f.m[2] = x[2]; f.m[6] = y[2]; f.m[10] = z[2];\n\n    Xform t = translation(origin[0], origin[1], origin[2]);\n    return t * f;\n}",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "xy_to_plane(origin: &Point, x_axis: &Vector, y_axis: &Vector, z_axis: &Vector) -> Self",
          "code": "pub fn xy_to_plane(origin: &Point, x_axis: &Vector, y_axis: &Vector, z_axis: &Vector) -> Self {\n        let mut x = x_axis.clone();\n        let mut y = y_axis.clone();\n        let mut z = z_axis.clone();\n        x.normalize();\n        y.normalize();\n        z.normalize();\n\n        let mut f = Self::identity();\n        f.m[0] = x[0];\n        f.m[4] = y[0];\n        f.m[8] = z[0];\n        f.m[1] = x[1];\n        f.m[5] = y[1];\n        f.m[9] = z[1];\n        f.m[2] = x[2];\n        f.m[6] = y[2];\n        f.m[10] = z[2];\n\n        let t = Self::translation(origin[0], origin[1], origin[2]);\n        &t * &f\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.scale_xyz",
      "implementations": {
        "python": {
          "sig": "scale_xyz(scale_x, scale_y, scale_z)",
          "code": "def scale_xyz(scale_x, scale_y, scale_z):\n\n        xform = Xform()\n        xform.m[0] = scale_x\n        xform.m[5] = scale_y\n        xform.m[10] = scale_z\n        return xform\n\n    @staticmethod\n    def scale_uniform(origin, scale_value):\n        t0 = Xform.translation(-origin[0], -origin[1], -origin[2])\n        t1 = Xform.scaling(scale_value, scale_value, scale_value)\n        t2 = Xform.translation(origin[0], origin[1], origin[2])\n        return t2 * (t1 * t0)\n\n    @staticmethod\n    def scale_non_uniform(origin, scale_x, scale_y, scale_z):\n        t0 = Xform.translation(-origin[0], -origin[1], -origin[2])\n        t1 = Xform.scale_xyz(scale_x, scale_y, scale_z)\n        t2 = Xform.translation(origin[0], origin[1], origin[2])\n        return t2 * (t1 * t0)",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "Xform scale_xyz(double scale_x, double scale_y, double scale_z)",
          "code": "Xform Xform::scale_xyz(double scale_x, double scale_y, double scale_z) {\n    Xform xform;\n    xform.m[0] = scale_x;\n    xform.m[5] = scale_y;\n    xform.m[10] = scale_z;\n    return xform;\n}",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "scale_xyz(scale_x: f64, scale_y: f64, scale_z: f64) -> Self",
          "code": "pub fn scale_xyz(scale_x: f64, scale_y: f64, scale_z: f64) -> Self {\n        let mut xform = Self::identity();\n        xform.m[0] = scale_x;\n        xform.m[5] = scale_y;\n        xform.m[10] = scale_z;\n        xform\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.scale_uniform",
      "implementations": {
        "python": {
          "sig": "scale_uniform(origin, scale_value)",
          "code": "def scale_uniform(origin, scale_value):\n\n        t0 = Xform.translation(-origin[0], -origin[1], -origin[2])\n        t1 = Xform.scaling(scale_value, scale_value, scale_value)\n        t2 = Xform.translation(origin[0], origin[1], origin[2])\n        return t2 * (t1 * t0)\n\n    @staticmethod\n    def scale_non_uniform(origin, scale_x, scale_y, scale_z):\n        t0 = Xform.translation(-origin[0], -origin[1], -origin[2])\n        t1 = Xform.scale_xyz(scale_x, scale_y, scale_z)\n        t2 = Xform.translation(origin[0], origin[1], origin[2])\n        return t2 * (t1 * t0)\n\n    @staticmethod\n    def axis_rotation(angle, axis):\n        c = math.cos(angle)\n        s = math.sin(angle)\n        ux = axis[0]\n        uy = axis[1]\n        uz = axis[2]",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "Xform scale_uniform(Point& origin, double scale_value)",
          "code": "Xform Xform::scale_uniform(Point& origin, double scale_value) {\n    Xform t0 = translation(-origin[0], -origin[1], -origin[2]);\n    Xform t1 = scaling(scale_value, scale_value, scale_value);\n    Xform t2 = translation(origin[0], origin[1], origin[2]);\n    return t2 * (t1 * t0);\n}",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "scale_uniform(origin: &Point, scale_value: f64) -> Self",
          "code": "pub fn scale_uniform(origin: &Point, scale_value: f64) -> Self {\n        let t0 = Self::translation(-origin[0], -origin[1], -origin[2]);\n        let t1 = Self::scaling(scale_value, scale_value, scale_value);\n        let t2 = Self::translation(origin[0], origin[1], origin[2]);\n        &t2 * &(&t1 * &t0)\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.scale_non_uniform",
      "implementations": {
        "python": {
          "sig": "scale_non_uniform(origin, scale_x, scale_y, scale_z)",
          "code": "def scale_non_uniform(origin, scale_x, scale_y, scale_z):\n\n        t0 = Xform.translation(-origin[0], -origin[1], -origin[2])\n        t1 = Xform.scale_xyz(scale_x, scale_y, scale_z)\n        t2 = Xform.translation(origin[0], origin[1], origin[2])\n        return t2 * (t1 * t0)\n\n    @staticmethod\n    def axis_rotation(angle, axis):\n        c = math.cos(angle)\n        s = math.sin(angle)\n        ux = axis[0]\n        uy = axis[1]\n        uz = axis[2]\n        t = 1.0 - c\n        xform = Xform()\n        xform.m[0] = t * ux * ux + c\n        xform.m[4] = t * ux * uy - uz * s\n        xform.m[8] = t * ux * uz + uy * s\n        xform.m[1] = t * ux * uy + uz * s\n        xform.m[5] = t * uy * uy + c",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "Xform scale_non_uniform(Point& origin, double scale_x, double scale_y, double scale_z)",
          "code": "Xform Xform::scale_non_uniform(Point& origin, double scale_x, double scale_y, double scale_z) {\n    Xform t0 = translation(-origin[0], -origin[1], -origin[2]);\n    Xform t1 = scale_xyz(scale_x, scale_y, scale_z);\n    Xform t2 = translation(origin[0], origin[1], origin[2]);\n    return t2 * (t1 * t0);\n}",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "scale_non_uniform(origin: &Point, scale_x: f64, scale_y: f64, scale_z: f64) -> Self",
          "code": "pub fn scale_non_uniform(origin: &Point, scale_x: f64, scale_y: f64, scale_z: f64) -> Self {\n        let t0 = Self::translation(-origin[0], -origin[1], -origin[2]);\n        let t1 = Self::scale_xyz(scale_x, scale_y, scale_z);\n        let t2 = Self::translation(origin[0], origin[1], origin[2]);\n        &t2 * &(&t1 * &t0)\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.axis_rotation",
      "implementations": {
        "python": {
          "sig": "axis_rotation(angle, axis)",
          "code": "def axis_rotation(angle, axis):\n\n        c = math.cos(angle)\n        s = math.sin(angle)\n        ux = axis[0]\n        uy = axis[1]\n        uz = axis[2]\n        t = 1.0 - c\n        xform = Xform()\n        xform.m[0] = t * ux * ux + c\n        xform.m[4] = t * ux * uy - uz * s\n        xform.m[8] = t * ux * uz + uy * s\n        xform.m[1] = t * ux * uy + uz * s\n        xform.m[5] = t * uy * uy + c\n        xform.m[9] = t * uy * uz - ux * s\n        xform.m[2] = t * ux * uz - uy * s\n        xform.m[6] = t * uy * uz + ux * s\n        xform.m[10] = t * uz * uz + c\n        return xform\n\n    @staticmethod",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "Xform axis_rotation(double angle, Vector& axis)",
          "code": "Xform Xform::axis_rotation(double angle, Vector& axis) {\n    double c = std::cos(angle);\n    double s = std::sin(angle);\n    double ux = axis[0];\n    double uy = axis[1];\n    double uz = axis[2];\n    double t = 1.0 - c;\n\n    Xform xform;\n    xform.m[0] = t * ux * ux + c;\n    xform.m[4] = t * ux * uy - uz * s;\n    xform.m[8] = t * ux * uz + uy * s;\n\n    xform.m[1] = t * ux * uy + uz * s;\n    xform.m[5] = t * uy * uy + c;\n    xform.m[9] = t * uy * uz - ux * s;\n\n    xform.m[2] = t * ux * uz - uy * s;\n    xform.m[6] = t * uy * uz + ux * s;\n    xform.m[10] = t * uz * uz + c;\n\n    return xform;\n}",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "axis_rotation(angle: f64, axis: &Vector) -> Self",
          "code": "pub fn axis_rotation(angle: f64, axis: &Vector) -> Self {\n        let c = angle.cos();\n        let s = angle.sin();\n        let ux = axis[0];\n        let uy = axis[1];\n        let uz = axis[2];\n        let t = 1.0 - c;\n\n        let mut xform = Self::identity();\n        xform.m[0] = t * ux * ux + c;\n        xform.m[4] = t * ux * uy - uz * s;\n        xform.m[8] = t * ux * uz + uy * s;\n\n        xform.m[1] = t * ux * uy + uz * s;\n        xform.m[5] = t * uy * uy + c;\n        xform.m[9] = t * uy * uz - ux * s;\n\n        xform.m[2] = t * ux * uz - uy * s;\n        xform.m[6] = t * uy * uz + ux * s;\n        xform.m[10] = t * uz * uz + c;\n\n        xform\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.look_at_rh",
      "implementations": {
        "python": {
          "sig": "look_at_rh(eye, target, up)",
          "code": "def look_at_rh(eye, target, up):\n\n        from .vector import Vector\n\n        f = (target - eye).normalize()\n        s = f.cross(up.normalize()).normalize()\n        u = s.cross(f)\n        xform = Xform()\n        xform.m[0] = s[0]\n        xform.m[4] = s[1]\n        xform.m[8] = s[2]\n        xform.m[1] = u[0]\n        xform.m[5] = u[1]\n        xform.m[9] = u[2]\n        xform.m[2] = -f[0]\n        xform.m[6] = -f[1]\n        xform.m[10] = -f[2]\n        eye_vec = Vector(eye[0], eye[1], eye[2])\n        xform.m[12] = -s.dot(eye_vec)\n        xform.m[13] = -u.dot(eye_vec)\n        xform.m[14] = f.dot(eye_vec)",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "Xform look_at_rh(const Point& eye, const Point& target, const Vector& up)",
          "code": "Xform Xform::look_at_rh(const Point& eye, const Point& target, const Vector& up) {\n    Vector f = target - eye;\n    f.normalize_self();\n    Vector up_copy = up;\n    up_copy.normalize_self();\n    Vector s = f.cross(up_copy);\n    s.normalize_self();\n    Vector u = s.cross(f);\n    \n    Xform xform;\n    xform.m[0] = s[0];\n    xform.m[4] = s[1];\n    xform.m[8] = s[2];\n    \n    xform.m[1] = u[0];\n    xform.m[5] = u[1];\n    xform.m[9] = u[2];\n    \n    xform.m[2] = -f[0];\n    xform.m[6] = -f[1];\n    xform.m[10] = -f[2];\n    \n    Vector eye_vec(eye[0], eye[1], eye[2]);\n    xform.m[12] = -s.dot(eye_vec);\n    xform.m[13] = -u.dot(eye_vec);\n    xform.m[14] = f.dot(eye_vec);\n    \n    return xform;\n}",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "look_at_rh(eye: &Point, target: &Point, up: &Vector) -> Self",
          "code": "pub fn look_at_rh(eye: &Point, target: &Point, up: &Vector) -> Self {\n        // Use direct coordinate access to avoid cloning Points\n        let fx = target[0] - eye[0];\n        let fy = target[1] - eye[1];\n        let fz = target[2] - eye[2];\n        let f_len = (fx * fx + fy * fy + fz * fz).sqrt();\n        let f = Vector::new(fx / f_len, fy / f_len, fz / f_len);\n        \n        let s = f.cross(&up.normalized()).normalized();\n        let u = s.cross(&f);\n\n        let mut xform = Self::identity();\n\n        xform.m[0] = s[0];\n        xform.m[4] = s[1];\n        xform.m[8] = s[2];\n\n        xform.m[1] = u[0];\n        xform.m[5] = u[1];\n        xform.m[9] = u[2];\n\n        xform.m[2] = -f[0];\n        xform.m[6] = -f[1];\n        xform.m[10] = -f[2];\n\n        xform.m[12] = -s.dot(&Vector::n",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.inverse",
      "implementations": {
        "python": {
          "sig": "inverse() -> Optional[\"Xform\"]",
          "code": "def inverse(self) -> Optional[\"Xform\"]:\n\n        a00 = self.m[0]\n        a01 = self.m[4]\n        a02 = self.m[8]\n        a10 = self.m[1]\n        a11 = self.m[5]\n        a12 = self.m[9]\n        a20 = self.m[2]\n        a21 = self.m[6]\n        a22 = self.m[10]\n        det = (\n            a00 * (a11 * a22 - a12 * a21)\n            - a01 * (a10 * a22 - a12 * a20)\n            + a02 * (a10 * a21 - a11 * a20)\n        )\n        if abs(det) < 1e-12:\n            return None\n        inv_det = 1.0 / det\n        m00 = (a11 * a22 - a12 * a21) * inv_det\n        m01 = (a02 * a21 - a01 * a22) * inv_det",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "std::optional<Xform> inverse()",
          "code": "std::optional<Xform> Xform::inverse() const {\n    double a00 = m[0], a01 = m[4], a02 = m[8];\n    double a10 = m[1], a11 = m[5], a12 = m[9];\n    double a20 = m[2], a21 = m[6], a22 = m[10];\n\n    double det = a00 * (a11 * a22 - a12 * a21) \n              - a01 * (a10 * a22 - a12 * a20)\n              + a02 * (a10 * a21 - a11 * a20);\n    \n    if (std::abs(det) < 1e-12) {\n        return std::nullopt;\n    }",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "inverse() -> Option<Xform>",
          "code": "pub fn inverse(&self) -> Option<Xform> {\n        let a00 = self[(0, 0)];\n        let a01 = self[(0, 1)];\n        let a02 = self[(0, 2)];\n        let a10 = self[(1, 0)];\n        let a11 = self[(1, 1)];\n        let a12 = self[(1, 2)];\n        let a20 = self[(2, 0)];\n        let a21 = self[(2, 1)];\n        let a22 = self[(2, 2)];\n\n        let det = a00 * (a11 * a22 - a12 * a21) - a01 * (a10 * a22 - a12 * a20)\n            + a02 * (a10 * a21 - a11 * a20);\n        if det.abs() < 1e-12 {\n            return None;\n        }\n        let inv_det = 1.0 / det;\n\n        let m00 = (a11 * a22 - a12 * a21) * inv_det;\n        let m01 = (a02 * a21 - a01 * a22) * inv_det;\n        let m02 = (a01 * a12 - a02 * a11) * inv_det;\n        let m10 = (a12 * a20 - a10 * a22) * inv_det;\n        let m11 = (a00 * a22",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.is_identity",
      "implementations": {
        "python": {
          "sig": "is_identity()",
          "code": "def is_identity(self):\n\n        identity = Xform.identity()\n        for i in range(16):\n            if abs(self.m[i] - identity.m[i]) > 1e-10:\n                return False\n        return True\n\n    def transformed_point(self, point):\n        from .point import Point\n\n        x = point[0]\n        y = point[1]\n        z = point[2]\n        w = self.m[3] * x + self.m[7] * y + self.m[11] * z + self.m[15]\n        w_inv = 1.0 / w if abs(w) > 1e-10 else 1.0\n        return Point(\n            (self.m[0] * x + self.m[4] * y + self.m[8] * z + self.m[12]) * w_inv,\n            (self.m[1] * x + self.m[5] * y + self.m[9] * z + self.m[13]) * w_inv,\n            (self.m[2] * x + self.m[6] * y + self.m[10] * z + self.m[14]) * w_inv,\n        )",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "bool is_identity()",
          "code": "bool Xform::is_identity() const {\n    Xform identity;\n    for (int i = 0; i < 16; i++) {\n        if (std::abs(m[i] - identity.m[i]) > 1e-10) {\n            return false;\n        }",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "is_identity() -> bool",
          "code": "pub fn is_identity(&self) -> bool {\n        let identity = Xform::identity();\n        for i in 0..16 {\n            if (self.m[i] - identity.m[i]).abs() > 1e-10 {\n                return false;\n            }\n        }\n        true\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.transformed_point",
      "implementations": {
        "python": {
          "sig": "transformed_point(point)",
          "code": "def transformed_point(self, point):\n\n        from .point import Point\n\n        x = point[0]\n        y = point[1]\n        z = point[2]\n        w = self.m[3] * x + self.m[7] * y + self.m[11] * z + self.m[15]\n        w_inv = 1.0 / w if abs(w) > 1e-10 else 1.0\n        return Point(\n            (self.m[0] * x + self.m[4] * y + self.m[8] * z + self.m[12]) * w_inv,\n            (self.m[1] * x + self.m[5] * y + self.m[9] * z + self.m[13]) * w_inv,\n            (self.m[2] * x + self.m[6] * y + self.m[10] * z + self.m[14]) * w_inv,\n        )\n\n    def transformed_vector(self, vector):\n        x = vector[0]\n        y = vector[1]\n        z = vector[2]\n        return Vector(\n            self.m[0] * x + self.m[4] * y + self.m[8] * z,",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "Point transformed_point(const Point& point)",
          "code": "Point Xform::transformed_point(const Point& point) const {\n    double x = point[0];\n    double y = point[1];\n    double z = point[2];\n    double w = m[3] * x + m[7] * y + m[11] * z + m[15];\n    double w_inv = (std::abs(w) > 1e-10) ? 1.0 / w : 1.0;\n\n    return Point(\n        (m[0] * x + m[4] * y + m[8] * z + m[12]) * w_inv,\n        (m[1] * x + m[5] * y + m[9] * z + m[13]) * w_inv,\n        (m[2] * x + m[6] * y + m[10] * z + m[14]) * w_inv\n    );\n}",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "transformed_point(point: &Point) -> Point",
          "code": "pub fn transformed_point(&self, point: &Point) -> Point {\n        let m = &self.m;\n        let w = m[3] * point[0] + m[7] * point[1] + m[11] * point[2] + m[15];\n        let w_inv = if w.abs() > 1e-10 { 1.0 / w } else { 1.0 };\n\n        Point::new(\n            (m[0] * point[0] + m[4] * point[1] + m[8] * point[2] + m[12]) * w_inv,\n            (m[1] * point[0] + m[5] * point[1] + m[9] * point[2] + m[13]) * w_inv,\n            (m[2] * point[0] + m[6] * point[1] + m[10] * point[2] + m[14]) * w_inv,\n        )\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.transformed_vector",
      "implementations": {
        "python": {
          "sig": "transformed_vector(vector)",
          "code": "def transformed_vector(self, vector):\n\n        x = vector[0]\n        y = vector[1]\n        z = vector[2]\n        return Vector(\n            self.m[0] * x + self.m[4] * y + self.m[8] * z,\n            self.m[1] * x + self.m[5] * y + self.m[9] * z,\n            self.m[2] * x + self.m[6] * y + self.m[10] * z,\n        )\n\n    def transform_point(self, point):\n        x = point[0]\n        y = point[1]\n        z = point[2]\n        w = self.m[3] * x + self.m[7] * y + self.m[11] * z + self.m[15]\n        w_inv = 1.0 / w if abs(w) > 1e-10 else 1.0\n        point[0] = (self.m[0] * x + self.m[4] * y + self.m[8] * z + self.m[12]) * w_inv\n        point[1] = (self.m[1] * x + self.m[5] * y + self.m[9] * z + self.m[13]) * w_inv\n        point[2] = (self.m[2] * x + self.m[6] * y + self.m[10] * z + self.m[14]) * w_inv",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "Vector transformed_vector(const Vector& vector)",
          "code": "Vector Xform::transformed_vector(const Vector& vector) const {\n    double x = vector[0];\n    double y = vector[1];\n    double z = vector[2];\n\n    return Vector(\n        m[0] * x + m[4] * y + m[8] * z,\n        m[1] * x + m[5] * y + m[9] * z,\n        m[2] * x + m[6] * y + m[10] * z\n    );\n}",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "transformed_vector(vector: &Vector) -> Vector",
          "code": "pub fn transformed_vector(&self, vector: &Vector) -> Vector {\n        let m = &self.m;\n\n        Vector::new(\n            m[0] * vector[0] + m[4] * vector[1] + m[8] * vector[2],\n            m[1] * vector[0] + m[5] * vector[1] + m[9] * vector[2],\n            m[2] * vector[0] + m[6] * vector[1] + m[10] * vector[2],\n        )\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.transform_point",
      "implementations": {
        "python": {
          "sig": "transform_point(point)",
          "code": "def transform_point(self, point):\n\n        x = point[0]\n        y = point[1]\n        z = point[2]\n        w = self.m[3] * x + self.m[7] * y + self.m[11] * z + self.m[15]\n        w_inv = 1.0 / w if abs(w) > 1e-10 else 1.0\n        point[0] = (self.m[0] * x + self.m[4] * y + self.m[8] * z + self.m[12]) * w_inv\n        point[1] = (self.m[1] * x + self.m[5] * y + self.m[9] * z + self.m[13]) * w_inv\n        point[2] = (self.m[2] * x + self.m[6] * y + self.m[10] * z + self.m[14]) * w_inv\n\n    def transform_vector(self, vector):\n        x = vector[0]\n        y = vector[1]\n        z = vector[2]\n        vector[0] = self.m[0] * x + self.m[4] * y + self.m[8] * z\n        vector[1] = self.m[1] * x + self.m[5] * y + self.m[9] * z\n        vector[2] = self.m[2] * x + self.m[6] * y + self.m[10] * z\n\n    def __mul__(self, other):\n        result = Xform()",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "void transform_point(Point& point)",
          "code": "void Xform::transform_point(Point& point) const {\n    double x = point[0];\n    double y = point[1];\n    double z = point[2];\n    double w = m[3] * x + m[7] * y + m[11] * z + m[15];\n    double w_inv = (std::abs(w) > 1e-10) ? 1.0 / w : 1.0;\n\n    point[0] = (m[0] * x + m[4] * y + m[8] * z + m[12]) * w_inv;\n    point[1] = (m[1] * x + m[5] * y + m[9] * z + m[13]) * w_inv;\n    point[2] = (m[2] * x + m[6] * y + m[10] * z + m[14]) * w_inv;\n}",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "transform_point(point: &mut Point)",
          "code": "pub fn transform_point(&self, point: &mut Point) {\n        let m = &self.m;\n        let x = point[0];\n        let y = point[1];\n        let z = point[2];\n        let w = m[3] * x + m[7] * y + m[11] * z + m[15];\n        let w_inv = if w.abs() > 1e-10 { 1.0 / w } else { 1.0 };\n\n        point[0] = (m[0] * x + m[4] * y + m[8] * z + m[12]) * w_inv;\n        point[1] = (m[1] * x + m[5] * y + m[9] * z + m[13]) * w_inv;\n        point[2] = (m[2] * x + m[6] * y + m[10] * z + m[14]) * w_inv;\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.transform_vector",
      "implementations": {
        "python": {
          "sig": "transform_vector(vector)",
          "code": "def transform_vector(self, vector):\n\n        x = vector[0]\n        y = vector[1]\n        z = vector[2]\n        vector[0] = self.m[0] * x + self.m[4] * y + self.m[8] * z\n        vector[1] = self.m[1] * x + self.m[5] * y + self.m[9] * z\n        vector[2] = self.m[2] * x + self.m[6] * y + self.m[10] * z\n\n    def __mul__(self, other):\n        result = Xform()\n        result.m = [0.0] * 16\n        for i in range(4):\n            for j in range(4):\n                sum_val = 0.0\n                for k in range(4):\n                    sum_val += self.m[k * 4 + i] * other.m[j * 4 + k]\n                result.m[j * 4 + i] = sum_val\n        return result\n\n    def __imul__(self, other):",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "void transform_vector(Vector& vector)",
          "code": "void Xform::transform_vector(Vector& vector) const {\n    double x = vector[0];\n    double y = vector[1];\n    double z = vector[2];\n\n    vector[0] = m[0] * x + m[4] * y + m[8] * z;\n    vector[1] = m[1] * x + m[5] * y + m[9] * z;\n    vector[2] = m[2] * x + m[6] * y + m[10] * z;\n}",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "transform_vector(vector: &mut Vector)",
          "code": "pub fn transform_vector(&self, vector: &mut Vector) {\n        let m = &self.m;\n        let x = vector[0];\n        let y = vector[1];\n        let z = vector[2];\n\n        vector[0] = m[0] * x + m[4] * y + m[8] * z;\n        vector[1] = m[1] * x + m[5] * y + m[9] * z;\n        vector[2] = m[2] * x + m[6] * y + m[10] * z;\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.__mul__",
      "implementations": {
        "python": {
          "sig": "__mul__(other)",
          "code": "def __mul__(self, other):\n\n        result = Xform()\n        result.m = [0.0] * 16\n        for i in range(4):\n            for j in range(4):\n                sum_val = 0.0\n                for k in range(4):\n                    sum_val += self.m[k * 4 + i] * other.m[j * 4 + k]\n                result.m[j * 4 + i] = sum_val\n        return result\n\n    def __imul__(self, other):\n        temp = self * other\n        self.m = temp.m\n        return self\n\n    def __getitem__(self, idx):\n        if isinstance(idx, tuple) and len(idx) == 2:\n            row, col = idx\n            if not (0 <= row < 4 and 0 <= col < 4):",
          "file": "xform.py"
        }
      }
    },
    {
      "name": "Xform.__imul__",
      "implementations": {
        "python": {
          "sig": "__imul__(other)",
          "code": "def __imul__(self, other):\n\n        temp = self * other\n        self.m = temp.m\n        return self\n\n    def __getitem__(self, idx):\n        if isinstance(idx, tuple) and len(idx) == 2:\n            row, col = idx\n            if not (0 <= row < 4 and 0 <= col < 4):\n                raise IndexError(f\"Index out of bounds: ({row}, {col})\")\n            return self.m[col * 4 + row]\n        raise TypeError(\"Index must be a tuple of (row, col)\")\n\n    def __setitem__(self, idx, value):\n        if isinstance(idx, tuple) and len(idx) == 2:\n            row, col = idx\n            if not (0 <= row < 4 and 0 <= col < 4):\n                raise IndexError(f\"Index out of bounds: ({row}, {col})\")\n            self.m[col * 4 + row] = value\n        else:",
          "file": "xform.py"
        }
      }
    },
    {
      "name": "Xform.__getitem__",
      "implementations": {
        "python": {
          "sig": "__getitem__(idx)",
          "code": "def __getitem__(self, idx):\n\n        if isinstance(idx, tuple) and len(idx) == 2:\n            row, col = idx\n            if not (0 <= row < 4 and 0 <= col < 4):\n                raise IndexError(f\"Index out of bounds: ({row}, {col})\")\n            return self.m[col * 4 + row]\n        raise TypeError(\"Index must be a tuple of (row, col)\")\n\n    def __setitem__(self, idx, value):\n        if isinstance(idx, tuple) and len(idx) == 2:\n            row, col = idx\n            if not (0 <= row < 4 and 0 <= col < 4):\n                raise IndexError(f\"Index out of bounds: ({row}, {col})\")\n            self.m[col * 4 + row] = value\n        else:\n            raise TypeError(\"Index must be a tuple of (row, col)\")\n\n    ###########################################################################################\n    # Polymorphic JSON Serialization\n    ###########################################################################################",
          "file": "xform.py"
        }
      }
    },
    {
      "name": "Xform.__setitem__",
      "implementations": {
        "python": {
          "sig": "__setitem__(idx, value)",
          "code": "def __setitem__(self, idx, value):\n\n        if isinstance(idx, tuple) and len(idx) == 2:\n            row, col = idx\n            if not (0 <= row < 4 and 0 <= col < 4):\n                raise IndexError(f\"Index out of bounds: ({row}, {col})\")\n            self.m[col * 4 + row] = value\n        else:\n            raise TypeError(\"Index must be a tuple of (row, col)\")\n\n    ###########################################################################################\n    # Polymorphic JSON Serialization\n    ###########################################################################################\n\n    def __jsondump__(self):\n        \"\"\"Serialize to polymorphic JSON format with type field.\n\n        Returns\n        -------\n        dict\n            Dictionary with 'type', 'guid', 'name', and object fields.",
          "file": "xform.py"
        }
      }
    },
    {
      "name": "Xform.__jsondump__",
      "implementations": {
        "python": {
          "sig": "__jsondump__()",
          "code": "def __jsondump__(self):\n\n        \"\"\"Serialize to polymorphic JSON format with type field.\n\n        Returns\n        -------\n        dict\n            Dictionary with 'type', 'guid', 'name', and object fields.\n\n        \"\"\"\n        # Alphabetical order to match Rust's serde_json\n        return {\n            \"guid\": self.guid,\n            \"m\": self.m,\n            \"name\": self.name,\n            \"type\": f\"{self.__class__.__name__}\",\n        }\n\n    @classmethod\n    def __jsonload__(cls, data, guid=None, name=None):\n        \"\"\"Deserialize from polymorphic JSON format.",
          "file": "xform.py"
        }
      }
    },
    {
      "name": "Xform.__jsonload__",
      "implementations": {
        "python": {
          "sig": "__jsonload__(cls, data, guid=None, name=None)",
          "code": "def __jsonload__(cls, data, guid=None, name=None):\n\n        \"\"\"Deserialize from polymorphic JSON format.\n\n        Parameters\n        ----------\n        data : dict\n            Dictionary containing xform data.\n        guid : str, optional\n            GUID for the xform.\n        name : str, optional\n            Name for the xform.\n\n        Returns\n        -------\n        :class:`Xform`\n            Reconstructed xform instance.\n\n        \"\"\"\n        xform = cls.from_matrix(data[\"m\"])\n        xform.guid = guid",
          "file": "xform.py"
        }
      }
    },
    {
      "name": "Xform.json_dump",
      "implementations": {
        "python": {
          "sig": "json_dump(filepath)",
          "code": "def json_dump(self, filepath):\n\n        \"\"\"Write JSON to file.\n\n        Parameters\n        ----------\n        filepath : str or Path\n            Path to the output file.\n\n        \"\"\"\n        import json\n        with open(filepath, 'w') as f:\n            json.dump(self.__jsondump__(), f, indent=2)\n\n    @classmethod\n    def json_load(cls, filepath):\n        \"\"\"Read JSON from file.\n\n        Parameters\n        ----------\n        filepath : str or Path",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "void json_dump(const std::string& filename)",
          "code": "void Xform::json_dump(const std::string& filename) const {\n    std::ofstream file(filename);\n    file << jsondump().dump(4);\n}",
          "file": "xform.cpp"
        }
      }
    },
    {
      "name": "Xform.json_load",
      "implementations": {
        "python": {
          "sig": "json_load(cls, filepath)",
          "code": "def json_load(cls, filepath):\n\n        \"\"\"Read JSON from file.\n\n        Parameters\n        ----------\n        filepath : str or Path\n            Path to the JSON file.\n\n        Returns\n        -------\n        :class:`Xform`\n            The deserialized Xform.\n\n        \"\"\"\n        import json\n        with open(filepath, 'r') as f:\n            data = json.load(f)\n        return cls.__jsonload__(data, data.get(\"guid\"), data.get(\"name\"))\n\n    ###########################################################################################",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "Xform json_load(const std::string& filename)",
          "code": "Xform Xform::json_load(const std::string& filename) {\n    std::ifstream file(filename);\n    nlohmann::json data = nlohmann::json::parse(file);\n    return jsonload(data);\n}",
          "file": "xform.cpp"
        }
      }
    },
    {
      "name": "Xform.to_protobuf",
      "implementations": {
        "python": {
          "sig": "to_protobuf()",
          "code": "def to_protobuf(self):\n\n        \"\"\"Convert to protobuf binary format.\n\n        Returns\n        -------\n        bytes\n            Serialized protobuf data.\n\n        \"\"\"\n        from .proto import xform_pb2\n\n        proto = xform_pb2.Xform()\n        proto.guid = self.guid\n        proto.name = self.name\n        proto.matrix.extend(self.m)\n        return proto.SerializeToString()\n\n    @classmethod\n    def from_protobuf(cls, data):\n        \"\"\"Create Xform from protobuf binary data.",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "std::string to_protobuf()",
          "code": "std::string Xform::to_protobuf() const {\n    session_proto::Xform proto;\n    proto.set_guid(guid);\n    proto.set_name(name);\n    for (int i = 0; i < 16; ++i) {\n        proto.add_matrix(m[i]);\n    }",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "to_protobuf() -> Vec<u8>",
          "code": "pub fn to_protobuf(&self) -> Vec<u8> {\n        use prost::Message;\n\n        let proto = crate::proto::Xform {\n            guid: self.guid.clone(),\n            name: self.name.clone(),\n            matrix: self.m.to_vec(),\n        };\n        proto.encode_to_vec()\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.from_protobuf",
      "implementations": {
        "python": {
          "sig": "from_protobuf(cls, data)",
          "code": "def from_protobuf(cls, data):\n\n        \"\"\"Create Xform from protobuf binary data.\n\n        Parameters\n        ----------\n        data : bytes\n            Protobuf-encoded xform data.\n\n        Returns\n        -------\n        :class:`Xform`\n            The deserialized Xform.\n\n        \"\"\"\n        from .proto import xform_pb2\n\n        proto = xform_pb2.Xform()\n        proto.ParseFromString(data)\n        xform = cls.from_matrix(list(proto.matrix))\n        xform.guid = proto.guid",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "Xform from_protobuf(const std::string& data)",
          "code": "Xform Xform::from_protobuf(const std::string& data) {\n    session_proto::Xform proto;\n    proto.ParseFromString(data);\n\n    Xform xform;\n    xform.guid = proto.guid();\n    xform.name = proto.name();\n    for (int i = 0; i < 16 && i < proto.matrix_size(); ++i) {\n        xform.m[i] = proto.matrix(i);\n    }",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "from_protobuf(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn from_protobuf(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {\n        use prost::Message;\n\n        let proto = crate::proto::Xform::decode(data)?;\n\n        let mut xform = Self::identity();\n        xform.guid = proto.guid;\n        xform.name = proto.name;\n        for (i, val) in proto.matrix.iter().enumerate() {\n            if i < 16 {\n                xform.m[i] = *val;\n            }\n        }\n        Ok(xform)\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.protobuf_dump",
      "implementations": {
        "python": {
          "sig": "protobuf_dump(filepath)",
          "code": "def protobuf_dump(self, filepath):\n\n        \"\"\"Write protobuf to file.\n\n        Parameters\n        ----------\n        filepath : str\n            Path to the output file.\n\n        \"\"\"\n        data = self.to_protobuf()\n        with open(filepath, 'wb') as f:\n            f.write(data)\n\n    @classmethod\n    def protobuf_load(cls, filepath):\n        \"\"\"Read protobuf from file.\n\n        Parameters\n        ----------\n        filepath : str",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "void protobuf_dump(const std::string& filename)",
          "code": "void Xform::protobuf_dump(const std::string& filename) const {\n    std::string data = to_protobuf();\n    std::ofstream file(filename, std::ios::binary);\n    file.write(data.data(), data.size());\n}",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "protobuf_dump(filepath: &str)",
          "code": "pub fn protobuf_dump(&self, filepath: &str) {\n        let data = self.to_protobuf();\n        std::fs::write(filepath, data).expect(\"Failed to write protobuf file\");\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.protobuf_load",
      "implementations": {
        "python": {
          "sig": "protobuf_load(cls, filepath)",
          "code": "def protobuf_load(cls, filepath):\n\n        \"\"\"Read protobuf from file.\n\n        Parameters\n        ----------\n        filepath : str\n            Path to the protobuf file.\n\n        Returns\n        -------\n        :class:`Xform`\n            The deserialized Xform.\n\n        \"\"\"\n        with open(filepath, 'rb') as f:\n            data = f.read()\n        return cls.from_protobuf(data)",
          "file": "xform.py"
        },
        "cpp": {
          "sig": "Xform protobuf_load(const std::string& filename)",
          "code": "Xform Xform::protobuf_load(const std::string& filename) {\n    std::ifstream file(filename, std::ios::binary);\n    std::string data((std::istreambuf_iterator<char>(file)),\n                      std::istreambuf_iterator<char>());\n    return from_protobuf(data);\n}",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "protobuf_load(filepath: &str) -> Self",
          "code": "pub fn protobuf_load(filepath: &str) -> Self {\n        let data = std::fs::read(filepath).expect(\"Failed to read protobuf file\");\n        Self::from_protobuf(&data).expect(\"Failed to parse protobuf\")\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Color.constructor",
      "implementations": {
        "cpp": {
          "sig": "Color(unsigned int r = 255, unsigned int g = 255, unsigned int b = 255,\n        unsigned int a = 255, std::string name = \"my_color\")",
          "code": "Color(unsigned int r = 255, unsigned int g = 255, unsigned int b = 255,\n        unsigned int a = 255, std::string name = \"my_color\")\n      : name(name), r(r), g(g), b(b), a(a) {}",
          "file": "color.h"
        }
      }
    },
    {
      "name": "fmt.format_to",
      "implementations": {
        "cpp": {
          "sig": "return format_to(ctx.out()",
          "code": "return fmt::format_to(ctx.out(), \"{}",
          "file": "vector.h"
        }
      }
    },
    {
      "name": "Color.str",
      "implementations": {
        "cpp": {
          "sig": "std::string str()",
          "code": "std::string Color::str() const {\n  return fmt::format(\"{}",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "str() -> String",
          "code": "pub fn str(&self) -> String {\n        format!(\"{}, {}, {}, {}\", self.r, self.g, self.b, self.a)\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.repr",
      "implementations": {
        "cpp": {
          "sig": "std::string repr()",
          "code": "std::string Color::repr() const {\n  return fmt::format(\"Color({}",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "repr() -> String",
          "code": "pub fn repr(&self) -> String {\n        format!(\"Color({}, {}, {}, {}, {})\", self.name, self.r, self.g, self.b, self.a)\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.to_string",
      "implementations": {
        "cpp": {
          "sig": "std::string to_string()",
          "code": "std::string Color::to_string() const {\n  return repr();\n}",
          "file": "color.cpp"
        }
      }
    },
    {
      "name": "Color.jsondump",
      "implementations": {
        "cpp": {
          "sig": "nlohmann::ordered_json jsondump()",
          "code": "nlohmann::ordered_json Color::jsondump() const {\n  // Alphabetical order to match Rust's serde_json\n  nlohmann::ordered_json data;\n  data[\"a\"] = static_cast<int>(a);\n  data[\"b\"] = static_cast<int>(b);\n  data[\"g\"] = static_cast<int>(g);\n  data[\"guid\"] = guid;\n  data[\"name\"] = name;\n  data[\"r\"] = static_cast<int>(r);\n  data[\"type\"] = \"Color\";\n  return data;\n}",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "jsondump() -> Result<String, Box<dyn std::error::Error>>",
          "code": "pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {\n        let mut buf = Vec::new();\n        let formatter = serde_json::ser::PrettyFormatter::with_indent(b\"    \");\n        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);\n        SerTrait::serialize(self, &mut ser)?;\n        Ok(String::from_utf8(buf)?)\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.jsonload",
      "implementations": {
        "cpp": {
          "sig": "Color jsonload(const nlohmann::json &data)",
          "code": "Color Color::jsonload(const nlohmann::json &data) {\n  Color color(static_cast<unsigned int>(data[\"r\"]),\n                      static_cast<unsigned int>(data[\"g\"]),\n                      static_cast<unsigned int>(data[\"b\"]),\n                      static_cast<unsigned int>(data[\"a\"]), data[\"name\"]);\n  color.guid = data[\"guid\"];\n  return color;\n}",
          "file": "color.cpp"
        },
        "rust": {
          "sig": "jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {\n        Ok(serde_json::from_str(json_data)?)\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "std.out_of_range",
      "implementations": {
        "cpp": {
          "sig": "throw out_of_range(\"Index out of bounds: (\" + std::to_string(row)",
          "code": "throw std::out_of_range(\"Index out of bounds: (\" + std::to_string(row) + \", \" + std::to_string(col) + \")\");\n    }",
          "file": "xform.cpp"
        }
      }
    },
    {
      "name": "fmt.format",
      "implementations": {
        "cpp": {
          "sig": "return format(\"Xform({}, {})",
          "code": "return fmt::format(\"Xform({}, {})\", name, guid.substr(0, 8));\n}",
          "file": "xform.cpp"
        }
      }
    },
    {
      "name": "Line.constructor",
      "implementations": {
        "cpp": {
          "sig": "Line(double x0, double y0, double z0, double x1, double y1, double z1)",
          "code": "Line(double x0, double y0, double z0, double x1, double y1, double z1);",
          "file": "line.h"
        }
      }
    },
    {
      "name": "Line.to_string",
      "implementations": {
        "cpp": {
          "sig": "std::string to_string()",
          "code": "std::string Line::to_string() const {\n    return fmt::format(\"Line({}",
          "file": "line.cpp"
        }
      }
    },
    {
      "name": "Line.str",
      "implementations": {
        "cpp": {
          "sig": "std::string str()",
          "code": "std::string Line::str() const {\n    int prec = static_cast<int>(Tolerance::ROUNDING);\n    return fmt::format(\n        \"{}",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "str() -> String",
          "code": "pub fn str(&self) -> String {\n        use crate::tolerance::TOLERANCE;\n        let prec = crate::tolerance::Tolerance::ROUNDING;\n        format!(\n            \"{}, {}, {}, {}, {}, {}\",\n            TOLERANCE.format_number(self._x0, prec),\n            TOLERANCE.format_number(self._y0, prec),\n            TOLERANCE.format_number(self._z0, prec),\n            TOLERANCE.format_number(self._x1, prec),\n            TOLERANCE.format_number(self._y1, prec),\n            TOLERANCE.format_number(self._z1, prec),\n        )\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.repr",
      "implementations": {
        "cpp": {
          "sig": "std::string repr()",
          "code": "std::string Line::repr() const {\n    int prec = static_cast<int>(Tolerance::ROUNDING);\n    return fmt::format(\n        \"Line({}",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "repr() -> String",
          "code": "pub fn repr(&self) -> String {\n        use crate::tolerance::TOLERANCE;\n        let prec = crate::tolerance::Tolerance::ROUNDING;\n        format!(\n            \"Line({}, {}, {}, {}, {}, {}, {}, Color({}, {}, {}, {}), {})\",\n            self.name,\n            TOLERANCE.format_number(self._x0, prec),\n            TOLERANCE.format_number(self._y0, prec),\n            TOLERANCE.format_number(self._z0, prec),\n            TOLERANCE.format_number(self._x1, prec),\n            TOLERANCE.format_number(self._y1, prec),\n            TOLERANCE.format_number(self._z1, prec),\n            self.linecolor.r,\n            self.linecolor.g,\n            self.linecolor.b,\n            self.linecolor.a,\n            TOLERANCE.format_number(self.width, prec),\n        )\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.jsondump",
      "implementations": {
        "cpp": {
          "sig": "nlohmann::ordered_json jsondump()",
          "code": "nlohmann::ordered_json Line::jsondump() const {\n    // Alphabetical order to match Rust's serde_json\n    nlohmann::ordered_json data;\n    data[\"guid\"] = guid;\n    data[\"linecolor\"] = linecolor.jsondump();\n    data[\"name\"] = name;\n    data[\"type\"] = \"Line\";\n    data[\"width\"] = width;\n    data[\"x0\"] = _x0;\n    data[\"x1\"] = _x1;\n    data[\"xform\"] = xform.jsondump();\n    data[\"y0\"] = _y0;\n    data[\"y1\"] = _y1;\n    data[\"z0\"] = _z0;\n    data[\"z1\"] = _z1;\n    return data;\n}",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "jsondump() -> Result<String, Box<dyn std::error::Error>>",
          "code": "pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {\n        Ok(serde_json::to_string_pretty(self)?)\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.jsonload",
      "implementations": {
        "cpp": {
          "sig": "Line jsonload(const nlohmann::json& data)",
          "code": "Line Line::jsonload(const nlohmann::json& data) {\n    Line line(data[\"x0\"], data[\"y0\"], data[\"z0\"], data[\"x1\"], data[\"y1\"], data[\"z1\"]);\n    line.guid = data[\"guid\"];\n    line.name = data[\"name\"];\n    line.linecolor = Color::jsonload(data[\"linecolor\"]);\n    line.width = data[\"width\"];\n    if (data.contains(\"xform\")) {\n        line.xform = Xform::jsonload(data[\"xform\"]);\n    }",
          "file": "line.cpp"
        },
        "rust": {
          "sig": "jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {\n        Ok(serde_json::from_str(json_data)?)\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "std.invalid_argument",
      "implementations": {
        "cpp": {
          "sig": "throw invalid_argument(\"Precision cannot be zero.\")",
          "code": "throw std::invalid_argument(\"Precision cannot be zero.\");\n    }",
          "file": "tolerance.cpp"
        }
      }
    },
    {
      "name": "std.runtime_error",
      "implementations": {
        "cpp": {
          "sig": "throw runtime_error(\"Protobuf support not enabled\")",
          "code": "throw std::runtime_error(\"Protobuf support not enabled\");\n}",
          "file": "vector.cpp"
        }
      }
    },
    {
      "name": "std.sqrt",
      "implementations": {
        "cpp": {
          "sig": "return sqrt(a * a + b * b - 2.0 * a * b * std::cos(ang_between * to_rad)",
          "code": "return std::sqrt(a * a + b * b - 2.0 * a * b * std::cos(ang_between * to_rad));\n}",
          "file": "vector.cpp"
        }
      }
    },
    {
      "name": "NormalWeighting.number_of_edges",
      "implementations": {
        "cpp": {
          "sig": "size_t number_of_edges()",
          "code": "size_t number_of_edges() const;",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "NormalWeighting.euler",
      "implementations": {
        "cpp": {
          "sig": "int euler()",
          "code": "int euler() const;",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "NormalWeighting.clear",
      "implementations": {
        "cpp": {
          "sig": "void clear()",
          "code": "void clear();",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "NormalWeighting.add_vertex",
      "implementations": {
        "cpp": {
          "sig": "size_t add_vertex(const Point& position, std::optional<size_t> vkey = std::nullopt)",
          "code": "size_t add_vertex(const Point& position, std::optional<size_t> vkey = std::nullopt);",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "NormalWeighting.add_face",
      "implementations": {
        "cpp": {
          "sig": "std::optional<size_t> add_face(const std::vector<size_t>& vertices, std::optional<size_t> fkey = std::nullopt)",
          "code": "std::optional<size_t> add_face(const std::vector<size_t>& vertices, std::optional<size_t> fkey = std::nullopt);",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "NormalWeighting.vertex_position",
      "implementations": {
        "cpp": {
          "sig": "std::optional<Point> vertex_position(size_t vertex_key)",
          "code": "std::optional<Point> vertex_position(size_t vertex_key) const;",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "NormalWeighting.face_vertices",
      "implementations": {
        "cpp": {
          "sig": "std::optional<std::vector<size_t>> face_vertices(size_t face_key)",
          "code": "std::optional<std::vector<size_t>> face_vertices(size_t face_key) const;",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "NormalWeighting.vertex_neighbors",
      "implementations": {
        "cpp": {
          "sig": "std::vector<size_t> vertex_neighbors(size_t vertex_key)",
          "code": "std::vector<size_t> vertex_neighbors(size_t vertex_key) const;",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "NormalWeighting.vertex_faces",
      "implementations": {
        "cpp": {
          "sig": "std::vector<size_t> vertex_faces(size_t vertex_key)",
          "code": "std::vector<size_t> vertex_faces(size_t vertex_key) const;",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "NormalWeighting.is_vertex_on_boundary",
      "implementations": {
        "cpp": {
          "sig": "bool is_vertex_on_boundary(size_t vertex_key)",
          "code": "bool is_vertex_on_boundary(size_t vertex_key) const;",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "NormalWeighting.face_normal",
      "implementations": {
        "cpp": {
          "sig": "std::optional<Vector> face_normal(size_t face_key)",
          "code": "std::optional<Vector> face_normal(size_t face_key) const;",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "NormalWeighting.vertex_normal",
      "implementations": {
        "cpp": {
          "sig": "std::optional<Vector> vertex_normal(size_t vertex_key)",
          "code": "std::optional<Vector> vertex_normal(size_t vertex_key) const;",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "NormalWeighting.vertex_normal_weighted",
      "implementations": {
        "cpp": {
          "sig": "std::optional<Vector> vertex_normal_weighted(size_t vertex_key, NormalWeighting weighting)",
          "code": "std::optional<Vector> vertex_normal_weighted(size_t vertex_key, NormalWeighting weighting) const;",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "NormalWeighting.face_area",
      "implementations": {
        "cpp": {
          "sig": "std::optional<double> face_area(size_t face_key)",
          "code": "std::optional<double> face_area(size_t face_key) const;",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "NormalWeighting.vertex_angle_in_face",
      "implementations": {
        "cpp": {
          "sig": "std::optional<double> vertex_angle_in_face(size_t vertex_key, size_t face_key)",
          "code": "std::optional<double> vertex_angle_in_face(size_t vertex_key, size_t face_key) const;",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "NormalWeighting.from_polygons",
      "implementations": {
        "cpp": {
          "sig": "Mesh from_polygons(const std::vector<std::vector<Point>>& polygons, std::optional<double> precision = std::nullopt)",
          "code": "static Mesh from_polygons(const std::vector<std::vector<Point>>& polygons, std::optional<double> precision = std::nullopt);",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "NormalWeighting.transform",
      "implementations": {
        "cpp": {
          "sig": "void transform()",
          "code": "void transform();",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "NormalWeighting.transformed",
      "implementations": {
        "cpp": {
          "sig": "Mesh transformed()",
          "code": "Mesh transformed() const;",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "NormalWeighting.jsondump",
      "implementations": {
        "cpp": {
          "sig": "nlohmann::ordered_json jsondump()",
          "code": "nlohmann::ordered_json jsondump() const;",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "NormalWeighting.jsonload",
      "implementations": {
        "cpp": {
          "sig": "Mesh jsonload(const nlohmann::json& data)",
          "code": "static Mesh jsonload(const nlohmann::json& data);",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "NormalWeighting.to_protobuf",
      "implementations": {
        "cpp": {
          "sig": "std::string to_protobuf()",
          "code": "std::string to_protobuf() const;",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "NormalWeighting.from_protobuf",
      "implementations": {
        "cpp": {
          "sig": "Mesh from_protobuf(const std::string& data)",
          "code": "static Mesh from_protobuf(const std::string& data);",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "NormalWeighting.protobuf_dump",
      "implementations": {
        "cpp": {
          "sig": "void protobuf_dump(const std::string& filename)",
          "code": "void protobuf_dump(const std::string& filename) const;",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "NormalWeighting.protobuf_load",
      "implementations": {
        "cpp": {
          "sig": "Mesh protobuf_load(const std::string& filename)",
          "code": "static Mesh protobuf_load(const std::string& filename);",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "NormalWeighting.build_triangle_bvh",
      "implementations": {
        "cpp": {
          "sig": "void build_triangle_bvh(bool force = false)",
          "code": "void build_triangle_bvh(bool force = false) const;",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "NormalWeighting.triangle_bvh_ray_cast",
      "implementations": {
        "cpp": {
          "sig": "bool triangle_bvh_ray_cast(const Point& origin, const Vector& direction, std::vector<int>& candidate_ids, bool find_all = false)",
          "code": "bool triangle_bvh_ray_cast(const Point& origin, const Vector& direction, std::vector<int>& candidate_ids, bool find_all = false) const;",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "NormalWeighting.get_triangle_by_id",
      "implementations": {
        "cpp": {
          "sig": "bool get_triangle_by_id(int tri_id, size_t& face_idx, size_t& sub_idx, Point& v0, Point& v1, Point& v2)",
          "code": "bool get_triangle_by_id(int tri_id, size_t& face_idx, size_t& sub_idx, Point& v0, Point& v1, Point& v2) const;",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "NormalWeighting.clear_triangle_bvh",
      "implementations": {
        "cpp": {
          "sig": "void clear_triangle_bvh()",
          "code": "void clear_triangle_bvh() const;",
          "file": "mesh.h"
        }
      }
    },
    {
      "name": "Mesh.constructor",
      "implementations": {
        "cpp": {
          "sig": "Mesh()",
          "code": "Mesh::Mesh() {\n    xform = Xform::identity();\n    default_vertex_attributes[\"x\"] = 0.0;\n    default_vertex_attributes[\"y\"] = 0.0;\n    default_vertex_attributes[\"z\"] = 0.0;\n}",
          "file": "mesh.cpp"
        }
      }
    },
    {
      "name": "std.acos",
      "implementations": {
        "cpp": {
          "sig": "return acos(cos_angle)",
          "code": "return std::acos(cos_angle);\n}",
          "file": "mesh.cpp"
        }
      }
    },
    {
      "name": "Mesh.jsondump",
      "implementations": {
        "cpp": {
          "sig": "nlohmann::ordered_json jsondump()",
          "code": "nlohmann::ordered_json Mesh::jsondump() const {\n    nlohmann::ordered_json data;\n    \n    // Alphabetical order to match Rust's serde_json output\n    data[\"default_edge_attributes\"] = default_edge_attributes;\n    data[\"default_face_attributes\"] = default_face_attributes;\n    data[\"default_vertex_attributes\"] = default_vertex_attributes;\n    \n    // Edge attributes\n    nlohmann::ordered_json edgedata_json;\n    for (const auto& [edge, attrs] : edgedata) {\n        std::string edge_key = std::to_string(edge.first) + \",\" + std::to_string(edge.second);\n        edgedata_json[edge_key] = attrs;\n    }",
          "file": "mesh.cpp"
        },
        "rust": {
          "sig": "jsondump() -> serde_json::Value",
          "code": "pub fn jsondump(&self) -> serde_json::Value {\n        let pointcolors_flat: Vec<u8> = self\n            .pointcolors\n            .iter()\n            .flat_map(|c| vec![c.r, c.g, c.b])\n            .collect();\n\n        let facecolors_flat: Vec<u8> = self\n            .facecolors\n            .iter()\n            .flat_map(|c| vec![c.r, c.g, c.b])\n            .collect();\n\n        let linecolors_flat: Vec<u8> = self\n            .linecolors\n            .iter()\n            .flat_map(|c| vec![c.r, c.g, c.b])\n            .collect();\n\n        serde_json::json!({\n            \"type\": \"Mesh\",\n            \"guid\": self.guid,\n            \"name\": self.name,\n            \"vertex\": self.vertex,\n            \"face\": self.face,\n            \"halfedge\": self.halfedge,\n            \"facedata\": self.facedata,",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.jsonload",
      "implementations": {
        "cpp": {
          "sig": "Mesh jsonload(const nlohmann::json& data)",
          "code": "Mesh Mesh::jsonload(const nlohmann::json& data) {\n    Mesh mesh;\n    \n    if (data.contains(\"guid\")) mesh.guid = data[\"guid\"];\n    if (data.contains(\"name\")) mesh.name = data[\"name\"];\n    \n    // Load halfedge connectivity\n    if (data.contains(\"halfedge\")) {\n        for (const auto& [u_str, neighbors] : data[\"halfedge\"].items()) {\n            size_t u = std::stoull(u_str);\n            mesh.halfedge[u] = {}",
          "file": "mesh.cpp"
        },
        "rust": {
          "sig": "jsonload(data: &serde_json::Value) -> Option<Self>",
          "code": "pub fn jsonload(data: &serde_json::Value) -> Option<Self> {\n        let mut mesh = Mesh::new();\n\n        if let Some(guid) = data.get(\"guid\").and_then(|v| v.as_str()) {\n            mesh.guid = guid.to_string();\n        }\n        if let Some(name) = data.get(\"name\").and_then(|v| v.as_str()) {\n            mesh.name = name.to_string();\n        }\n\n        if let Some(vertex_data) = data.get(\"vertex\") {\n            mesh.vertex = serde_json::from_value(vertex_data.clone()).ok()?;\n        }\n        if let Some(face_data) = data.get(\"face\") {\n            mesh.face = serde_json::from_value(face_data.clone()).ok()?;\n        }\n        if let Some(halfedge_data) = data.get(\"halfedge\") {\n            mesh.halfedge = serde_json::from_value(halfedge_data.clone()).ok()?;\n        }\n        if let Some(",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.build_triangle_bvh",
      "implementations": {
        "cpp": {
          "sig": "void build_triangle_bvh(bool force)",
          "code": "void Mesh::build_triangle_bvh(bool force) const {\n    if (triangle_bvh_built && !force) return;\n\n    triangle_boxes_cache.clear();\n    triangle_aabbs_cache.clear();\n    triangle_indices_cache.clear();\n    triangle_face_subidx_cache.clear();\n    vertices_cache.clear();\n\n    auto vf = to_vertices_and_faces();\n    vertices_cache = vf.first;\n    const std::vector<std::vector<size_t>>& faces_vec = vf.second;\n\n    size_t tri_count = 0;\n    for (const auto& f : faces_vec) if (f.size() >= 3) tri_count += (f.size() - 2);\n    triangle_aabbs_cache.resize(tri_count);\n    triangle_indices_cache.resize(tri_count);\n    triangle_face_subidx_cache.resize(tri_count);\n\n    struct TriTask { uint32_t i0, i1, i2; size_t face_idx; size_t sub_idx; size_t out_idx; }",
          "file": "mesh.cpp"
        }
      }
    },
    {
      "name": "Mesh.triangle_bvh_ray_cast",
      "implementations": {
        "cpp": {
          "sig": "bool triangle_bvh_ray_cast(const Point& origin, const Vector& direction, std::vector<int>& candidate_ids, bool find_all)",
          "code": "bool Mesh::triangle_bvh_ray_cast(const Point& origin, const Vector& direction, std::vector<int>& candidate_ids, bool find_all) const {\n    build_triangle_bvh(false);\n    if (!triangle_bvh) return false;\n    return triangle_bvh->ray_cast(origin, direction, candidate_ids, find_all);\n}",
          "file": "mesh.cpp"
        }
      }
    },
    {
      "name": "Mesh.get_triangle_by_id",
      "implementations": {
        "cpp": {
          "sig": "bool get_triangle_by_id(int tri_id, size_t& face_idx, size_t& sub_idx, Point& v0, Point& v1, Point& v2)",
          "code": "bool Mesh::get_triangle_by_id(int tri_id, size_t& face_idx, size_t& sub_idx, Point& v0, Point& v1, Point& v2) const {\n    if (tri_id < 0) return false;\n    size_t id = static_cast<size_t>(tri_id);\n    if (id >= triangle_indices_cache.size() || id >= triangle_face_subidx_cache.size()) return false;\n    const auto& tri = triangle_indices_cache[id];\n    const auto& fs = triangle_face_subidx_cache[id];\n    face_idx = fs.first;\n    sub_idx = fs.second;\n    if (tri.i0 >= vertices_cache.size() || tri.i1 >= vertices_cache.size() || tri.i2 >= vertices_cache.size()) return false;\n    v0 = vertices_cache[tri.i0];\n    v1 = vertices_cache[tri.i1];\n    v2 = vertices_cache[tri.i2];\n    return true;\n}",
          "file": "mesh.cpp"
        }
      }
    },
    {
      "name": "Mesh.clear_triangle_bvh",
      "implementations": {
        "cpp": {
          "sig": "void clear_triangle_bvh()",
          "code": "void Mesh::clear_triangle_bvh() const {\n    triangle_bvh_built = false;\n    triangle_bvh.reset();\n    triangle_boxes_cache.clear();\n    triangle_aabbs_cache.clear();\n    triangle_indices_cache.clear();\n    triangle_face_subidx_cache.clear();\n    vertices_cache.clear();\n}",
          "file": "mesh.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.constructor",
      "implementations": {
        "cpp": {
          "sig": "NurbsCurve(int dimension, bool is_rational, int order, int cv_count)",
          "code": "NurbsCurve(int dimension, bool is_rational, int order, int cv_count);",
          "file": "nurbscurve.h"
        }
      }
    },
    {
      "name": "NurbsCurve.cv",
      "implementations": {
        "cpp": {
          "sig": "const double* cv(int cv_index)",
          "code": "const double* NurbsCurve::cv(int cv_index) const {\n    if (cv_index < 0 || cv_index >= m_cv_count) return nullptr;\n    return &m_cv[cv_index * m_cv_stride];\n}",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.jsondump",
      "implementations": {
        "cpp": {
          "sig": "nlohmann::ordered_json jsondump()",
          "code": "nlohmann::ordered_json NurbsCurve::jsondump() const {\n    nlohmann::ordered_json j;\n    j[\"guid\"] = guid;\n    j[\"name\"] = name;\n    j[\"dimension\"] = m_dim;\n    j[\"is_rational\"] = m_is_rat != 0;\n    j[\"order\"] = m_order;\n    j[\"cv_count\"] = m_cv_count;\n    j[\"knots\"] = m_knot;\n    j[\"control_points\"] = nlohmann::json::array();\n    \n    for (int i = 0; i < m_cv_count; i++) {\n        Point p = get_cv(i);\n        j[\"control_points\"].push_back({p[0], p[1], p[2]}",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.jsonload",
      "implementations": {
        "cpp": {
          "sig": "NurbsCurve jsonload(const nlohmann::json& data)",
          "code": "NurbsCurve NurbsCurve::jsonload(const nlohmann::json& data) {\n    NurbsCurve curve;\n    \n    if (data.contains(\"dimension\") && data.contains(\"order\") && data.contains(\"cv_count\")) {\n        int dim = data[\"dimension\"];\n        bool is_rat = data.value(\"is_rational\", false);\n        int order = data[\"order\"];\n        int cv_count = data[\"cv_count\"];\n        \n        curve.create(dim, is_rat, order, cv_count);\n        \n        if (data.contains(\"knots\")) {\n            curve.m_knot = data[\"knots\"].get<std::vector<double>>();\n        }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.reserve_cv_capacity",
      "implementations": {
        "cpp": {
          "sig": "bool reserve_cv_capacity(int capacity)",
          "code": "bool NurbsCurve::reserve_cv_capacity(int capacity) {\n    if (capacity > static_cast<int>(m_cv.size())) {\n        m_cv.resize(capacity);\n        m_cv_capacity = capacity;\n    }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.reserve_knot_capacity",
      "implementations": {
        "cpp": {
          "sig": "bool reserve_knot_capacity(int capacity)",
          "code": "bool NurbsCurve::reserve_knot_capacity(int capacity) {\n    if (capacity > static_cast<int>(m_knot.size())) {\n        m_knot.resize(capacity);\n    }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.remove_span",
      "implementations": {
        "cpp": {
          "sig": "bool remove_span(int span_index)",
          "code": "bool NurbsCurve::remove_span(int span_index) {\n    if (!is_valid()) return false;\n    if (span_index < 0 || span_index > m_cv_count - m_order) return false;\n    \n    // Need at least 2 spans\n    if (span_count() < 2) return false;\n    \n    // Check if span is non-empty\n    int ki0 = span_index + m_order - 2;\n    int ki1 = span_index + m_order - 1;\n    if (ki0 >= knot_count() || ki1 >= knot_count()) return false;\n    if (m_knot[ki0] >= m_knot[ki1]) return false;\n    \n    // Get multiplicities\n    int m0 = knot_multiplicity(ki0);\n    int m1 = knot_multiplicity(ki1);\n    \n    // Calculate how many CVs to remove\n    int cvs_to_remove = m_order - (m0 + m1);\n    if (cvs_to_remove <= 0) return false;\n    \n    // Remove knots\n    int knots_to_remove = m_order;\n    m_knot.erase(m_knot.begin() + ki0",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.remove_singular_spans",
      "implementations": {
        "cpp": {
          "sig": "int remove_singular_spans()",
          "code": "int NurbsCurve::remove_singular_spans() {\n    if (!is_valid()) return 0;\n    \n    int removed_count = 0;\n    int span_cnt = span_count();\n    \n    // Iterate backwards to avoid index shifting issues\n    for (int i = span_cnt - 1; i >= 0; i--) {\n        if (span_is_singular(i)) {\n            if (remove_span(i)) {\n                removed_count++;\n            }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.get_cubic_bezier_approximation",
      "implementations": {
        "cpp": {
          "sig": "double get_cubic_bezier_approximation(double max_deviation, std::vector<Point>& bezier_cvs)",
          "code": "double NurbsCurve::get_cubic_bezier_approximation(double max_deviation, std::vector<Point>& bezier_cvs) const {\n    bezier_cvs.clear();\n    \n    if (!is_valid()) return std::numeric_limits<double>::quiet_NaN();\n    if (m_cv_count < 2) return std::numeric_limits<double>::quiet_NaN();\n    \n    // Get Greville abcissae for sampling\n    std::vector<double> greville;\n    if (!get_greville_abcissae(greville)) {\n        return std::numeric_limits<double>::quiet_NaN();\n    }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.is_duplicate",
      "implementations": {
        "cpp": {
          "sig": "bool is_duplicate(const NurbsCurve& other,\n                              bool ignore_parameterization,\n                              double tolerance)",
          "code": "bool NurbsCurve::is_duplicate(const NurbsCurve& other,\n                              bool ignore_parameterization,\n                              double tolerance) const {\n    if (!is_valid() || !other.is_valid()) return false;\n    if (m_dim != other.m_dim) return false;\n    if (m_is_rat != other.m_is_rat) return false;\n    if (m_order != other.m_order) return false;\n    if (m_cv_count != other.m_cv_count) return false;\n    \n    // Check control points\n    for (int i = 0; i < m_cv_count; i++) {\n        Point p1 = get_cv(i);\n        Point p2 = other.get_cv(i);\n        if (p1.distance(p2) > tolerance) {\n            return false;\n        }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.get_next_discontinuity",
      "implementations": {
        "cpp": {
          "sig": "bool get_next_discontinuity(int continuity_type,\n                                       double t0, double t1,\n                                       double& t_out,\n                                       int* hint,\n                                       double cos_angle_tolerance,\n                                       double curvature_tolerance)",
          "code": "bool NurbsCurve::get_next_discontinuity(int continuity_type,\n                                       double t0, double t1,\n                                       double& t_out,\n                                       int* hint,\n                                       double cos_angle_tolerance,\n                                       double curvature_tolerance) const {\n    if (!is_valid()) return false;\n    if (t0 >= t1) return false;\n    \n    auto [d0, d1] = domain();\n    if (t0 < d0) t0 = d0;\n    if (t1 > d1) t1 = d1;\n    if (t0 >= t1) return false;\n    \n    // Check each interior knot\n    for (int i = m_order - 1; i < m_cv_count - 1; i++) {\n        double t = m_knot[i];\n        if (t <= t0 || t >= t1) continue;\n        \n        // Check if there's a discontinuity at this knot\n        int mu",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.is_continuous",
      "implementations": {
        "cpp": {
          "sig": "bool is_continuous(int continuity_type,\n                              double t,\n                              int* hint,\n                              double point_tolerance,\n                              double d1_tolerance,\n                              double d2_tolerance,\n                              double cos_angle_tolerance,\n                              double curvature_tolerance)",
          "code": "bool NurbsCurve::is_continuous(int continuity_type,\n                              double t,\n                              int* hint,\n                              double point_tolerance,\n                              double d1_tolerance,\n                              double d2_tolerance,\n                              double cos_angle_tolerance,\n                              double curvature_tolerance) const {\n    if (!is_valid()) return false;\n    \n    auto [d0, d1] = domain();\n    if (t < d0 || t > d1) return false;\n    \n    // Find knot span\n    int span = find_span(t);\n    \n    // Check if t is at a knot\n    bool at_knot = false;\n    int knot_idx = -1;\n    for (int i = 0; i < knot_count(); i++) {\n        if (std::abs(m_knot[i] - t) < Tolerance::ZERO_TOLERANCE) {\n            at_knot = tr",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.reparameterize",
      "implementations": {
        "cpp": {
          "sig": "bool reparameterize(double c)",
          "code": "bool NurbsCurve::reparameterize(double c) {\n    if (!std::isfinite(c) || c == 0.0) return false;\n    if (c == 1.0) return true;\n    \n    // Must be rational for this operation\n    if (!m_is_rat) {\n        if (!make_rational()) return false;\n    }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.change_end_weights",
      "implementations": {
        "cpp": {
          "sig": "bool change_end_weights(double w0, double w1)",
          "code": "bool NurbsCurve::change_end_weights(double w0, double w1) {\n    if (m_cv_count < m_order || m_order < 2) return false;\n    if (!std::isfinite(w0) || !std::isfinite(w1)) return false;\n    if (w0 == 0.0 || w1 == 0.0) return false;\n    if ((w0 < 0.0 && w1 > 0.0) || (w0 > 0.0 && w1 < 0.0)) return false;\n    \n    // Make rational if needed\n    if (!m_is_rat) {\n        if (!make_rational()) return false;\n    }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.find_span",
      "implementations": {
        "cpp": {
          "sig": "int find_span(double t)",
          "code": "int NurbsCurve::find_span(double t) const {\n    // OpenNURBS shifts knot pointer by (order-2) to work with compressed format\n    // Domain is knot[order-2] to knot[cv_count-1]\n    const double* knot = m_knot.data() + (m_order - 2);\n    int len = m_cv_count - m_order + 2;\n    \n    // Binary search for span\n    int low = 0;\n    int high = len - 1;\n    \n    if (t <= knot[0]) return 0;\n    if (t >= knot[len-1]) return len - 2;\n    \n    // Binary search\n    while (high > low + 1) {\n        int mid = (low + high) / 2;\n        if (t < knot[mid]) {\n            high = mid;\n        }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.basis_functions",
      "implementations": {
        "cpp": {
          "sig": "void basis_functions(int span, double t, std::vector<double>& basis)",
          "code": "void NurbsCurve::basis_functions(int span, double t, std::vector<double>& basis) const {\n    basis.resize(m_order);\n    std::vector<double> left(m_order);\n    std::vector<double> right(m_order);\n    \n    const double eps = 1e-14;\n    \n    // Offset knot pointer like OpenNURBS does\n    const double* knot = m_knot.data() + (m_order - 2) + span;\n    \n    basis[0] = 1.0;\n    \n    for (int j = 1; j < m_order; j++) {\n        left[j] = t - knot[1 - j];\n        right[j] = knot[j] - t;\n        double saved = 0.0;\n        \n        for (int r = 0; r < j; r++) {\n            double denom = right[r + 1] + left[j - r];\n            double temp;\n            if (std::abs(denom) <= eps) {\n                temp = 0.0;  // Safe fallback for zero denominator\n            }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.basis_functions_derivatives",
      "implementations": {
        "cpp": {
          "sig": "void basis_functions_derivatives(int span, double t, int deriv_order,\n                                            std::vector<std::vector<double>>& ders)",
          "code": "void NurbsCurve::basis_functions_derivatives(int span, double t, int deriv_order,\n                                            std::vector<std::vector<double>>& ders) const {\n    // Algorithm A2.3 from \"The NURBS Book\" (Piegl & Tiller)\n    int p = degree();\n    int n_der = std::min(deriv_order, p);\n\n    ders.assign(n_der + 1, std::vector<double>(p + 1, 0.0));\n\n    std::vector<double> left(p + 1);\n    std::vector<double> right(p + 1);\n    std::vector<std::vector<double>> ndu(p + 1, std::vector<double>(p + 1, 0.0));\n\n    // Offset knot pointer like OpenNURBS and basis_functions do\n    const double* knot = m_knot.data() + (m_order - 2) + span;\n    \n    ndu[0][0] = 1.0;\n    for (int j = 1; j <= p; ++j) {\n        left[j] = t - knot[1 - j];\n        right[j] = knot[j] - t;\n        double saved = 0",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.insert_knot",
      "implementations": {
        "cpp": {
          "sig": "bool insert_knot(double knot_value, int knot_multiplicity)",
          "code": "bool NurbsCurve::insert_knot(double knot_value, int knot_multiplicity) {\n    if (!is_valid()) return false;\n\n    int p = degree();\n    if (knot_multiplicity < 1 || knot_multiplicity > p) {\n        return false;\n    }",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "NurbsCurve.deep_copy_from",
      "implementations": {
        "cpp": {
          "sig": "void deep_copy_from(const NurbsCurve& src)",
          "code": "void NurbsCurve::deep_copy_from(const NurbsCurve& src) {\n    m_dim = src.m_dim;\n    m_is_rat = src.m_is_rat;\n    m_order = src.m_order;\n    m_cv_count = src.m_cv_count;\n    m_cv_stride = src.m_cv_stride;\n    m_cv_capacity = src.m_cv_capacity;\n    m_knot = src.m_knot;\n    m_cv = src.m_cv;\n    guid = src.guid;\n    name = src.name;\n    width = src.width;\n    linecolor = src.linecolor;\n    xform = src.xform;\n}",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "BoundingBox.from_points",
      "implementations": {
        "cpp": {
          "sig": "return from_points(points)",
          "code": "return BoundingBox::from_points(points);\n}",
          "file": "nurbscurve.cpp"
        }
      }
    },
    {
      "name": "std.abs",
      "implementations": {
        "cpp": {
          "sig": "return abs(a - b)",
          "code": "return std::abs(a - b) <= angular();\n}",
          "file": "tolerance.cpp"
        }
      }
    },
    {
      "name": "Plane.constructor",
      "implementations": {
        "cpp": {
          "sig": "Plane(Point& point, Vector& x_axis, Vector& y_axis, std::string name = \"my_plane\")",
          "code": "Plane(Point& point, Vector& x_axis, Vector& y_axis, std::string name = \"my_plane\");",
          "file": "plane.h"
        }
      }
    },
    {
      "name": "Plane.to_string",
      "implementations": {
        "cpp": {
          "sig": "std::string to_string()",
          "code": "std::string Plane::to_string() const {\n    return fmt::format(\"Plane(origin={}",
          "file": "plane.cpp"
        }
      }
    },
    {
      "name": "Plane.jsondump",
      "implementations": {
        "cpp": {
          "sig": "nlohmann::ordered_json jsondump()",
          "code": "nlohmann::ordered_json Plane::jsondump() const {\n    auto clean_float = [](double val) -> double {\n        return static_cast<double>(std::round(val * 100.0) / 100.0);\n    }",
          "file": "plane.cpp"
        },
        "rust": {
          "sig": "jsondump() -> Result<String, Box<dyn std::error::Error>>",
          "code": "pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {\n        Ok(serde_json::to_string_pretty(self)?)\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.jsonload",
      "implementations": {
        "cpp": {
          "sig": "Plane jsonload(const nlohmann::json &data)",
          "code": "Plane Plane::jsonload(const nlohmann::json &data) {\n    Plane plane;\n    // Parse flat frame array of 12 numbers: [ox, oy, oz, xx, xy, xz, yx, yy, yz, zx, zy, zz]\n    auto frame = data[\"frame\"];\n\n    plane._origin = Point(frame[0].get<double>(), frame[1].get<double>(), frame[2].get<double>());\n    plane._x_axis = Vector(frame[3].get<double>(), frame[4].get<double>(), frame[5].get<double>());\n    plane._y_axis = Vector(frame[6].get<double>(), frame[7].get<double>(), frame[8].get<double>());\n    plane._z_axis = Vector(frame[9].get<double>(), frame[10].get<double>(), frame[11].get<double>());\n    plane.guid = data[\"guid\"];\n    plane.name = data[\"name\"];\n    if (data.contains(\"width\")) {\n        plane.width = data[\"width\"].get<double>();\n    }",
          "file": "plane.cpp"
        },
        "rust": {
          "sig": "jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {\n        Ok(serde_json::from_str(json_data)?)\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Point.constructor",
      "implementations": {
        "cpp": {
          "sig": "Point(double x, double y, double z, std::string point_name = \"my_point\")",
          "code": "Point(double x, double y, double z, std::string point_name = \"my_point\")\n      : name(std::move(point_name)), _x(x), _y(y), _z(z) {}",
          "file": "point.h"
        }
      }
    },
    {
      "name": "Point.str",
      "implementations": {
        "cpp": {
          "sig": "std::string str()",
          "code": "std::string Point::str() const {\n  int prec = static_cast<int>(Tolerance::ROUNDING);\n  return fmt::format(\n      \"{}",
          "file": "point.cpp"
        },
        "rust": {
          "sig": "str() -> String",
          "code": "pub fn str(&self) -> String {\n        use crate::tolerance::TOLERANCE;\n        let prec = crate::tolerance::Tolerance::ROUNDING;\n        format!(\n            \"{}, {}, {}\",\n            TOLERANCE.format_number(self._x, prec),\n            TOLERANCE.format_number(self._y, prec),\n            TOLERANCE.format_number(self._z, prec),\n        )\n    }",
          "file": "point.rs"
        }
      }
    },
    {
      "name": "Point.repr",
      "implementations": {
        "cpp": {
          "sig": "std::string repr()",
          "code": "std::string Point::repr() const {\n  int prec = static_cast<int>(Tolerance::ROUNDING);\n  return fmt::format(\n      \"Point({}",
          "file": "point.cpp"
        },
        "rust": {
          "sig": "repr() -> String",
          "code": "pub fn repr(&self) -> String {\n        use crate::tolerance::TOLERANCE;\n        let prec = crate::tolerance::Tolerance::ROUNDING;\n        format!(\n            \"Point({}, {}, {}, {}, Color({}, {}, {}, {}), {})\",\n            self.name,\n            TOLERANCE.format_number(self._x, prec),\n            TOLERANCE.format_number(self._y, prec),\n            TOLERANCE.format_number(self._z, prec),\n            self.pointcolor.r,\n            self.pointcolor.g,\n            self.pointcolor.b,\n            self.pointcolor.a,\n            TOLERANCE.format_number(self.width, prec),\n        )\n    }",
          "file": "point.rs"
        }
      }
    },
    {
      "name": "Point.jsondump",
      "implementations": {
        "cpp": {
          "sig": "nlohmann::ordered_json jsondump()",
          "code": "nlohmann::ordered_json Point::jsondump() const {\n  auto clean_float = [](double val) -> double { return std::round(val * 100.0) / 100.0; }",
          "file": "point.cpp"
        },
        "rust": {
          "sig": "jsondump() -> Result<String, Box<dyn std::error::Error>>",
          "code": "pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {\n        let mut buf = Vec::new();\n        let formatter = serde_json::ser::PrettyFormatter::with_indent(b\"    \");\n        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);\n        SerTrait::serialize(self, &mut ser)?;\n        Ok(String::from_utf8(buf)?)\n    }",
          "file": "point.rs"
        }
      }
    },
    {
      "name": "Point.jsonload",
      "implementations": {
        "cpp": {
          "sig": "Point jsonload(const nlohmann::json &data)",
          "code": "Point Point::jsonload(const nlohmann::json &data) {\n  Point point(data[\"x\"], data[\"y\"], data[\"z\"]);\n  point.guid = data[\"guid\"];\n  point.name = data[\"name\"];\n  point.pointcolor = Color::jsonload(data[\"pointcolor\"]);\n  point.width = data[\"width\"];\n  if (data.contains(\"xform\")) {\n    point.xform = Xform::jsonload(data[\"xform\"]);\n  }",
          "file": "point.cpp"
        },
        "rust": {
          "sig": "jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {\n        Ok(serde_json::from_str(json_data)?)\n    }",
          "file": "point.rs"
        }
      }
    },
    {
      "name": "Point.ccw",
      "implementations": {
        "cpp": {
          "sig": "bool ccw(const Point& a, const Point& b, const Point& c)",
          "code": "bool Point::ccw(const Point& a, const Point& b, const Point& c) {\n    return (c._y - a._y) * (b._x - a._x) > (b._y - a._y) * (c._x - a._x);\n}",
          "file": "point.cpp"
        },
        "rust": {
          "sig": "ccw(a: &Point, b: &Point, c: &Point) -> bool",
          "code": "pub fn ccw(a: &Point, b: &Point, c: &Point) -> bool {\n        (c._y - a._y) * (b._x - a._x) > (b._y - a._y) * (c._x - a._x)\n    }",
          "file": "point.rs"
        }
      }
    },
    {
      "name": "PointCloud.constructor",
      "implementations": {
        "cpp": {
          "sig": "PointCloud(const std::vector<Point>& points,\n               const std::vector<Vector>& normals,\n               const std::vector<Color>& colors)",
          "code": "PointCloud(const std::vector<Point>& points,\n               const std::vector<Vector>& normals,\n               const std::vector<Color>& colors);",
          "file": "pointcloud.h"
        }
      }
    },
    {
      "name": "PointCloud.jsondump",
      "implementations": {
        "cpp": {
          "sig": "nlohmann::ordered_json jsondump()",
          "code": "nlohmann::ordered_json PointCloud::jsondump() const {\n    // Alphabetical order to match Rust's serde_json\n    nlohmann::ordered_json data;\n    data[\"colors\"] = _colors;\n    data[\"coords\"] = _coords;\n    data[\"guid\"] = guid;\n    data[\"name\"] = name;\n    data[\"normals\"] = _normals;\n    data[\"point_size\"] = point_size;\n    data[\"type\"] = \"PointCloud\";\n    data[\"xform\"] = xform.jsondump();\n    return data;\n}",
          "file": "pointcloud.cpp"
        },
        "rust": {
          "sig": "jsondump() -> Result<String, Box<dyn std::error::Error>>",
          "code": "pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {\n        let mut buf = Vec::new();\n        let formatter = serde_json::ser::PrettyFormatter::with_indent(b\"  \");\n        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);\n        serde::Serialize::serialize(self, &mut ser)?;\n        Ok(String::from_utf8(buf)?)\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.jsonload",
      "implementations": {
        "cpp": {
          "sig": "PointCloud jsonload(const nlohmann::json& data)",
          "code": "PointCloud PointCloud::jsonload(const nlohmann::json& data) {\n    std::vector<double> coords = data.value(\"coords\", std::vector<double>{}",
          "file": "pointcloud.cpp"
        },
        "rust": {
          "sig": "jsonload(json_str: &str) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn jsonload(json_str: &str) -> Result<Self, Box<dyn std::error::Error>> {\n        Ok(serde_json::from_str(json_str)?)\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "Polyline.constructor",
      "implementations": {
        "cpp": {
          "sig": "Polyline()",
          "code": "Polyline();",
          "file": "polyline.h"
        }
      }
    },
    {
      "name": "Polyline.Polyline",
      "implementations": {
        "cpp": {
          "sig": "explicit Polyline(const std::vector<Point>& pts)",
          "code": "explicit Polyline(const std::vector<Point>& pts);",
          "file": "polyline.h"
        }
      }
    },
    {
      "name": "Polyline.str",
      "implementations": {
        "cpp": {
          "sig": "std::string str()",
          "code": "std::string Polyline::str() const {\n    std::ostringstream oss;\n    oss << \"[\";\n    for (size_t i = 0; i < point_count(); i++) {\n        if (i > 0) oss << \", \";\n        size_t idx = i * 3;\n        oss << \"(\" << _coords[idx] << \", \" << _coords[idx + 1] << \", \" << _coords[idx + 2] << \")\";\n    }",
          "file": "polyline.cpp"
        }
      }
    },
    {
      "name": "Polyline.repr",
      "implementations": {
        "cpp": {
          "sig": "std::string repr()",
          "code": "std::string Polyline::repr() const {\n    return \"Polyline(\" + name + \", \" + std::to_string(point_count()) + \" points)\";\n}",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "repr() -> String",
          "code": "pub fn repr(&self) -> String {\n        format!(\"Polyline({}, {} points)\", self.name, self.point_count())\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.len",
      "implementations": {
        "cpp": {
          "sig": "size_t len()",
          "code": "size_t Polyline::len() const {\n    return point_count();\n}",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "len() -> usize",
          "code": "pub fn len(&self) -> usize {\n        self.point_count()\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.jsondump",
      "implementations": {
        "cpp": {
          "sig": "nlohmann::ordered_json jsondump()",
          "code": "nlohmann::ordered_json Polyline::jsondump() const {\n    // Alphabetical order to match Rust's serde_json\n    nlohmann::ordered_json j;\n    j[\"coords\"] = _coords;\n    j[\"guid\"] = guid;\n    j[\"linecolor\"] = linecolor.jsondump();\n    j[\"name\"] = name;\n    j[\"type\"] = \"Polyline\";\n    j[\"width\"] = width;\n    j[\"xform\"] = xform.jsondump();\n    return j;\n}",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "jsondump() -> Result<String, Box<dyn std::error::Error>>",
          "code": "pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {\n         let mut buf = Vec::new();\n         let formatter = serde_json::ser::PrettyFormatter::with_indent(b\"    \");\n         let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);\n         self.serialize(&mut ser)?;\n         Ok(String::from_utf8(buf)?)\n     }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.jsonload",
      "implementations": {
        "cpp": {
          "sig": "Polyline jsonload(const nlohmann::json& data)",
          "code": "Polyline Polyline::jsonload(const nlohmann::json& data) {\n    Polyline polyline;\n    polyline.guid = data[\"guid\"];\n    polyline.name = data[\"name\"];\n\n    // Support both new coords format and legacy points format\n    if (data.contains(\"coords\")) {\n        polyline._coords = data[\"coords\"].get<std::vector<double>>();\n    }",
          "file": "polyline.cpp"
        },
        "rust": {
          "sig": "jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {\n        Ok(serde_json::from_str(json_data)?)\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.length_squared",
      "implementations": {
        "cpp": {
          "sig": "double length_squared()",
          "code": "double Polyline::length_squared() const {\n    double len = 0.0;\n    for (size_t i = 0; i < segment_count(); i++) {\n        size_t idx0 = i * 3;\n        size_t idx1 = (i + 1) * 3;\n        double dx = _coords[idx1] - _coords[idx0];\n        double dy = _coords[idx1 + 1] - _coords[idx0 + 1];\n        double dz = _coords[idx1 + 2] - _coords[idx0 + 2];\n        len += dx * dx + dy * dy + dz * dz;\n    }",
          "file": "polyline.cpp"
        }
      }
    },
    {
      "name": "Polyline.recompute_plane_if_needed",
      "implementations": {
        "cpp": {
          "sig": "void recompute_plane_if_needed()",
          "code": "void Polyline::recompute_plane_if_needed() {\n    if (point_count() >= 3) {\n        std::vector<Point> pts = get_points();\n        plane = Plane::from_points(pts);\n    }",
          "file": "polyline.cpp"
        }
      }
    },
    {
      "name": "Polyline.average_normal",
      "implementations": {
        "cpp": {
          "sig": "void average_normal(Vector& avg_normal)",
          "code": "void Polyline::average_normal(Vector& avg_normal) const {\n    size_t n = point_count();\n    if (n < 3) {\n        avg_normal = Vector(0, 0, 1);\n        return;\n    }",
          "file": "polyline.cpp"
        }
      }
    },
    {
      "name": "std.round",
      "implementations": {
        "cpp": {
          "sig": "return round(value * factor)",
          "code": "return std::round(value * factor) / factor;\n    }",
          "file": "tolerance.h"
        }
      }
    },
    {
      "name": "Tolerance.Tolerance",
      "implementations": {
        "cpp": {
          "sig": "explicit Tolerance(const std::string& unit = \"M\")",
          "code": "explicit Tolerance(const std::string& unit = \"M\");",
          "file": "tolerance.h"
        }
      }
    },
    {
      "name": "Tolerance.set_unit",
      "implementations": {
        "cpp": {
          "sig": "void set_unit(const std::string& value)",
          "code": "void Tolerance::set_unit(const std::string& value) {\n    if (value != \"M\" && value != \"MM\") {\n        throw std::invalid_argument(\"Invalid unit: \" + value);\n    }",
          "file": "tolerance.cpp"
        }
      }
    },
    {
      "name": "Tolerance.set_absolute",
      "implementations": {
        "cpp": {
          "sig": "void set_absolute(double value)",
          "code": "void set_absolute(double value);",
          "file": "tolerance.h"
        },
        "rust": {
          "sig": "set_absolute(value: f64)",
          "code": "pub fn set_absolute(&mut self, value: f64) {\n        self.absolute = Some(value);\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.set_relative",
      "implementations": {
        "cpp": {
          "sig": "void set_relative(double value)",
          "code": "void set_relative(double value);",
          "file": "tolerance.h"
        },
        "rust": {
          "sig": "set_relative(value: f64)",
          "code": "pub fn set_relative(&mut self, value: f64) {\n        self.relative = Some(value);\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.set_angular",
      "implementations": {
        "cpp": {
          "sig": "void set_angular(double value)",
          "code": "void set_angular(double value);",
          "file": "tolerance.h"
        },
        "rust": {
          "sig": "set_angular(value: f64)",
          "code": "pub fn set_angular(&mut self, value: f64) {\n        self.angular = Some(value);\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.set_approximation",
      "implementations": {
        "cpp": {
          "sig": "void set_approximation(double value)",
          "code": "void Tolerance::set_approximation(double value) {\n    _approximation = value;\n    _has_approximation = true;\n}",
          "file": "tolerance.cpp"
        },
        "rust": {
          "sig": "set_approximation(value: f64)",
          "code": "pub fn set_approximation(&mut self, value: f64) {\n        self.approximation = Some(value);\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.set_precision",
      "implementations": {
        "cpp": {
          "sig": "void set_precision(int value)",
          "code": "void Tolerance::set_precision(int value) {\n    if (value == 0) {\n        throw std::invalid_argument(\"Precision cannot be zero.\");\n    }",
          "file": "tolerance.cpp"
        },
        "rust": {
          "sig": "set_precision(value: i32)",
          "code": "pub fn set_precision(&mut self, value: i32) {\n        self.precision = Some(value);\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.set_lineardeflection",
      "implementations": {
        "cpp": {
          "sig": "void set_lineardeflection(double value)",
          "code": "void Tolerance::set_lineardeflection(double value) {\n    _lineardeflection = value;\n    _has_lineardeflection = true;\n}",
          "file": "tolerance.cpp"
        },
        "rust": {
          "sig": "set_lineardeflection(value: f64)",
          "code": "pub fn set_lineardeflection(&mut self, value: f64) {\n        self.lineardeflection = Some(value);\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.set_angulardeflection",
      "implementations": {
        "cpp": {
          "sig": "void set_angulardeflection(double value)",
          "code": "void Tolerance::set_angulardeflection(double value) {\n    _angulardeflection = value;\n    _has_angulardeflection = true;\n}",
          "file": "tolerance.cpp"
        },
        "rust": {
          "sig": "set_angulardeflection(value: f64)",
          "code": "pub fn set_angulardeflection(&mut self, value: f64) {\n        self.angulardeflection = Some(value);\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.constructor",
      "implementations": {
        "cpp": {
          "sig": "Tolerance(const std::string& unit)",
          "code": "Tolerance::Tolerance(const std::string& unit) \n    : _unit(unit), _absolute(0), _relative(0), _angular(0), _approximation(0), \n      _precision(0), _lineardeflection(0), _angulardeflection(0),\n      _has_absolute(false), _has_relative(false), _has_angular(false), \n      _has_approximation(false), _has_precision(false), \n      _has_lineardeflection(false), _has_angulardeflection(false) {\n}",
          "file": "tolerance.cpp"
        }
      }
    },
    {
      "name": "std.stoi",
      "implementations": {
        "cpp": {
          "sig": "return stoi(s.substr(pos + 2)",
          "code": "return std::stoi(s.substr(pos + 2));\n        }",
          "file": "tolerance.cpp"
        }
      }
    },
    {
      "name": "std.isfinite",
      "implementations": {
        "cpp": {
          "sig": "return isfinite(x)",
          "code": "return std::isfinite(x);\n}",
          "file": "tolerance.cpp"
        }
      }
    },
    {
      "name": "Vector.constructor",
      "implementations": {
        "cpp": {
          "sig": "Vector(double x, double y, double z)",
          "code": "Vector(double x, double y, double z) : _x(x), _y(y), _z(z) {}",
          "file": "vector.h"
        }
      }
    },
    {
      "name": "Vector.cached_magnitude",
      "implementations": {
        "cpp": {
          "sig": "double cached_magnitude()",
          "code": "double Vector::cached_magnitude() const {\n  if (!_has_magnitude) {\n    _magnitude = compute_magnitude();\n    _has_magnitude = true;\n  }",
          "file": "vector.cpp"
        }
      }
    },
    {
      "name": "Vector.compute_magnitude",
      "implementations": {
        "cpp": {
          "sig": "double compute_magnitude()",
          "code": "double Vector::compute_magnitude() const {\n  double mag = 0.0;\n\n  double ax = std::abs(_x);\n  double ay = std::abs(_y);\n  double az = std::abs(_z);\n\n  const bool x_zero = ax < static_cast<double>(session_cpp::Tolerance::ZERO_TOLERANCE);\n  const bool y_zero = ay < static_cast<double>(session_cpp::Tolerance::ZERO_TOLERANCE);\n  const bool z_zero = az < static_cast<double>(session_cpp::Tolerance::ZERO_TOLERANCE);\n\n  if (x_zero && y_zero && z_zero)\n    return 0.0;\n  else if (x_zero && y_zero)\n    return az;\n  else if (x_zero && z_zero)\n    return ay;\n  else if (y_zero && z_zero)\n    return ax;\n\n  // Ensure ax is the largest\n  if (ay >= ax && ay >= az) {\n    std::swap(ax, ay);\n  }",
          "file": "vector.cpp"
        }
      }
    },
    {
      "name": "Vector.to_string",
      "implementations": {
        "cpp": {
          "sig": "std::string to_string()",
          "code": "std::string Vector::to_string() const {\n  return fmt::format(\"Vector({}",
          "file": "vector.cpp"
        }
      }
    },
    {
      "name": "Vector.jsondump",
      "implementations": {
        "cpp": {
          "sig": "nlohmann::ordered_json jsondump()",
          "code": "nlohmann::ordered_json Vector::jsondump() const {\n  auto clean_float = [](double val) -> double { return std::round(val * 100.0) / 100.0; }",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "jsondump() -> Result<String, Box<dyn std::error::Error>>",
          "code": "pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {\n        let mut buf = Vec::new();\n        let formatter = serde_json::ser::PrettyFormatter::with_indent(b\"    \");\n        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);\n        SerTrait::serialize(self, &mut ser)?;\n        Ok(String::from_utf8(buf)?)\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.jsonload",
      "implementations": {
        "cpp": {
          "sig": "Vector jsonload(const nlohmann::json &data)",
          "code": "Vector Vector::jsonload(const nlohmann::json &data) {\n  Vector vector(data[\"x\"], data[\"y\"], data[\"z\"]);\n  vector.guid = data[\"guid\"];\n  vector.name = data[\"name\"];\n  return vector;\n}",
          "file": "vector.cpp"
        },
        "rust": {
          "sig": "jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {\n        Ok(serde_json::from_str(json_data)?)\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "std.asin",
      "implementations": {
        "cpp": {
          "sig": "return asin((b * std::sin(A * to_rad)",
          "code": "return std::asin((b * std::sin(A * to_rad)) / a) * to_deg;\n}",
          "file": "vector.cpp"
        }
      }
    },
    {
      "name": "std.atan2",
      "implementations": {
        "cpp": {
          "sig": "return atan2(vector[1], vector[0])",
          "code": "return std::atan2(vector[1], vector[0]) * static_cast<double>(Tolerance::TO_DEGREES);\n}",
          "file": "vector.cpp"
        }
      }
    },
    {
      "name": "std.fabs",
      "implementations": {
        "cpp": {
          "sig": "return fabs(dot(other)",
          "code": "return std::fabs(dot(other)) < static_cast<double>(Tolerance::ZERO_TOLERANCE);\n}",
          "file": "vector.cpp"
        }
      }
    },
    {
      "name": "Xform.constructor",
      "implementations": {
        "cpp": {
          "sig": "Xform(const std::array<double, 16>& matrix)",
          "code": "Xform(const std::array<double, 16>& matrix);",
          "file": "xform.h"
        }
      }
    },
    {
      "name": "Xform.jsondump",
      "implementations": {
        "cpp": {
          "sig": "nlohmann::ordered_json jsondump()",
          "code": "nlohmann::ordered_json Xform::jsondump() const {\n    // Alphabetical order to match Rust's serde_json\n    nlohmann::ordered_json data;\n    data[\"guid\"] = guid;\n    data[\"m\"] = m;\n    data[\"name\"] = name;\n    data[\"type\"] = \"Xform\";\n    return data;\n}",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "jsondump() -> Result<String, Box<dyn std::error::Error>>",
          "code": "pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {\n        let mut buf = Vec::new();\n        let formatter = serde_json::ser::PrettyFormatter::with_indent(b\"    \");\n        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);\n        SerTrait::serialize(self, &mut ser)?;\n        Ok(String::from_utf8(buf)?)\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.jsonload",
      "implementations": {
        "cpp": {
          "sig": "Xform jsonload(const nlohmann::json& data)",
          "code": "Xform Xform::jsonload(const nlohmann::json& data) {\n    Xform xform;\n    xform.guid = data[\"guid\"].get<std::string>();\n    xform.name = data[\"name\"].get<std::string>();\n    xform.m = data[\"m\"].get<std::array<double, 16>>();\n    return xform;\n}",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {\n        Ok(serde_json::from_str(json_data)?)\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.str",
      "implementations": {
        "cpp": {
          "sig": "std::string str()",
          "code": "std::string Xform::str() const {\n    std::ostringstream oss;\n    for (int i = 0; i < 4; i++) {\n        oss << \"[\" << fmt::format(\"{:.6f}",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "str() -> String",
          "code": "pub fn str(&self) -> String {\n        let mut rows = Vec::new();\n        for i in 0..4 {\n            rows.push(format!(\n                \"[{:.6}, {:.6}, {:.6}, {:.6}]\",\n                self.m[i],\n                self.m[4 + i],\n                self.m[8 + i],\n                self.m[12 + i]\n            ));\n        }\n        rows.join(\"\\n\")\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.repr",
      "implementations": {
        "cpp": {
          "sig": "std::string repr()",
          "code": "std::string Xform::repr() const {\n    return fmt::format(\"Xform({}",
          "file": "xform.cpp"
        },
        "rust": {
          "sig": "repr() -> String",
          "code": "pub fn repr(&self) -> String {\n        format!(\"Xform({}, {})\", self.name, &self.guid[..8])\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.operator",
      "implementations": {
        "cpp": {
          "sig": "const double& operator()",
          "code": "const double& Xform::operator()(int row, int col) const {\n    if (row < 0 || row >= 4 || col < 0 || col >= 4) {\n        throw std::out_of_range(\"Index out of bounds: (\" + std::to_string(row) + \", \" + std::to_string(col) + \")\");\n    }",
          "file": "xform.cpp"
        }
      }
    },
    {
      "name": "Color.new",
      "implementations": {
        "rust": {
          "sig": "new(r: u8, g: u8, b: u8, a: u8) -> Self",
          "code": "pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {\n        Color {\n            guid: Uuid::new_v4().to_string(),\n            name: \"my_color\".to_string(),\n            r,\n            g,\n            b,\n            a,\n        }\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.with_name",
      "implementations": {
        "rust": {
          "sig": "with_name(r: u8, g: u8, b: u8, a: u8, name: &str) -> Self",
          "code": "pub fn with_name(r: u8, g: u8, b: u8, a: u8, name: &str) -> Self {\n        Color {\n            guid: Uuid::new_v4().to_string(),\n            name: name.to_string(),\n            r,\n            g,\n            b,\n            a,\n        }\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.to_json",
      "implementations": {
        "rust": {
          "sig": "to_json(filepath: &str) -> Result<(), Box<dyn std::error::Error>>",
          "code": "pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {\n        let json = self.jsondump()?;\n        std::fs::write(filepath, json)?;\n        Ok(())\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.from_json",
      "implementations": {
        "rust": {
          "sig": "from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {\n        let json = std::fs::read_to_string(filepath)?;\n        Self::jsonload(&json)\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.to_float_array",
      "implementations": {
        "rust": {
          "sig": "to_float_array() -> [f64; 4]",
          "code": "pub fn to_float_array(&self) -> [f64; 4] {\n        [\n            self.r as f64 / 255.0,\n            self.g as f64 / 255.0,\n            self.b as f64 / 255.0,\n            self.a as f64 / 255.0,\n        ]\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Color.from_float",
      "implementations": {
        "rust": {
          "sig": "from_float(r: f64, g: f64, b: f64, a: f64) -> Self",
          "code": "pub fn from_float(r: f64, g: f64, b: f64, a: f64) -> Self {\n        Color::new(\n            (r * 255.0).round() as u8,\n            (g * 255.0).round() as u8,\n            (b * 255.0).round() as u8,\n            (a * 255.0).round() as u8,\n        )\n    }",
          "file": "color.rs"
        }
      }
    },
    {
      "name": "Line.new",
      "implementations": {
        "rust": {
          "sig": "new(x0: f64, y0: f64, z0: f64, x1: f64, y1: f64, z1: f64) -> Self",
          "code": "pub fn new(x0: f64, y0: f64, z0: f64, x1: f64, y1: f64, z1: f64) -> Self {\n        Self {\n            _x0: x0,\n            _y0: y0,\n            _z0: z0,\n            _x1: x1,\n            _y1: y1,\n            _z1: z1,\n            ..Default::default()\n        }\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.to_json",
      "implementations": {
        "rust": {
          "sig": "to_json(filepath: &str) -> Result<(), Box<dyn std::error::Error>>",
          "code": "pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {\n        let json = self.jsondump()?;\n        std::fs::write(filepath, json)?;\n        Ok(())\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "Line.from_json",
      "implementations": {
        "rust": {
          "sig": "from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {\n        let json = std::fs::read_to_string(filepath)?;\n        Self::jsonload(&json)\n    }",
          "file": "line.rs"
        }
      }
    },
    {
      "name": "VertexData.new",
      "implementations": {
        "rust": {
          "sig": "new(point: Point) -> Self",
          "code": "pub fn new(point: Point) -> Self {\n        Self {\n            x: point[0],\n            y: point[1],\n            z: point[2],\n            attributes: HashMap::new(),\n        }\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.new",
      "implementations": {
        "rust": {
          "sig": "new() -> Self",
          "code": "pub fn new() -> Self {\n        let mut default_vertex_attributes = HashMap::new();\n        default_vertex_attributes.insert(\"x\".to_string(), 0.0);\n        default_vertex_attributes.insert(\"y\".to_string(), 0.0);\n        default_vertex_attributes.insert(\"z\".to_string(), 0.0);\n\n        Mesh {\n            halfedge: HashMap::new(),\n            vertex: HashMap::new(),\n            face: HashMap::new(),\n            facedata: HashMap::new(),\n            edgedata: HashMap::new(),\n            default_vertex_attributes,\n            default_face_attributes: HashMap::new(),\n            default_edge_attributes: HashMap::new(),\n            triangulation: HashMap::new(),\n            max_vertex: 0,\n            max_face: 0,\n            guid: uuid::Uuid::new_v4().to_string(),\n            name: \"my_mesh\".t",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.ray_cast_bvh",
      "implementations": {
        "rust": {
          "sig": "ray_cast_bvh(ray: &Line, epsilon: f64) -> Option<Point>",
          "code": "pub fn ray_cast_bvh(&mut self, ray: &Line, epsilon: f64) -> Option<Point> {\n        self.ensure_triangle_bvh();\n        let bvh = match &self.tri_bvh {\n            Some(b) => b,\n            None => return None,\n        };\n\n        let origin = ray.start();\n        let dir = ray.to_vector();\n        let len = dir.magnitude();\n        if len <= Tolerance::ZERO_TOLERANCE {\n            return None;\n        }\n        let dir_unit = Vector::new(dir[0] / len, dir[1] / len, dir[2] / len);\n\n        let mut candidate_ids: Vec<usize> = Vec::new();\n        bvh.ray_cast(&origin, &dir_unit, &mut candidate_ids, true);\n        if candidate_ids.is_empty() {\n            return None;\n        }\n\n        let mut best_t = f64::INFINITY;\n        let mut best_p: Option<Point> = None;\n\n        for idx in cand",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.to_json",
      "implementations": {
        "rust": {
          "sig": "to_json(filename: &str) -> std::io::Result<()>",
          "code": "pub fn to_json(&self, filename: &str) -> std::io::Result<()> {\n        let data = self.jsondump();\n        std::fs::write(filename, serde_json::to_string_pretty(&data)?)\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "Mesh.from_json",
      "implementations": {
        "rust": {
          "sig": "from_json(filename: &str) -> std::io::Result<Self>",
          "code": "pub fn from_json(filename: &str) -> std::io::Result<Self> {\n        let content = std::fs::read_to_string(filename)?;\n        let data: serde_json::Value = serde_json::from_str(&content)?;\n        Self::jsonload(&data).ok_or_else(|| {\n            std::io::Error::new(std::io::ErrorKind::InvalidData, \"Invalid mesh data\")\n        })\n    }",
          "file": "mesh.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.new",
      "implementations": {
        "rust": {
          "sig": "new(dimension: usize, is_rational: bool, order: usize, cv_count: usize) -> Self",
          "code": "pub fn new(dimension: usize, is_rational: bool, order: usize, cv_count: usize) -> Self {\n        let cv_stride = if is_rational { dimension + 1 } else { dimension };\n        let knot_count = if order > 0 && cv_count >= order { order + cv_count - 2 } else { 0 };\n        \n        NurbsCurve {\n            m_dim: dimension,\n            m_is_rat: is_rational,\n            m_order: order,\n            m_cv_count: cv_count,\n            m_cv_stride: cv_stride,\n            m_knot: vec![0.0; knot_count],\n            m_cv: vec![0.0; cv_count * cv_stride],\n        }\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.default",
      "implementations": {
        "rust": {
          "sig": "default() -> Self",
          "code": "pub fn default() -> Self {\n        NurbsCurve {\n            m_dim: 0,\n            m_is_rat: false,\n            m_order: 0,\n            m_cv_count: 0,\n            m_cv_stride: 0,\n            m_knot: Vec::new(),\n            m_cv: Vec::new(),\n        }\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.set_cv_point",
      "implementations": {
        "rust": {
          "sig": "set_cv_point(index: usize, point: &Point) -> bool",
          "code": "pub fn set_cv_point(&mut self, index: usize, point: &Point) -> bool {\n        if index >= self.m_cv_count {\n            return false;\n        }\n        self.set_cv(index, point);\n        true\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "NurbsCurve.cv_array_mut",
      "implementations": {
        "rust": {
          "sig": "cv_array_mut() -> &mut [f64]",
          "code": "pub fn cv_array_mut(&mut self) -> &mut [f64] {\n        &mut self.m_cv\n    }",
          "file": "nurbscurve.rs"
        }
      }
    },
    {
      "name": "Plane.new",
      "implementations": {
        "rust": {
          "sig": "new(point: Point, mut x_axis: Vector, mut y_axis: Vector) -> Self",
          "code": "pub fn new(point: Point, mut x_axis: Vector, mut y_axis: Vector) -> Self {\n        x_axis.normalize();\n        let dot_product = y_axis.dot(&x_axis);\n        y_axis -= x_axis.clone() * dot_product;\n        y_axis.normalize();\n        let mut z_axis = x_axis.cross(&y_axis);\n        z_axis.normalize();\n\n        let a = z_axis[0];\n        let b = z_axis[1];\n        let c = z_axis[2];\n        let d = -(a * point[0] + b * point[1] + c * point[2]);\n\n        Self {\n            guid: Uuid::new_v4().to_string(),\n            name: \"my_plane\".to_string(),\n            width: 1.0,\n            _origin: point,\n            _x_axis: x_axis,\n            _y_axis: y_axis,\n            _z_axis: z_axis,\n            _a: a,\n            _b: b,\n            _c: c,\n            _d: d,\n            xform: Xform::iden",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.with_name",
      "implementations": {
        "rust": {
          "sig": "with_name(point: Point, mut x_axis: Vector, mut y_axis: Vector, name: String) -> Self",
          "code": "pub fn with_name(point: Point, mut x_axis: Vector, mut y_axis: Vector, name: String) -> Self {\n        x_axis.normalize();\n        let dot_product = y_axis.dot(&x_axis);\n        y_axis -= x_axis.clone() * dot_product;\n        y_axis.normalize();\n        let mut z_axis = x_axis.cross(&y_axis);\n        z_axis.normalize();\n\n        let a = z_axis[0];\n        let b = z_axis[1];\n        let c = z_axis[2];\n        let d = -(a * point[0] + b * point[1] + c * point[2]);\n\n        Self {\n            guid: Uuid::new_v4().to_string(),\n            name,\n            width: 1.0,\n            _origin: point,\n            _x_axis: x_axis,\n            _y_axis: y_axis,\n            _z_axis: z_axis,\n            _a: a,\n            _b: b,\n            _c: c,\n            _d: d,\n            xform: Xform::identit",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.origin_ref",
      "implementations": {
        "rust": {
          "sig": "origin_ref() -> &Point",
          "code": "pub fn origin_ref(&self) -> &Point {\n        &self._origin\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.x_axis_ref",
      "implementations": {
        "rust": {
          "sig": "x_axis_ref() -> &Vector",
          "code": "pub fn x_axis_ref(&self) -> &Vector {\n        &self._x_axis\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.y_axis_ref",
      "implementations": {
        "rust": {
          "sig": "y_axis_ref() -> &Vector",
          "code": "pub fn y_axis_ref(&self) -> &Vector {\n        &self._y_axis\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Plane.z_axis_ref",
      "implementations": {
        "rust": {
          "sig": "z_axis_ref() -> &Vector",
          "code": "pub fn z_axis_ref(&self) -> &Vector {\n        &self._z_axis\n    }",
          "file": "plane.rs"
        }
      }
    },
    {
      "name": "Point.new",
      "implementations": {
        "rust": {
          "sig": "new(x: f64, y: f64, z: f64) -> Self",
          "code": "pub fn new(x: f64, y: f64, z: f64) -> Self {\n        Self {\n            _x: x,\n            _y: y,\n            _z: z,\n            ..Default::default()\n        }\n    }",
          "file": "point.rs"
        }
      }
    },
    {
      "name": "Point.with_name",
      "implementations": {
        "rust": {
          "sig": "with_name(x: f64, y: f64, z: f64, name: &str) -> Self",
          "code": "pub fn with_name(x: f64, y: f64, z: f64, name: &str) -> Self {\n        Self {\n            _x: x,\n            _y: y,\n            _z: z,\n            name: name.to_string(),\n            ..Default::default()\n        }\n    }",
          "file": "point.rs"
        }
      }
    },
    {
      "name": "Point.to_json",
      "implementations": {
        "rust": {
          "sig": "to_json(filepath: &str) -> Result<(), Box<dyn std::error::Error>>",
          "code": "pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {\n        let json = self.jsondump()?;\n        std::fs::write(filepath, json)?;\n        Ok(())\n    }",
          "file": "point.rs"
        }
      }
    },
    {
      "name": "Point.from_json",
      "implementations": {
        "rust": {
          "sig": "from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {\n        let json = std::fs::read_to_string(filepath)?;\n        Self::jsonload(&json)\n    }",
          "file": "point.rs"
        }
      }
    },
    {
      "name": "PointCloud.new",
      "implementations": {
        "rust": {
          "sig": "new(points: Vec<Point>, normals: Vec<Vector>, colors: Vec<Color>) -> Self",
          "code": "pub fn new(points: Vec<Point>, normals: Vec<Vector>, colors: Vec<Color>) -> Self {\n        let mut pc = Self::default();\n\n        pc._coords.reserve(points.len() * 3);\n        for p in &points {\n            pc._coords.push(p[0]);\n            pc._coords.push(p[1]);\n            pc._coords.push(p[2]);\n        }\n\n        pc._colors.reserve(colors.len() * 4);\n        for c in &colors {\n            pc._colors.push(c.r as i32);\n            pc._colors.push(c.g as i32);\n            pc._colors.push(c.b as i32);\n            pc._colors.push(c.a as i32);\n        }\n\n        pc._normals.reserve(normals.len() * 3);\n        for n in &normals {\n            pc._normals.push(n[0]);\n            pc._normals.push(n[1]);\n            pc._normals.push(n[2]);\n        }\n\n        pc\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "PointCloud.len",
      "implementations": {
        "rust": {
          "sig": "len() -> usize",
          "code": "pub fn len(&self) -> usize {\n        self.point_count()\n    }",
          "file": "pointcloud.rs"
        }
      }
    },
    {
      "name": "Polyline.new",
      "implementations": {
        "rust": {
          "sig": "new(points: Vec<Point>) -> Self",
          "code": "pub fn new(points: Vec<Point>) -> Self {\n        // Convert points to flat coords\n        let mut coords = Vec::with_capacity(points.len() * 3);\n        for p in &points {\n            coords.push(p[0]);\n            coords.push(p[1]);\n            coords.push(p[2]);\n        }\n        \n        // Delegate plane computation to Plane::from_points\n        let plane = if points.len() >= 3 {\n            Plane::from_points(points)\n        } else {\n            Plane::default()\n        };\n\n        Self {\n            guid: Uuid::new_v4().to_string(),\n            name: \"my_polyline\".to_string(),\n            coords,\n            plane,\n            width: 1.0,\n            linecolor: Color::white(),\n            xform: Xform::identity(),\n        }\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.to_json",
      "implementations": {
        "rust": {
          "sig": "to_json(filepath: &str) -> Result<(), Box<dyn std::error::Error>>",
          "code": "pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {\n        let json = self.jsondump()?;\n        std::fs::write(filepath, json)?;\n        Ok(())\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.from_json",
      "implementations": {
        "rust": {
          "sig": "from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {\n        let json = std::fs::read_to_string(filepath)?;\n        Self::jsonload(&json)\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.center_vec",
      "implementations": {
        "rust": {
          "sig": "center_vec() -> Vector",
          "code": "pub fn center_vec(&self) -> Vector {\n        let center = self.center();\n        Vector::new(center[0], center[1], center[2])\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.extend_line",
      "implementations": {
        "rust": {
          "sig": "extend_line(\n        line_start: &mut Point,\n        line_end: &mut Point,\n        distance0: f64,\n        distance1: f64,\n    )",
          "code": "pub fn extend_line(\n        line_start: &mut Point,\n        line_end: &mut Point,\n        distance0: f64,\n        distance1: f64,\n    ) {\n        let mut v = line_end.clone() - line_start.clone();\n        v.normalize();\n\n        *line_start = line_start.clone() - (v.clone() * distance0);\n        *line_end = line_end.clone() + (v * distance1);\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.scale_line",
      "implementations": {
        "rust": {
          "sig": "scale_line(line_start: &mut Point, line_end: &mut Point, distance: f64)",
          "code": "pub fn scale_line(line_start: &mut Point, line_end: &mut Point, distance: f64) {\n        let v = line_end.clone() - line_start.clone();\n        *line_start = line_start.clone() + (v.clone() * distance);\n        *line_end = line_end.clone() - (v * distance);\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.move_by",
      "implementations": {
        "rust": {
          "sig": "move_by(direction: &Vector)",
          "code": "pub fn move_by(&mut self, direction: &Vector) {\n        for i in 0..self.point_count() {\n            let idx = i * 3;\n            self.coords[idx] += direction[0];\n            self.coords[idx + 1] += direction[1];\n            self.coords[idx + 2] += direction[2];\n        }\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Polyline.flip",
      "implementations": {
        "rust": {
          "sig": "flip()",
          "code": "pub fn flip(&mut self) {\n        self.reverse();\n    }",
          "file": "polyline.rs"
        }
      }
    },
    {
      "name": "Tolerance.new",
      "implementations": {
        "rust": {
          "sig": "new(unit: &str) -> Self",
          "code": "pub fn new(unit: &str) -> Self {\n        Self {\n            unit: unit.to_string(),\n            absolute: None,\n            relative: None,\n            angular: None,\n            approximation: None,\n            precision: None,\n            lineardeflection: None,\n            angulardeflection: None,\n        }\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Tolerance.round_to",
      "implementations": {
        "rust": {
          "sig": "round_to(value: f64, ndigits: i32) -> f64",
          "code": "pub fn round_to(value: f64, ndigits: i32) -> f64 {\n        let factor = 10f64.powi(ndigits);\n        (value * factor).round() / factor\n    }",
          "file": "tolerance.rs"
        }
      }
    },
    {
      "name": "Vector.new",
      "implementations": {
        "rust": {
          "sig": "new(x: f64, y: f64, z: f64) -> Self",
          "code": "pub fn new(x: f64, y: f64, z: f64) -> Self {\n        Self {\n            _x: x,\n            _y: y,\n            _z: z,\n            ..Default::default()\n        }\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.with_name",
      "implementations": {
        "rust": {
          "sig": "with_name(x: f64, y: f64, z: f64, name: &str) -> Self",
          "code": "pub fn with_name(x: f64, y: f64, z: f64, name: &str) -> Self {\n        Self {\n            _x: x,\n            _y: y,\n            _z: z,\n            name: name.to_string(),\n            ..Default::default()\n        }\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.normalized",
      "implementations": {
        "rust": {
          "sig": "normalized() -> Self",
          "code": "pub fn normalized(&self) -> Self {\n        let mut result = self.clone();\n        result.normalize();\n        result\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.to_json",
      "implementations": {
        "rust": {
          "sig": "to_json(filepath: &str) -> Result<(), Box<dyn std::error::Error>>",
          "code": "pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {\n        let json = self.jsondump()?;\n        std::fs::write(filepath, json)?;\n        Ok(())\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Vector.from_json",
      "implementations": {
        "rust": {
          "sig": "from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {\n        let json = std::fs::read_to_string(filepath)?;\n        Self::jsonload(&json)\n    }",
          "file": "vector.rs"
        }
      }
    },
    {
      "name": "Xform.new",
      "implementations": {
        "rust": {
          "sig": "new() -> Self",
          "code": "pub fn new() -> Self {\n        Self::identity()\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.from_cols",
      "implementations": {
        "rust": {
          "sig": "from_cols(col_x: Vector, col_y: Vector, col_z: Vector) -> Self",
          "code": "pub fn from_cols(col_x: Vector, col_y: Vector, col_z: Vector) -> Self {\n        let mut xform = Self::identity();\n        xform.m[0] = col_x[0];\n        xform.m[1] = col_x[1];\n        xform.m[2] = col_x[2];\n        xform.m[4] = col_y[0];\n        xform.m[5] = col_y[1];\n        xform.m[6] = col_y[2];\n        xform.m[8] = col_z[0];\n        xform.m[9] = col_z[1];\n        xform.m[10] = col_z[2];\n        xform\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.x",
      "implementations": {
        "rust": {
          "sig": "x() -> Vector",
          "code": "pub fn x(&self) -> Vector {\n        Vector::new(self.m[0], self.m[1], self.m[2])\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.y",
      "implementations": {
        "rust": {
          "sig": "y() -> Vector",
          "code": "pub fn y(&self) -> Vector {\n        Vector::new(self.m[4], self.m[5], self.m[6])\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.z",
      "implementations": {
        "rust": {
          "sig": "z() -> Vector",
          "code": "pub fn z(&self) -> Vector {\n        Vector::new(self.m[8], self.m[9], self.m[10])\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.to_json",
      "implementations": {
        "rust": {
          "sig": "to_json(filepath: &str) -> Result<(), Box<dyn std::error::Error>>",
          "code": "pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {\n        let json = self.jsondump()?;\n        std::fs::write(filepath, json)?;\n        Ok(())\n    }",
          "file": "xform.rs"
        }
      }
    },
    {
      "name": "Xform.from_json",
      "implementations": {
        "rust": {
          "sig": "from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {\n        let json = std::fs::read_to_string(filepath)?;\n        Self::jsonload(&json)\n    }",
          "file": "xform.rs"
        }
      }
    }
  ]
};
