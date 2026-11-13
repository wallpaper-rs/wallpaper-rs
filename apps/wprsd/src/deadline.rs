const LIMIT_SECONDS: libc::c_uint = 1500;

extern "C" fn expired(_: libc::c_int) {
    let note = b"the test binary ran past its time limit: a test is blocked, ending the run as failed\n";
    unsafe {
        libc::write(2, note.as_ptr().cast(), note.len());
        libc::_exit(124)
    }
}

extern "C" fn arm() {
    unsafe {
        libc::signal(libc::SIGALRM, expired as *const () as libc::sighandler_t);
        libc::alarm(LIMIT_SECONDS);
    }
}

#[used]
#[unsafe(link_section = ".init_array")]
static ARM: extern "C" fn() = arm;
