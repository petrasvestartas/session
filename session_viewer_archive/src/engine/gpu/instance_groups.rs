#![allow(dead_code)]
//! GPU mesh instancing — upload geometry once, render N copies in one draw call.
//!
//! # How it works
//!
//! `mesh.wgsl` already reads `instances[@builtin(instance_index)]`. When wgpu runs
//! `draw_indexed(geom, base, 1000..1100)` the hardware sets `instance_index` to
//! 1000…1099 — each copy reads its own transform from `instance_buffer`. No new
//! pipeline or shader is needed.
//!
//! # Instance buffer layout
//!
//! ```text
//! instance_buffer (96 bytes / slot):
//!   [0 .. TEMPLATE_INSTANCE_BASE)    ← regular objects (PickTable)
//!   [TEMPLATE_INSTANCE_BASE .. END)  ← template instance groups (bump allocator)
//! ```
//!
//! # Usage example
//!
//! ```rust,ignore
//! // Register template geometry once (idempotent).
//! let (verts, inds) = mesh_to_vertices(&mesh);
//! let key = TemplateKey::new("chair_v1");
//! gpu_session.register_template_mesh(&key, &verts, &inds, device, queue);
//!
//! // Add N copies with individual transforms.
//! let h = gpu_session.add_instance(&key, "chair_42", model, color, 0, device, queue);
//!
//! // Per-instance updates — writes 96 bytes to GPU, no full re-upload.
//! gpu_session.update_instance(&h, new_model, color, 0, queue);
//! gpu_session.set_instance_flag(&h, InstanceData::FLAG_SELECTED, true, queue);
//! gpu_session.remove_instance(&h, queue);  // O(1), no compaction
//! ```

use std::collections::HashMap;
use crate::gpu_session::InstanceData;

/// First slot index in `instance_buffer` exclusively for template instance groups.
/// Slots `0..TEMPLATE_INSTANCE_BASE` are owned by the normal PickTable.
/// 16 384 is safely above any realistic single-scene regular object count.
pub const TEMPLATE_INSTANCE_BASE: u32 = 16_384;

// ── TemplateKey ───────────────────────────────────────────────────────────────

/// Identifies a reusable template mesh. Use any stable string ("bolt_m6", "chair_v1").
/// Equality and hashing are on the inner string.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TemplateKey(pub String);

impl TemplateKey {
    pub fn new(s: impl Into<String>) -> Self { Self(s.into()) }
}

// ── InstanceHandle ────────────────────────────────────────────────────────────

/// Returned by `add_instance`. Store this; pass it back to `update_instance`,
/// `set_instance_flag`, or `remove_instance`.
///
/// `slot_index` is the index within the group's contiguous block — NOT the absolute
/// `instance_buffer` index. The absolute index = `TEMPLATE_INSTANCE_BASE + block_start + slot_index`.
#[derive(Clone, Debug)]
pub struct InstanceHandle {
    pub group_key:  TemplateKey,
    pub slot_index: u32,
    pub guid:       String,
}

// ── InstanceGroup ─────────────────────────────────────────────────────────────

/// One contiguous block of `InstanceData` entries in `instance_buffer`, all sharing
/// the same template mesh geometry.
///
/// Removed slots stay in the buffer with `FLAG_HIDDEN` set — the shader discards
/// them with no geometry cost. Slots are recycled via a LIFO free list. No GPU
/// buffer compaction is ever needed.
pub struct InstanceGroup {
    pub template_key: TemplateKey,
    /// Offset from `TEMPLATE_INSTANCE_BASE`. Absolute buffer index of local slot i:
    ///   `TEMPLATE_INSTANCE_BASE + block_start + i`
    pub block_start:   u32,
    /// Number of `InstanceData` slots allocated in `instance_buffer`.
    pub capacity:      u32,
    /// Number of live (non-removed) instances.
    pub live_count:    u32,
    /// CPU mirror of every slot in this group's block. `len() == capacity` always.
    pub instances_cpu: Vec<InstanceData>,
    /// LIFO free list of slot indices within `[0..capacity)`.
    pub free_slots:    Vec<u32>,
    /// Maps instance GUID → slot index within this group.
    pub guid_to_slot:  HashMap<String, u32>,
    /// True when `instances_cpu` was modified and the GPU buffer needs re-upload.
    pub dirty:         bool,
}

