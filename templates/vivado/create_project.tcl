# Create project
create_project {{ project_name }} vivado -part {{ target_part }}

# Add source files
{%- for file in sources %}
add_files {{ file }}
{%- endfor %}

# Add constraint files
{%- for file in constraints %}
add_files -fileset constrs_1 {{ file }}
{%- endfor %}

close_project
