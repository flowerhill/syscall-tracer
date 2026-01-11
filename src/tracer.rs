use crate::process;
use crate::syscalls;
use anyhow::Result;

#[cfg(target_os = "linux")]
use nix::libc::user_regs_struct;
#[cfg(target_os = "linux")]
use nix::sys::ptrace;
#[cfg(target_os = "linux")]
use nix::sys::wait::WaitStatus;
#[cfg(target_os = "linux")]
use nix::sys::wait::waitpid;

#[cfg(target_os = "linux")]
pub fn trace(cmd: &str, args: &[String]) -> Result<()> {
    let child = process::spawn_tracee(cmd, args)?;

    println!("Tracing PID: {}", child);

    waitpid(child, None)?;

    loop {
        ptrace::syscall(child, None)?;

        let status = waitpid(child, None)?;

        match status {
            WaitStatus::Exited(_, code) => {
                println!("Process exited with code: {}", code);
                break;
            }
            WaitStatus::Stopped(_, _) => {
                let user_regs_struct {
                    rax, rdi, rsi, rdx, ..
                } = ptrace::getregs(child)?;

                if rax == syscalls::SYS_WRITE {
                    println!("{}({}, {}, {})", syscalls::syscall_name(rax), rdi, rsi, rdx);
                }

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
    use std::result;

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
