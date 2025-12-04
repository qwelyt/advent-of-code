const objectify = a => {const o = {}; o[a[1]] = a[0]; return o;}
const splitObjects = a => a.split(")")
const combine = (a,b) => {
	return {...a, ...b}
}
const reduceObjArr = a => a.reduce((acc, curr) => combine(acc,curr), {})

const mapArr = arr => reduceObjArr(arr.map(a => splitObjects(a)).map(a => objectify(a)))

function count(obj, key, i){
	const v = obj[key]
	if(v == undefined) return i
	return count(obj,v,i+1)
}

function countPaths(obj){
	let counter = 0
	for(k in obj){
		if(!obj.hasOwnProperty(k)) continue

		counter += count(obj, k, 0)
	}
	return counter
}

function solveA(input){
	const arr = input.split("\n")
	console.log(arr)
	const obj = mapArr(arr)
	return countPaths(obj)
}

module.exports = {
	solveA: solveA
}
