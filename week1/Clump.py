#
#   Find k-clumps of frequency >= t in windows of length L
#



# import numpy as np


# INPUT = 'foo.txt'
# INPUT = 'dataset_4_5.txt'
INPUT = 'E_coli.txt'


def PatternCount(counts, pattern, window, k, L):
    count = 0
    for i in xrange(0, L-k):
        sub = data[i:i+k]
        if sub == pattern: count += 1
    return count

def FindClumps(clumps, window, k, L, t):
    # counts of each 4**k - 1 patterns
    num_patterns = 4**k # - 1
    # counts = np.zeros(num_patterns)
    counts = {}

    # for i in xrange(num_patterns):
    #     pattern = IndexToPattern(i)
    #     count = PatternCount(pattern, window, k, L)
    #     if count >= t: clumps.add(pattern)
    for i in xrange(0, L-k):
        pattern = window[i:i+k]

        # increment the count of that pattern
        index = PatternToIndex(k, ''.join(pattern))
        if index not in counts:
            counts[index] = 1
        else:
            counts[index] += 1

    # now add to our set of clumps
    # for i in xrange(num_patterns):
    #     if counts[i] >= t: clumps.add(IndexToPattern(k, i))
    for i in counts:
        if counts[i] >= t: clumps.add(IndexToPattern(k, i))

    return


def NucleotideToValue(n):
    if n=='A': return 0
    if n=='C': return 1
    if n=='G': return 2
    if n=='T': return 3

    print "Invalid nucleotide: " + str(n)

def ValueToNucleotide(v):
    nucleotides = ['A','C','G','T']
    try:
        return nucleotides[v]
    except:
        print "Invalid nucleotide value: " + str(v)
        raise



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

# we want to find all patterns that form
# clumps of at least t occurences
# within some window of length L
with open(INPUT, 'r') as myfile:
    data=myfile.read().split('\n')
    # line2 = data[1].split(" ")

    genome = data[0]
    k = 9 #int(line2[0])
    L = 500 # int(line2[1])
    t = 3 #int(line2[2])


    # num_patterns = 4**k - 1
    # using a set to save memory
    clumps = set() # 1 if clump at that index occured

    for i in xrange(len(genome)-L):
        window = genome[i:i+L]

        FindClumps(clumps, window, k, L, t)

        # TEMP
        # break

    print clumps
