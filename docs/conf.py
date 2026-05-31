# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = "rslua"
copyright = "2026, benniekiss"
author = "benniekiss"

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

exclude_patterns = ["LICENSE.*"]

extensions = [
    "myst_parser",
    "sphinx.ext.viewcode",
    "sphinx_copybutton",
    "sphinx_lua_ls",
    "sphinx_inline_tabs",
]

# -- myst-parser --------------------------------------------------------------
# https://github.com/executablebooks/MyST-Parser

myst_enable_extensions = [
    "amsmath",
    "attrs_inline",
    "colon_fence",
    "deflist",
    "dollarmath",
    "fieldlist",
    "html_admonition",
    "html_image",
    "replacements",
    "smartquotes",
    "strikethrough",
    "substitution",
    "tasklist",
]

# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = "furo"

# -- lua ls -------------------------------------------------------------------
# Path to the folder containing the `.emmyrc.json`/`.luarc.json` file,
# relative to the directory with `conf.py`.
lua_ls_project_root = "../"
lua_ls_backend = "emmylua"
lua_ls_class_default_function_name = "new"
lua_ls_apidoc_format = "md"

lua_ls_project_directories = [
    "crates/minijinja-lua/library/",
    # "crates/jsonschema-lua/library/",
    "crates/rsjson-lua/library/",
    "crates/rsre-lua/library/",
]

lua_ls_apidoc_roots = {
    "minijinja": "api/minijinja",
    # "jsonschema": "api/jsonschema",
    "rsjson": "api/rsjson",
    "rsre": "api/rsre",
}

lua_ls_apidoc_default_options = {
    "members": "",
    "undoc-members": "",
    "private-members": "",
    "recursive": "",
    "index-table": "",
    "inherited-members-table": "",
    "class-doc-from": "separate",
    "annotate-require": "always",
    "class-signature": "both",
}
