
def Hamming(a, b):
    count = 0
    for i in xrange(len(a)):
        if a[i] != b[i]: count += 1

    return count

def Count(d, text, pattern):
    N = len(text)
    k = len(pattern)

    count = 0
    for i in xrange(0, N-k+1):
        dist = Hamming(pattern, text[i:i+k])
        if dist <= d:
            count += 1

    return count


def Neighbourhood(d, pattern):
    return


print Count(2, 'CATGCCATTCGCATTGTCCCAGTGA', 'CCC')
# print Hamming('CAGAAAGGAAGGTCCCCATACACCGACGCACCAGTTTA', 'CACGCCGTATGCATAAACGAGCCGCACGAACCAGAGAG')
# print len('CCAGTCAATG')
