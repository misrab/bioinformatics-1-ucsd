#!/usr/bin/python

import sys
from operator import itemgetter

### old boilerplate

def ValueToNucleotide(v):
    nucleotides = ['A','C','G','T']
    try:
        return nucleotides[v]
    except:
        print "Invalid nucleotide value: " + str(v)
        raise

def NucleotideToValue(n):
    if n=='A': return 0
    if n=='C': return 1
    if n=='G': return 2
    if n=='T': return 3

    print "Invalid nucleotide: " + str(n)

def GetReversePattern(k, pattern):
    result = ''
    for n in pattern:
        if n=='A': result += 'T'
        if n=='C': result += 'G'
        if n=='G': result += 'C'
        if n=='T': result += 'A'

    return result[::-1]

def PatternToIndex(k, pattern):
    # print "finding pattern index for " + pattern + " of length " + str(len(pattern))
    index = 0
    for i in xrange(k):
        index += 4**(k-i-1) * NucleotideToValue(pattern[i])


    return index


def IndexToPattern(k, index):
    result = ''

    for i in xrange(k):
        base = 4**(k-i-1)
        # how many of these are there
        # e.g. how many 4^6's
        count = index / base
        nucleotide = ValueToNucleotide(count)
        result += nucleotide

        # subtract the value found
        index -= count*base

    return result


####

def Hamming(a, b):
    # print len(a)
    # print len(b)

    count = 0
    for i in xrange(len(a)):
        if a[i] != b[i]: count += 1

    return count


def GetPatternApproxCount(pattern, text):
    N = len(text)
    k = len(pattern)

    # pattern_index = PatternToIndex(k, pattern)

    count = 0
    for i in xrange(0, N-k+1):
        dist = Hamming(pattern, text[i:i+k])
        if dist <= d:
            count += 1

    return count


if __name__ == "__main__":
    try:
        filename = sys.argv[1]
    except:
        print "Please specify an input file"
        raise

    print "input: " + filename

    positions = []
    with open(filename, 'r') as myfile:
        # data='TAAAGACTGCCGAGAGGCCAACACGAGTGCTAGAACGAGGGGCGTAAACGCGGGTCCGAT'#myfile.read() #.replace('\n', '')
        data = myfile.read().split("\n")

        line2 = data[1].split(" ")


        text = data[0]
        k = int(line2[0])
        d = int(line2[1])

        N = len(text)
        num_patterns = 4**k #- 1
        counts = [0 for x in xrange(num_patterns)]
        counts_with_complements = [0 for x in xrange(num_patterns)]

        for i in xrange(num_patterns):
            pattern = IndexToPattern(k, i)
            count = GetPatternApproxCount(pattern, text)
            pattern_index = PatternToIndex(k, pattern)
            counts[pattern_index] = count

        # for each possible pattern go through text
        # for i in xrange(0, N-k+1):
        #     dist = Hamming(pattern, text[i:i+k])
        #     if dist <= d:
        #         positions.append(i)
                # print "matched " + pattern + " and " + text[i:i+k] + " " + str(dist)

    # print " ".join([str(x) for x in positions])
    # print len(positions)

    for i in xrange(num_patterns):
        pattern = IndexToPattern(k, i)
        reverse_complement = GetReversePattern(k, pattern)

        pattern_index = PatternToIndex(k, pattern)
        reverse_complement_index = PatternToIndex(k, reverse_complement)

        # print pattern
        # print reverse_complement

        counts_with_complements[pattern_index] = counts[pattern_index] + counts[reverse_complement_index]

    # print counts_with_complements
    maximum = max(counts_with_complements)
    results = []

    for i in xrange(num_patterns):
        if counts_with_complements[i] == maximum:
            results.append(IndexToPattern(k, i))
    print " ".join([str(x) for x in results])
