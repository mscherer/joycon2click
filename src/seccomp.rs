use {
    nix::libc,
    seccompiler::{
        apply_filter, BpfProgram, SeccompAction, SeccompCmpArgLen, SeccompCmpOp, SeccompCondition,
        SeccompFilter, SeccompRule,
    },
    std::env::consts::ARCH,
};

pub struct SeccompConfiner {
    allow_setuid: bool,
    device_fd: i32,
}

impl SeccompConfiner {
    pub fn new(allow_setuid: bool, device_fd: i32) -> SeccompConfiner {
        Self {
            allow_setuid: allow_setuid,
            device_fd: device_fd,
        }
    }

    pub fn confine(&self) -> Result<(), seccompiler::Error> {
        #[cfg(debug_assertions)]
        let action = SeccompAction::Log;

        // 1 is EPERM
        #[cfg(not(debug_assertions))]
        let action = SeccompAction::Errno(1);

        let mut rules = vec![];

        // safe syscalls
        // shouldn't requires any specific arguments filtering
        for s in vec![
            libc::SYS_close,
            libc::SYS_read,
            libc::SYS_recvfrom,
            libc::SYS_getpid,
            libc::SYS_getdents64,
            libc::SYS_bind,
            libc::SYS_fstat,
            //            libc::SYS_write,
        ]
        .into_iter()
        {
            rules.push((s, vec![]));
        }

        // all SeccompCondition are combined with and
        // all SeccompRule are combined with or
        let mut scr = vec![];
        // we can't easily check the arguments of write due to the way it work
        for f in vec![libc::STDOUT_FILENO, libc::STDERR_FILENO, self.device_fd].into_iter() {
            scr.push(
                SeccompRule::new(vec![SeccompCondition::new(
                    0,
                    SeccompCmpArgLen::Dword,
                    SeccompCmpOp::Eq,
                    f as u64,
                )?])?,
            );
        }
        rules.push((libc::SYS_write, scr));

        // once setuid is done, no risk of setuid again
        if self.allow_setuid {
            rules.push((libc::SYS_setuid, vec![]));
        }

        // these need more works
        let mut complex_rules = vec![
            // only allow opening a socket to uevent
            (
                libc::SYS_socket,
                vec![SeccompRule::new(vec![
                    SeccompCondition::new(
                        0,
                        SeccompCmpArgLen::Dword,
                        SeccompCmpOp::Eq,
                        libc::AF_NETLINK as u64,
                    )?,
                    SeccompCondition::new(
                        2,
                        SeccompCmpArgLen::Dword,
                        SeccompCmpOp::Eq,
                        libc::NETLINK_KOBJECT_UEVENT as u64,
                    )?,
                ])?],
            ),
            (
                libc::SYS_fcntl,
                vec![SeccompRule::new(vec![SeccompCondition::new(
                    1,
                    SeccompCmpArgLen::Dword,
                    SeccompCmpOp::Eq,
                    libc::F_GETFD as u64,
                )?])?],
            ),
            // TODO check args
            // EVIOCGBIT EVIOCGNAME EVIOCGPHYS EVIOCGUNIQ EVIOCGID, EVIOCGVERSION, EVIOCGPROP
            (libc::SYS_ioctl, vec![]),
            // TODO should be only in /dev/input ?
            (libc::SYS_openat, vec![]),
        ];

        rules.append(&mut complex_rules);

        let filter = SeccompFilter::new(
            rules.into_iter().collect(),
            action,
            SeccompAction::Allow,
            ARCH.try_into()?,
        )?;

        let filter: BpfProgram = filter.try_into()?;
        apply_filter(&filter)
    }
}
