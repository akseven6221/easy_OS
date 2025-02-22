

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