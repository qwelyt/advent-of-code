const location = (arr,i,offset) => arr[i+offset];
const operand = (arr,i,offset) => arr[location(arr,i,offset)];
const addition = (a,b) => a+b;
const multiplication = (a,b) => a*b;
const apply = (f,a,b) => f(a,b);

const operate = (arr, i) => {
  const op = arr[i];
  if(op == 99) return;
  if(op == 1) return apply(addition, operand(arr,i,1), operand(arr,i,2));
  if(op == 2) return apply(multiplication, operand(arr,i,1), operand(arr,i,2));
  throw new Error("Unknown op-code: "+op);
}

function process(input){
  let arr = [].concat(input);
  for(var i=0,len=arr.length; i<len; i=i+4){
    if(arr[i] == 99){break;}
    arr[location(arr,i,3)] = operate(arr, i);
  }
  return arr;
}

module.exports = {operand: operand, process: process};