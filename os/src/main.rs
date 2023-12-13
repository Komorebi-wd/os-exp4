#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod syscall;
mod trap;
mod loader;
mod config;
mod task;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("[Kernel] Hello, world!");
    trap::init();
    loader::load_apps();
    task::run_first_task();
    panic!("Unreachable in rust_main!");
}

/*
1. 应用程序的编译和链接
编译:

应用程序源代码首先需要被编译成可执行文件。这通常涉及到将源代码编译成机器码，并链接任何必要的库。
链接脚本 (linker.ld):

使用特定的链接脚本来确定应用程序在内存中的位置。在多道程序系统中，每个应用程序可能需要被加载到不同的内存地址。
2. 加载应用程序到内存
加载程序 (loader::load_apps):
操作系统在启动时或在需要时将应用程序加载到内存中。这通常涉及到读取应用程序的可执行文件，并将其内容复制到内存的指定位置。
3. 设置应用程序的初始上下文
初始化任务上下文 (init_app_cx):

为每个应用程序创建和初始化 TaskContext，包括设置初始的程序计数器（PC）、栈指针等。这些信息存储在内核栈上。
内核栈和用户栈:

每个应用程序通常有自己的内核栈和用户栈，这些栈用于存储执行期间的数据。
4. 启动第一个应用程序
运行第一个任务 (task::run_first_task):
选择第一个应用程序作为启动点，并通过 __switch 函数进行上下文切换，将控制权转交给该应用程序。
5. 系统调用和中断处理
系统调用 (sys_yield, sys_exit 等):

应用程序可以通过系统调用与操作系统交互，例如请求资源或主动让出 CPU。
中断处理:

操作系统处理来自硬件或应用程序的中断请求，这可能导致任务切换或资源分配。
6. 调度和任务切换
任务管理器 (TaskManager):

负责维护任务的状态，并根据调度策略切换不同的任务。
协作式调度:

在协作式调度模式中，应用程序通过 sys_yield 显式地让出 CPU，允许操作系统切换到其他任务。
7. 应用程序的退出处理
退出和清理 (sys_exit):
当应用程序完成其执行或需要终止时，它会调用 sys_exit 来通知操作系统。

*/
