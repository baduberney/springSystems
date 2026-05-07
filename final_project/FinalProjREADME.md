# Concurrent Task Dispatcher (Rust)

## Project Overview

This project implements a concurrent task dispatcher and scheduler in Rust.
The system simulates an operating system style scheduler where tasks arrive over time, enter queues, and are dispatched to a bounder worker pool according to a scheduling policy.

This project demonstrates:

- multi-threaded system design
- bounded worker pools
- concurrent queues
- scheduling policies
- synchronization with Rust concurrency primitives
- workload simulation
- runtime metrics collection
- clean shutdown behavior

The simulation supports both CPU-bound and IO-bound tasks and compares scheduler behavior under different workloads.

---

# Features

## Implemented Components

- Concurrent task generation
- Queue-based scheduling system
- Fixed-size worker pool
- FIFO scheduler
- Optimized scheduler with CPU-aware dispatching
- Real-time monitoring thread
- Metrics collection and reporting
- Graceful shutdown

---

## Tool Use Disclosure

The main tool that was used for this project was ChatGPT.

The main help that was provided by ChatGPT was:

- debugging Rust compile errors
- improving synchronization design
- understanding Rust concurrency primitives
- formatting output and metrics reporting

### Advice Accepted:
One piece of advice that I accepted from AI was separating CPU-bound and IO-tasks into different queues. This resulted in improved worker utilization and reduced overall makespan during the optimized scheduling experiment.

### Advice Rejected:
An earlier suggestion attempted to clone Rust "Receiver" objects from channels so multiple workers could consume tasks directly. This caused compilation errors because standard Rust "Receiver" types are not cloneable. The design was corrected by replacing shared receivers with Arc<Mutex<VecDeque<Task>>> queues protected by synchronization primitives.

---

## How to Run

All thats needed to run the program is the command below on the final_project directory:

```
cargo run
```

---