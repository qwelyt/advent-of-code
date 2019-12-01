const fs = require('fs');
const path = require('path');
const filePath = path.join(__dirname, 'input.txt');

const file = fs.readFileSync(filePath, {encoding: 'UTF-8'}, function(err, data){
  if(err){
    console.log(err);
  } else {
    return data;
  }
});

const lines = file.split("\n");

const calculateFuelNeeded = moduleMass => {
  // Fuel required to launch a given module is based on its mass. 
  // Specifically, to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2.
  return Math.floor(moduleMass / 3) - 2;
}

// console.log(calculateFuelNeeded(12)); // 2
// console.log(calculateFuelNeeded(14)); // 2
// console.log(calculateFuelNeeded(1969)); // 654
// console.log(calculateFuelNeeded(100756)); // 33583

const fuelNeeded = lines.map(calculateFuelNeeded).reduce((acc,curr) => acc+curr);
console.log(fuelNeeded);