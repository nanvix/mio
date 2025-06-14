cfg_os_poll! {


    mod selector;
    pub(crate) use self::selector::*;


    mod waker;
    // NOTE: the `Waker` type is expected in the selector module as the
    // `poll(2)` implementation needs to do some special stuff.

    mod sourcefd;
    #[cfg(feature = "os-ext")]
    pub use self::sourcefd::SourceFd;

    cfg_net! {
        // mod net;

        pub(crate) mod tcp;
        pub(crate) mod udp;
    }

    pub(crate) mod pipe;
}

cfg_not_os_poll! {
    cfg_any_os_ext! {
        mod sourcefd;
        #[cfg(feature = "os-ext")]
        pub use self::sourcefd::SourceFd;
    }
}
