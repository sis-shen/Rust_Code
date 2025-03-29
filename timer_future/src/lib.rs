use std::{
    future::Future,
    pin::Pin,
    sync::{Arc,Mutex},
    task::{Context,Poll,Waker},
    thread,
    time::Duration,
};

pub struct TimeFuture{
    shared_state: Arc<Mutex<SharedState>>,
}

/// 在Future和等待的线程共享状态
struct SharedState{
    /// 定时是否结束
    completed:bool,

    /// 当睡眠结束后，线程可以用`waker`通知`TimeFuture`来唤醒任务
    waker: Option<Waker>,
}

impl Future for TimeFuture{
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed{
            Poll::Ready(())
        }else{
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimeFuture{
    /// 创建新的TimeFuture
    pub fn new(duration:Duration)->Self{
        let shared_state = Arc::new(Mutex::new(SharedState{
            completed:false,
            waker:None,
        }));

        //创建新线程
        let thread_shared_state = shared_state.clone();
        thread::spawn(move ||{
            //睡眠指定时间
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take(){
                waker.wake()
            }
        });

        TimeFuture { shared_state }
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
