import math
#Program I wrote today while testing bounds of duplicate walks, kinda just interactive notes from today.
#Im trying to figure out how to model duplicate walks for implicit suffix trees. And then what can actually
#Be considered duplicate and what nodes to remove.

#The calculations in this program exclude root as its not considered when assessing duplication
S = 10
alphabetSize = 5

if alphabetSize > S:
    print("Error, alphabet size cannot exceed string length S")
    exit(1)


allNodesLB = alphabetSize # Root must have alphabet sized number of children
allNodesUB = (2*S) - 2
internalNodesUB = (S-2)
internalNodesLB = 0 # Case of only leaves
leafNodesLB = alphabetSize # Lower bound is alphabet size
leafNodesUB = S


#So far I've only found that  we can count suffix linked nodes with only leaves as children as
#nodes that certainly share duplicate children.
#I find that implicit upper bound for total number of leaves that can be the children
#of a suffix linked node to be half of leaves rounded down if odd since you need a pair.

duplicateLeavesUB = leafNodesUB // 2

if duplicateLeavesUB % 2 == 0:
    pass
else:
    duplicateLeavesUB -= 1 #If odd subtract one for odd one out.


print("Length of the string is: " + str(S))
print("Alphabet size is: " + str(alphabetSize))
print("Lower bound of all nodes: " + str(allNodesLB))
print("Upper bound of all nodes: " + str(allNodesUB))
print("Lower bound of internal nodes: " + str(internalNodesLB))
print("Upper bound of internal nodes " + str(internalNodesUB))
print("Lower bound of leaf nodes: " + str(leafNodesLB))
print("Upper bound of leaf nodes: " + str(leafNodesUB))

print("Upper bound of duplicate leaves: " + str(duplicateLeavesUB))

