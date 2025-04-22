# Evaluator Readme

The evaluator application demonstrates an implementation of an external evaluator and its integration into an optimization based on Opt4J.

## Setup / Prerequisites

The only prerequisite for running the evaluator application is an installation of [Rust](https://www.rust-lang.org/tools/install). Make sure that the port 3000 of your machine is free, since the evaluator application will attempt to open a port there.

## Running the application

The evaluator application is run via 

```
cargo run
```

from the evaluator directory. 

## Adjusting the timing

To get a feel how the evaluation latency affects the overall progress of the optimization, you can adjust the delay config in `delay_config.json` (you will have to restart the evaluator application for the config change to take effect). 