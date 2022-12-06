import sys

sys.path.append('../utils/')

from utils import get_lines 

def get_elf_calories(input):
    lines = get_lines(input)

    elf_cals = []
    elf_cals.append(0)

    i = 0
    for line in lines:
        if line == "":
            elf_cals.append(0)
            i += 1
        else:
            elf_cals[i] += int(line) 

    elf_cals.sort(reverse=True)

    return elf_cals

