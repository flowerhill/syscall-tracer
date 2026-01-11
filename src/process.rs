use anyhow::Result;
#[cfg(target_os = "linux")]
use nix::sys::ptrace;
#[cfg(target_os = "linux")]
use nix::unistd::execv;
#[cfg(target_os = "linux")]
use nix::unistd::{ForkResult, Pid, fork};
#[cfg(target_os = "linux")]
use std::ffi::CString;

#[cfg(target_os = "linux")]
pub fn spawn_tracee(cmd: &str, args: &[String]) -> Result<Pid> {
    match unsafe { fork() }? {
        ForkResult::Parent { child } => Ok(child),
        ForkResult::Child => {
            ptrace::traceme()?;

            let cmd = CString::new(cmd)?;
            let mut exec_args = vec![cmd.clone()];
            for arg in args {
                exec_args.push(CString::new(arg.as_str())?);
            }

            execv(&cmd, &exec_args)?;

            unreachable!();
        }
    }
}

#[cfg(not(target_os = "linux"))]
pub fn spawn_tracee(_cmd: &str, _args: &[String]) -> Result<Pid> {
    unreachable!("This function should not be called on non-Linux systems");
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use core::panic;
    use std::result;

    use super::*;
    use nix::libc::SIGTRAP;
    use nix::sys::signal::Signal;
    use nix::sys::wait::{WaitStatus, waitpid};

    #[test]
    fn test_spawn_tracee_returns_valid_pid() {
        let child = spawn_tracee("true", &[]).expect("Failed to Spawn");

        assert!(child.as_raw() > 0);

        let status = waitpid(child, None).expect("Failed to wait");

        // クリーンアップ
        match status {
            // stoppedしたプロセスは一時停止なのでゾンビプロセスが残る
            WaitStatus::Stopped(_, _) => {
                // stoppedしたプロセスをcontしてwaitpidしたらちゃんとexitするらしい
                ptrace::cont(child, None).expect("Failed to continue");
                let _ = waitpid(child, None);
            }
            _ => {}
        }
    }

    #[test]
    fn test_spawn_tracee_with_args() {
        let child = spawn_tracee("echo", &["test".to_string()]).expect("Failed to Spawn");

        assert!(child.as_raw() > 0);

        let status = waitpid(child, None).expect("Failed to wait");

        match status {
            WaitStatus::Stopped(_, _) => {
                ptrace::cont(child, None).expect("Failed to continue");
                let _ = waitpid(child, None);
            }
            _ => {}
        }
    }

    #[test]
    fn test_spawn_tracee_invalid_command() {
        let result = spawn_tracee("nonexistent_command_xyz", &[]);

        assert!(result.is_ok());

        // プロセスが残ってたら消す
        if let Ok(child) = result {
            let status = waitpid(child, None);
            assert!(status.is_ok());
        }
    }

    #[test]
    fn test_spawn_tracee_stops_at_first_syscall() {
        let child = spawn_tracee("true", &[]).expect("Failed to spawn");

        let status = waitpid(child, None).expect("Faild to spawn");

        match status {
            WaitStatus::Stopped(pid, signal) => {
                assert_eq!(pid, child);
                assert!(
                    signal == Signal::SIGTRAP || signal == Signal::SIGSTOP,
                    "Expecterd to SIGTRAP or SIGSTOP, got {:?}",
                    signal
                );
            }
            WaitStatus::Exited(pid, code) => {
                assert_eq!(pid, child);
                println!("exit code: {}", code);
            }
            other => {
                panic!("Unexpected status: {:?}", other);
            }
        }

        ptrace::cont(child, None).ok();
        let _ = waitpid(child, None);
    }
}
