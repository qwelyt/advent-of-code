
import os
import math

inputFilePath = os.path.join(os.getcwd(), "2019", "1", "input.txt")

file = open(inputFilePath, "r")

lines = list(map(lambda x: int(x),file.read().split("\n")))

fuelForMass = lambda m: math.floor(m/3) - 2

def totalFuel(m):
  f = fuelForMass(m)
  if f <= 0:
    return 0
  else:
    return f + totalFuel(f)

print(sum(map(totalFuel, lines)))