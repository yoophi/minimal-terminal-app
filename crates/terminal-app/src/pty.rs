use std::env;
use std::ffi::CString;
use std::fs::File;
use std::io::{self, Read, Write};
use std::mem::MaybeUninit;
use std::os::fd::{AsRawFd, FromRawFd, RawFd};
use std::ptr;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::logging;
use crate::terminal_buffer::TerminalBuffer;

const DEFAULT_PTY_ROWS: libc::c_ushort = 32;
const DEFAULT_PTY_COLS: libc::c_ushort = 100;
const PTY_PROGRESS_LOG_READ_INTERVAL: u64 = 4096;

#[derive(Clone, Debug)]
pub struct PtyWriter {
    writer: Arc<Mutex<File>>,
}

impl PtyWriter {
    pub fn write_all(&self, bytes: &[u8]) -> io::Result<()> {
        let mut writer = self
            .writer
            .lock()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "PTY writer lock poisoned"))?;
        writer.write_all(bytes)
    }

    pub fn resize(&self, rows: usize, cols: usize) -> io::Result<()> {
        let writer = self
            .writer
            .lock()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "PTY writer lock poisoned"))?;
        let winsize = libc::winsize {
            ws_row: rows.min(libc::c_ushort::MAX as usize) as libc::c_ushort,
            ws_col: cols.min(libc::c_ushort::MAX as usize) as libc::c_ushort,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        let result = unsafe { libc::ioctl(writer.as_raw_fd(), libc::TIOCSWINSZ, &winsize) };
        if result < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

pub fn spawn_login_shell(buffer: Arc<Mutex<TerminalBuffer>>) -> io::Result<PtyWriter> {
    logging::pty_info("pty spawn requested");

    let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/zsh".to_string());
    let shell_name = shell.rsplit('/').next().unwrap_or("zsh");
    let login_argv0 = format!("-{shell_name}");

    let mut master = MaybeUninit::<libc::c_int>::uninit();
    let mut winsize = libc::winsize {
        ws_row: DEFAULT_PTY_ROWS,
        ws_col: DEFAULT_PTY_COLS,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let pid = unsafe {
        libc::forkpty(
            master.as_mut_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
            &mut winsize,
        )
    };

    if pid < 0 {
        let error = io::Error::last_os_error();
        logging::pty_error(&format!("forkpty failed: {error}"));
        return Err(error);
    }

    if pid == 0 {
        exec_login_shell(&shell, &login_argv0);
    }

    let master_fd = unsafe { master.assume_init() };
    logging::pty_info(&format!(
        "pty spawn succeeded: child_pid={pid} shell={shell} rows={DEFAULT_PTY_ROWS} cols={DEFAULT_PTY_COLS}"
    ));

    let reader_fd = duplicate_fd(master_fd)?;
    let reader = unsafe { File::from_raw_fd(reader_fd) };
    let writer = unsafe { File::from_raw_fd(master_fd) };

    let writer = PtyWriter {
        writer: Arc::new(Mutex::new(writer)),
    };

    start_reader_thread(pid, reader, buffer, writer.clone());

    Ok(writer)
}

fn duplicate_fd(fd: RawFd) -> io::Result<RawFd> {
    let duplicated = unsafe { libc::dup(fd) };
    if duplicated < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(duplicated)
    }
}

fn exec_login_shell(shell: &str, login_argv0: &str) -> ! {
    let shell = CString::new(shell).unwrap_or_else(|_| CString::new("/bin/zsh").unwrap());
    let argv0 = CString::new(login_argv0).unwrap_or_else(|_| CString::new("-zsh").unwrap());
    let term_key = CString::new("TERM").unwrap();
    let term_value = CString::new("xterm-256color").unwrap();

    unsafe {
        libc::setenv(term_key.as_ptr(), term_value.as_ptr(), 1);
        libc::execl(shell.as_ptr(), argv0.as_ptr(), ptr::null::<libc::c_char>());
        libc::_exit(127);
    }
}

fn start_reader_thread(
    pid: libc::pid_t,
    mut reader: File,
    buffer: Arc<Mutex<TerminalBuffer>>,
    writer: PtyWriter,
) {
    logging::pty_info("pty reader thread starting");

    thread::spawn(move || {
        let mut bytes = [0_u8; 8192];
        let mut read_events = 0_u64;
        let mut total_bytes = 0_u64;

        loop {
            match reader.read(&mut bytes) {
                Ok(0) => {
                    logging::pty_info(&format!("pty reader reached EOF: child_pid={pid}"));
                    break;
                }
                Ok(n) => {
                    read_events += 1;
                    total_bytes += n as u64;

                    if read_events == 1 {
                        logging::pty_info(&format!("pty first output received: bytes={n}"));
                    } else if read_events % PTY_PROGRESS_LOG_READ_INTERVAL == 0 {
                        logging::pty_info(&format!(
                            "pty output progress: reads={read_events} total_bytes={total_bytes}"
                        ));
                    }

                    let responses = if let Ok(mut buffer) = buffer.lock() {
                        buffer.append_bytes(&bytes[..n])
                    } else {
                        logging::pty_error("terminal buffer lock poisoned");
                        break;
                    };

                    if !responses.is_empty() {
                        if let Err(error) = writer.write_all(&responses) {
                            logging::pty_error(&format!("pty DSR response write failed: {error}"));
                        }
                    }
                }
                Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
                Err(error) => {
                    logging::pty_error(&format!("pty read failed: {error}"));
                    break;
                }
            }
        }

        let mut status = 0;
        let waited = unsafe { libc::waitpid(pid, &mut status, libc::WNOHANG) };
        logging::pty_info(&format!(
            "pty reader thread exiting: waitpid={waited} status={status}"
        ));
    });
}
