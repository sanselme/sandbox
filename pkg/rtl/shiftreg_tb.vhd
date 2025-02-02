library ieee;

use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity shiftreg_tb is
end shiftreg_tb;

architecture testbench of shiftreg_tb is
  signal data_in    : std_logic := '0';
  signal rst        : std_logic := '0';
  signal clk        : std_logic := '1';
  signal A, B, C, D : std_logic;
  component shiftreg
    port (
      A, B, C, D  : out std_logic;
      data_in     : in std_logic;
      rst         : in std_logic;
      clk         : in std_logic);
  end component;
begin
  DUT: shiftreg
    port map (A, B, C, D, data_in, rst, clk);

  clk_stimulus: process
  begin
    wait for 10 ns;
    clk <= not clk;
  end process clk_stimulus;

  data_stimulus: process
  begin
    wait for 40 ns;
    data_in <= not data_in;
    wait for 150 ns;
  end process data_stimulus;
end testbench;
