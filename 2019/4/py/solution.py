
verifyLength = lambda n: len(str(n)) == 6

def verifyDoubleSequence(n):
  chars = list(str(n))
  last = None
  for c in chars:
    if(last != None):
      if(c==last):
        return True
    last = c
  return False

def verifyExclusiveDoubleSequence(n):
  chars = list(str(n))
  sdict = {}
  for c in chars:
    if(c not in sdict):
      sdict[c] = 0
    sdict[c] += 1
  return 2 in sdict.values()


def verifyIncreasingSequence(n):
  nums = list(map(lambda n: int(n),list(str(n))))
  last = None
  for n in nums:
    if(last != None):
      if(n < last):
        return False
    last = n

  return True

def verify(number, verifySequence):
  if(not verifyLength(number)):
     return False
  if(not verifySequence(number)):
     return False
  if(not verifyIncreasingSequence(number)):
     return False
  return True

def generate(start, end, verifySequence):
  ans = []
  for number in range(start,end+1):
    if(verify(number, verifySequence)):
      ans.append(number)
  return ans

def solve(input):
  range = list(map(lambda s: int(s), input.split("-")))
  nums = generate(range[0], range[1], verifyDoubleSequence)
  return nums

def solveB(input):
  range = list(map(lambda s: int(s), input.split("-")))
  nums = generate(range[0], range[1], verifyExclusiveDoubleSequence)
  return nums

    



input = "284639-748759"

resultA = solve(input)
resultB = solveB(input)
print("----")
print("A", len(resultA), len(resultA) == 895)
print("B", len(resultB), len(resultB) == 591)

