
const readMode = {
  positionMode: (arr, i, offset, relativeBase) => arr[arr[i + offset]]
  , immediateMode: (arr, i, offset, relativeBase) => arr[i + offset]
  , relativeMode: (arr, i, offset, relativeBase) => arr[arr[i+offset] + relativeBase]
  , modeDecider: n => n === 0 ? readMode.positionMode : n === 1 ? readMode.immediateMode : readMode.relativeMode
  , mode: s => readMode.modeDecider(Number.parseInt(s))
}
const writeMode = {
  positionMode: (arr, i, offset, relativeBase) => arr[i + offset]
  , immediateMode: (arr,i,offset,relativeBase) => writeMode.positionMode(arr,i,offset)
  , relativeMode: (arr, i, offset,relativeBase) => arr[i+offset] + relativeBase
  , modeDecider: n => n === 0 ? writeMode.positionMode : n === 1 ? writeMode.immediateMode : writeMode.relativeMode
  , mode: s => writeMode.modeDecider(Number.parseInt(s))
}

let output = []
let inputs = []

const add = (a, b) => a + b
const multiply = (a, b) => a * b
const jumpIfTrue = a => a != 0
const jumpIfFalse = a => a == 0
const lessThan = (a, b) => a < b ? 1 : 0
const equalTo = (a, b) => a === b ? 1 : 0
const write = () => inputs.pop()
const read = value => output.push(value)
const adjustRelativeBase = (a, b) => a + b


function parseInstruction(code) {
  const ins = ("" + code).padStart(5, '0')
  const opCode = ins.substring(ins.length - 2)
  const params = ins.substring(0, ins.length - 2).split('').reverse()

  let instruction = {
    opCode: opCode
    , operation: null
    , paramModes: null
    , storeMode: null
    , length: null

  }
  switch (opCode) {
    case "01":
      instruction.operation = add
      instruction.paramModes = [readMode.mode(params[0]), readMode.mode(params[1])]
      instruction.storeMode = writeMode.mode(params[2])
      instruction.length = 4
      break
    case "02":
      instruction.operation = multiply
      instruction.paramModes = [readMode.mode(params[0]), readMode.mode(params[1])]
      instruction.storeMode = writeMode.mode(params[2])
      instruction.length = 4
      break
    case "03":
      instruction.operation = write
      instruction.storeMode = writeMode.mode(params[0])
      instruction.length = 2
      break
    case "04":
      instruction.operation = read
      instruction.paramModes = [readMode.mode(params[0])]
      instruction.length = 2
      break
    case "05":
      instruction.operation = jumpIfTrue
      instruction.paramModes = [readMode.mode(params[0]), readMode.mode(params[1])]
      instruction.length = 3
      break
    case "06":
      instruction.operation = jumpIfFalse
      instruction.paramModes = [readMode.mode(params[0]), readMode.mode(params[1])]
      instruction.length = 3
      break
    case "07":
      instruction.operation = lessThan
      instruction.paramModes = [readMode.mode(params[0]), readMode.mode(params[1])]
      instruction.storeMode = writeMode.mode(params[2])
      instruction.length = 4
      break
    case "08":
      instruction.operation = equalTo
      instruction.paramModes = [readMode.mode(params[0]), readMode.mode(params[1])]
      instruction.storeMode = writeMode.mode(params[2])
      instruction.length = 4
      break
    case "09":
      instruction.operation = adjustRelativeBase
      instruction.paramModes = [readMode.mode(params[0])]
      instruction.length = 2
      break
    default:
      throw new Error("Unknown operation: " + opCode + " (" + code + ")")
  }
  return instruction
}

function arrify(args) {
  if (typeof args === "string") return args.split(",").map(s => Number.parseInt(s))
  if (args.constructor == Array) return args
}

let indexes = []
function execute(args) {
  let arr = arrify(args)
  let relativeBase = 0

  for (let i = 0; i < arr.length;) {
    indexes.push(i)
    if (arr[i] === 99) break;
    const instruction = parseInstruction(arr[i])

    if ("01" === instruction.opCode // add
      || "02" === instruction.opCode // multiply
      || "07" === instruction.opCode // lessThan
      || "08" === instruction.opCode // equalTo
      ) { 
      const result = instruction.operation(
        instruction.paramModes[0](arr, i, 1, relativeBase)
        , instruction.paramModes[1](arr, i, 2, relativeBase)
      )
      const store = instruction.storeMode(arr, i, 3, relativeBase)
      arr[store] = result
      i += instruction.length
    } else if ("03" === instruction.opCode) { // write
      const result = instruction.operation()
      const store = instruction.storeMode(arr, i, 1, relativeBase)
      arr[store] = result
      i += instruction.length
    } else if ("04" === instruction.opCode) { // read
      const param = instruction.paramModes[0](arr, i, 1, relativeBase)
      instruction.operation(param)
      i += instruction.length
    } else if ("05" === instruction.opCode // jumpIfTrue
      || "06" === instruction.opCode) { // jumpIfFalse
      const param1 = instruction.paramModes[0](arr, i, 1,relativeBase)
      const param2 = instruction.paramModes[1](arr, i, 2,relativeBase)
      const result = instruction.operation(param1)
      if (result) i = param2
      else i += instruction.length
    } else if ("09" === instruction.opCode) {
      const param = instruction.paramModes[0](arr,i,1,relativeBase)
      relativeBase += param//instruction.operation(relativeBase, param)
      i += instruction.length
    } else {
      throw new Error("Unknown operation: " + instruction.operation)
    }
  }

  return {
    arr: arr
    , output: output
  }
}

function compute(args, input) {
  output = []
  inputs = input === undefined ? [] : [].concat(input)
  return execute(args)
}

module.exports = {
  compute: compute
  , parseInstruction: parseInstruction
  , readMode: {
    positionMode: readMode.positionMode
    , immediateMode: readMode.immediateMode
    , relativeMode: readMode.relativeMode
  }
  , writeMode: {
    positionMode: writeMode.positionMode
    , relativeMode: writeMode.relativeMode
  }
  , operations: {
    add: add
    , multiply: multiply
    , write: write
    , read, read
    , jumpIfTrue: jumpIfTrue
    , jumpIfFalse: jumpIfFalse
    , lessThan: lessThan
    , equalTo: equalTo
    , adjustPointer: adjustRelativeBase
  }
}