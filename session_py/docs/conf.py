# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

import os
import sys

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = 'session_py'
copyright = '2024, Petras Vestartas'
author = 'Petras Vestartas'
release = '0.1.0'

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

# Add the source directory to the Python path so Sphinx can import the modules
sys.path.insert(0, os.path.abspath('../src'))

extensions = [
    'sphinx.ext.autodoc',
    'sphinx.ext.viewcode',
    'sphinx.ext.napoleon',
]

templates_path = ['_templates']
exclude_patterns = ['_build', 'Thumbs.db', '.DS_Store']

# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = 'sphinx_rtd_theme'
html_static_path = ['_static']

# Custom build directory
html_build_dir = '../docs_output'

# Disable some navigation elements
html_show_sphinx = False
html_show_copyright = False

# -- Extension configuration -------------------------------------------------

# Autodoc settings - more concise
autodoc_default_options = {
    'members': True,
    'member-order': 'bysource',
    'special-members': '__init__',
    'undoc-members': False,  # Hide undocumented members
    'exclude-members': '__weakref__, __dict__, __module__'
}

# Don't add class names to toctree to avoid duplication
html_theme_options = {
    'navigation_depth': 2,
}

# Napoleon settings for Google/NumPy style docstrings - concise
napoleon_google_docstring = True
napoleon_numpy_docstring = True
napoleon_include_init_with_doc = False
napoleon_include_private_with_doc = False
napoleon_include_special_with_doc = False
