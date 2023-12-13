#[repr(C)]
pub struct TaskContext {
    ra: usize,
    s: [usize; 12],
}

//goto_restore 方法:
//创建并返回 TaskContext 的一个实例。
//设置 ra 字段为 __restore 函数的地址。这意味着当任务上下文被恢复时，执行流将跳转到 __restore 函数。
//extern "C" 声明:
//extern "C" { fn __restore(); } 声明了一个外部的 C 函数 __restore。这通常表示 __restore 函数是在汇编语言中实现的，用于在任务切换时恢复任务的状态。
//初始化:
//s 数组被初始化为全零，这可能是为了在任务启动时提供一个干净的状态。

impl TaskContext {
    pub fn goto_restore() -> Self {//初始化执行上下文
        extern "C" { fn __restore(); }
        Self {
            ra: __restore as usize,
            s: [0; 12],
        }
    }
}
