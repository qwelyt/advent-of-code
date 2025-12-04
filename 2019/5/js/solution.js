const positionMode = (arr,i,offset) => arr[arr[i+offset]]
const immediateMode = (arr,i,offset) => arr[i+offset]
const mode = s => Number.parseInt(s) === 1 ? immediateMode : positionMode
const inverter = s => s === "0" ? "1" : "0"

const add = (a,b) => a+b;
const multiply = (a,b) => a*b;

let output = []

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
	return writeRequireInput(1, arr, position)
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
			ret["operation"] = add
			ret["length"] = 4
			ret["modifies"] = true
			break
		case "02":
			ret["operation"] = multiply
			ret["modifies"] = true
			ret["length"] = 4
			break
		case "03":
			ret["operation"] = write
			ret["modifies"] = true
			ret["length"] = 2
			break
		case "04":
			ret["operation"] = read
			ret["modifies"] = false
			ret["length"] = 2
			break
		default:
			throw new Error("Unknown opcode: "+opCode+" :  "+instruction)
	}

	return ret
}

function solveA(input){
	let arr = input.split(",").map(s => Number.parseInt(s))
	//console.log(arr)
	for(let i=0; i<arr.length;){
		//console.log(i, arr[i])
		if(arr[i] === 99 || arr[i] == undefined) break;
		const instruction = parseInstruction(arr[i])

		if(instruction.length === 4){
			const param1 = instruction.param1(arr, i, 1)
			const param2 = instruction.param2(arr, i, 2)
			const storePosition = instruction.storePosition(arr, i, 3)

			arr[storePosition] = instruction.operation(param1,param2)
		} else if(instruction.length === 2){
			const storePosition = instruction.storePosition(arr, i, 1)
			const result =  instruction.operation(arr, storePosition)
			if(instruction.modifies) arr[storePosition] = result
		} else {
			throw new Error("Unknown instruction: "+JSON.stringify(instruction))
		}
		i += instruction.length
	}
	return {
		output: output
		, arr: arr
	}
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
}
