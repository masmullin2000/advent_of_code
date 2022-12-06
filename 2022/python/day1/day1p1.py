#!/usr/bin/python

from lib import *

def main():
    lines = get_elf_calories("input")

    if lines:
        print(lines[0])
    else:
        print("Error: No calories")


if __name__ == "__main__":
    main()
