use std::sync::{Arc, Condvar, Mutex};
use std::{collections::VecDeque, thread::JoinHandle};

pub struct MyThreadPool {
    work_queue: Arc<Mutex<VecDeque<Box<dyn Send + FnOnce() -> ()>>>>,
    join_handles: Vec<Option<JoinHandle<()>>>,

    work_item_count: Arc<Mutex<i32>>,
    signal: Arc<Condvar>,
}

impl Drop for MyThreadPool {
    fn drop(&mut self) {
        {
            // A.
            let mut g = self.work_item_count.lock().unwrap();
            (*g) = -1; // B.
            self.signal.notify_all();
        }

        for h in &mut self.join_handles {
            // C.
            let x: JoinHandle<()> = std::mem::replace(h, None).unwrap();
            _ = x.join();
        }
    }
}

impl MyThreadPool {
    pub fn new(n: usize) -> Self {
        let workq = Arc::new(Mutex::new(VecDeque::new()));
        let itemcount = Arc::new(Mutex::new(0_i32));
        let signal = Arc::new(Condvar::new());

        let mut pool: MyThreadPool = MyThreadPool {
            work_queue: workq.clone(),
            join_handles: Vec::with_capacity(n),
            work_item_count: itemcount.clone(),
            signal: signal.clone(),
        };

        for _idx in 0..n {
            let wq = workq.clone();
            let itmcnt = itemcount.clone();
            let sig = signal.clone();

            let h: JoinHandle<()> = std::thread::spawn(move || {
                loop {
                    let shouldbreak: bool = {
                        let mut g = itmcnt.lock().unwrap();
                        while *g == 0 {
                            g = sig.wait(g).unwrap(); // C.
                            if (*g) == 0 {
                                println!("{:?} Spurious Wakeup.", std::thread::current().id());
                            }
                        }

                        if (*g) > 0 {
                            (*g) -= 1;
                        }

                        (*g) == -1
                    };

                    if shouldbreak {
                        break;
                    }

                    let workfnopt = {
                        let mut g = wq.lock().unwrap();
                        g.pop_back()
                    };

                    if let Some(workfn) = workfnopt {
                        workfn();
                    } else {
                        println!(
                            "Worker {:?} SOMETHING IS WRONG!",
                            std::thread::current().id()
                        );
                    }
                }

                // Wind-down loop
                loop {
                    let workfnopt = {
                        let mut g = wq.lock().unwrap();
                        g.pop_back()
                    };

                    if let Some(workfn) = workfnopt {
                        workfn();
                    } else {
                        break;
                    }
                }
            });

            pool.join_handles.push(Some(h)); // J.
        }

        pool
    }

    pub fn queue_work(&self, work: Box<dyn Send + FnOnce() -> ()>) {
        let mut g = self.work_item_count.lock().unwrap();
        assert!((*g) >= 0);
        (*g) += 1;
        let mut guard = self.work_queue.lock().unwrap();
        guard.push_front(work);
        self.signal.notify_one();
    }
}
