macro_rules! calls {
    ($($iface:ident $proxy:ident { $($member:ident $method:ident($($arg:ident: $sig:tt),*) $($json:ident)?;)* })*) => {
        #[cfg(test)]
        pub const CALLS: &[(&str, &str, &[&str])] =
            &[$($((stringify!($iface), stringify!($member), &[$(stringify!($arg)),*]),)*)*];

        pub async fn dispatch(daemon: &Daemon, iface: &str, member: &str, args: Vec<Value>) -> Result<String, Error> {
            let mut args = args.into_iter();
            match (iface, member) {
                $($((stringify!($iface), stringify!($member)) => {
                    $(let $arg: arg!($sig) = next(&mut args, stringify!($arg))?;)*
                    if args.len() > 0 {
                        return Err(invalid(format!("{member} takes fewer arguments")));
                    }
                    match daemon {
                        Daemon::Bus(c) => reply!($($json)? c.$proxy.$method($(pass!($sig, $arg)),*).await),
                        Daemon::Fixtures => Ok(fixture(iface, member)),
                    }
                })*)*
                _ => Err(invalid(format!("unknown method {iface}.{member}"))),
            }
        }
    };
}
