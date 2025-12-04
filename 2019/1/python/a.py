import os
import math

inputFilePath = os.path.join(os.getcwd(), "2019", "1", "input.txt")

file = open(inputFilePath, "r")

lines = list(map(lambda x: int(x),file.read().split("\n")))

fuelForMass = lambda m: math.floor(m/3) - 2

# print(fuelForMass(12))
# print(fuelForMass(14))
# print(fuelForMass(1969))

print(sum(map(fuelForMass, lines)))
