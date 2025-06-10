<!--
Config(
  github: ( repo: "rustic-games/jilu" ),
  accept_types: ["feat", "fix", "perf"],
)

Template(
{%- set release = releases | first -%}
v{{ release.version }}{% if release.subject %} — {{ release.subject }}{% endif %}

{%- if release.notes %}

{{ release.notes }}

---
{%- endif %}
{% for type, changes in release.changeset.changes | group_by(attribute="type") %}
**{{ type | typeheader }}**

{% for change in changes -%}
{% if change.merge_commit_description -%}
- [`{{ change.commit.short_id }}`] {% if change.scope %}{{ change.scope }}: {% endif %}{{ change.merge_commit_description.description }} ([#{{change.merge_commit_description.pr_number }}][pr#{{ change.merge_commit_description.pr_number }}])
{%- else -%}
- [`{{ change.commit.short_id }}`] {% if change.scope %}{{ change.scope }}: {% endif %}{{ change.description }}
{%- endif %}
{% endfor -%}
{% endfor %}

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
{% endfor %}
{%- endif %}

{%- if config.github.repo -%}
  {%- set url = "https://github.com/" ~ config.github.repo -%}
{%- else -%}
  {%- set url = "#" -%}
{%- endif -%}
{% for change in release.changeset.changes %}
[`{{ change.commit.short_id }}`]: {{ url }}/commit/{{ change.commit.id }}
{%- endfor -%}
{%- for change in release.changeset.changes %}
{%- if change.merge_commit_description -%}
[pr#{{ change.merge_commit_description.pr_number }}]: {{ url }}/pull/{{ change.merge_commit_description.pr_number }}
{% endif -%}
{%- endfor -%}
)
-->
