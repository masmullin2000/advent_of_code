#!/usr/bin/python

from lib import *

import functools

def main():
    lines = get_elf_calories("input")

    if len(lines) >= 3:
        rc = functools.reduce(lambda x, y: x+y, lines[0:3])
        print(rc)
    else:
        print("Error: No calories")


if __name__ == "__main__":
    main()
