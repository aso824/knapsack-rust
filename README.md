# Knapsack problem

## Introduction

The knapsack problem goal is to reach the maximum value of items, from pool of available ones, in a limited cargo space.  
This problem has been studied for more than one century and is classified as [NP-complete](https://en.wikipedia.org/wiki/NP-completeness).  
Full description is available on [Wikipedia](https://en.wikipedia.org/wiki/Knapsack_problem)

## How to run this program
 
Set up newest Rust version using [rustup](https://rustup.rs/) and run main program:

    cargo run data/small.json 15.00

Arguments:
* **data/small.json** - input file with map of weights and values
* **15.00** - maximum allowed weight

Output has same structure as input. 

## Data structures

Data stored in JSON, examples available in `/data` directory.  
Input file should be JSON array with objects:

    [
        {
            "weight": 12.0,
            "value": 4.0
        },
        ...
    ]
 
 