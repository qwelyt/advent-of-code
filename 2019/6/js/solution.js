const objectify = a => {const o = {}; o[a[1]] = a[0]; return o;}
const removeUndefined = a => {
	delete a[undefined]
	return a
}
const splitObjects = a => a.trim().split(")")
const combine = (a,b) => {
	return {...a, ...b}
}
const reduceObjArr = a => a.reduce((acc, curr) => combine(acc,curr), {})

const mapArr = arr => reduceObjArr(arr.map(a => splitObjects(a))
	                                  .filter(a => a != undefined)
	                                  .map(a => objectify(a))
	                                  .map(a => removeUndefined(a))
	                              )

function count(obj, key, i){
	const v = obj[key]
	if(v == undefined) return i
	return count(obj,v,i+1)
}

function countPaths(obj){
	let counter = 0
	let pathsForKey = {}
	for(k in obj){
		if(!obj.hasOwnProperty(k)) continue

		const c = count(obj, k, 0)
		pathsForKey[k] = c
		counter += c

	}
	//console.log("Paths for keys:\n",pathsForKey)
	return counter
}

function solveA(input){
	const arr = input.split("\n")
	const obj = mapArr(arr)
	return countPaths(obj)
}

module.exports = {
	solveA: solveA
}
