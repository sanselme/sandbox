transcript off

vcom ../pkg/rtl/demos/shiftreg.vhd
vcom ../pkg/rtl/demos/shiftreg_tb.vhd

vsim shiftreg_tb
add wave sim:/shiftreg_tb/DUT/*

run 350 ns
