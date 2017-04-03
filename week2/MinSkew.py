#!/usr/bin/python

import sys
from operator import itemgetter






if __name__ == "__main__":
    try:
        filename = sys.argv[1]
    except:
        print "Please specify an input file"
        raise

    print "input: " + filename

    skews = [0]
    with open(filename, 'r') as myfile:
        # data='TAAAGACTGCCGAGAGGCCAACACGAGTGCTAGAACGAGGGGCGTAAACGCGGGTCCGAT'#myfile.read() #.replace('\n', '')
        data = myfile.read()

        data = 'GATACACTTCCCGAGTAGGTACTG'

        skew = 0
        for i in xrange(len(data)):
            n = data[i]

            if n == 'C':
                skew = skew - 1
            if n == 'G':
                skew = skew + 1

            skews.append(skew)

    m = min(skews)
    print [i for i, j in enumerate(skews) if j == m]
    # print minSkew
    # print skews[:100]
