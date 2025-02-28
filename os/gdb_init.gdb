# 首先删除已有的符号表
symbol-file

# 先加载用户程序的符号表 (这个顺序很重要)
add-symbol-file ../user/target/riscv64gc-unknown-none-elf/release/00power_3 0x80400000

# 然后加载内核的符号表
file ./target/riscv64gc-unknown-none-elf/release/os

# 设置源码搜索路径
dir ../user/src/bin
dir ../user/src
dir ../

# 配置显示设置
set print pretty on
set disassemble-next-line off

# 连接到调试器
target remote localhost:1234