
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

def verifyIncreasingSequence(n):
  nums = list(map(lambda n: int(n),list(str(n))))
  last = None
  for n in nums:
    if(last != None):
      if(n < last):
        return False
    last = n

  return True

def verify(number):
  if(not verifyLength(number)):
     return False
  if(not verifyDoubleSequence(number)):
     return False
  if(not verifyIncreasingSequence(number)):
     return False
  return True

def generate(start, end):
  ans = []
  for number in range(start,end+1):
    if(verify(number)):
      ans.append(number)
  return ans

def solve(input):
  range = list(map(lambda s: int(s), input.split("-")))
  nums = generate(range[0], range[1])
  return nums
    



input = "284639-748759"
result = solve(input)
print(result)
print("----")
print(len(result))

