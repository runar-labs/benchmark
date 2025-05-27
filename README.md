# Benchmark

This repository is intended for benchmarking and performance testing related to the `runar-labs` ecosystem.

## Intention

- To serve as a home for reproducible performance benchmarks.
- To document benchmark scenarios and results.
- To provide scripts and tools for automated benchmarking.

## Service Interaction Pattern

This benchmark scenario consists of five cooperating services (A–E) and a dedicated broker (e.g., MQTT server), each running in its own container. The intention and contract for each service are as follows:

- **Service A**: Starts the workflow with a list of 5,000 items. For each item, it requests lookup information from Service B and Service C (in parallel), combines their responses, and forwards the aggregated result to Service D.
- **Service B**: Receives lookup requests from Service A, processes each item, and returns corresponding information to Service A.
- **Service C**: Receives lookup requests from Service A, processes each item, and returns corresponding information to Service A.
- **Service D**: Receives the aggregated result from Service A, appends additional data, and forwards it to Service E.
- **Service E**: Receives data from Service D, appends final data, and completes the pipeline (e.g., logs or stores the result).
- **Broker**: A dedicated container (e.g., MQTT server) that mediates all inter-service communication.

### Data Types and Payloads
- All inter-service messages use well-defined, serializable data types (see `services/common/types.rs`).
- The primary data structure is a list, and the number of items and size of payloads can be varied via configuration to adjust the benchmark's difficulty and simulate different workloads.

### Architectural Boundaries
- Each service is isolated in its own container and communicates only via the broker.
- No direct inter-service communication; all messages are brokered.
- The broker is not modified or instrumented beyond its standard configuration.
- All service and data type intentions are documented in the source and test files.

## Benchmarking Strategy

Benchmarks in this repository are designed to be reproducible, fair, and comparable across multiple messaging solutions.

### Approach
- **Docker Compose** is used to orchestrate benchmarks, ensuring each test runs in a controlled, isolated environment.
- All benchmarks use the same base images for consistency.
- Each service under test runs in its own container image.
- A typical test scenario involves **five services** communicating with each other, simulating realistic distributed workloads.

### Metrics Captured
- **Memory usage**
- **CPU consumption**
- **Response time** (latency)

### Generic Benchmarking Framework
- The benchmarking setup is generic, allowing for the addition of new messaging solutions with minimal changes.
- Scripts and configuration files are provided to automate test execution and metrics collection.

### Solutions Compared
- Our own **FMNK** (Fast Message Network Kernel)
- **MQTT**
- **NATS**
- Other similar messaging or eventing solutions

### Goals
- Provide apples-to-apples comparisons between different messaging technologies.
- Capture and document system resource usage and latency under identical workloads.
- Enable reproducible results for future reference and improvement tracking.

## Directory Structure

```
benchmark/
├── README.md
├── docker-compose/
│   ├── mqtt/
│   │   └── docker-compose.yml
│   ├── fmnk/
│   │   └── docker-compose.yml
│   ├── nats/
│   │   └── docker-compose.yml
│   └── ... (other solutions)
├── services/
│   ├── common/
│   │   └── types.rs
│   ├── mqtt/
│   │   └── service_{a..e}/main.rs
│   ├── fmnk/
│   │   └── service_{a..e}/main.rs
│   ├── nats/
│   │   └── service_{a..e}/main.rs
│   └── ... (other solutions)
├── scripts/
│   ├── run_benchmark.sh
│   ├── collect_metrics.sh
│   └── compare_results.py
├── results/
│   └── ...
└── config/
    └── ...
```

## Results Versioning

- Service and docker-compose implementations are shared per solution (not per version).
- Each benchmark run stores its results in the `results/` directory, with the filename encoding:
  - The solution (e.g., fmnk, mqtt, nats)
  - The version or git commit/tag/branch of the solution under test (provided as an argument or detected automatically)
  - The timestamp of the run
- Example results filenames:
  - `results/fmnk_main_20250526_2135.json` (main branch)
  - `results/fmnk_v1.2.0_20250526_2150.json` (tagged release)
  - `results/mqtt_20250526_2200.json`

## How to Run and Compare Benchmarks

1. **Running a Benchmark**
   - Use: `scripts/run_benchmark.sh <solution> [version]`
   - This script:
     - Spins up the appropriate docker-compose stack (e.g., `docker-compose/mqtt/docker-compose.yml`)
     - Runs the benchmark scenario
     - Collects memory, CPU, and latency metrics
     - Saves results to a timestamped, versioned file in `results/`

2. **Comparing Results**
   - Use: `scripts/compare_results.py`
   - This script:
     - Loads results from the `results/` directory
     - Tabulates and visualizes CPU, memory, and latency across solutions and versions
     - Enables easy comparison of FMNK to itself (across versions) and to other solutions

## Architectural Boundaries

- All test and benchmark intentions are documented in this README and in test files.
- No service code duplication for versioning; only results are versioned.
- Scripts and configs are generic and parameterized for extensibility.

---

*This README documents the architectural boundaries and intentions for benchmarking in this repository. All changes to test strategy or structure must update this documentation first.*
