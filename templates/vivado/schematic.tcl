open_project vivado/{{ project_name }}.xpr
synth_design -rtl -rtl_skip_mlo -name rtl_1
write_schematic -format pdf -orientation portrait {{ project_name }}-schematic.pdf
