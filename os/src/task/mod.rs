mod context;
mod switch;
mod task;

use crate::config::MAX_APP_NUM;
use crate::loader::{get_num_app, init_app_cx};
use core::cell::RefCell;
use lazy_static::*;
use switch::__switch;
use task::{TaskControlBlock, TaskStatus};

pub use context::TaskContext;

pub struct TaskManager {
    num_app: usize,
    inner: RefCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
}

unsafe impl Sync for TaskManager {}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        //let mut tasks = [TaskControlBlock { task_cx_ptr: 0, task_status: TaskStatus::UnInit }; MAX_APP_NUM]; 定义并初始化一个 TaskControlBlock 数组。每个 TaskControlBlock 初始时都设置为未初始化状态
        let mut tasks = [
            TaskControlBlock { task_cx_ptr: 0, task_status: TaskStatus::UnInit };
            MAX_APP_NUM
        ];
        for i in 0..num_app {
            //tasks[i].task_cx_ptr = init_app_cx(i) as *const _ as usize; 初始化每个任务的上下文指针。
            tasks[i].task_cx_ptr = init_app_cx(i) as * const _ as usize;
            //tasks[i].task_status = TaskStatus::Ready; 将每个任务的状态设置为就绪。
            tasks[i].task_status = TaskStatus::Ready;
        }
        TaskManager {
            num_app,//num_app 字段记录了应用的数量。
            inner: RefCell::new(TaskManagerInner {
                tasks,//tasks 字段记录了所有任务的 TaskControlBlock。
                current_task: 0,//current_task 字段记录了当前正在运行的任务的 ID。
            }),
        }
    };
}

impl TaskManager {
    fn run_first_task(&self) {
        self.inner.borrow_mut().tasks[0].task_status = TaskStatus::Running;
        let next_task_cx_ptr2 = self.inner.borrow().tasks[0].get_task_cx_ptr2();
        let _unused: usize = 0;
        unsafe {
            __switch(
                &_unused as *const _,
                next_task_cx_ptr2,
            );
        }
    }

    fn mark_current_suspended(&self) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    fn mark_current_exited(&self) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }

    fn find_next_task(&self) -> Option<usize> {
//用 TaskManagerInner:
// let inner = self.inner.borrow(); 获取 TaskManagerInner 的一个不可变引用。这使得方法可以访问和检查当前的任务和它们的状态。
// 获取当前任务索引:
// let current = inner.current_task; 获取当前正在运行或最近运行的任务的索引。
 
// 查找就绪任务:
// (current + 1..current + self.num_app + 1) 创建一个从当前任务之后开始，直到所有任务都被考虑一遍的范围。
// .map(|id| id % self.num_app) 确保任务索引在有效范围内（即循环到任务列表的开始）。
// .find(|id| { inner.tasks[*id].task_status == TaskStatus::Ready }) 在这个范围内查找第一个状态为“就绪”的任务。

// 返回值:
// 方法返回 Option<usize> 类型。如果找到了一个就绪的任务，它将返回 Some(索引)，其中“索引”是就绪任务在 tasks 数组中的位置；如果没有找到就绪的任务，则返回 None。

        let inner = self.inner.borrow();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| {
                inner.tasks[*id].task_status == TaskStatus::Ready
            })
    }

    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.borrow_mut();
            let current = inner.current_task;
            inner.tasks[next].task_status = TaskStatus::Running;
            inner.current_task = next;
            let current_task_cx_ptr2 = inner.tasks[current].get_task_cx_ptr2();
            let next_task_cx_ptr2 = inner.tasks[next].get_task_cx_ptr2();
            core::mem::drop(inner);
            unsafe {
                __switch(
                    current_task_cx_ptr2,
                    next_task_cx_ptr2,
                );
            }
        } else {
            panic!("All applications completed!");
        }
    }
}

pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}
