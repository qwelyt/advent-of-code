const positionMode = (arr,i,offset) => arr[arr[i+offset]]
const immediateMode = (arr,i,offset) => arr[i+offset]
const mode = s => Number.parseInt(s) === 1 ? immediateMode : positionMode
const inverter = s => s === "0" ? "1" : "0"

const add = (a,b) => a+b
const multiply = (a,b) => a*b
const jumpIfTrue = a => a != 0
const jumpIfFalse = a => a == 0
const lessThan = (a,b) => a<b ? 1 : 0
const equalTo = (a,b) => a === b ? 1 : 0

let output = []
let valueToWrite = undefined

const read = (arr, position) => {
	const val = arr[position]
	console.log("Read: ",val)
	output.push(val)
	return val
}


const writeRequireInput = (input, arr, position) => {
	return input
}

const write = (arr, position) => {
	return writeRequireInput(valueToWrite, arr, position)
}


function parseInstruction(instruction){
	const ins = (""+instruction).padStart(5, '0')
	const opCode = ins.substring(ins.length-2)
	const params = ins.substring(0, ins.length-2)


	let ret = {
		opCode: opCode
		, param1:  mode(params[2])
		, param2:  mode(params[1])
		, storePosition: mode(inverter(params[0]))
	}
	
	switch(opCode){
		case "01":
			ret.operation = add
			ret.length = 4
			ret.modifies = true
			break
		case "02":
			ret.operation = multiply
			ret.modifies = true
			ret.length = 4
			break
		case "03":
			ret.operation = write
			ret.modifies = true
			ret.length = 2
			break
		case "04":
			ret.operation = read
			ret.modifies = false
			ret.length = 2
			break
		case "05":
			ret.operation = jumpIfTrue
			ret.modifies = false
			ret.length = 3
			break
		case "06":
			ret.operation = jumpIfFalse
			ret.modifies = false
			ret.length = 3
			break
		case "07":
			ret.operation = lessThan
			ret.modifies = true
			ret.length = 4
			break
		case "08":
			ret.operation = equalTo
			ret.modifies = true
			ret.length = 4
			break
		default:
			throw new Error("Unknown opcode: "+opCode+" :  "+instruction)
	}

	return ret
}

function run(codes){
	let arr = codes.split(",").map(s => Number.parseInt(s))
	//console.log(arr)
	for(let i=0; i<arr.length;){
		//console.log(i, arr[i])
		if(arr[i] === 99 || arr[i] == undefined) break;
		const instruction = parseInstruction(arr[i])

		if(instruction.length === 4){
			// Addition (01), Multiply (02), equalTo (07), lessThan (08)
			const param1 = instruction.param1(arr, i, 1)
			const param2 = instruction.param2(arr, i, 2)
			const storePosition = instruction.storePosition(arr, i, 3)

			if(storePosition !== undefined) arr[storePosition] = instruction.operation(param1,param2)
			i += instruction.length
		}  else if(instruction.length === 3){
			// jumpIfTrue (05), jumpIfFalse(06)
			const param1 = instruction.param1(arr, i, 1)
			const param2 = instruction.param2(arr, i, 2)
			const result = instruction.operation(param1)
			if(result) i = param2
			else i+= instruction.length
			
		} else if(instruction.length === 2){
			// write (03), read (04)
			const storePosition = instruction.storePosition(arr, i, 1)
			const result =  instruction.operation(arr, storePosition)
			if(instruction.modifies) arr[storePosition] = result
			i += instruction.length
		} else {
			throw new Error("Unknown instruction: "+JSON.stringify(instruction))
		}
	}
	return {
		output: output
		, arr: arr
	}
}

function solveA(codes){
	valueToWrite = 1
	output = []
	return run(codes)
}

function solveB(codes, input){
	valueToWrite = input
	output = []
	return run(codes)
}

module.exports = {
	parseInstruction: parseInstruction
	, add: add
	, multiply: multiply
	, read: read
	, write: write
	, positionMode: positionMode
	, immediateMode: immediateMode
	, solveA: solveA
	, solveB: solveB
}
