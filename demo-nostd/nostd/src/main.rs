#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let msg = b"Hello, no_std world!\n";

    sys_wirte(1, msg);
    sys_exit(666);
}

unsafe fn syscall3(id: usize, arg0: usize, arg1: usize, arg2: usize) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!("syscall",
            inlateout("rax") id => ret,
            in("rdi") arg0, // 文件描述符
            in("rsi") arg1, // 写入缓冲区的起始地址
            in("rdx") arg2, // 写入缓冲区长度
            out("rcx") _,   // 保留寄存器
            out("r11") _,   // 保留寄存器
        );
    }
    ret
}

fn sys_wirte(fd: usize, buf: &[u8]) -> isize {
    unsafe { syscall3(1, fd, buf.as_ptr() as usize, buf.len()) }
}

fn sys_exit(exit_code: usize) -> ! {
    unsafe {
        syscall3(60, exit_code, 0, 0);
    }
    loop {}
}

pub extern "C" fn _exit() {
    unsafe {
        core::arch::asm!("syscall",
        in("rax") 60usize,  // syscall number
        in("rdi") 0usize );
    }
}
