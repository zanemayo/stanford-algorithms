"use strict";
/// <reference path="./typings/index.d.ts" />
var fs = require('fs');
function loadJobs(filename) {
    var lines = fs.readFileSync(filename, 'UTF-8').split('\n');
    var jobs = [];
    for (var i = 1; i <= +lines[0]; i++) {
        var _a = lines[i].split(' ').map(function (s) { return +s; }), length_1 = _a[0], weight = _a[1];
        jobs.push({ length: length_1, weight: weight, order: 0 });
    }
    return jobs;
}
var scheduleDiff = function (jobs) {
    return jobs
        .map(function (j) { return ({ weight: j.weight, length: j.length, order: j.weight - j.length }); })
        .sort(function (a, b) { return b.order - a.order || a.weight - b.weight; });
};
var scheduleRatio = function (jobs) {
    return jobs.map(function (j) { return ({ length: j.length, weight: j.weight, order: j.length / j.weight }); })
        .sort(function (a, b) { return a.order - b.order; });
};
var calcScore = function (jobs) {
    return jobs
        .reduce(function (_a, j) {
        var completionTime = _a.completionTime, score = _a.score;
        return ({
            completionTime: completionTime + j.length,
            score: score + (completionTime + j.length) * j.weight
        });
    }, { completionTime: 0, score: 0 })
        .score;
};
var jobs = loadJobs('jobs.txt');
console.log(calcScore(scheduleDiff(jobs)));
// console.log(calcScore(scheduleRatio(jobs)))
// {
//  let completionTime = 0
//  let score = 0
//  for (let i = 0; i < jobs.length; i++) {
//    completionTime += jobs[i][1]
//    score += completionTime * jobs[i][0]
//  }
//  return score
