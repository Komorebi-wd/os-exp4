use core::arch::global_asm;
//global_asm! 是 Rust 提供的一个宏，用于将汇编代码直接嵌入到 Rust 程序中。
//include_str!("switch.S") 通过将指定的汇编文件内容作为字符串包含进来，使得 switch.S 中定义的汇编代码成为程序的一部分。

global_asm!(include_str!("switch.S"));

extern "C" {
    //这是一个外部函数声明，意味着 __switch 函数的实际定义不在当前的 Rust 文件中，而是在别处定义，
    //函数接受两个参数，都是 *const usize 类型的指针，分别指向当前任务和下一个任务的上下文。
    pub fn __switch(
        current_task_cx_ptr2: *const usize,
        next_task_cx_ptr2: *const usize
    );
}
