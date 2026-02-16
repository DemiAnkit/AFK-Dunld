// Performance optimization utilities
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Connection pool for FTP connections
pub struct FtpConnectionPool {
    connections: Arc<RwLock<HashMap<String, PooledConnection>>>,
    max_idle_time: Duration,
    max_connections_per_host: usize,
}

struct PooledConnection {
    last_used: Instant,
    // In a real implementation, this would hold the actual FTP connection
    // For now, it's a placeholder
    connection_info: String,
}

impl FtpConnectionPool {
    pub fn new(max_idle_time: Duration, max_connections_per_host: usize) -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            max_idle_time,
            max_connections_per_host,
        }
    }

    /// Get or create a connection for a host
    pub async fn get_connection(&self, host: &str) -> Option<String> {
        let mut connections = self.connections.write().await;
        
        // Remove expired connections
        connections.retain(|_, conn| {
            conn.last_used.elapsed() < self.max_idle_time
        });

        // Check if we have a connection for this host
        if let Some(conn) = connections.get_mut(host) {
            conn.last_used = Instant::now();
            return Some(conn.connection_info.clone());
        }

        // Create new connection if under limit
        let host_connections = connections
            .iter()
            .filter(|(k, _)| k.starts_with(host))
            .count();

        if host_connections < self.max_connections_per_host {
            let conn = PooledConnection {
                last_used: Instant::now(),
                connection_info: format!("Connection to {}", host),
            };
            connections.insert(host.to_string(), conn);
            Some(format!("Connection to {}", host))
        } else {
            None
        }
    }

    /// Return a connection to the pool
    pub async fn return_connection(&self, host: &str) {
        let mut connections = self.connections.write().await;
        if let Some(conn) = connections.get_mut(host) {
            conn.last_used = Instant::now();
        }
    }

    /// Clear all connections
    pub async fn clear(&self) {
        self.connections.write().await.clear();
    }

    /// Get pool statistics
    pub async fn stats(&self) -> PoolStats {
        let connections = self.connections.read().await;
        PoolStats {
            total_connections: connections.len(),
            active_hosts: connections.keys().cloned().collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PoolStats {
    pub total_connections: usize,
    pub active_hosts: Vec<String>,
}

/// Memory-efficient buffer pool
pub struct BufferPool {
    buffers: Arc<RwLock<Vec<Vec<u8>>>>,
    buffer_size: usize,
    max_buffers: usize,
}

impl BufferPool {
    pub fn new(buffer_size: usize, max_buffers: usize) -> Self {
        Self {
            buffers: Arc::new(RwLock::new(Vec::new())),
            buffer_size,
            max_buffers,
        }
    }

    /// Get a buffer from the pool or create a new one
    pub async fn acquire(&self) -> Vec<u8> {
        let mut buffers = self.buffers.write().await;
        buffers.pop().unwrap_or_else(|| vec![0u8; self.buffer_size])
    }

    /// Return a buffer to the pool
    pub async fn release(&self, mut buffer: Vec<u8>) {
        let mut buffers = self.buffers.write().await;
        
        if buffers.len() < self.max_buffers {
            // Clear the buffer before returning to pool
            buffer.clear();
            buffer.resize(self.buffer_size, 0);
            buffers.push(buffer);
        }
        // Otherwise, let it be dropped
    }

    /// Get pool statistics
    pub async fn stats(&self) -> BufferPoolStats {
        let buffers = self.buffers.read().await;
        BufferPoolStats {
            available_buffers: buffers.len(),
            max_buffers: self.max_buffers,
            buffer_size: self.buffer_size,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BufferPoolStats {
    pub available_buffers: usize,
    pub max_buffers: usize,
    pub buffer_size: usize,
}

/// Cache for frequently accessed data
pub struct DataCache<T: Clone> {
    cache: Arc<RwLock<HashMap<String, CachedItem<T>>>>,
    max_age: Duration,
    max_items: usize,
}

struct CachedItem<T> {
    data: T,
    cached_at: Instant,
}

impl<T: Clone> DataCache<T> {
    pub fn new(max_age: Duration, max_items: usize) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            max_age,
            max_items,
        }
    }

    /// Get an item from cache
    pub async fn get(&self, key: &str) -> Option<T> {
        let mut cache = self.cache.write().await;
        
        // Remove expired items
        cache.retain(|_, item| {
            item.cached_at.elapsed() < self.max_age
        });

        cache.get(key).map(|item| item.data.clone())
    }

    /// Put an item in cache
    pub async fn put(&self, key: String, data: T) {
        let mut cache = self.cache.write().await;

        // Check if we need to evict items
        if cache.len() >= self.max_items {
            // Simple FIFO eviction - remove oldest
            if let Some(oldest_key) = cache.keys().next().cloned() {
                cache.remove(&oldest_key);
            }
        }

        cache.insert(key, CachedItem {
            data,
            cached_at: Instant::now(),
        });
    }

    /// Clear cache
    pub async fn clear(&self) {
        self.cache.write().await.clear();
    }

    /// Get cache statistics
    pub async fn stats(&self) -> CacheStats {
        let cache = self.cache.read().await;
        CacheStats {
            items: cache.len(),
            max_items: self.max_items,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub items: usize,
    pub max_items: usize,
}

/// Optimized scheduler with 1-second precision
pub mod optimized_scheduler {
    use std::time::Duration;
    
    /// Scheduler configuration
    pub struct SchedulerConfig {
        pub check_interval: Duration,
        pub batch_size: usize,
    }

    impl Default for SchedulerConfig {
        fn default() -> Self {
            Self {
                check_interval: Duration::from_secs(1), // Optimized from 10s to 1s
                batch_size: 10, // Process up to 10 tasks per interval
            }
        }
    }

    impl SchedulerConfig {
        /// Create high-performance config
        pub fn high_performance() -> Self {
            Self {
                check_interval: Duration::from_millis(500), // 0.5s for near-realtime
                batch_size: 20,
            }
        }

        /// Create balanced config
        pub fn balanced() -> Self {
            Self::default()
        }

        /// Create low-resource config
        pub fn low_resource() -> Self {
            Self {
                check_interval: Duration::from_secs(5),
                batch_size: 5,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connection_pool() {
        let pool = FtpConnectionPool::new(Duration::from_secs(60), 3);
        
        let conn1 = pool.get_connection("ftp.example.com").await;
        assert!(conn1.is_some());

        let stats = pool.stats().await;
        assert_eq!(stats.total_connections, 1);
    }

    #[tokio::test]
    async fn test_buffer_pool() {
        let pool = BufferPool::new(1024, 5);
        
        let buffer = pool.acquire().await;
        assert_eq!(buffer.len(), 1024);

        pool.release(buffer).await;

        let stats = pool.stats().await;
        assert_eq!(stats.available_buffers, 1);
    }

    #[tokio::test]
    async fn test_data_cache() {
        let cache: DataCache<String> = DataCache::new(Duration::from_secs(10), 5);
        
        cache.put("key1".to_string(), "value1".to_string()).await;
        
        let value = cache.get("key1").await;
        assert_eq!(value, Some("value1".to_string()));

        let stats = cache.stats().await;
        assert_eq!(stats.items, 1);
    }

    #[tokio::test]
    async fn test_cache_expiration() {
        let cache: DataCache<String> = DataCache::new(Duration::from_millis(100), 5);
        
        cache.put("key1".to_string(), "value1".to_string()).await;
        
        tokio::time::sleep(Duration::from_millis(150)).await;
        
        let value = cache.get("key1").await;
        assert_eq!(value, None); // Should be expired
    }

    #[tokio::test]
    async fn test_cache_eviction() {
        let cache: DataCache<String> = DataCache::new(Duration::from_secs(10), 3);
        
        cache.put("key1".to_string(), "value1".to_string()).await;
        cache.put("key2".to_string(), "value2".to_string()).await;
        cache.put("key3".to_string(), "value3".to_string()).await;
        cache.put("key4".to_string(), "value4".to_string()).await; // Should evict oldest
        
        let stats = cache.stats().await;
        assert_eq!(stats.items, 3); // Max is 3
    }
}
