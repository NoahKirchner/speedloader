SECTIONS
{
  . = ALIGN(16); /* I don't know if this actually does anything in the end result */
  .text : { /* Only pulls out .text.prologue (Entry point), .text (function calls), and .rodata */
    *(.text.prologue)
    *(.text)
    *(.rodata)
  }
  .data : { /* Pulls out .data */
    *(.data)
  }
  /DISCARD/ : { /* Discards everything else, but objdump will ignore it later anyway */
    *(.bss)
    *(.comment)
    *(.note*)
    *(.debug_frame)
    *(.interp)
    *(.eh_frame)
  }
}
