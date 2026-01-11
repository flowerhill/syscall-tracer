pub const SYS_READ: u64 = 0;
pub const SYS_WRITE: u64 = 1;
pub const SYS_OPEN: u64 = 2;
pub const SYS_CLOSE: u64 = 3;

pub fn syscall_name(num: u64) -> &'static str {
    match num {
        0 => "read",
        1 => "write",
        2 => "open",
        3 => "close",
        _ => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syscall_name_known() {
        assert_eq!(syscall_name(SYS_READ), "read");
        assert_eq!(syscall_name(SYS_WRITE), "write");
        assert_eq!(syscall_name(SYS_OPEN), "open");
        assert_eq!(syscall_name(SYS_CLOSE), "close");
    }

    #[test]
    fn test_syscall_name_unknown() {
        assert_eq!(syscall_name(999), "unknown");
        assert_eq!(syscall_name(1000), "unknown");
    }
}
