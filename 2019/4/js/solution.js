// 6 digits
// Never decreasing
// Atleast one repetition of adjecent digits

const inputToRange = input => {const a=input.split("-"); return {start: a[0], end:a[1]}};
const hasRepetition = a => {
	const chars = (""+a).split('');
	let last = null;
	for(const c of chars){
		if(last == null){
			last = c;
		} else {
			if(last === c){
				return true;
			}
			last = c;
		}
	}
	return false;
};
const hasRepetitionNoLargeSets = a => {
	const chars = (""+a).split('');
	let obj = {};
	for(c of chars){
		if(!obj.hasOwnProperty(c)) obj[c] = 0;

		obj[c] = obj[c] += 1;
	}
	const values = Object.entries(obj).map(a => a[1]).filter(n => n===2);
	return values.length > 0;
};
const decresingSequence = a => {
	const numbers = (""+a).split('').map(c => Number.parseInt(c));
	if(new Set(numbers).length === 1) return false;
	let last = null;
	for(const n of numbers){
		if(last == null){
			last = n;
		} else {
			if(last > n){
				return true;
			}
			last = n;
		}
	}
	return false;
}

function verify(number){
	if((""+number).length !== 6) return false;
	if(!hasRepetition(number)) return false;
	if(decresingSequence(number)) return false;
	return true;
}

function verifyWithLimits(number){
	if((""+number).length !== 6) return false;
	if(decresingSequence(number)) return false;
	if(!hasRepetitionNoLargeSets(number)) return false;
	return true;
}

function generate(start, end, verify){
	let ans = [];
	for(let i=start; i<=end; i++){
		if(verify(i)){
			ans.push(i);
		}
	}
	return ans;
}

function anyGroup(input){
	const range = inputToRange(input);
	return generate(range.start, range.end, verify);
}

function groupLimits(input){
	const range = inputToRange(input);
	return generate(range.start, range.end, verifyWithLimits);
}

module.exports = {
	verify: verify
	, verifyWithLimits: verifyWithLimits
	, anyGroup: anyGroup
	, groupLimits: groupLimits
	, hasRepetition: hasRepetition
	,decresingSequence: decresingSequence
}
