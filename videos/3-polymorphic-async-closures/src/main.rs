use futures::StreamExt;
use futures::stream::FuturesUnordered;
use std::pin::Pin;
use std::sync::Mutex;

pub type Handler = Box<dyn for<'a> Fn(&'a str, &'a u32) -> Pin<Box<dyn Future<Output = ()> + 'a>>>;

pub trait Dispatcher {
    /// Dispatches a message to all handlers
    async fn dispatch(&self, address: &str);
}

pub trait DispatcherFiltered {
    /// Filters messages to handlers based on their index
    async fn dispatch_filtered(
        &self,
        address: &str,
        filter: Box<dyn Fn(usize) -> Pin<Box<dyn Future<Output = bool>>>>,
    );
}

#[derive(Default)]
pub struct DispatcherImpl {
    number_of_dispatches: Mutex<u32>,
    handlers: Vec<Handler>,
}

impl DispatcherImpl {
    pub async fn register_handler(&mut self, handler: Handler) {
        self.handlers.push(handler);
    }
}

impl Dispatcher for DispatcherImpl {
    async fn dispatch(&self, message: &str) {
        let mut num_dispatches = self
            .number_of_dispatches
            .lock()
            .expect("dispatch count mutex was poisoned");
        *num_dispatches += 1;

        let num_dispatches = num_dispatches;

        let mut dispatch_task_set = FuturesUnordered::new();

        for cb in self.handlers.iter() {
            dispatch_task_set.push(cb(message, &*num_dispatches));
        }

        while let Some(_) = dispatch_task_set.next().await {}
    }
}

/// Exercise:
///
/// Fill in this trait implementation so that the test passes!
impl DispatcherFiltered for DispatcherImpl {
    async fn dispatch_filtered(
        &self,
        address: &str,
        filter: Box<dyn Fn(usize) -> Pin<Box<dyn Future<Output = bool>>>>,
    ) {
        todo!()
    }
}



async fn report(message: &str) {
    println!("[report]: {message}");
}

#[tokio::main]
async fn main() {
    let mut dispatcher = DispatcherImpl::default();

    for _ in 0..5 {
        dispatcher
            .register_handler(Box::new(|message, dispatch_number| {
                Box::pin(async move {
                    report(&format!("{message} for the {dispatch_number}th time")).await
                })
            }))
            .await;
    }

    println!("First dispatch:\n");
    dispatcher.dispatch("Hello, world!").await;

    println!("\nSecond dispatch:\n");
    dispatcher.dispatch("Hello, world!").await;
}

#[cfg(test)]
mod test {
    use crate::{Dispatcher, DispatcherFiltered, DispatcherImpl};
    use std::rc::Rc;
    use std::sync::Mutex;

    #[tokio::test]
    async fn test_filtered_dispatch() {
        let number_of_messages_processed = Rc::new(Mutex::new(0u32));
        let mut dispatcher = DispatcherImpl::default();

        for _ in 0..10 {
            dispatcher
                .register_handler({
                    let number_of_messages_processed = number_of_messages_processed.clone();

                    Box::new(move |_, _| {
                        let number_of_messages_processed = number_of_messages_processed.clone();

                        Box::pin(async move {
                            let mut num_messages = number_of_messages_processed
                                .lock()
                                .expect("mutex was poisoned");

                            *num_messages += 1;
                        })
                    })
                })
                .await;
        }

        async fn is_receiver(idx: usize) -> bool {
            idx % 2 == 0
        }

        dispatcher
            .dispatch_filtered(
                "",
                Box::new(|idx| Box::pin(async move { is_receiver(idx).await })),
            )
            .await;

        assert_eq!(
            5,
            number_of_messages_processed
                .lock()
                .expect("mutex was poisoned")
                .clone()
        );
    }
}
