import subprocess
import sys
from pathlib import Path
import shutil
import os
cwd = os.path.abspath(os.path.dirname(__file__))
os.chdir(cwd)

b = Path("build")
if b.exists():
    shutil.rmtree(b)
b.mkdir()
os.chdir(str(b.resolve()))
c = "cmake .. -DCMAKE_BUILD_TYPE=Release"
subprocess.run(c, shell=True, check=True)
subprocess.run("cmake --build . --config Release", shell=True, check=True)
os.chdir(cwd)

