INPUT = 'Vibrio_cholerae.txt'


with open(INPUT, 'r') as myfile:
    data=myfile.read() #.split('\n')

    pattern = 'CTTGATCAT' #data[0]
    genome = data #[1]

    result = []
    k = len(pattern)

    for i in xrange(len(genome)-k):
        if genome[i:i+k] == pattern: result.append(i)


    print result
