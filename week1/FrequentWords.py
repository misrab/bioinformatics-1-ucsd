INPUT = 'foo.txt'


def PatternCount(data, pattern):
    count = 0
    N=len(pattern)
    for i in xrange(0,len(data)-N):
        sub = data[i:i+N]
        print sub
        if sub == pattern: count += 1
    return count

counts = []
substrings = []
with open(INPUT, 'r') as myfile:
    data=myfile.read().split('\n')
    raw='TAAACGTGAGAGAAACGTGCTGATTACACTTGTTCGTGTGGTAT' # data[0]
    k=3#int(data[1])

    N = len(raw)
    maxCount = 0

    # get the substrings
    for i in xrange(N-k):
        substrings.append(raw[i:i+k])

    # now get counts for each
    countMap = {}
    for s in substrings:
        if s in countMap: continue
        cnt = PatternCount(raw, s)
        countMap[s] = cnt
        if cnt > maxCount: maxCount = cnt


    # now populate results
    for i in xrange(len(substrings)):
        counts.append(countMap[substrings[i]])


    print counts
    d = {}

    for i in xrange(len(counts)):
        cnt = counts[i]
        if cnt == maxCount:
            d[substrings[i]] = 1
    print d
