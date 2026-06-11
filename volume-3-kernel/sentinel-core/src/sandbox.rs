use std::process::{Command as StdCommand, Stdio};
use std::fs::File;
use std::io;
use std::os::unix::process::CommandExt;

use tokio::process::{Command as AsyncCommand, Child};

pub struct ProcessSupervisor {
    pub target_binary: String,
    pub arguments: Vec<String>,
}

impl ProcessSupervisor {
    pub fn new(target_binary: &str, arguments: Vec<String>) -> Self {
        Self { target_binary: target_binary.to_string(), arguments }
    }

    pub fn execute_isolated(&self, audit_log_path: &str) -> Result<Child, io::Error> {
        let log_file = File::create(audit_log_path)?;
        let error_log_file = log_file.try_clone()?;

        let mut std_blueprint = StdCommand::new(&self.target_binary);
        std_blueprint
            .args(&self.arguments)
            .stdin(Stdio::null())
            .stdout(Stdio::from(log_file))
            .stderr(Stdio::from(error_log_file));

        unsafe {
            std_blueprint.pre_exec(|| {
                let isolation_flags = libc::CLONE_NEWUTS | libc::CLONE_NEWNET | libc::CLONE_NEWPID | libc::CLONE_NEWNS;
                if libc::unshare(isolation_flags) != 0 {
                    return Err(io::Error::last_os_error());
                }

                let new_hostname = "sentinel-jail\0";
                if libc::sethostname(new_hostname.as_ptr() as *const libc::c_char, new_hostname.len() - 1) != 0 {
                    return Err(io::Error::last_os_error());
                }

                if libc::mount(std::ptr::null(), "/\0".as_ptr() as _, std::ptr::null(), libc::MS_PRIVATE | libc::MS_REC, std::ptr::null()) != 0 {
                    return Err(io::Error::last_os_error());
                }
                if libc::mount("proc\0".as_ptr() as _, "/proc\0".as_ptr() as _, "proc\0".as_ptr() as _, 0, std::ptr::null()) != 0 {
                    return Err(io::Error::last_os_error());
                }
                if libc::mount("tmpfs\0".as_ptr() as _, "/home\0".as_ptr() as _, "tmpfs\0".as_ptr() as _, 0, std::ptr::null()) != 0 {
                    return Err(io::Error::last_os_error());
                }

                Ok(())
            });
        }

        let mut async_blueprint = AsyncCommand::from(std_blueprint);
        let child_process = async_blueprint.spawn()?;
        Ok(child_process)
    }
}