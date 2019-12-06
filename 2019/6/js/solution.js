const objectify = a => {const o = {}; o[a[1]] = a[0]; return o;}
const removeUndefined = a => {
	delete a[undefined]
	return a
}
const splitObjects = a => a.trim().split(")")
const combine = (a,b) => {return {...a, ...b}}
const reduceObjArr = a => a.reduce((acc, curr) => combine(acc,curr), {})
const difference = (a,b) => {bSet = new Set(b);return a.filter(x => !bSet.has(x));}
const symmetricDifference = (a,b) => difference(a,b).concat(difference(b,a))

const mapArr = arr => reduceObjArr(arr.map(a => splitObjects(a))
	                                  .filter(a => a != undefined)
	                                  .map(a => objectify(a))
	                                  .map(a => removeUndefined(a))
	                              )

function path(obj, key, p){
	const v = obj[key]
	if(v == undefined) return p
	p.push(v)
	return path(obj, v, p)
}

function countPaths(obj){
	let paths = []
	for(k in obj){
		if(!obj.hasOwnProperty(k)) continue

		const p = path(obj, k, [])
		paths = paths.concat(p)

	}
	return paths.length
}

function findPath(obj, from, to){
	const p = path(obj, from, [])
	const s = path(obj, to, [])
	const ps = symmetricDifference(p,s)
	return ps.length
}

function solveA(input){
	const arr = input.split("\n")
	const obj = mapArr(arr)
	return countPaths(obj)
}

function solveB(input, from, to){
	const arr = input.split("\n")
	const obj = mapArr(arr)
	return findPath(obj, from, to)
}

module.exports = {
	solveA: solveA
	, solveB: solveB
}
