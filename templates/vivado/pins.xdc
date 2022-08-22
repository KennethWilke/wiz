{% for name, pin in pins -%}
set_property PACKAGE_PIN {{ pin }} [get_ports {{ name }}]
set_property IOSTANDARD LVCMOS33 [get_ports {{ name }}]
{% endfor -%}
