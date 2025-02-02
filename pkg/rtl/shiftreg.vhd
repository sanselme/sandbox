library ieee;

use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity shiftreg is
  port (
    A, B, C, D  : out std_logic;
    data_in     : in  std_logic;
    rst         : in std_logic;
    clk         : in std_logic);
end shiftreg;

architecture rtl of shiftreg is
  signal  A_reg, B_reg,
          C_reg, D_reg : std_logic := '0';
begin
  -- signal assignment
  A <= A_reg;
  B <= B_reg;
  C <= C_reg;
  D <= D_reg;

  -- process to shift values
  process(clk)
  begin
    if rising_edge(clk) then
      if rst = '1' then
        A_reg <= '0';
        B_reg <= '0';
        C_reg <= '0';
        D_reg <= '0';
      else
        A_reg <= data_in;
        B_reg <= A_reg;
        C_reg <= B_reg;
        D_reg <= C_reg;
      end if;
    end if;
  end process;
end rtl;
