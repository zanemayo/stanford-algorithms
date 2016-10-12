/// <reference path="./typings/index.d.ts" />
"use strict";
var fs = require('fs');
var UnionFind;
(function (UnionFind) {
    function create(v) {
        var cc = { components: [], sizes: [] };
        for (var i = 1; i <= v; i++) {
            cc.components[i] = i;
            cc.sizes[i] = 1;
        }
        return { ufUnion: UnionFind.union(cc), ufFind: UnionFind.find(cc) };
    }
    UnionFind.create = create;
    UnionFind.find = function (cc) { return function (v) {
        return cc.components[v];
    }; };
    UnionFind.union = function (cc) { return function (v1, v2) {
        //     const [renameFrom, renameTo] = 
        //       cc.sizes[cc.components[v1]] > cc.sizes[cc.components[v2]] 
        //         ? [cc.components[v2], cc.components[v1]] 
        //         : [cc.components[v1], cc.components[v2]]
        //     cc.sizes[cc.components[renameTo]] += cc.sizes[cc.components[renameFrom]]
        for (var i = 0; i < cc.components.length; i++) {
            if (cc.components[i] === v1)
                cc.components[i] = v2;
        }
    }; };
})(UnionFind || (UnionFind = {}));
var splitLine = function (line) { return line.split(' ').map(function (n) { return +n; }); };
function loadGraph(filename) {
    var lines = fs.readFileSync(filename, 'UTF-8').split('\n');
    var _a = splitLine(lines[0]), v = _a[0], e = _a[1];
    var graph = { v: v, e: e, edges: [] };
    for (var i = 1; i <= e; i++) {
        var _b = splitLine(lines[i]), v1 = _b[0], v2 = _b[1], w = _b[2];
        graph.edges.push({ v1: v1, v2: v2, w: w });
    }
    return graph;
}
function calculateKruskelsCost(graph) {
    graph.edges.sort(function (a, b) { return a.w - b.w; });
    var _a = UnionFind.create(graph.v), ufFind = _a.ufFind, ufUnion = _a.ufUnion;
    var cost = 0;
    var size = graph.v;
    for (var i = 0; i < graph.e; i++) {
        var edge = graph.edges[i];
        if (ufFind(edge.v1) === ufFind(edge.v2))
            continue;
        ufUnion(edge.v1, edge.v2);
        cost += edge.w;
        if (!--size)
            break;
    }
    return cost;
}
function equals(expected, actual) {
    if (expected !== actual)
        throw new Error("expected " + expected + "  |  Actual " + actual);
    console.log(" / " + expected + " === " + actual + " ");
}
// edges10 = 175ms
// edges2 = 6ms
var graph = loadGraph('q3-10k.txt');
// console.log('graph', graph)
console.time('min spanning tree');
var cost = calculateKruskelsCost(graph);
console.timeEnd('min spanning tree');
equals(-3612829, calculateKruskelsCost(loadGraph('edges2.txt')));
equals(5, calculateKruskelsCost(loadGraph('test.txt')));
var _a = UnionFind.create(8), ufFind = _a.ufFind, ufUnion = _a.ufUnion;
equals(5, ufFind(5));
ufUnion(1, 2);
ufUnion(2, 5);
equals(2, ufFind(5));
