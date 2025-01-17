/* With some help from ChatGPT, but its not very good at linker files so Aidan Jacobsen reworked most of this from firmware.ld*/
MEMORY {
    ROM         (rx) : ORIGIN = 0x00000000, LENGTH = 0x00010000 /* 64kB ROM */
    BOOTLOADER  (rx) : ORIGIN = 0x10000000, LENGTH = 0x0000E000 /* Bootloader flash */
    START_FLASH (rx) : ORIGIN = 0x1000E000, LENGTH = 0x0000020C /* start flash to work with bootloader and rust toolchain */
    FLASH       (rx) : ORIGIN = 0x1000E20C, LENGTH = 0x00037E04 /* Location of team firmware, skipping 200 bytes to make it work for this toolchain */
    RESERVED    (rw) : ORIGIN = 0x10046000, LENGTH = 0x00038000 /* Reserved */
    ROM_BL_PAGE (rw) : ORIGIN = 0x1007E000, LENGTH = 0x00002000 /* Reserved */
    RAM        (rwx): ORIGIN = 0x20000000, LENGTH = 0x00020000 /* 128kB SRAM */
}

SECTIONS {
    /* Combined Section: start + Firmware Startup */
    .combined_section ORIGIN(START_FLASH) :
    {
        /* Start the start Section */
        __start_section = .;
        . += 0x200;             /* Space for the start data */
        FILL(0xFFFFFFFF);       /* Fill with all 1s */

        /* Start the Firmware Startup Section */
        firmware_startup = .; /* Label for Disassembly */
        . = ALIGN(4);           /* Align to a 2-byte boundary */
        SHORT(0x4800)            /* LDR R0, [PC, #0] */
        SHORT(0x4780)            /* BLX R0 */
        
        /* Insert the reset handler address */
        LONG(Reset)    /* Address of the reset vector */

        . = ALIGN(4);           /* Align to a 4-byte boundary */
        
        KEEP(*(.start_section));  /* Keep the section in the ELF file */
        KEEP(*(.firmware_startup)); /* Keep the startup section in the ELF file */
    } > START_FLASH
}