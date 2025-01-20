library ieee;
use ieee.std_logic_1164.all;

entity DE1 is
	port (
    -- CLOCKS
		CLOCK_24    : in std_logic;
		CLOCK_24_1  : in std_logic;
		CLOCK_27    : in std_logic;
		CLOCK_27_1  : in std_logic;
		CLOCK_50    : in std_logic;
		EXT_CLOCK   : in std_logic;

    -- KEY
		KEY : in std_logic_vector(0 to 3);

    -- SW
		SW : in std_logic_vector(0 to 9);

    -- LEDS
		LEDG : in std_logic_vector(0 to 7);
		LEDR : in std_logic_vector(0 to 9);

    -- HEX
		HEX0 : in std_logic_vector(0 to 6);
		HEX1 : in std_logic_vector(0 to 6);
		HEX2 : in std_logic_vector(0 to 6);
		HEX3 : in std_logic_vector(0 to 6);

    -- GPIO
		GPIO_0 : in std_logic_vector(0 to 35);
		GPIO_1 : in std_logic_vector(0 to 35);

    -- UART
		UART_RXD : in std_logic;
		UART_TXD : in std_logic;

    -- I2C
		I2C_SCLK : in std_logic;
		I2C_SDAT : in std_logic;

    -- PS2
		PS2_CLK : in std_logic;
		PS2_DAT : in std_logic;

    -- FL
		FL_ADDR   : in std_logic_vector(0 to 21);
		FL_DQ     : in std_logic_vector(0 to 7);
		FL_OE_N   : in std_logic;
		FL_RST_N  : in std_logic;
		FL_WE_N   : in std_logic;

    -- TD
		TCK : in std_logic;
		TCS : in std_logic;
		TDI : in std_logic;
		TDO : in std_logic;

    -- AUDIO
		AUD_ADCDAT  : in std_logic;
		AUD_ADCLRCK : in std_logic;
		AUD_BCLK    : in std_logic;
		AUD_DACDAT  : in std_logic;
		AUD_DACLRCK : in std_logic;
		AUD_XCK     : in std_logic;

    -- VGA
		VGA_B   : in std_logic_vector(0 to 3);
		VGA_G   : in std_logic_vector(0 to 3);
		VGA_HS  : in std_logic;
		VGA_R   : in std_logic_vector(0 to 3);
		VGA_VS  : in std_logic;

    -- SD
		SD_CLK  : in std_logic;
		SD_CMD  : in std_logic;
		SD_DAT  : in std_logic;
		SD_DAT3 : in std_logic;

    -- SRAM
		SRAM_ADDR : in std_logic_vector(0 to 17);
		SRAM_CE_N : in std_logic;
		SRAM_DQ   : in std_logic_vector(0 to 15);
		SRAM_LB_N : in std_logic;
		SRAM_OE_N : in std_logic;
		SRAM_UB_N : in std_logic;
		SRAM_WE_N : in std_logic;

    -- DRAM
		DRAM_ADDR   : in std_logic_vector(0 to 11);
		DRAM_BA_0   : in std_logic;
		DRAM_BA_1   : in std_logic;
		DRAM_CAS_N  : in std_logic;
		DRAM_CKE    : in std_logic;
		DRAM_CLK    : in std_logic;
		DRAM_CS_N   : in std_logic;
		DRAM_DQ     : in std_logic_vector(0 to 15);
		DRAM_LDQM   : in std_logic;
		DRAM_RAS_N  : in std_logic;
		DRAM_UDQM   : in std_logic;
		DRAM_WE_N   : in std_logic);
end DE1;

architecture ppl_type of DE1 is
begin
end;
