OUTPUT_FORMAT(elf64-x86-64)
ENTRY(_start)

SECTIONS
{
	. = 0x00;

	_KernelStart = .;
	.data : ALIGN(0x1000)
	{
		*(.data)
	}
    .text : ALIGN(0x1000)
	{
		*(.text)
	}
	.rodata : ALIGN(0x1000)
	{
		*(.rodata)
	}
	.bss : ALIGN(0x1000)
	{
		*(COMMON)
		*(.bss)
	}
	_KernelEnd = .;
}