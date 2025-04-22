# DSE EXAMPLE

This repository contains an example showcasing how the [Opt4J Framework](https://github.com/SDARG/opt4j) can be used for the optimization of the configuration of a process, e.g., a server.

In particular, the example here shows the interaction between an optimizer implemented within Opt4J and an external evaluator. The example is supposed to run according to the schema detailed below:

![Overview](https://private-user-images.githubusercontent.com/24762889/435947395-22bc8dc3-6d6c-4672-a4c5-6d6d085abb78.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3NDUzMDU5NDksIm5iZiI6MTc0NTMwNTY0OSwicGF0aCI6Ii8yNDc2Mjg4OS80MzU5NDczOTUtMjJiYzhkYzMtNmQ2Yy00NjcyLWE0YzUtNmQ2ZDA4NWFiYjc4LnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNTA0MjIlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjUwNDIyVDA3MDcyOVomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPWVmYWI2Y2VkZjVmNmQ5YjVhY2YwOTZiZWY5MTA1Y2Y3YjQyZDNiNjhjNWJhNTljOWEwM2IxYzZkNDgzMmE2ZWImWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0In0.FHuRRjGcDrQSAbBnt8jWMe9DpPmQgE0Bs-eiWd6EUIY)

In general, the example is supposed to run under Windows. The optimizer (left side) is implemented as a gradle application and is supposed to be run directly within Windows. The evaluator (right side) is implemented as a Rust application. While you should be able to compile it for Windows and run it there, I implemented and tested it while running it within WSL.

Please refer to the [Optimizer Readme](https://github.com/FedorSmirnov89/dse-example/blob/master/optimizer/README.md) and the [Evaluator Readme](https://github.com/FedorSmirnov89/dse-example/blob/master/evaluator/README.md) for details on setup and running the two components.