# Advent of Code 2022

Solutions constructed in Rust as a workspace.
Each crate corresponds to one exercise.

## Performance
Execute the bash script `time.sh` to execute a crude timing function.

```
sh time.sh
```

#### Approximate timings

| Exercise | Time (s) |
|:--------:|:--------:|
| Day01    | .00179   |
| Day02    | .00211   |
| Day03    | .00183   |
| Day04    | .00246   |
| Day05    | .00193   |
| Day06    | .00321   |
| Day07    | .00178   |
| Day08    | .00461   |
| Day09    | .00387   |
| Day10    | .00208   |
| Day11    | .01237   |
| Day12    | .00308   |
| Day13    | .00305   |
| Day14    | .00948   |
| Day15    | .07506   |
| Day16    | Insert   |
| Day17    | .00316   |
| Day18    | .01101   |
| Day19    | Insert   |
| Day20    | .10719   |
| Day21    | .00241   |
| Day22    | .00210   |
| Day23    | .97738   |
| Day24    | .00185   |
| Day25    | .00159   |
| -------- | -------- |
| **TOTAL** | **1.30939** |


## Inspiration
The exercises have all been solved by me but for some of them inspiration from other programmers have been taking in order to either optimise the solution, or help solve it completely.
Below is a list of credits given where credit is due:

* Day 13 - [albheim](https://github.com/albheim)
    - Solved the parsing that I didn't understand.
* Day 15 - [Sh4d1](https://github.com/Sh4d1)
    - Realisation that we only have to search one edge of each "diamond"-shape 
* Day 17 - [mvmorin](https://github.com/mvmorin)
    - Neat solution to finding the periodic pattern of falling rocks based on the pattern of jet streams and falling rocks
* Day 24 - [SLiV9](https://github.com/SLiV9)
    - Clever way of letting particles float around in the feasible configurations until a solution has been found.
