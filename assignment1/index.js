// import fs from 'fs'
var splitLine = function (line) { return line.split(' ').map(function (n) { return +n; }); };
function loadGraph(filename) {
    var lines = ''; // fs.readFileSync(filename, 'UTF-8').split('\n')
    var _a = splitLine(lines[0]), v = _a[0], e = _a[1];
    var graph = { v: v, e: e, edges: [] };
    for (var i = 1; i <= e; i++) {
        graph.edges.push(lines[i].split(' ').map(function (n) { return +n; }));
    }
    return graph;
}
function getPrimmsCost(graph) {
    var inGraph = [];
    var edges = graph.edges;
    var cost = 0;
    inGraph[1] = true;
    for (var j = 1; j < graph.v; j++) {
        var minCost = Number.MAX_SAFE_INTEGER;
        var minEdge = void 0;
        for (var i = 0; i < graph.e; i++) {
            var edge = edges[i];
            if (inGraph[edge[0]] !== inGraph[edge[1]] &&
                edge[2] < minCost) {
                minCost = edge[2];
                minEdge = i;
            }
        }
        cost += minCost;
        inGraph[edges[minEdge][0]] = true;
        inGraph[edges[minEdge][1]] = true;
    }
    return cost;
}
function equal(expected, actual) {
    if (expected !== actual)
        throw new Error("Expected " + expected + " | Found " + actual);
    console.log('Test passed');
}
function testLoadGraph() {
    var graph = loadGraph('test.txt');
    equal(5, graph.e);
    equal(4, graph.v);
    equal(3, graph.edges[2][0]);
    equal(4, graph.edges[2][1]);
    equal(1, graph.edges[2][2]);
}
function testPrimms() {
    equal(-3612829, getPrimmsCost(loadGraph('edges2.txt')));
}
function perf() {
    printTime(function () { return getPrimmsCost(loadGraph('edges2.txt')); }); // 52ms
    printTime(function () { return loadGraph('edges2.txt'); }); // 4ms
}
function printTime(fn) {
    var start = new Date().getTime();
    fn();
    var end = new Date().getTime();
    console.log('Time taken: ' + (end - start) + 'ms');
}
// testLoadGraph()
// testPrimms()
// performance()
/*
4 5
1 2 2
2 3 4
3 4 1
4 1 5
2 4 2
*/
//        2
//   1---------2
//   |        /|
//   |       / |
//  5|  __2_/  | 4
//   | /       |
//   |/        |
//   4---------3
//        1
// function loadGraph (filename) {
//   const lines = fs.readFileSync(filename, 'UTF-8').split('\n')
//   // const numVertices =  lines[0].split(' ')[0]
//   const numEdges = lines[0].split(' ')[1]
//
//   const graph = {}
//   for (let i = 1; i <= numEdges; i++) {
//     const [v1, v2] = lines[i].split(' ')
//     if (!graph[+v1]) graph[+v1] = []
//     graph[+v1].push(+v2)
//   }
//   return graph
// }
//
// function testLoadGraph () {
//   const graph = loadGraph('test.txt')
//   equal(4, numVertices(graph))
//   equal(3, graph[2][0])
//   equal(4, graph[2][1])
// }
//
// const numVertices = graph => Object.keys(graph).length
