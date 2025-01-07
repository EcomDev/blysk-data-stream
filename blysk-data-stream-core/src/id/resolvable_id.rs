use crate::id::{Id, Notifier};
use std::sync::Arc;
use tokio::sync::oneshot;

#[derive(Clone)]
pub enum ResolvableId {
    Resolved(Arc<Id>),
    Unresolved(Arc<Id>, Notifier),
}

impl ResolvableId {
    pub fn new(notifier: Notifier) -> Self {
        Self::Unresolved(Arc::new(Id::default()), notifier)
    }

    pub fn resolved(id: u64) -> Self {
        Self::Resolved(Arc::new(Id::resolved(id)))
    }

    pub fn resolve(&self, value: Option<u64>) {
        let id = match self {
            Self::Resolved(id) => id,
            Self::Unresolved(id, _) => id,
        };

        id.store(value)
    }

    pub async fn fetch(&self) -> Option<u64> {
        let id = match self {
            Self::Resolved(id) => id,
            Self::Unresolved(id, notifier) if id.is_resolvable() => {
                let (tx, rx) = oneshot::channel();
                match notifier.send(tx).await {
                    Ok(_) => match rx.await {
                        Ok(_) => id,
                        Err(_) => return None,
                    },
                    Err(_) => return None,
                }
            }
            ResolvableId::Unresolved(id, _) => id,
        };

        id.load()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;
    use tokio::task::spawn;

    struct Channel {
        tx: Notifier,
        rx: Option<mpsc::Receiver<oneshot::Sender<()>>>,
    }

    impl Channel {
        fn new() -> Self {
            let (tx, rx) = mpsc::channel(1);

            Self { tx, rx: Some(rx) }
        }

        fn watch(&mut self, callback: impl Future + Send + 'static) {
            if let Some(mut rx) = self.rx.take() {
                spawn(async move {
                    let response: oneshot::Sender<()> = rx.recv().await.unwrap();
                    callback.await;
                    response.send(()).unwrap();
                });
            }
        }

        fn notifier(&self) -> Notifier {
            self.tx.clone()
        }
    }

    #[tokio::test]
    async fn resolves_to_id_names() {
        let mut channel = Channel::new();

        let id = ResolvableId::new(channel.notifier());

        channel.watch({
            let id = id.clone();
            async move {
                id.resolve(Some(10));
            }
        });

        assert_eq!(Some(10), id.fetch().await);
    }

    #[tokio::test]
    async fn does_not_resolve_value_when_resolved_to_empty() {
        let mut channel = Channel::new();

        let id = ResolvableId::new(channel.notifier());

        channel.watch({
            let id = id.clone();
            async move {
                id.resolve(None);
            }
        });

        assert_eq!(None, id.fetch().await);
    }

    #[tokio::test]
    async fn does_not_execute_notifier_when_value_was_already_resolved() {
        let mut channel = Channel::new();

        let id_one = ResolvableId::new(channel.notifier());
        let id_two = ResolvableId::new(channel.notifier());

        id_one.resolve(Some(1));
        id_two.resolve(None);

        channel.watch({
            let id_one = id_one.clone();
            let id_two = id_two.clone();
            async move {
                id_one.resolve(None);
                id_two.resolve(Some(2));
            }
        });

        assert_eq!(Some(1), id_one.fetch().await);
        assert_eq!(None, id_two.fetch().await);
    }
}
