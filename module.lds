OUTPUT_ARCH(i386:x86-64)

SECTIONS
{
	. = 0x4000000;
	.text : {
		*(.text)
		*(.text.*)
	}
	. = ALIGN(4096);
	.rodata : { *(.rodata) *(.rodata.*) }
	. = ALIGN(4096);
	.data : { *(.data) *(.data.*) }
	. = ALIGN(4096);
	.bss : {
		*(.bss) *(.bss.*)
		. = ALIGN(4096);
	}
	. = ALIGN(4096);
}

ENTRY(module_main)
