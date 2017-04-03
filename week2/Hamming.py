#!/usr/bin/python

import sys
from operator import itemgetter



def Hamming(a, b):
    count = 0
    for i in xrange(len(a)):
        if a[i] != b[i]: count += 1

    return count


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
        data = myfile.read().split("\n")

        print Hamming(data[0], data[1])
