# Decoder
Hey, this a decoder! Run all this from the docker! Unless it tells you not to. 

![Justin Hammer](https://static.wikia.nocookie.net/marvelcinematicuniverse/images/b/b1/Iron_man_2_50.jpg/revision/latest?cb=20141204045154)

## Building
```
cargo build
```

## Debugging via Semihosting
My felow Americans, let me be clear, you must have the MaximSDK installed to run these steps. Additionally, don't be in the docker for this. These commands are for windows but if you use linux you can def figure this out.

In one terminal, run this (openocd):
```
C:/MaximSDK/Tools/OpenOCD/openocd.exe --search "C:/MaximSDK/Tools/OpenOCD/scripts"
```

In another, run this (gdb):
```
C:\MaximSDK\Tools\GNUTools\10.3\bin\arm-none-eabi-gdb.exe --command=openocd.gdb ./target/thumbv7em-none-eabihf/debug/decoder
```

Then just use gdb like normal. Since I know you forgot all of the commands from 264 (I'm talking to you Spencer,) here are the most useful ones:
- `b <line OR function name>` Breakpoint
- `c` Continue
- `q` Quit
