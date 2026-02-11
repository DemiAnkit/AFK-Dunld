// src-tauri/src/core/queue_manager.rs

use std::collections::VecDeque;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueInfo {
    pub max_concurrent: u32,
    pub active_count: u32,
    pub queued_count: u32,
    pub total_count: u32,
}

/// Manages download queue with concurrency control
pub struct QueueManager {
    /// Queue of waiting download IDs
    queue: VecDeque<Uuid>,
    /// Currently active download IDs
    active: Vec<Uuid>,
    /// Maximum concurrent downloads
    max_concurrent: u32,
}

impl QueueManager {
    pub fn new(max_concurrent: u32) -> Self {
        Self {
            queue: VecDeque::new(),
            active: Vec::new(),
            max_concurrent: max_concurrent.max(1),
        }
    }

    /// Add a download to the queue
    /// Returns true if the download should start immediately
    pub fn enqueue(&mut self, id: Uuid) -> bool {
        if self.active.len() < self.max_concurrent as usize {
            self.active.push(id);
            tracing::debug!(
                "Download {} started immediately ({}/{} active)",
                id,
                self.active.len(),
                self.max_concurrent
            );
            true
        } else {
            self.queue.push_back(id);
            tracing::debug!(
                "Download {} queued (position {}, {}/{} active)",
                id,
                self.queue.len(),
                self.active.len(),
                self.max_concurrent
            );
            false
        }
    }

    /// Mark a download as complete and return next queued download
    pub fn complete(&mut self, id: Uuid) -> Option<Uuid> {
        self.active.retain(|&active_id| active_id != id);
        self.dequeue_next()
    }

    /// Remove a download from queue or active list
    pub fn remove(&mut self, id: Uuid) -> Option<Uuid> {
        self.active.retain(|&active_id| active_id != id);
        self.queue.retain(|&queued_id| queued_id != id);
        self.dequeue_next()
    }

    /// Get the next download from the queue if there's capacity
    fn dequeue_next(&mut self) -> Option<Uuid> {
        if self.active.len() < self.max_concurrent as usize {
            if let Some(next_id) = self.queue.pop_front() {
                self.active.push(next_id);
                tracing::debug!(
                    "Dequeued download {} ({}/{} active, {} queued)",
                    next_id,
                    self.active.len(),
                    self.max_concurrent,
                    self.queue.len()
                );
                return Some(next_id);
            }
        }
        None
    }

    /// Set maximum concurrent downloads
    /// Returns list of downloads that should start now
    pub fn set_max_concurrent(&mut self, max: u32) -> Vec<Uuid> {
        self.max_concurrent = max.max(1);
        let mut to_start = Vec::new();

        while self.active.len() < self.max_concurrent as usize {
            if let Some(id) = self.queue.pop_front() {
                self.active.push(id);
                to_start.push(id);
            } else {
                break;
            }
        }

        tracing::info!(
            "Max concurrent set to {}, starting {} queued downloads",
            self.max_concurrent,
            to_start.len()
        );

        to_start
    }

    /// Check if a download is currently active
    pub fn is_active(&self, id: &Uuid) -> bool {
        self.active.contains(id)
    }

    /// Check if a download is in the queue
    pub fn is_queued(&self, id: &Uuid) -> bool {
        self.queue.contains(id)
    }

    /// Get queue info
    pub fn info(&self) -> QueueInfo {
        QueueInfo {
            max_concurrent: self.max_concurrent,
            active_count: self.active.len() as u32,
            queued_count: self.queue.len() as u32,
            total_count: (self.active.len() + self.queue.len()) as u32,
        }
    }

    /// Reorder queue - move download to position
    pub fn reorder(&mut self, id: Uuid, position: usize) {
        if let Some(pos) = self.queue.iter().position(|&qid| qid == id) {
            self.queue.remove(pos);
            let insert_pos = position.min(self.queue.len());
            self.queue.insert(insert_pos, id);
        }
    }

    /// Get queue contents
    pub fn get_queue(&self) -> Vec<Uuid> {
        self.queue.iter().copied().collect()
    }

    /// Get active downloads
    pub fn get_active(&self) -> Vec<Uuid> {
        self.active.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_basic() {
        let mut queue = QueueManager::new(2);

        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();
        let id3 = Uuid::new_v4();

        assert!(queue.enqueue(id1));  // starts immediately
        assert!(queue.enqueue(id2));  // starts immediately
        assert!(!queue.enqueue(id3)); // queued

        assert_eq!(queue.info().active_count, 2);
        assert_eq!(queue.info().queued_count, 1);

        // Complete id1, id3 should start
        let next = queue.complete(id1);
        assert_eq!(next, Some(id3));
        assert_eq!(queue.info().active_count, 2);
        assert_eq!(queue.info().queued_count, 0);
    }
}