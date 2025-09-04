# Different Sphinx theme configurations with persistent sidebars
# Copy the desired configuration to your conf.py file

# ============================================================================
# 1. FURO THEME (Current - Recommended for C++ docs feel)
# ============================================================================
"""
html_theme = 'furo'

html_theme_options = {
    "sidebar_hide_name": False,
    "navigation_with_keys": True,
    "top_of_page_button": "edit",
    "source_repository": "https://github.com/your-username/session_py/",
    "source_branch": "main",
    "source_directory": "docs/",
}
"""

# ============================================================================
# 2. SPHINX BOOK THEME (GitBook-like, persistent sidebar)
# ============================================================================
"""
html_theme = 'sphinx_book_theme'

html_theme_options = {
    "repository_url": "https://github.com/your-username/session_py",
    "use_repository_button": True,
    "use_issues_button": True,
    "use_edit_page_button": True,
    "path_to_docs": "docs/",
    "repository_branch": "main",
    "launch_buttons": {
        "binderhub_url": "",
        "colab_url": "",
    },
    "toc_title": "Contents",
    "show_navbar_depth": 2,
}
"""

# ============================================================================
# 3. PYDATA THEME (Scientific Python ecosystem standard)
# ============================================================================
"""
html_theme = 'pydata_sphinx_theme'

html_theme_options = {
    "github_url": "https://github.com/your-username/session_py",
    "twitter_url": "",
    "navbar_start": ["navbar-logo"],
    "navbar_center": ["navbar-nav"],
    "navbar_end": ["navbar-icon-links"],
    "navbar_persistent": ["search-button"],
    "primary_sidebar_end": ["sidebar-ethical-ads"],
    "secondary_sidebar_items": ["page-toc", "edit-this-page", "sourcelink"],
    "footer_items": ["copyright", "sphinx-version"],
    "show_toc_level": 2,
}
"""

# ============================================================================
# 4. CLASSIC THEME (Traditional Sphinx, very stable sidebar)
# ============================================================================
"""
html_theme = 'classic'

html_theme_options = {
    "rightsidebar": "false",
    "relbarbgcolor": "#000000",
    "sidebarbgcolor": "#ffffff", 
    "sidebartextcolor": "#000000",
    "sidebarlinkcolor": "#0000ee",
    "relbartextcolor": "#ffffff",
    "relbarlinkcolor": "#ffffff",
    "bgcolor": "#ffffff",
    "textcolor": "#000000",
    "headbgcolor": "#f2f2f2",
    "linkcolor": "#0000ee",
    "visitedlinkcolor": "#551a8b",
    "codebgcolor": "#eeffcc",
    "codetextcolor": "#333333"
}
"""

# ============================================================================
# 5. NATURE THEME (Simple, stable, similar to Python docs)
# ============================================================================
"""
html_theme = 'nature'
html_theme_options = {}
"""
