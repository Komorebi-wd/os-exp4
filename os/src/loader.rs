use core::arch::asm;
use crate::trap::TrapContext;
use crate::task::TaskContext;
use crate::config::*;

#[repr(align(4096))]//确保每个栈在内存中按照 4096 字节（4KB）对齐，页对齐的常见要求。
//Copy Trait
//Copy 是一个标记 trait，表示类型的值可以通过字节级的复制来复制。换句话说，类型的值可以被“复制粘贴”而不会违反任何内存安全规则。
//当一个类型实现了 Copy trait，意味着变量赋值、函数参数传递等操作会产生该类型值的副本。原始值在这些操作后仍然有效并保持不变。
//不是所有类型都可以安全地实现 Copy。例如，对于包含堆分配的内存、文件句柄、网络套接字等资源的类型，简单的字节复制可能会导致资源泄露或数据竞争。


//Clone Trait
//Clone trait 允许显式地创建类型的值的副本。
//当调用 .clone() 方法时，它会返回值的副本。与 Copy 不同，Clone 可以执行深复制（deep copy），这对于包含如堆分配内存的复杂类型很重要。
//Clone 实现可以更加复杂，允许自定义复制逻辑，确保副本的创建是安全和正确的。

#[derive(Copy, Clone)]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
#[derive(Copy, Clone)]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

static KERNEL_STACK: [KernelStack; MAX_APP_NUM] = [
    KernelStack { data: [0; KERNEL_STACK_SIZE], };
    MAX_APP_NUM
];

static USER_STACK: [UserStack; MAX_APP_NUM] = [
    UserStack { data: [0; USER_STACK_SIZE], };
    MAX_APP_NUM
];

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
    pub fn push_context(&self, trap_cx: TrapContext, task_cx: TaskContext) -> &'static mut TaskContext {
        unsafe {
            let trap_cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
            *trap_cx_ptr = trap_cx;
            let task_cx_ptr = (trap_cx_ptr as usize - core::mem::size_of::<TaskContext>()) as *mut TaskContext;
            *task_cx_ptr = task_cx;
            task_cx_ptr.as_mut().unwrap()
        }
    }
}

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

fn get_base_i(app_id: usize) -> usize {
    APP_BASE_ADDRESS + app_id * APP_SIZE_LIMIT
}

pub fn get_num_app() -> usize {
    extern "C" { fn _num_app(); }
    unsafe { (_num_app as usize as *const usize).read_volatile() }
}

pub fn load_apps() {
    extern "C" { fn _num_app(); }
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = get_num_app();
    let app_start = unsafe {
        core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1)
    };
    // clear i-cache first
    unsafe { asm!("fence.i"); }
    // load apps
    for i in 0..num_app {
        let base_i = get_base_i(i);// 计算当前应用程序的基础内存地址。该地址是应用程序将要加载到内存中的位置。
        // clear region
        (base_i..base_i + APP_SIZE_LIMIT).for_each(|addr| unsafe {
            (addr as *mut u8).write_volatile(0)
        });//清空应用程序将要加载的内存区域。
        // load app from data section to memory

        //加载应用程序：
        //从应用程序的起始地址 (app_start[i]) 到结束地址 (app_start[i + 1]) 创建 src 切片，该切片代表内存中实际的应用程序代码/数据。
        //dst 切片代表应用程序将要加载到的内存区域。它与 src 切片长度相同，并从之前计算的基地址开始。
        //dst.copy_from_slice(src); 执行实际的复制操作，将应用程序数据从源位置（src）复制到目标位置（dst）

        let src = unsafe {
            core::slice::from_raw_parts(app_start[i] as *const u8, app_start[i + 1] - app_start[i])
        };
        let dst = unsafe {
            core::slice::from_raw_parts_mut(base_i as *mut u8, src.len())
        };
        dst.copy_from_slice(src);
    }
}

//init_app_cx 函数的目的是为指定的应用程序（通过 app_id 索引）初始化其任务上下文。
//访问内核栈:
//KERNEL_STACK[app_id] 访问的是与特定应用程序关联的内核栈。KERNEL_STACK 是一个数组，其中每个元素都是一个内核栈，用于不同的应用程序。
//推送上下文到内核栈:
//push_context 方法用于将 TrapContext 和 TaskContext 对象推入内核栈。
//TrapContext::app_init_context(get_base_i(app_id), USER_STACK[app_id].get_sp()) 创建一个 TrapContext 实例，其初始化的内容基于应用程序的基地址和用户栈的栈顶指针。
//TaskContext::goto_restore() 创建一个 TaskContext 实例，用于后续的任务恢复操作。

pub fn init_app_cx(app_id: usize) -> &'static TaskContext {
    KERNEL_STACK[app_id].push_context(
        TrapContext::app_init_context(get_base_i(app_id), USER_STACK[app_id].get_sp()),
        TaskContext::goto_restore(),
    )
}
