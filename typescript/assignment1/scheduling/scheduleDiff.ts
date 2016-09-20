/// <reference path="./typings/index.d.ts" />
import * as fs from 'fs'

interface Job {
  length: number,
  weight: number,
  order: number
}

function loadJobs(filename) {
  const lines = fs.readFileSync(filename, 'UTF-8').split('\n')
  const jobs: Array<Job> = []
  for (let i = 1; i <= +lines[0]; i++) {
    const [length, weight] = lines[i].split(' ').map(s => +s)
    jobs.push({ length, weight, order: 0 })
  }
  return jobs
}

const scheduleDiff = (jobs: Array<Job>) =>
  jobs
    .map(j => ({ weight: j.weight, length: j.length, order: j.weight - j.length }))
    .sort((a,b) => b.order - a.order || a.weight - b.weight)

const scheduleRatio = (jobs: Array<Job>) =>
  jobs.map(j => ({ length: j.length, weight: j.weight, order: j.length / j.weight }))
    .sort((a,b) => a.order - b.order)

const calcScore = (jobs: Array<Job>) => 
  jobs
    .reduce(({completionTime, score}, j) => ({ 
      completionTime: completionTime + j.length,
      score: score + (completionTime + j.length) * j.weight
    }), { completionTime: 0, score: 0 })
    .score


const jobs = loadJobs('jobs.txt')
console.log(calcScore(scheduleDiff(jobs))) // 69119377652
console.log(calcScore(scheduleRatio(jobs))) // 67311454237

// {
//  let completionTime = 0
//  let score = 0
//  for (let i = 0; i < jobs.length; i++) {
//    completionTime += jobs[i][1]
//    score += completionTime * jobs[i][0]
//  }
//  return score
