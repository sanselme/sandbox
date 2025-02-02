library ieee;
use ieee.std_logic_1164.all;

entity BUTTON_LED is
  generic (NUM_BUTTONS : integer := 4);
  port (
    button_in : in  std_logic_vector(NUM_BUTTONS - 1 downto 0);
    enable    : in  std_logic;
    led_out   : out std_logic_vector(NUM_BUTTONS - 1 downto 0));
end BUTTON_LED;

architecture rtl of BUTTON_LED is
begin
  -- set output of led
  led_out <= button_in when enable = '0' else (others => '0');
end rtl;
