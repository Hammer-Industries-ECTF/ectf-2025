# ectf-2025
A repository containing the full design for the MITRE eCTF 2025 from team Purdue2: <b>Hammer Industries</b>

![Justin Hammer](https://static.wikia.nocookie.net/marvelcinematicuniverse/images/b/b1/Iron_man_2_50.jpg/revision/latest?cb=20141204045154)

## Requirements
You'll need the following to run the code in this design
- Docker
- Python >=3.11

If you also want to debug the design (outside of Docker), you'll need
- Rustc
- Cargo
- MaximSDK

## Layout
- `decoder/` - Firmware for the TV decoder
  - `src/` - Rust source code
  - `build.rs` - Build script that generates flash memory data
  - `Dockerfile` - For building the firmware into a binary file
  - `memory.x` - Flash memory mapping
- `design/` - Software for the TV encoder
  - `ectf25_design/` - Python source code
    - `encoder.py` - Encodes frames
    - `gen_secrets.py` - Generates AES secrets
    - `gen_subscription.py` - Generates subscription update packages
  - `tests/` - Tests for each module powered by a fuzzer (unstable)
  - `pyproject.toml` - Pip import instructions
- `frames/` - A series of frames for use with the MITRE tool suite
- `tools/` - MITRE tool suite
- `design_purdue2.pdf` - Design Documentation

## Example Flow
Here is an example flow using frames from the `frames/` folder.

1) Install encoder design, MITRE tool suite, and build decoder Docker
```bash
cd /path/to/ectf/git/root
pip install ./design
pip install ./tools
docker build -t decoder ./decoder
```

2) Generate the secrets and a subscription for the deployment
```bash
mkdir -p test
py -m ectf25_design.gen_secrets ./test/global.secrets 1
py -m ectf25_design.gen_subscription ./test/global.secrets ./test/subscription.bin 0xdeadbeef 0 10000 1
```

3) Generate the decoder binary and flash to microcontroller
```bash
mkdir -p ./test/deadbeef_build
docker run -m 10g --rm -v ./decoder:/decoder -v ./test/global.secrets:/global.secrets:ro -v ./test/deadbeef_build:/out -e DECODER_ID=0xdeadbeef decoder
py -m ectf25.utils.flash ./test/deadbeef_build/max78000.bin <PORT>
```

4) Subscribe to channel, list subscriptions, and run tester
```bash
py -m ectf25.tv.subscribe ./test/subscription.bin <PORT>
py -m ectf25.tv.list <PORT>
py -m ectf25.utils.tester --secrets ./test/global.secrets --port <PORT> --perf json ./frames/x_c1.json
```
