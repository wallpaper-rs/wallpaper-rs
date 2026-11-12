use super::{Command, Compositor};

impl Compositor {
    pub fn waker(&self) -> impl Fn() + Send + Sync + 'static {
        let tx = self.tx.clone();
        move || drop(tx.send(Command::Wake))
    }

    pub fn framer(&self) -> impl Fn(&str) + Send + Sync + 'static {
        self.relay(Command::Frame)
    }

    pub fn repainter(&self) -> impl Fn(&str) + Send + Sync + 'static {
        self.relay(Command::Repaint)
    }

    pub fn regioner(&self) -> impl Fn(&str) + Send + Sync + 'static {
        self.relay(Command::Region)
    }

    pub fn backdropper(&self) -> impl Fn(&str) + Send + Sync + 'static {
        self.relay(Command::Backdrop)
    }

    fn relay(&self, command: fn(String) -> Command) -> impl Fn(&str) + Send + Sync + 'static {
        let tx = self.tx.clone();
        move |output| drop(tx.send(command(output.into())))
    }
}
