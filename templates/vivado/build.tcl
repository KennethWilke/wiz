{% include "vivado/create_project.tcl" %}

open_project vivado/{{ project_name }}.xpr

# Synthesize the design
launch_runs synth_1 -jobs {{ num_jobs }}
wait_on_runs synth_1

# Implement the design
launch_runs impl_1 -to_step write_bitstream -jobs {{ num_jobs }}
wait_on_runs impl_1
