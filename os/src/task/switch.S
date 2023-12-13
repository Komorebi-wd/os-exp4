.altmacro
.macro SAVE_SN n
    sd s\n, (\n+1)*8(sp)
.endm
.macro LOAD_SN n
    ld s\n, (\n+1)*8(sp)
.endm
    .section .text
    .globl __switch
//__switch 函数接受两个参数：current_task_cx_ptr2 和 next_task_cx_ptr2，分别指向当前任务和下一个任务的 TaskContext 的指针。
__switch:
    # __switch(
    #     current_task_cx_ptr2: &*const TaskContext,
    #     next_task_cx_ptr2: &*const TaskContext
    # )
    # push TaskContext to current sp and save its address to where a0 points to
//保存当前任务上下文
//addi sp, sp, -13*8：调整栈指针，为保存任务上下文腾出空间。
//sd sp, 0(a0)：将当前栈指针地址保存到 current_task_cx_ptr2 指向的位置。
//接下来的部分用于保存 ra 寄存器和 s0-s11 寄存器的状态到栈上。  
    addi sp, sp, -13*8
    sd sp, 0(a0)
    # fill TaskContext with ra & s0-s11
    sd ra, 0(sp)
    .set n, 0
    .rept 12
        SAVE_SN %n
        .set n, n + 1
    .endr
    # ready for loading TaskContext a1 points to
 //加载新任务上下文
//ld sp, 0(a1)：加载新任务的栈指针地址到 sp.作用是从 a1 指向的地址（即 next_task_cx_ptr2，一个指向 TaskContext 的指针）中加载新任务的栈指针地址到 sp（栈指针寄存器）。
//紧接着的代码从栈上恢复 ra 和 s0-s11 寄存器的状态
    ld sp, 0(a1)
    # load registers in the TaskContext
    ld ra, 0(sp)
    .set n, 0
    .rept 12
        LOAD_SN %n
        .set n, n + 1
    .endr
    # pop TaskContext
//addi sp, sp, 13*8：重置栈指针到原来的位置。
//ret：返回，这将跳转到新任务的上下文中继续执行。    
    addi sp, sp, 13*8
    ret


//sp 寄存器，栈指针寄存器（Stack Pointer Register）

//addi sp, sp, -13*8:减小栈顶指针
//这条指令将栈顶指针向下移动，腾出足够的空间（13个8字节的槽位）来保存当前任务的上下文（包括 ra 和 s0-s11 寄存器）。栈是向下增长的，因此需要减小栈顶指针。
//保存当前任务状态到栈上:
//接下来的指令将当前任务的寄存器状态保存到刚刚腾出的空间中。


//ld sp, 0(a1)
//加载新任务的栈顶指针:
//这条指令将新任务的栈顶指针地址（保存在 a1 中）加载到 sp 寄存器中。这个地址指向新任务的上下文所在的位置。
//恢复新任务状态:
//接下来的指令从栈上加载新任务的 ra 和 s0-s11 寄存器的状态，这些状态之前被保存在栈上。


//恢复栈顶指针
//增加栈顶指针:addi sp, sp, 13*8
//在新任务的寄存器状态被加载之后，这条指令将栈顶指针恢复到原来的位置，即在保存当前任务状态之前的位置