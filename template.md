# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning]. The file is auto-generated using [Conventional Commits].

[keep a changelog]: https://keepachangelog.com/en/1.0.0/
[semantic versioning]: https://semver.org/spec/v2.0.0.html
[conventional commits]: https://www.conventionalcommits.org/en/v1.0.0/

## Overview

- [unreleased](#unreleased)

{%- for release in releases %}
- [`{{ release.version }}`](#{{ release.version }}) – _{{ release.date | date(format="%Y.%m.%d")}}_
{%- endfor %}

## _[Unreleased]_

{% if unreleased.changes -%}
{%- for change in unreleased.changes -%}
{% if change.merge_commit_description -%}
- {{ change.type }}: {{ change.merge_commit_description.description }} ([#{{change.merge_commit_description.pr_number }}][pr#{{ change.merge_commit_description.pr_number }}]) ([`{{ change.commit.short_id }}`])
{%- else -%}
- {{ change.type }}: {{ change.description }} ([`{{ change.commit.short_id }}`])
{%- endif %}
{% endfor %}
{% else -%}
_nothing new to show for… yet!_

{% endif -%}
{%- for release in releases -%}
<a id="{{ release.version }}" />

## [{{ release.version }}]{% if release.title %} – _{{ release.title }}_{% endif %}

_{{ release.date | date(format="%Y.%m.%d") }}_
{%- if release.notes %}

{{ release.notes }}
{% endif -%}
{%- set ignored_contributors = get_env(name="IGNORE_CONTRIBUTORS", default="") | split(pat=",") -%}
{%- set_global contributors = [] -%}
{%- for contributor in release.changeset.contributors -%}
  {%- if ignored_contributors is not containing(contributor.email) -%}
    {%- set_global contributors = contributors | concat(with=contributor) -%}
  {%- endif -%}
{%- endfor -%}
{%- if contributors %}

### Contributions

This release is made possible by the following people (in alphabetical order).
Thank you all for your contributions. Your work – no matter how significant – is
greatly appreciated by the community. 💖
{% for contributor in contributors %}
- {{ contributor.name }} (<{{ contributor.email }}>)
{%- endfor %}
{%- endif %}

### Changes

{% for type, changes in release.changeset.changes | group_by(attribute="type") -%}

#### {{ type | typeheader }}

{% for change in changes -%}
- **{{ change.description }}** ([`{{ change.commit.short_id }}`])

{% if change.body -%}
{{ change.body | indent(n=2) }}

{% endif -%}
{%- endfor -%}

{% endfor %}
{%- endfor -%}

<!-- [releases] -->

{% if config.github.repo -%}
  {%- set url = "https://github.com/" ~ config.github.repo -%}
{%- else -%}
  {%- set url = "#" -%}
{%- endif -%}
{% if releases -%}
[unreleased]: {{ url }}/compare/v{{ releases | first | get(key="version") }}...HEAD
{%- else -%}
[unreleased]: {{ url }}/commits
{%- endif -%}
{%- for release in releases %}
[{{ release.version }}]: {{ url }}/releases/tag/v{{ release.version }}
{%- endfor %}

<!-- [commits] -->
{% for change in unreleased.changes %}
[`{{ change.commit.short_id }}`]: {{ url }}/commit/{{ change.commit.id }}
{%- endfor -%}
{%- for release in releases %}
{%- for change in release.changeset.changes %}
[`{{ change.commit.short_id }}`]: {{ url }}/commit/{{ change.commit.id }}
{%- endfor -%}
{%- endfor %}

<!-- [pull requests] -->

{% for change in unreleased.changes %}
{%- if change.merge_commit_description -%}
[pr#{{ change.merge_commit_description.pr_number }}]: {{ url }}/pull/{{ change.merge_commit_description.pr_number }}
{% endif -%}
{%- endfor -%}
{%- for release in releases %}
{%- for change in release.changeset.changes %}
{%- if change.merge_commit_description -%}
[pr#{{ change.merge_commit_description.pr_number }}]: {{ url }}/pull/{{ change.merge_commit_description.pr_number }}
{% endif -%}
{%- endfor -%}
{%- endfor -%}
