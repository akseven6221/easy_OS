

- os
  - os/src/sync
  - os/src/syscall
  - os/src/trap/
  - os/src/main.rs
  - os/src/linker.ld
  - os/src/batch.rs

- user
  - `user/src/bin/*.rs`   apps
  - `user/src/*rs`        user libs(include enter function, init function, I/O function and syscall interface)
  - `user/src/linker.ld`  apps's memory layout


## 关键代码部分
- stvec: 发生exception/syscall/trap/interrupt时跳转到stvec寄存器处. 
- alltraps: 
  1. 交换 sscratch 和 sp , 使得当前sp指向内核栈
  2. 将x1 x3 x5-x31保存到内核栈中
  3. 将sstatus, sepc读到x5, x6中, 然后将x5, x6保存到内核栈中; 再把sscratch读到x7中再将x7保存到内核栈中(x1和x3中间)
  4. 把sp传给参数a0(x10), 调用trap_handler
- restore:
  1. 把 a0 的值传给 sp,
  2. 从内核栈里拿到(32*8(sp))sstatus, (33*8(sp))sepc, (2*8(sp))sscratch装载到t0, t1, t2中, 然后给这三个csr复原
  3. 给之前的普通寄存器复原
  4. 删栈
  5. 再交换sp, sscratch, 使得sp现在是指向user_stack, sscratch指向kernel_stack

- ch2中关于用户程序从哪里跑起来呢?
  - 从restore函数进入用户程序
  1. 构建一个TrapContext, sepc是apps起始处, sp 是UserStack栈顶, sstatus的SPP字段是USER
  2. 将这个TrapContext压入KernelStack, (此时还在Kernel态)
  3. 将KernelStack中压入TrapContext的栈顶指针传入restore函数, sp在trapcontext的上面(kernel_stack)
  4. 第一个app执行完之后返回0触发系统调用进入alltraps