impl InstanceGroup {
    pub fn new(key: TemplateKey, block_start: u32, capacity: u32) -> Self {
        let ph = hidden_placeholder();
        Self {
            template_key: key,
            block_start,
            capacity,
            live_count: 0,
            instances_cpu: vec![ph; capacity as usize],
            free_slots: (0..capacity).rev().collect(),
            guid_to_slot: HashMap::new(),
            dirty: true,
        }
    }

    /// Absolute `instance_buffer` index for local slot `i`.
    #[inline]
    pub fn abs_index(&self, slot: u32) -> u32 {
        TEMPLATE_INSTANCE_BASE + self.block_start + slot
    }

    /// `first_instance` argument for `draw_indexed`. All slots of this group
    /// start here; hidden slots discard in the shader without geometry cost.
    #[inline]
    pub fn first_instance(&self) -> u32 {
        TEMPLATE_INSTANCE_BASE + self.block_start
    }
}

fn hidden_placeholder() -> InstanceData {
    let mut d = InstanceData::new(0);
    d.flags = InstanceData::FLAG_HIDDEN;
    d
}

// ── InstanceGroupAllocator ────────────────────────────────────────────────────

/// Manages the template region `[TEMPLATE_INSTANCE_BASE..)` of `instance_buffer`
/// via a bump pointer. Assigns non-overlapping contiguous blocks per `TemplateKey`.
///
/// Blocks grow in-place when they are at the bump frontier. When a group that is
/// NOT at the frontier runs out of capacity it is relocated to the frontier —
/// the old block becomes a permanently-hidden dead zone (no draw call references it).
/// This relocation is rare in practice since groups are typically created once.
pub struct InstanceGroupAllocator {
    pub groups:   HashMap<TemplateKey, InstanceGroup>,
    /// Next free slot index within the template region, relative to `TEMPLATE_INSTANCE_BASE`.
    pub bump:     u32,
    /// Available slots in the template region. Matches `instance_capacity - TEMPLATE_INSTANCE_BASE`.
    pub capacity: u32,
}

impl InstanceGroupAllocator {
    pub fn new() -> Self {
        Self { groups: HashMap::new(), bump: 0, capacity: 128 }
    }

    /// Get-or-create a group with `cap` initial slots.
    /// Returns `(is_new, needs_instance_buffer_grow)`.
    pub fn ensure_group(&mut self, key: TemplateKey, cap: u32) -> (bool, bool) {
        if self.groups.contains_key(&key) { return (false, false); }
        let needs_grow = self.bump + cap > self.capacity;
        self.groups.insert(key.clone(), InstanceGroup::new(key, self.bump, cap));
        self.bump += cap;
        (true, needs_grow)
    }

    /// Allocate one slot in the group, growing its block if full.
    /// Returns `(slot_index, needs_instance_buffer_grow, new_template_slots_used)`.
    pub fn alloc_in_group(&mut self, key: &TemplateKey) -> (u32, bool, u32) {
        let group = self.groups.get_mut(key).expect("call ensure_group before alloc_in_group");
        if let Some(slot) = group.free_slots.pop() {
            return (slot, false, self.bump);
        }
        // Group is full — grow it.
        let old_cap = group.capacity;
        let growth  = old_cap.max(8);
        let at_frontier = group.block_start + old_cap == self.bump;
        if at_frontier {
            // Extend in-place: bump forward, no data movement needed.
            let ph = hidden_placeholder();
            group.instances_cpu.extend(std::iter::repeat(ph).take(growth as usize));
            group.free_slots.extend((old_cap..old_cap + growth).rev());
            group.capacity += growth;
            self.bump += growth;
        } else {
            // Another group was allocated after this one.
            // Relocate this group to the bump frontier; the old block becomes a dead zone.
            let new_cap = old_cap + growth;
            let ph = hidden_placeholder();
            group.instances_cpu.extend(std::iter::repeat(ph).take(growth as usize));
            group.free_slots.extend((old_cap..new_cap).rev());
            group.block_start = self.bump;
            group.capacity = new_cap;
            self.bump += new_cap;
            group.dirty = true; // re-upload entire CPU mirror at new location
        }
        let needs_grow = self.bump > self.capacity;
        let slot = group.free_slots.pop().unwrap();
        (slot, needs_grow, self.bump)
    }
}
