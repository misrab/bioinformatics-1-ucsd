INPUT = 'foo.txt'


with open(INPUT, 'r') as myfile:
    data=myfile.read()

    result = []
    for letter in data:
        if letter == 'G': result.append('C')
        if letter == 'A': result.append('T')
        if letter == 'T': result.append('A')
        if letter == 'C': result.append('G')

    result.reverse()
    print ''.join(result)
