# Decoder
Hey, this a decoder! Run all this from the docker! Unless it tells you not to.

## Stats
By our measurement, we're able to decode around 25-30 64B frames per second.

## Building
```bash
cd /path/to/ectf/git/root
docker build -t decoder ./decoder
docker run -m 10g --rm -v ./decoder:/decoder -v /path/to/secrets:/global.secrets:ro -v ./test/deadbeef_build:/out -e DECODER_ID=0xdeadbeef decoder
```

## Debugging via Semihosting
My fellow Americans, let me be clear, you must have the MaximSDK installed to run these steps. Additionally, don't be in the docker for this. These commands are for windows but if you use linux you can def figure this out.
You will likely have to edit the `build.rs` script to hardcode the environment variable `DECODER_ID` and a different path for `global.secrets` as they are both intended for the docker.

### VS Code
Hit the start debug button. If it doesnt work, you probably installed the MaximSDK in the wrong spot.

### No VS Code
In one terminal, run this (openocd):
```
C:/MaximSDK/Tools/OpenOCD/openocd.exe --search "C:/MaximSDK/Tools/OpenOCD/scripts"
```

In another, run this (gdb):
```
C:\MaximSDK\Tools\GNUTools\10.3\bin\arm-none-eabi-gdb.exe --command=openocd.gdb ./target/thumbv7em-none-eabihf/debug/decoder
```
