use core::arch::asm;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        //ecall 是一个特殊的汇编指令，用于触发一个系统调用
        //当这个指令执行时，操作系统内核接管控制，
        //根据寄存器中的系统调用号和参数提供相应的服务。
        asm!("ecall",
             in("x10") args[0],
             in("x11") args[1],
             in("x12") args[2],
             in("x17") id,
             lateout("x10") ret
        );
    }
    ret
}

//sys_write 函数:
//功能：用于写数据到指定的文件描述符 fd。
//参数：
//fd: 文件描述符，用于指定要写入的文件或设备。
//buffer: 要写入的数据的缓冲区。
//实现：调用 syscall 函数，传入系统调用号 SYSCALL_WRITE（一个常量，表示写操作的系统调用），以及三个参数：文件描述符 fd，数据缓冲区指针 buffer.as_ptr() as usize，和数据长度 buffer.len()。

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}
///
/// sys_exit 函数:
/// 功能：终止当前进程并返回一个退出码。
/// 参数：exit_code: 进程退出时返回的状态码。
/// 实现：调用 syscall 函数，传入系统调用号 SYSCALL_EXIT（一个常量，表示退出操作的系统调用），以及三个参数：退出码 exit_code as usize，其余两个参数为 0
/// 

pub fn sys_exit(exit_code: i32) -> isize {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
}


///
/// sys_yield 函数:
/// 功能：使当前进程放弃处理器，允许操作系统调度其他进程。
/// 参数：无。
/// 实现：调用 syscall 函数，传入系统调用号 SYSCALL_YIELD（一个常量，表示让出 CPU 的操作），所有参数都设为 0，因为 sys_yield 不需要任何参数。
/// 
pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0, 0, 0])
}
