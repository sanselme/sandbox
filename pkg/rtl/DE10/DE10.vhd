library ieee;
use ieee.std_logic_1164.all;

entity DE10 is
  port (
    -- CLOCK
    -- CLOCK_50  : in  std_logic;
    -- CLOCK2_50 : in  std_logic;
    -- CLOCK3_50 : in  std_logic;
    -- CLOCK4_50 : in  std_logic;

    -- KEY
    KEY       : in  std_logic_vector(3 downto 0);

    -- SW
    SW        : in  std_logic_vector(9 downto 0);

    -- LED
    LEDR      : out std_logic_vector(9 downto 0));
end entity;

architecture rtl of DE10 is
begin
  BUTTON_LED: entity work.BUTTON_LED
    port map (KEY, SW(0), LEDR(3 downto 0));
end rtl;
