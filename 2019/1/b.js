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

const fuelForMass = mass => {
  // Fuel required to launch a given module is based on its mass. 
  // Specifically, to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2.
  return Math.floor(mass / 3) - 2;
}

const massFuelRequirement = mass => {
  // For each module mass, calculate its fuel and add it to the total. 
  // Then, treat the fuel amount you just calculated as the input mass and repeat the process.
  // Continuing until a fuel requirement is zero or negative.
  const f = fuelForMass(mass);
  if(f <= 0){
    return 0;
  }
  return f + massFuelRequirement(f);
}

console.log(massFuelRequirement(12)); // 2
console.log(massFuelRequirement(14)); // 2
console.log(massFuelRequirement(1969)); // 966
console.log(massFuelRequirement(100756)); // 50346

const fuelNeeded = lines.map(massFuelRequirement).reduce((acc,curr) => acc+curr);
console.log(fuelNeeded);