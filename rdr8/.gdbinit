target remote :3333
set print asm-demangle on
load
break _start
continue
