create_project attiny_tinkering vivado -part xc7a35ticsg324-1L
add_files top.sv
add_files -fileset constrs_1 test.xdc
close_project
open_project vivado/attiny_tinkering.xpr
launch_runs synth_1 -jobs 8
wait_on_runs synth_1
launch_runs impl_1 -to_step write_bitstream -jobs 8
wait_on_runs impl_1
