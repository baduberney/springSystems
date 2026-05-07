use rand::{RngCore, SeedableRng};
use rand::rngs::StdRng;

use std::{
    collections::VecDeque,
    fs::File,
    io::Write,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant},
};

const TOTAL_TASKS: usize = 1000;
const WORKERS: usize = 8;
const CPU_LIMIT: usize = 100;

#[derive(Clone, Debug)]
enum TaskKind {
    CPU,
    IO,
}

#[derive(Clone, Debug)]
struct Task {
    id: usize,
    kind: TaskKind,
    duration_ms: u64,
    cpu_cost: usize,

    arrival_time: Instant,
    start_time: Option<Instant>,
    finish_time: Option<Instant>,
}

#[derive(Default)]
struct Metrics {
    completed: usize,

    io_completed: usize,
    cpu_completed: usize,

    total_wait: u128,
    total_turnaround: u128,

    io_wait_total: u128,
    cpu_wait_total: u128,

    io_wait_count: usize,
    cpu_wait_count: usize,

    max_wait: u128,
    max_wait_task: usize,

    cpu_usage_samples: usize,
    cpu_usage_total: usize,

    worker_samples: usize,
    workers_active_total: usize,

    monitor_samples: usize,
}

fn build_tasks(io_ratio: f64, seed: u64) -> Vec<Task> {
    let mut rng = StdRng::seed_from_u64(seed);

    let mut tasks = Vec::new();

    for i in 0..TOTAL_TASKS {
        let is_io =
            (rng.next_u32() % 100) < (io_ratio * 100.0) as u32;

        let kind = if is_io {
            TaskKind::IO
        } else {
            TaskKind::CPU
        };

        let duration = if is_io {
            180 + (rng.next_u32() % 40) as u64
        } else {
            180 + (rng.next_u32() % 40) as u64
        };

        let cpu_cost = if is_io { 10 } else { 35 };

        tasks.push(Task {
            id: i,
            kind,
            duration_ms: duration,
            cpu_cost,
            arrival_time: Instant::now(),
            start_time: None,
            finish_time: None,
        });
    }

    tasks
}

