use crate::process;
use anyhow::Result;

#[cfg(target_os = "linux")]
use nix::libc::user_regs_struct;
#[cfg(target_os = "linux")]
use nix::sys::ptrace;
#[cfg(target_os = "linux")]
use nix::sys::wait::WaitStatus;
#[cfg(target_os = "linux")]
use nix::sys::wait::waitpid;

type SyscallNum = u64;
type SyscallArgs = (u64, u64, u64);

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn get_syscall_number(regs: user_regs_struct) -> SyscallNum {
    regs.orig_rax
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn getsyscall_args(regs: user_regs_struct) -> SyscallArgs {
    (regs.rdi, regs.rsi, regs.rdx)
}

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
fn get_syscall_number(regs: &user_regs_struct) -> SyscallNum {
    regs.regs[8]
}

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
fn getsyscall_args(regs: &user_regs_struct) -> SyscallArgs {
    (regs.regs[0], regs.regs[1], regs.regs[2])
}

#[cfg(target_os = "linux")]
pub fn trace(cmd: &str, args: &[String]) -> Result<()> {
    let child = process::spawn_tracee(cmd, args)?;

    println!("Tracing PID: {}", child);

    loop {
        ptrace::syscall(child, None)?;

        let status = waitpid(child, None)?;

        match status {
            WaitStatus::Exited(_, code) => {
                println!("Process exited with code: {}", code);
                break;
            }
            WaitStatus::Stopped(_, _) => {
                let regs = ptrace::getregs(child)?;
                let syscall_num = get_syscall_number(regs);
                let (arg0, arg1, arg2) = getsyscall_args(regs);

                println!("syscall {syscall_num} ({arg0}, {arg1}, {arg2})");

                ptrace::syscall(child, None)?;
                waitpid(child, None)?;
            }
            _ => {}
        }
    }

    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub fn trace(_cmd: &str, _args: &[String]) -> Result<()> {
    unreachable!("This function should not be called on non-Linux systems")
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;

    #[test]
    fn test_trace_simple_command() {
        let result = trace("true", &[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_trace_command_with_args() {
        let result = trace("echo", &["hello".to_string()]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_trace_command_with_multiple_args() {
        let result = trace("echo", &["hello".to_string(), "world".to_string()]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_trace_nonzero_exit_code() {
        let result = trace("false", &[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_trace_multiple_times() {
        for _ in 0..3 {
            let result = trace("true", &[]);
            assert!(result.is_ok());
        }
    }
}
