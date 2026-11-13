use super::{Changes, Signal};

impl Changes {
    pub fn notifier(&self) -> impl Fn(Vec<String>, bool) + Send + 'static {
        let tx = self.tx.clone();
        move |outputs, problems| {
            if problems {
                drop(tx.send(Signal::Changed("library", Vec::new())));
            }
            drop(tx.send(Signal::Changed("state", outputs)))
        }
    }
}
