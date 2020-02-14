use crate::network::connection;
use crate::network::order;
use std::hash::Hash;

/// Topologically sorted list of actions to perform when evalutating network
#[derive(Clone)]
pub struct BasicOrder<T> {
    order: order::Order<T>,
}

#[allow(dead_code)]
impl<T: Hash + Eq + Copy> BasicOrder<T> {
    pub fn new() -> BasicOrder<T> {
        BasicOrder {
            order: order::Order::new(),
        }
    }

    pub fn sort_topologically(&mut self, connections: &connection::Connections<T>) {
        self.order.hiddens = connections.enabled_sources().cloned().collect();
        self.order.sort_topologically(connections);
    }

    pub fn iter(&self) -> impl Iterator<Item = &order::Action<T>> {
        self.order.iter()
    }
}
