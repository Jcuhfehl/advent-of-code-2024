import sys
import os
import shutil
import cookie

UTILS_SRC_DIRECTORY = "../utils/src"

day = sys.argv[1]

dir_path = f"day{day}"
if os.path.isdir(dir_path):
    raise Exception("Day already exists")

os.mkdir(dir_path)
os.chdir(dir_path)

os.system("cargo init")

shutil.rmtree("./src")
shutil.copytree(UTILS_SRC_DIRECTORY, "./src")

os.system(
    f"curl --cookie {cookie.COOKIE} https://adventofcode.com/2024/day/{day}/input > input.txt"
)
