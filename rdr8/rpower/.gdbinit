target remote :3333
set monitor print asm-demangle on
load
break _start
continue
