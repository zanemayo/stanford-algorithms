/// <reference path="./typings/index.d.ts" />

import * as fs from 'fs'

module UnionFind {
  interface UF { 
    components: number[],
    sizes: number[]
  }

  export function create (v: number) {
    const cc = { components: [], sizes: [] }
    for (let i = 1; i <= v; i++) {
      cc.components[i] = i
      cc.sizes[i] = 1
    }
    return { ufUnion: union(cc), ufFind: find(cc) }
  }

  export const find = (cc: UF) => (v: number) => {
    return cc.components[v]
  }

  export const union = (cc: UF) => (v1: number, v2: number) => {
    const [renameFrom, renameTo] = 
      cc.sizes[cc.components[v1]] > cc.sizes[cc.components[v2]] 
        ? [cc.components[v2], cc.components[v1]] 
        : [cc.components[v1], cc.components[v2]]
    cc.sizes[cc.components[renameTo]] += cc.sizes[cc.components[renameFrom]]
    for (let i = 0; i <  cc.components.length; i++) {
      if (cc.components[i] === renameFrom) cc.components[i] = renameTo 
    }
  }
}

const splitLine = line => line.split(' ').map(n => +n)

function loadGraph (filename) {
  const lines = fs.readFileSync(filename, 'UTF-8').split('\n')
  const [ v, e ] = splitLine(lines[0])
  const graph = { v, e, edges: [] }
  for (let i = 1; i <= e; i++) {
    const [ v1, v2, w ] = splitLine(lines[i])
    graph.edges.push({v1, v2, w})
  }
  return graph
}

function calculateKruskelsCost (graph) {
  graph.edges.sort((a, b) => a.w - b.w)
  const { ufFind, ufUnion } = UnionFind.create(graph.v)
  let cost = 0

  for (let i = 0; i < graph.e; i++) {
    const edge = graph.edges[i]
    if (ufFind(edge.v1) === ufFind(edge.v2)) continue
    ufUnion(edge.v1, edge.v2)
    cost += edge.w
  }

  return cost
}


function equals (expected, actual) {
  if (expected !== actual) throw new Error(`expected ${expected}  |  Actual ${actual}`)
  console.log(` / ${expected} === ${actual} `)
}

// edges10 = 175ms
// edges2 = 6ms
const graph = loadGraph('q3-10k.txt')
// console.log('graph', graph)
console.time('min spanning tree')
const cost = calculateKruskelsCost(graph)
console.timeEnd('min spanning tree')

equals(-3612829, calculateKruskelsCost(loadGraph('edges2.txt')));
equals(5, calculateKruskelsCost(loadGraph('test.txt')));


const { ufFind, ufUnion } = UnionFind.create(8)

equals(5, ufFind(5))
ufUnion(1, 2)
ufUnion(2, 5)
equals(2, ufFind(5))

