INPUT = 'dataset_2_7 (1).txt'

PATTERN = 'TCGCGAATC'
N = len(PATTERN)

count = 0
with open(INPUT, 'r') as myfile:
    data=myfile.read().replace('\n', '')


    for i in xrange(0,len(data)-N):
        sub = data[i:i+N]
        print sub
        if sub == PATTERN: count += 1

print count
