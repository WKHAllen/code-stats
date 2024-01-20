use std::future::Future;
use std::pin::Pin;
use tokio::spawn;
use tokio::sync::mpsc::{channel, Sender};
use tokio::task::JoinHandle;

/// Type alias for a heap-allocated thread-safe asynchronous task.
type AsyncTask = Pin<Box<dyn Future<Output = ()> + Send>>;

/// A task request for a worker.
enum TaskRequest {
    /// A task needs to be run.
    Run(AsyncTask),
    /// Exit the task running context.
    Exit,
}

/// A message signifying that a worker is ready.
enum TaskResponse {
    /// The worker is ready.
    Ready(usize),
}

/// A worker that can perform tasks.
struct Worker {
    /// The join handle for the worker.
    handle: JoinHandle<()>,
    /// The channel through which task requests can be sent.
    task_sender: Sender<TaskRequest>,
}

impl Worker {
    /// Creates a new worker.
    pub fn new(id: usize, ready_sender: Sender<TaskResponse>) -> Self {
        let (task_sender, mut task_receiver) = channel(1);

        let handle = spawn(async move {
            ready_sender.send(TaskResponse::Ready(id)).await.unwrap();

            while let Some(req) = task_receiver.recv().await {
                match req {
                    TaskRequest::Run(job) => {
                        job.await;
                    }
                    TaskRequest::Exit => {
                        break;
                    }
                }

                ready_sender.send(TaskResponse::Ready(id)).await.unwrap();
            }
        });

        Self {
            handle,
            task_sender,
        }
    }

    /// Signals the worker to run the given task.
    pub async fn run(&self, task: AsyncTask) {
        self.task_sender.send(TaskRequest::Run(task)).await.unwrap();
    }

    /// Sends the exit request and waits for the worker to finish.
    pub async fn finish(self) {
        self.task_sender.send(TaskRequest::Exit).await.unwrap();
        self.handle.await.unwrap();
    }
}

/// A task pool.
pub struct TaskPool {
    /// The channel through which task requests can be sent.
    task_sender: Sender<TaskRequest>,
    /// The handle to the background task handling task requests.
    handle: JoinHandle<()>,
}

impl TaskPool {
    /// Creates a new task pool with the given number of tasks.
    pub fn new(size: usize) -> Self {
        let (task_sender, mut task_receiver) = channel(size);
        let (ready_sender, mut ready_receiver) = channel(size);

        let handle = spawn(async move {
            let workers = (0..size)
                .map(|id| Worker::new(id, ready_sender.clone()))
                .collect::<Vec<_>>();

            while let Some(req) = task_receiver.recv().await {
                match req {
                    TaskRequest::Run(task) => {
                        let res = ready_receiver.recv().await.unwrap();

                        match res {
                            TaskResponse::Ready(worker_id) => {
                                workers.get(worker_id).unwrap().run(task).await;
                            }
                        }
                    }
                    TaskRequest::Exit => {
                        break;
                    }
                }
            }

            for worker in workers {
                worker.finish().await;
            }
        });

        Self {
            task_sender,
            handle,
        }
    }

    /// Queues a task to be executed.
    pub async fn queue<F>(&self, task: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.task_sender
            .send(TaskRequest::Run(Box::pin(task)))
            .await
            .unwrap();
    }

    /// Instructs all tasks to finish and awaits their collective completion.
    pub async fn finish(self) {
        self.task_sender.send(TaskRequest::Exit).await.unwrap();
        self.handle.await.unwrap();
    }
}

/// Task pool tests.
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
    use tokio::time::sleep;

    /// Tests the task pool.
    #[tokio::test]
    async fn test_task_pool() {
        async fn send_with_delay<T>(sender: UnboundedSender<T>, message: T, secs: f64) {
            sleep(Duration::from_secs_f64(secs)).await;
            sender.send(message).unwrap();
        }

        let pool = TaskPool::new(3);
        let (sender, mut receiver) = unbounded_channel::<usize>();

        pool.queue(send_with_delay(sender.clone(), 1, 0.5)).await; // 0.0 - 0.5
        pool.queue(send_with_delay(sender.clone(), 2, 0.3)).await; // 0.0 - 0.3
        pool.queue(send_with_delay(sender.clone(), 3, 0.4)).await; // 0.0 - 0.4
        pool.queue(send_with_delay(sender.clone(), 4, 0.3)).await; // 0.3 - 0.6
        pool.queue(send_with_delay(sender.clone(), 5, 0.0)).await; // 0.4 - 0.4
        pool.queue(send_with_delay(sender.clone(), 6, 0.4)).await; // 0.4 - 0.8
        pool.queue(send_with_delay(sender.clone(), 7, 0.2)).await; // 0.5 - 0.7
        pool.finish().await;
        drop(sender);

        assert_eq!(receiver.recv().await, Some(2));
        assert_eq!(receiver.recv().await, Some(3));
        assert_eq!(receiver.recv().await, Some(5));
        assert_eq!(receiver.recv().await, Some(1));
        assert_eq!(receiver.recv().await, Some(4));
        assert_eq!(receiver.recv().await, Some(7));
        assert_eq!(receiver.recv().await, Some(6));
        assert_eq!(receiver.recv().await, None);
    }
}
