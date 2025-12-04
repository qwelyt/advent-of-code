const point = (_x,_y) => {return {x:_x, y:_y}}
const degrees = r => r/Math.PI
const angle = (a,b) => Math.atan2(b.x-a.x, -(b.y-a.y) ) % (2 * Math.PI)
const str = a => a.x+','+a.y

function asteroids(arr){
  let roids = []
  for(let y=0; y<arr.length;++y){
    for(let x=0; x<arr[y].length;x++){
      if(arr[y][x] === '#') roids.push(point(x,y))
    }
  }
  return roids;
}

function angles(roids){
  let ang = {}
  for(a of roids){
    for(b of roids){
      if(a == b) {
        // console.log("Cont!")
        continue
      }
      const k = str(a)
      if(!ang.hasOwnProperty(k)) ang[k] = []
      ang[k].push(angle(a,b))
    }
  }
  return ang
}


function solveA(map){
  const arr = map.split('\n').map(s => s.split(''))
  const roids = asteroids(arr)
  const angls = angles(roids)

  const entries = Object.entries(angls)
                        .map(a => {return [a[0], new Set(a[1])]})
                        .map(a => {return [a[0], Array.from(a[1])]})

  const lenghts = entries.map(a => [a[0], a[1].length])
                         .map(a => {
                           const coords = a[0].split(',').map(s => Number.parseInt(s))
                           const obj = {
                             coord: {
                               x: coords[0]
                               , y: coords[1]
                              }
                              , sees: a[1]
                            }
                            return obj
                          })
lenghts
  const max = lenghts.reduce((a,c) => {
                       if(a.sees < c.sees) return c
                       else return a
                      })
  return max
}

  const map = ".#..##.###...#######" +
  "\n##.############..##." +
  "\n.#.######.########.#" +
  "\n.###.#######.####.#." +
  "\n#####.##.#.##.###.##" +
  "\n..#####..#.#########" +
  "\n####################" +
  "\n#.####....###.#.#.##" +
  "\n##.#################" +
  "\n#####.##.###..####.." +
  "\n..######..##.#######" +
  "\n####.##.####...##..#" +
  "\n.#####..#.######.###" +
  "\n##...#.##########..." +
  "\n#.##########.#######" +
  "\n.####.#.###.###.#.##" +
  "\n....##.##.###..#####" +
  "\n.#.#.###########.###" +
  "\n#.#.#.#####.####.###" +
  "\n###.##.####.##.#..##" 
            solveA(map)

module.exports = {
  solveA: solveA
}