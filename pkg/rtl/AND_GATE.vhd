library ieee;
use ieee.std_logic_1164.all;

entity AND_GATE is
  port (
    A : in  std_logic;
    B : in  std_logic;
    C : out std_logic);
end AND_GATE;

architecture rtl of AND_GATE is
begin
  C <= A and B;
end rtl;
