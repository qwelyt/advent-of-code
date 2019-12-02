const operand = (arr,i,offset) => arr[arr[i+offset]]; // Only fetches location

function process(inputArr){
  let arr = [].concat(inputArr);
  for(var i=0,len=arr.length; i<len; i=i+4){
    let operation = arr[i];
    if(operation == 99) {
      break;
    } else if (operation == 1){
      // addition
      let operand1 = operand(arr,i,1);
      let operand2 = operand(arr,i,2);
      let storeLocation = arr[i+3];
      arr[storeLocation] = operand1 + operand2;
    } else if(operation == 2){
      // multiplication
      let operand1 = operand(arr,i,1);
      let operand2 = operand(arr,i,2);
      let storeLocation = arr[i+3];
      arr[storeLocation] = operand1 * operand2;
    } else {
      throw new Error("Unknown op-code: "+i);
    }
  }
  return arr;
}

module.exports = {operand: operand, process: process};