open_hw_manager
connect_hw_server -allow_non_jtag

open_hw_target

set_property PROGRAM.FILE { {{ project_name }}.bit} [get_hw_devices {{ target_device }}]

program_hw_devices [get_hw_devices {{ target_device }}]
