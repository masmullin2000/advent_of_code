
def get_lines(input):
    file = open(input, "r")

    lines = []
    for line in file.readlines():
        lines.append(line.replace('\n',''))

    return lines
