from loguru import logger
import argparse
from pathlib import Path
import asyncio
from tools.utils import package_binary, run_shell
import tools.update as flash
import glob
import os
from serial import serialutil

# cd rust_example/example && 
# cargo build && 
# arm-none-eabi-objcopy -O binary ./target/thumbv7em-none-eabihf/debug/example ../example.bin && 
# cd ../.. && 
# python tools/build_example.py -b ./rust_example/example.bin -i ./rust_example/example.img

def generate_elf_and_bin(project_folder, project_name):
    output = asyncio.run(run_shell(
        f"cd {project_folder} && "
        f"cargo build && "
        f"arm-none-eabi-objcopy -O binary ./target/thumbv7em-none-eabihf/debug/example ../build/{project_name}.bin &&"
        f"cp ./target/thumbv7em-none-eabihf/debug/example ../build/{project_name}.elf"
    ))

def build_project(project_name):
    if(not(project_name.isalnum())):
        print("\033[31mError\033[0m: Project Name must be alphanumeric")
        raise ValueError("Project Name Invalid")

    folders = glob.glob(os.getcwd() + "/projects/*")
    project_names = [os.path.basename(folder) for folder in folders if os.path.basename(folder) != "build"]

    if(not(project_name in project_names)):
        print("\033[31mError\033[0m: Project folder not found. Ensure --name arg is a subfolder in $PWD/projects/")
        raise ValueError("Project Name Invalid")
    
    project_folder = os.getcwd() + "/projects/" + project_name + "/"

    bin_name = generate_elf_and_bin(project_folder, project_name)

    package_binary(f"{os.getcwd()}/projects/build/{project_name}.bin", f"{os.getcwd()}/projects/build/{project_name}.img")

def main():
    parser = argparse.ArgumentParser(
        prog="Tool To Flash Processor",
        description="Just builds the image from a binary"
    )

    parser.add_argument(
        "-n", "--name", required=True, type=Path,
        help="Folder name of project within ectf-2025/projects"
    )

    parser.add_argument(
        "-f", "--flash", required=False,
        default="True",
        help="Set to false (-f False) to disable flashing of microprocessor"
    )

    args = parser.parse_args()

    project_name = str(args.name)

    build_project(project_name)

    flash_requested = False
    if("t" in str(args.flash).lower()):
        flash_requested = True

    if(flash_requested):
        try:
            flash.image_update(f"{os.getcwd()}/projects/build/{project_name}.img", "/dev/ttyACM0")
        except serialutil.SerialException as e:
            print("Did not detect MAX78000. Did you remember to pass the port to docker (.\connect_max78000.ps1 -y in docker-tool-suite folder in windows powershell)?")

if __name__ == '__main__':
    main()
