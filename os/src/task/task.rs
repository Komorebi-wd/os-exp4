#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    pub task_cx_ptr: usize,//指向 TaskContext 的指针, TaskContext 用于保存任务的上下文信息。
    pub task_status: TaskStatus,//任务状态, 用于标识任务的状态。
}

impl TaskControlBlock {
    pub fn get_task_cx_ptr2(&self) -> *const usize {
        &self.task_cx_ptr as *const usize
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit,//未初始化
    Ready,//就绪
    Running,//运行
    Exited,//退出
}
