const point = (_x,_y) => {return {x:_x, y:_y}}
const degrees = r => r/Math.PI
const angle = (a,b) => Math.atan2(a.x-b.x, a.y-b.y) % (2 * Math.PI)
const distance = (a,b) => Math.sqrt((a.x - b.x)**2 + (a.y-b.y)**2)
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

const arrToObj = a => {
    const o = {}
    o[a[0]] = a[1]
    return o
}

function solveB(map, base){
  const arr = map.split('\n').map(s => s.split(''))
  const roids = asteroids(arr).filter(a => !(a.y == base.y && a.x==base.x))

  let ang = {}
  for(asteroid of roids){
    let a=  360 - degrees(angle(base, asteroid))
    if(a == 360) a = 0

    if(!ang.hasOwnProperty(a)) ang[a] = []
    ang[a].push(asteroid)
  }
  const sortByAngle = Object.entries(ang).map(a => [Number.parseFloat(a[0]), a[1]]).sort((a,b) => a[0] >= b[0])
  const obj = sortByAngle.reduce((a,c) => {
    const k = c[0]
    a[k] = c[1]
    return a
  }, {})
  let sortByDistance = sortByAngle.map(a => [a[0], a[1].sort((a,b) => {
                                    const aDist = distance(a,base)
                                    const bDist = distance(b, base)
                                    return aDist >= bDist
                                  })
                       ])


  let destroyed = 0
  for(let i=0; i<sortByDistance.length; ++i){
    let roid = sortByDistance[i]
    if(roid){
      const r = roid[1].pop()
      ++destroyed
      if(destroyed == 200){
        return r
      }
    }
    if(i == sortByDistance.lenght-1) i = 0
  }
  return "lolwut"
}

  const map = ".###.#...#.#.##.#.####.."+
  "\n.#....#####...#.######.." +
  "\n#.#.###.###.#.....#.####" +
  "\n##.###..##..####.#.####." +
  "\n###########.#######.##.#" +
  "\n##########.#########.##." +
  "\n.#.##.########.##...###." +
  "\n###.#.##.#####.#.###.###" +
  "\n##.#####.##..###.#.##.#." +
  "\n.#.#.#####.####.#..#####" +
  "\n.###.#####.#..#..##.#.##" +
  "\n########.##.#...########" +
  "\n.####..##..#.###.###.#.#" +
  "\n....######.##.#.######.#" +
  "\n###.####.######.#....###" +
  "\n############.#.#.##.####" +
  "\n##...##..####.####.#..##" +
  "\n.###.#########.###..#.##" +
  "\n#.##.#.#...##...#####..#" +
  "\n##.#..###############.##" +
  "\n##.###.#####.##.######.." +
  "\n##.#####.#.#.##..#######" +
  "\n...#######.######...####" +
  "\n#....#.#.#.####.#.#.#.##" 

            solveB(map, {x:20,y:18})

module.exports = {
  solveA: solveA
  , solveB: solveB
}