fn run_simulation(name: &str, optimized: bool) {
    println!("\n== {} simulation ==", name);

    println!(
        "{} tasks, 70% IO / 30% CPU, {} workers, cap {}%",
        TOTAL_TASKS,
        WORKERS,
        CPU_LIMIT
    );

    let metrics = Arc::new(Mutex::new(Metrics::default()));

    let current_cpu = Arc::new(AtomicUsize::new(0));
    let workers_active = Arc::new(AtomicUsize::new(0));

    let done = Arc::new(AtomicBool::new(false));

    let queue = Arc::new(Mutex::new(VecDeque::<Task>::new()));

    let start = Instant::now();

    

    let monitor_metrics = metrics.clone(); // Monitor
    let monitor_cpu = current_cpu.clone();
    let monitor_workers = workers_active.clone();
    let monitor_done = done.clone();

    let mut csv =
        File::create("monitor_log.csv").unwrap();

    writeln!(
        csv,
        "time_ms,cpu_usage,workers_active"
    )
    .unwrap();

    let monitor_handle = thread::spawn(move || {
        let monitor_start = Instant::now();

        while !monitor_done.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(10));

            let cpu =
                monitor_cpu.load(Ordering::SeqCst);

            let active =
                monitor_workers.load(Ordering::SeqCst);

            {
                let mut m =
                    monitor_metrics.lock().unwrap();

                m.cpu_usage_total += cpu;
                m.cpu_usage_samples += 1;

                m.workers_active_total += active;
                m.worker_samples += 1;

                m.monitor_samples += 1;
            }

            writeln!(
                csv,
                "{},{},{}",
                monitor_start.elapsed().as_millis(),
                cpu,
                active
            )
            .unwrap();
        }
    });

    

    let generator_queue = queue.clone(); // Task generator

    let generator = thread::spawn(move || {
        let tasks = build_tasks(0.70, 42);

        for task in tasks {
            generator_queue
                .lock()
                .unwrap()
                .push_back(task);

            thread::sleep(Duration::from_millis(20));
        }
    });

    

    let mut worker_handles = Vec::new(); // Workers

    for _ in 0..WORKERS {
        let queue = queue.clone();

        let metrics = metrics.clone();

        let current_cpu = current_cpu.clone();

        let workers_active = workers_active.clone();

        let done = done.clone();

        let handle = thread::spawn(move || {
            loop {
                let maybe_task = {
                    let mut q = queue.lock().unwrap();

                    if optimized {
                        /* optimized:
                           prioritize CPU tasks
                           when enough CPU headroom exists
                        */

                        let current =
                            current_cpu.load(
                                Ordering::SeqCst,
                            );

                        let mut index = None;

                        for (i, t) in
                            q.iter().enumerate()
                        {
                            match t.kind {
                                TaskKind::CPU => {
                                    if current
                                        + t.cpu_cost
                                        <= CPU_LIMIT
                                    {
                                        index =
                                            Some(i);
                                        break;
                                    }
                                }

                                TaskKind::IO => {
                                    if index.is_none()
                                    {
                                        index =
                                            Some(i);
                                    }
                                }
                            }
                        }

                        if let Some(i) = index {
                            q.remove(i)
                        } else {
                            None
                        }
                    } else {
                        q.pop_front()
                    }
                };

                match maybe_task {
                    Some(mut task) => {
                        while current_cpu
                            .load(Ordering::SeqCst)
                            + task.cpu_cost
                            > CPU_LIMIT
                        {
                            thread::sleep(
                                Duration::from_millis(
                                    1,
                                ),
                            );
                        }

                        current_cpu.fetch_add(
                            task.cpu_cost,
                            Ordering::SeqCst,
                        );

                        workers_active.fetch_add(
                            1,
                            Ordering::SeqCst,
                        );

                        task.start_time =
                            Some(Instant::now());

                        let wait = task
                            .start_time
                            .unwrap()
                            .duration_since(
                                task.arrival_time,
                            )
                            .as_millis();

                        thread::sleep(
                            Duration::from_millis(
                                task.duration_ms,
                            ),
                        );

                        task.finish_time =
                            Some(Instant::now());

                        let turnaround = task
                            .finish_time
                            .unwrap()
                            .duration_since(
                                task.arrival_time,
                            )
                            .as_millis();

                        {
                            let mut m =
                                metrics.lock().unwrap();

                            m.completed += 1;

                            m.total_wait += wait;

                            m.total_turnaround +=
                                turnaround;

                            if wait > m.max_wait {
                                m.max_wait = wait;
                                m.max_wait_task =
                                    task.id;
                            }

                            match task.kind {
                                TaskKind::CPU => {
                                    m.cpu_completed +=
                                        1;

                                    m.cpu_wait_total +=
                                        wait;

                                    m.cpu_wait_count +=
                                        1;
                                }

                                TaskKind::IO => {
                                    m.io_completed +=
                                        1;

                                    m.io_wait_total +=
                                        wait;

                                    m.io_wait_count +=
                                        1;
                                }
                            }
                        }

                        current_cpu.fetch_sub(
                            task.cpu_cost,
                            Ordering::SeqCst,
                        );

                        workers_active.fetch_sub(
                            1,
                            Ordering::SeqCst,
                        );
                    }

                    None => {
                        if done.load(
                            Ordering::SeqCst,
                        ) {
                            break;
                        }

                        thread::sleep(
                            Duration::from_millis(
                                1,
                            ),
                        );
                    }
                }
            }
        });

        worker_handles.push(handle);
    }

    generator.join().unwrap();

    loop {
        {
            let m = metrics.lock().unwrap();

            if m.completed >= TOTAL_TASKS {
                break;
            }
        }

        thread::sleep(Duration::from_millis(50));
    }

    done.store(true, Ordering::SeqCst);

    for h in worker_handles {
        h.join().unwrap();
    }

    monitor_handle.join().unwrap();

    // Results 

    let runtime = start.elapsed().as_millis();

    let m = metrics.lock().unwrap();

    println!("\n-- results --");

    println!(
        "total runtime         : {} ms",
        runtime
    );

    println!(
        "makespan              : {} ms",
        runtime
    );

    println!(
        "tasks completed       : {} (IO={}, CPU={})",
        m.completed,
        m.io_completed,
        m.cpu_completed
    );

    println!(
        "avg wait time         : {:.2} ms",
        m.total_wait as f64
            / m.completed as f64
    );

    if optimized {
        println!(
            "avg wait (IO only)   : {:.2} ms",
            m.io_wait_total as f64
                / m.io_wait_count as f64
        );

        println!(
            "avg wait (CPU only)  : {:.2} ms",
            m.cpu_wait_total as f64
                / m.cpu_wait_count as f64
        );
    }

    println!(
        "avg turnaround time   : {:.2} ms",
        m.total_turnaround as f64
            / m.completed as f64
    );

    println!(
        "max wait time         : {} ms (task #{})",
        m.max_wait,
        m.max_wait_task
    );

    println!(
        "avg CPU usage         : {:.2} %",
        m.cpu_usage_total as f64
            / m.cpu_usage_samples as f64
    );

    println!(
        "avg workers active    : {:.2} / {}",
        m.workers_active_total as f64
            / m.worker_samples as f64,
        WORKERS
    );

    println!(
        "monitor samples       : {}",
        m.monitor_samples
    );

    println!(
        "monitor csv           : monitor_log.csv"
    );
}

fn main() {
    run_simulation("FIFO", false);

    run_simulation("Optimized", true);
}