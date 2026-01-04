use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;

pub const SAMPLER_BUFFER_BASE: u32 = 100;
pub const SAMPLER_BUFFER_MAX: u32 = 227;

#[derive(Debug)]
pub enum BufferError {
    NoFreeSlots,
    MemoryExceeded { requested: usize, available: usize },
    BufferNotFound(u32),
}

impl std::fmt::Display for BufferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BufferError::NoFreeSlots => write!(f, "No free buffer slots available"),
            BufferError::MemoryExceeded { requested, available } => {
                write!(f, "Memory exceeded: requested {} bytes, available {} bytes", requested, available)
            }
            BufferError::BufferNotFound(id) => write!(f, "Buffer {} not found", id),
        }
    }
}

impl std::error::Error for BufferError {}

#[derive(Debug, Clone)]
pub struct BufferAllocation {
    pub buffer_id: u32,
    pub file_path: PathBuf,
    pub frames: usize,
    pub channels: u16,
    pub sample_rate: u32,
    pub bytes: usize,
}

impl BufferAllocation {
    pub fn new(
        buffer_id: u32,
        file_path: PathBuf,
        frames: usize,
        channels: u16,
        sample_rate: u32,
    ) -> Self {
        let bytes = frames * channels as usize * 4;
        Self {
            buffer_id,
            file_path,
            frames,
            channels,
            sample_rate,
            bytes,
        }
    }
}

#[derive(Debug)]
pub struct BufferManager {
    allocations: HashMap<u32, BufferAllocation>,
    next_free_id: u32,
    total_bytes: usize,
    memory_limit_bytes: usize,
}

impl BufferManager {
    pub fn new(memory_limit_mb: u32) -> Self {
        Self {
            allocations: HashMap::new(),
            next_free_id: SAMPLER_BUFFER_BASE,
            total_bytes: 0,
            memory_limit_bytes: memory_limit_mb as usize * 1024 * 1024,
        }
    }

    pub fn allocate_id(&mut self) -> Result<u32, BufferError> {
        for offset in 0..=(SAMPLER_BUFFER_MAX - SAMPLER_BUFFER_BASE) {
            let candidate = SAMPLER_BUFFER_BASE + offset;
            if !self.allocations.contains_key(&candidate) {
                self.next_free_id = candidate;
                return Ok(candidate);
            }
        }
        Err(BufferError::NoFreeSlots)
    }

    pub fn register(&mut self, id: u32, allocation: BufferAllocation) -> Result<(), BufferError> {
        let new_total = self.total_bytes + allocation.bytes;
        if new_total > self.memory_limit_bytes {
            return Err(BufferError::MemoryExceeded {
                requested: allocation.bytes,
                available: self.memory_limit_bytes.saturating_sub(self.total_bytes),
            });
        }
        self.total_bytes = new_total;
        self.allocations.insert(id, allocation);
        Ok(())
    }

    pub fn release(&mut self, id: u32) -> Option<BufferAllocation> {
        if let Some(allocation) = self.allocations.remove(&id) {
            self.total_bytes = self.total_bytes.saturating_sub(allocation.bytes);
            Some(allocation)
        } else {
            None
        }
    }

    pub fn memory_usage_percent(&self) -> f32 {
        if self.memory_limit_bytes == 0 {
            0.0
        } else {
            (self.total_bytes as f32 / self.memory_limit_bytes as f32) * 100.0
        }
    }

    pub fn memory_status_string(&self) -> String {
        let used_mb = self.total_bytes as f32 / (1024.0 * 1024.0);
        let limit_mb = self.memory_limit_bytes as f32 / (1024.0 * 1024.0);
        let percent = self.memory_usage_percent();
        format!("Buffer: {:.1}MB / {:.0}MB ({:.0}%)", used_mb, limit_mb, percent)
    }

    pub fn clear_all(&mut self) {
        self.allocations.clear();
        self.total_bytes = 0;
        self.next_free_id = SAMPLER_BUFFER_BASE;
    }
}

impl Default for BufferManager {
    fn default() -> Self {
        Self::new(64)
    }
}
