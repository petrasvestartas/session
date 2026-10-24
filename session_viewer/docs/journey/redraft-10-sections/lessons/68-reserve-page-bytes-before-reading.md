# 12.4 · Charged residency and current streamed pages

**Goal:** Connect current pages to the browser renderer. · **Steps:** 31 · **Source:** [app/manifest.rs](../../../../src/app/manifest.rs) · [app/session_io.rs](../../../../src/app/session_io.rs) · [app/stream.rs](../../../../src/app/stream.rs) · [app/range_gate.rs](../../../../src/app/range_gate.rs)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Octree Cell Subdivision: 8 Child AABBs (L0 -&gt; L1 -&gt; L2)</text>
  <rect x="68" y="48" width="108" height="108" fill="#f8fafc" stroke="#111111" stroke-width="2"/>
  <line x1="122" y1="48" x2="122" y2="156" stroke="#111111" stroke-width="1.5"/>
  <line x1="68" y1="102" x2="176" y2="102" stroke="#111111" stroke-width="1.5"/>
  <rect x="122" y="48" width="54" height="54" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <line x1="149" y1="48" x2="149" y2="102" stroke="#2563eb" stroke-width="1.2"/>
  <line x1="122" y1="75" x2="176" y2="75" stroke="#2563eb" stroke-width="1.2"/>
  <rect x="149" y="48" width="27" height="27" fill="#fef08a" stroke="#dc2626" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="238" y="88" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">sse = d_cell / z</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="238" y="110" text-anchor="middle" font-family="monospace" font-size="10" fill="#16a34a">sse > τ_split</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Frustum Cull + HTTP Range: bytes=offset..offset+len</text>
  <circle cx="432" cy="104" r="5" fill="#111111"/>
  <polygon points="432,104 562,50 562,156" fill="#fef9c3" fill-opacity="0.4" stroke="#111111" stroke-width="1.5" stroke-dasharray="4 3"/>
  <rect x="504" y="68" width="38" height="34" fill="#bbf7d0" stroke="#16a34a" stroke-width="2"/>
  <rect x="504" y="110" width="38" height="32" fill="#dbeafe" stroke="#2563eb" stroke-width="2"/>
  <rect x="592" y="64" width="102" height="78" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="600" y="74" width="86" height="16" fill="#bbf7d0" stroke="#16a34a" stroke-width="1.2"/>
  <rect x="600" y="96" width="86" height="16" fill="#dbeafe" stroke="#2563eb" stroke-width="1.2"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Progressive Multi-Resolution Point Cloud</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="832" y="58" width="96" height="90" fill="none" stroke="#94a3b8" stroke-width="1.2" stroke-dasharray="3 3"/>
  <circle cx="860" cy="84" r="6" fill="#94a3b8"/>
  <circle cx="898" cy="116" r="6" fill="#94a3b8"/>
  <rect x="936" y="58" width="96" height="90" fill="none" stroke="#16a34a" stroke-width="1.75"/>
  <circle cx="956" cy="76" r="3.5" fill="#16a34a"/><circle cx="972" cy="76" r="3.5" fill="#16a34a"/><circle cx="988" cy="76" r="3.5" fill="#16a34a"/><circle cx="1004" cy="76" r="3.5" fill="#16a34a"/>
  <circle cx="956" cy="96" r="3.5" fill="#2563eb"/><circle cx="972" cy="96" r="3.5" fill="#2563eb"/><circle cx="988" cy="96" r="3.5" fill="#2563eb"/><circle cx="1004" cy="96" r="3.5" fill="#2563eb"/>
  <circle cx="956" cy="116" r="3.5" fill="#dc2626"/><circle cx="972" cy="116" r="3.5" fill="#dc2626"/><circle cx="988" cy="116" r="3.5" fill="#dc2626"/><circle cx="1004" cy="116" r="3.5" fill="#dc2626"/>
</svg>
</div>

## <a id="checkpoint-68-reserve-page-bytes-before-reading"></a>1. Reserve page bytes before reading

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU Scheduler: Permit Pool + HTTP Range Request</text>
  <rect x="52" y="50" width="268" height="44" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="76" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">GET Range: bytes=off..off+len</text>
  <rect x="52" y="104" width="268" height="44" rx="4" fill="#fef9c3" stroke="#d97706" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="130" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Bounded In-Flight Permits &amp; Epoch</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU Byte Budget Check -&gt; queue.write_buffer(page)</text>
  <rect x="424" y="52" width="272" height="96" rx="6" fill="#dcfce7" stroke="#16a34a" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="78" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">charged_bytes &lt;= max_residency</text>
  <line x1="452" y1="100" x2="668" y2="100" stroke="#16a34a" stroke-width="3" marker-end="url(#arrG)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">Upload Decoded Page to GPU Slot</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Resident GPU Pages Rendered in Viewport</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="832" y="60" width="92" height="86" fill="#dcfce7" stroke="#16a34a" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="878" y="106" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">Page #1 ✓</text>
  <rect x="938" y="60" width="92" height="86" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="984" y="106" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">Page #2 ✓</text>
</svg>
</div>

### 1. `src/page_cache.rs`

Reserve bytes before allocation and release them with the owning charge.

Create the file and type:

```rust
use std::{cell::Cell, collections::BTreeMap, rc::Rc};

pub struct Budget {
    limit: usize,
    used: Cell<usize>,
}
impl Budget {
    pub fn new(limit: usize) -> Rc<Self> {
        Rc::new(Self {
            limit,
            used: Cell::new(0),
        })
    }
    pub fn used(&self) -> usize {
        self.used.get()
    }
    pub fn available(&self) -> usize {
        self.limit - self.used.get()
    }
    pub fn reserve(self: &Rc<Self>, bytes: usize) -> Result<Charge, &'static str> {
        if bytes > self.available() {
            return Err("Page memory budget exhausted");
        }
        self.used.set(self.used.get() + bytes);
        Ok(Charge {
            budget: self.clone(),
            bytes,
        })
    }
}
pub struct Charge {
    budget: Rc<Budget>,
    bytes: usize,
}
impl Charge {
    pub fn resize(&mut self, bytes: usize) -> Result<(), &'static str> {
        if bytes > self.bytes {
            let extra = bytes - self.bytes;
            if extra > self.budget.available() {
                return Err("Page memory budget exhausted");
            }
            self.budget
                .used
                .set(self.budget.used.get() + extra);
        } else {
            self.budget
                .used
                .set(self.budget.used.get() - (self.bytes - bytes));
        }
        self.bytes = bytes;
        Ok(())
    }
    pub fn bytes(&self) -> usize {
        self.bytes
    }
}
impl Drop for Charge {
    fn drop(&mut self) {
        self.budget
            .used
            .set(self.budget.used.get() - self.bytes);
    }
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod range_source;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod range_source;

pub mod page_cache;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-68a-keep-shared-pages-alive-during-eviction"></a>2. Keep shared pages alive during eviction

### 1. `src/page_cache.rs`

Keep page values and their accounting alive across cache removal.

**Locate this existing block:**

```rust
            .set(self.budget.used.get() - self.bytes);
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            .set(self.budget.used.get() - self.bytes);
    }
}

pub trait ByteSize {
    fn bytes(&self) -> usize;
}
impl ByteSize for Vec<u8> {
    fn bytes(&self) -> usize {
        self.capacity()
    }
}

pub struct Held<T> {
    pub value: T,
    charge: Charge,
}
impl<T> Held<T> {
    pub fn bytes(&self) -> usize {
        self.charge.bytes()
    }
}
struct Entry<T> {
    value: Rc<Held<T>>,
    touch: u64,
}
pub struct Cache<K: Ord, T> {
    budget: Rc<Budget>,
    entries: BTreeMap<K, Entry<T>>,
    clock: u64,
}
impl<K: Ord + Clone, T: ByteSize> Cache<K, T> {
    pub fn new(budget: Rc<Budget>) -> Self {
        Self {
            budget,
            entries: BTreeMap::new(),
            clock: 0,
        }
    }
    pub fn get(&mut self, key: &K) -> Option<Rc<Held<T>>> {
        self.clock = self.clock.checked_add(1)?;
        let entry = self.entries.get_mut(key)?;
        entry.touch = self.clock;
        Some(entry.value.clone())
    }
    pub fn oldest(&self) -> Option<K> {
        self.entries
            .iter()
            .min_by_key(|(_, entry)| entry.touch)
            .map(|(key, _)| key.clone())
    }
    pub fn remove(&mut self, key: &K) -> bool {
        self.entries.remove(key).is_some()
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    pub fn keys(&self) -> Vec<K> {
        self.entries.keys().cloned().collect()
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn used(&self) -> usize {
        self.budget.used()
    }
}
```

## <a id="checkpoint-68b-evict-unpinned-pages-in-access-order"></a>3. Evict unpinned pages in access order

### 1. `src/page_cache.rs`

Reclaim sufficient unpinned payload before reserving a new page. Copy cache ownership, reservation and eviction regression fixtures.

**Locate this existing block:**

```rust
    }
    pub fn oldest(&self) -> Option<K> {
        self.entries
            .iter()
            .min_by_key(|(_, entry)| entry.touch)
            .map(|(key, _)| key.clone())
    }
    pub fn remove(&mut self, key: &K) -> bool {
        self.entries.remove(key).is_some()
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    pub fn keys(&self) -> Vec<K> {
        self.entries.keys().cloned().collect()
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn used(&self) -> usize {
        self.budget.used()
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn reserve(&mut self, bytes: usize) -> Result<Charge, &'static str> {
        if bytes > self.budget.limit {
            return Err("One page exceeds its memory budget");
        }
        let mut victims = self
            .entries
            .iter()
            .filter(|(_, entry)| Rc::strong_count(&entry.value) == 1)
            .map(|(key, entry)| (entry.touch, key.clone(), entry.value.bytes()))
            .collect::<Vec<_>>();
        victims.sort_by_key(|v| v.0);
        let mut room = self.budget.available();
        let mut chosen = Vec::new();
        for (_, key, size) in victims {
            if room >= bytes {
                break;
            }
            room = room
                .checked_add(size)
                .ok_or("Page memory accounting overflow")?;
            chosen.push(key);
        }
        if room < bytes {
            return Err("Page memory is still held by a renderer or pending reader");
        }
        for key in chosen {
            self.entries.remove(&key);
        }
        self.budget.reserve(bytes)
    }
    pub fn hold(
        &self,
        value: T,
        mut charge: Charge,
    ) -> Result<Rc<Held<T>>, &'static str> {
        if !Rc::ptr_eq(&charge.budget, &self.budget) {
            return Err("Page charged to another cache budget");
        }
        charge.resize(value.bytes())?;
        Ok(Rc::new(Held { value, charge }))
    }
    pub fn insert(
        &mut self,
        key: K,
        value: T,
        charge: Charge,
    ) -> Result<Rc<Held<T>>, &'static str> {
        let value = self.hold(value, charge)?;
        self.clock = self
            .clock
            .checked_add(1)
            .ok_or("Page cache revision exhausted")?;
        self.entries.insert(
            key,
            Entry {
                value: value.clone(),
                touch: self.clock,
            },
        );
        Ok(value)
    }
    pub fn oldest(&self) -> Option<K> {
        self.entries
            .iter()
            .min_by_key(|(_, entry)| entry.touch)
            .map(|(key, _)| key.clone())
    }
    pub fn remove(&mut self, key: &K) -> bool {
        self.entries.remove(key).is_some()
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    pub fn keys(&self) -> Vec<K> {
        self.entries.keys().cloned().collect()
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn used(&self) -> usize {
        self.budget.used()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn add(
        cache: &mut Cache<u32, Vec<u8>>,
        key: u32,
        bytes: usize,
    ) -> Rc<Held<Vec<u8>>> {
        let charge = cache.reserve(bytes).unwrap();
        cache
            .insert(key, vec![key as u8; bytes], charge)
            .unwrap()
    }
    #[test]
    fn lru_eviction_reuses_room_without_dropping_live_renderer_pages() {
        let budget = Budget::new(12);
        let mut cache = Cache::new(budget.clone());
        let first = add(&mut cache, 1, 4);
        drop(add(&mut cache, 2, 4));
        drop(add(&mut cache, 3, 4));
        assert_eq!(budget.used(), 12);
        assert!(cache.remove(&1));
        assert_eq!(first.value, [1; 4]);
        assert_eq!(budget.used(), 12);
        drop(add(&mut cache, 4, 4));
        assert!(cache.get(&2).is_none());
        assert!(cache.get(&3).is_some());
        assert_eq!(budget.used(), 12);
        cache.clear();
        assert_eq!(budget.used(), 4);
        drop(first);
        assert_eq!(budget.used(), 0);
    }
    #[test]
    fn pinned_pages_and_failed_reservations_preserve_the_current_cache() {
        let budget = Budget::new(8);
        let mut cache = Cache::new(budget.clone());
        let first = add(&mut cache, 1, 4);
        let second = add(&mut cache, 2, 4);
        assert!(cache.reserve(4).is_err());
        assert_eq!(cache.len(), 2);
        assert_eq!(budget.used(), 8);
        drop(first);
        drop(second);
        let charge = cache.reserve(6).unwrap();
        assert_eq!(cache.len(), 0);
        assert_eq!(budget.used(), 6);
        drop(charge);
        assert_eq!(budget.used(), 0);
    }
    #[test]
    fn pending_bytes_are_reserved_before_allocation_and_failed_work_releases_them() {
        let budget = Budget::new(12);
        let mut pending = budget.reserve(10).unwrap();
        assert_eq!(budget.used(), 10);
        assert!(pending.resize(13).is_err());
        assert_eq!(pending.bytes(), 10);
        pending.resize(6).unwrap();
        assert_eq!(budget.used(), 6);
        drop(pending);
        assert_eq!(budget.used(), 0);
        let other = Budget::new(12);
        let mut cache = Cache::new(budget.clone());
        assert!(
            cache
                .insert(1u32, vec![0u8], other.reserve(1).unwrap())
                .is_err()
        );
        assert_eq!(other.used(), 0);
        assert_eq!(budget.used(), 0);
    }
```

## <a id="checkpoint-68c-reject-late-source-generations"></a>4. Reject late source generations

### 1. `src/stream_generation.rs`

Check source generation and exactly one completion per request.

Create the file and type:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ticket {
    generation: u64,
    request: u64,
}
#[derive(Default)]
pub struct Generation {
    generation: u64,
    issued: u64,
    pending: std::collections::BTreeSet<u64>,
    active: bool,
}
impl Generation {
    pub fn start(&mut self) -> Result<(), &'static str> {
        self.active = false;
        self.pending.clear();
        self.generation = self
            .generation
            .checked_add(1)
            .ok_or("Stream generations exhausted")?;
        self.active = true;
        Ok(())
    }
    pub fn request(&mut self) -> Result<Ticket, &'static str> {
        if !self.active {
            return Err("Stream source is closed");
        }
        self.issued = self
            .issued
            .checked_add(1)
            .ok_or("Stream requests exhausted")?;
        self.pending.insert(self.issued);
        Ok(Ticket {
            generation: self.generation,
            request: self.issued,
        })
    }
    pub fn current(&self, ticket: Ticket) -> bool {
        self.active
            && ticket.generation == self.generation
            && self.pending.contains(&ticket.request)
    }
    pub fn finish(&mut self, ticket: Ticket) -> bool {
        if !self.current(ticket) {
            return false;
        }
        self.pending.remove(&ticket.request)
    }
    pub fn close(&mut self) {
        self.active = false;
        self.pending.clear();
    }
    pub fn pending(&self) -> usize {
        self.pending.len()
    }
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod page_cache;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod page_cache;

pub mod stream_generation;

#[cfg(target_arch = "wasm32")]
```

### 3. `src/stream_generation.rs`

Copy concurrent completion, cancellation and overflow tests.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn multiple_current_reads_finish_once_and_cancelled_sources_never_revive() {
        let mut source = Generation::default();
        assert!(source.request().is_err());
        source.start().unwrap();
        let first = source.request().unwrap();
        let second = source.request().unwrap();
        assert!(source.finish(second));
        assert!(!source.finish(second));
        assert!(source.current(first));
        source.close();
        assert!(!source.finish(first));
        assert!(source.request().is_err());
        source.start().unwrap();
        let newer = source.request().unwrap();
        assert_ne!(newer, first);
        assert!(!source.finish(first));
        assert!(source.finish(newer));
        assert_eq!(source.pending(), 0);
    }
    #[test]
    fn exhausted_stream_generations_and_requests_do_not_wrap() {
        let mut source = Generation {
            generation: u64::MAX,
            active: true,
            ..Default::default()
        };
        assert!(source.start().is_err());
        assert!(!source.active);
        assert!(source.request().is_err());
        let mut source = Generation {
            issued: u64::MAX,
            active: true,
            ..Default::default()
        };
        assert!(source.request().is_err());
        assert_eq!(source.pending(), 0);
    }
}
```

## <a id="checkpoint-68d-bound-source-prefixes-and-decode-work"></a>5. Bound source prefixes and decode work

### 1. `src/stream_budget.rs`

Limit point ranges and reserve checked decode work.

Create the file and type:

```rust
pub const PREFIX_POINTS: u32 = 2_000_000;
pub const CHUNK_POINTS: u32 = 2_000_000;
pub const MAX_POINTS: u32 = 6_000_000;
pub const MIN_PREFIX: u32 = 250_000;
pub const MIN_BYTES: u64 = 64 * 1024 * 1024;
pub const PAGE_BYTES: usize = 256 * 1024 * 1024;

pub fn prefix(count: u32, share: u32, remaining: u32) -> u32 {
    count
        .min(PREFIX_POINTS)
        .min(share.min(remaining.max(MIN_PREFIX)))
}
pub fn next(from: u32, total: u32, remaining: u32) -> Option<std::ops::Range<u32>> {
    if from >= total || remaining == 0 {
        return None;
    }
    let count = (total - from)
        .min(CHUNK_POINTS)
        .min(remaining);
    Some(from..from + count)
}
pub fn temporary_page_bytes(count: u32) -> Option<usize> {
    usize::try_from(count).ok()?.checked_mul(160)
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod stream_generation;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod stream_generation;

pub mod stream_budget;

#[cfg(target_arch = "wasm32")]
```

### 3. `src/stream_budget.rs`

Copy range ceiling and allocation overflow fixtures.

**Locate this existing block:**

```rust
    usize::try_from(count).ok()?.checked_mul(160)
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    usize::try_from(count).ok()?.checked_mul(160)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn current_prefix_chunk_ceiling_and_overflow_rules_are_explicit() {
        assert_eq!(
            prefix(10_000_000, 2_000_000, 6_000_000),
            2_000_000
        );
        assert_eq!(prefix(10_000_000, 2_000_000, 0), MIN_PREFIX);
        assert_eq!(prefix(100, 2_000_000, 0), 100);
        assert_eq!(
            next(2_000_000, 10_000_000, 500_000),
            Some(2_000_000..2_500_000)
        );
        assert_eq!(
            next(u32::MAX - 3, u32::MAX, 10),
            Some(u32::MAX - 3..u32::MAX)
        );
        assert_eq!(next(0, 1, 0), None);
        assert_eq!(
            temporary_page_bytes(2_000_000),
            Some(320_000_000)
        );
    }
}
```

## <a id="checkpoint-68e-validate-original-streamed-presentation"></a>6. Validate streamed presentation

Validate original streamed presentation.

### 1. `src/stream_properties.rs`

Retain and validate original presentation fields.

Create the file and type:

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Properties {
    pub point_px: f32,
    pub visible: bool,
    pub locked: bool,
    seen: u8,
}
impl Default for Properties {
    fn default() -> Self {
        Self {
            point_px: 1.,
            visible: true,
            locked: false,
            seen: 0,
        }
    }
}
impl Properties {
    pub fn field(&mut self, field: &crate::stream::Field, bytes: &[u8]) -> bool {
        let bit = match field.field {
            6 => 1,
            16 => 2,
            17 => 4,
            _ => return true,
        };
        if self.seen & bit != 0 {
            return false;
        }
        match (field.field, field.wire) {
            (6, 1) => {
                let Ok(raw) = bytes.try_into() else {
                    return false;
                };
                let px = f64::from_le_bytes(raw);
                if !px.is_finite() || px <= 0. || !(px as f32).is_finite() {
                    return false;
                }
                self.point_px = px as f32;
            }
            (16, 0) if field.value <= 1 => self.visible = field.value == 1,
            (17, 0) if field.value <= 1 => self.locked = field.value == 1,
            _ => return false,
        }
        self.seen |= bit;
        true
    }
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod stream_budget;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod stream_budget;

pub mod stream_properties;

#[cfg(target_arch = "wasm32")]
```

### 3. `src/stream_properties.rs`

Copy original presentation value/error fixtures.

**Locate this existing block:**

```rust
        true
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_size_visibility_and_locking_are_checked_separately_from_placement_overrides()
     {
        let mut properties = Properties::default();
        let field = |field, wire, value| crate::stream::Field {
            field,
            wire,
            value,
            body: 0,
            next: 0,
        };
        assert!(properties.field(&field(6, 1, 0), &3.5f64.to_le_bytes()));
        assert!(properties.field(&field(16, 0, 0), &[]));
        assert!(properties.field(&field(17, 0, 1), &[]));
        assert_eq!(
            (
                properties.point_px,
                properties.visible,
                properties.locked
            ),
            (3.5, false, true)
        );
        assert!(!properties.field(&field(6, 1, 0), &4f64.to_le_bytes()));
        for px in [0., -1., f64::NAN, f64::INFINITY, f64::MAX] {
            assert!(!Properties::default().field(&field(6, 1, 0), &px.to_le_bytes()));
        }
        assert!(!Properties::default().field(&field(16, 0, 2), &[]));
        assert!(!Properties::default().field(&field(6, 2, 8), &1f64.to_le_bytes()));
    }
}
```

## <a id="checkpoint-68f-reuse-a-checked-source-probe"></a>7. Reuse a checked source probe

### 1. `src/stream.rs`

Read checked original style fields through bounded metadata windows.

**Locate this existing block:**

```rust
    }

    /// Points `[from, to)` of the cloud.
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub async fn cloud_properties(
        url: &str,
        fields: &CloudFields,
    ) -> Option<crate::stream_properties::Properties> {
        let mut properties = crate::stream_properties::Properties::default();
        let mut window = MetadataWindow::default();
        let mut at = body_end(
            fields.coords_at,
            fields.coords_len,
            fields.end,
        )?;
        while at < fields.end {
            let header = window
                .read(
                    url,
                    at,
                    64.min(fields.end - at),
                    fields.end,
                    &fields.revision,
                )
                .await?;
            let f = field_at(header, at, fields.end)?;
            let bytes = if (f.field, f.wire) == (6, 1) {
                window
                    .read(url, f.body, 8, fields.end, &fields.revision)
                    .await?
            } else {
                &[]
            };
            if !properties.field(&f, bytes) {
                return None;
            }
            at = f.next;
        }
        Some(properties)
    }

    /// Points `[from, to)` of the cloud.
```

### 2. `src/range_source.rs`

Reuse the probe and retain original source properties.

**Locate this existing block:**

```rust
#[cfg(target_arch = "wasm32")]
pub struct Source {
    pub guid: String,
    pub name: String,
    pub url: String,
    pub fields: crate::stream::CloudFields,
    pub lod: crate::stream::CloudLod,
}

#[cfg(target_arch = "wasm32")]
impl Source {
    pub async fn open(url: String) -> Result<Self, String> {
        let probe = crate::stream::probe(&url)
            .await
            .ok_or("Source probe failed")?;
        if probe.etag.is_none() && !crate::manifest::immutable_key(&url) {
            return Err("Streaming requires an ETag or immutable source URL".into());
        }
        let (guid, name) = crate::stream::cloud_identity(&probe.bytes)
            .ok_or("Missing original cloud identity")?;
        let mut fields = crate::stream::cloud_fields(&url, &probe)
            .await
            .ok_or("Not a single range-readable cloud")?;
        let lod = crate::stream::cloud_lod(&url, &mut fields)
            .await
            .ok_or("Invalid original cloud array or octree layout")?;
        Ok(Self {
            guid,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub struct Source {
    pub properties: crate::stream_properties::Properties,
    pub guid: String,
    pub name: String,
    pub url: String,
    pub fields: crate::stream::CloudFields,
    pub lod: crate::stream::CloudLod,
}

#[cfg(target_arch = "wasm32")]
impl Source {
    pub async fn open(url: String) -> Result<Self, String> {
        let probe = crate::stream::probe(&url)
            .await
            .ok_or("Source probe failed")?;
        Self::from_probe(url, &probe).await
    }
    pub async fn from_probe(
        url: String,
        probe: &crate::range_fetch::Reply,
    ) -> Result<Self, String> {
        if probe.etag.is_none() && !crate::manifest::immutable_key(&url) {
            return Err("Streaming requires an ETag or immutable source URL".into());
        }
        let (guid, name) = crate::stream::cloud_identity(&probe.bytes)
            .ok_or("Missing original cloud identity")?;
        let mut fields = crate::stream::cloud_fields(&url, probe)
            .await
            .ok_or("Not a single range-readable cloud")?;
        let lod = crate::stream::cloud_lod(&url, &mut fields)
            .await
            .ok_or("Invalid original cloud array or octree layout")?;
        let properties = crate::stream::cloud_properties(&url, &fields)
            .await
            .ok_or("Invalid original cloud presentation fields")?;
        Ok(Self {
            properties,
            guid,
```

## <a id="checkpoint-68fa-validate-complete-source-metadata"></a>8. Validate complete source metadata

Refuse mixed or placed originals before choosing sampled range display.

### 1. `src/range_shape.rs`

Validate single-cloud layout and complete original metadata before sampling. Copy original placement, source extension and metadata refusal fixtures.

Create the file and type:

```rust
pub fn single_cloud(head: &[u8]) -> bool {
    let mut at = 0u64;
    let objects = loop {
        let Some(raw) = head.get(at as usize..) else {
            return false;
        };
        let Some(field) = crate::stream::field_at(raw, at, u64::MAX) else {
            return false;
        };
        if field.field == 3 && field.wire == 2 {
            break field;
        }
        if field.field > 2 || field.wire != 2 {
            return false;
        }
        at = field.next;
    };
    at = objects.body;
    loop {
        let Some(raw) = head.get(at as usize..) else {
            return false;
        };
        let Some(field) = crate::stream::field_at(raw, at, objects.next) else {
            return false;
        };
        if field.field == 8 && field.wire == 2 {
            return field.next == objects.next;
        }
        if field.field > 2 || field.wire != 2 {
            return false;
        }
        at = field.next;
    }
}

pub fn metadata(raw: &[u8], guid: &str) -> Result<(), String> {
    use prost::Message;
    let tail = session_rust::proto::Session::decode(raw)
        .map_err(|_| "Invalid original source metadata")?;
    if tail.objects.is_some()
        || tail.definitions.is_some()
        || !tail.interactions.is_empty()
        || !tail.xforms.is_empty()
    {
        return Err(
            "Placed or extended originals require complete document loading".into(),
        );
    }
    if let Some(root) = tail
        .tree
        .as_ref()
        .and_then(|tree| tree.root.as_ref())
    {
        let guids = [guid].into_iter().collect();
        crate::hierarchy::validate(root, &guids).map_err(str::to_owned)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use {prost::Message, session_rust::proto};
    #[test]
    fn ranged_originals_refuse_unapplied_placements_extended_sources_and_wrong_tree_owners()
     {
        assert!(metadata(&[], "cloud").is_ok());
        let tail = proto::Session {
            xforms: vec![proto::XformEntry {
                guid: "cloud".into(),
                xform: Some(
                    session_rust::Xform::translation(1_000_000_000.125, 0., 0.)
                        .to_proto(),
                ),
            }],
            ..Default::default()
        };
        assert!(metadata(&tail.encode_to_vec(), "cloud").is_err());
        let tail = proto::Session {
            definitions: Some(Default::default()),
            ..Default::default()
        };
        assert!(metadata(&tail.encode_to_vec(), "cloud").is_err());
        let tail = proto::Session {
            tree: Some(proto::Tree {
                root: Some(proto::TreeNode {
                    name: "root".into(),
                    children: vec![proto::TreeNode {
                        name: "missing".into(),
                        ..Default::default()
                    }],
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert!(metadata(&tail.encode_to_vec(), "cloud").is_err());
        assert!(metadata(b"broken", "cloud").is_err());
    }
}
```

### 2. `src/lib.rs`

Register complete original metadata validation.

**Locate this existing block:**

```rust
pub mod stream_properties;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod stream_properties;

pub mod range_shape;

#[cfg(target_arch = "wasm32")]
```

### 3. `src/range_source.rs`

Refuse unapplied original metadata and mixed source geometry before range adoption.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        assert_eq!(page.coordinates, proto.coords);
        assert_eq!(page.ids, proto.point_ids);
        assert!(crate::stream::walk_to_coords(&raw[..at as usize - 1]).is_none());
    }
```

Replace it with:

```rust
        assert_eq!(page.coordinates, proto.coords);
        assert_eq!(page.ids, proto.point_ids);
        assert!(crate::stream::walk_to_coords(&raw[..at as usize - 1]).is_none());
        let mut mixed = document.clone();
        mixed
            .objects
            .as_mut()
            .unwrap()
            .points
            .push(session_rust::Point::new(0., 0., 0.).to_proto());
        assert!(!crate::range_shape::single_cloud(
            &mixed.encode_to_vec()
        ));
    }
```

**Locate this block:**

```rust
            ..Default::default()
        };
        let raw = document.encode_to_vec();
        assert_eq!(
            crate::stream::cloud_identity(&raw).unwrap(),
            (proto.guid.clone(), proto.name.clone())
```

Replace it with:

```rust
            ..Default::default()
        };
        let raw = document.encode_to_vec();
        assert!(crate::range_shape::single_cloud(&raw));
        assert_eq!(
            crate::stream::cloud_identity(&raw).unwrap(),
            (proto.guid.clone(), proto.name.clone())
```

**Locate this block:**

```rust
        let properties = crate::stream::cloud_properties(&url, &fields)
            .await
            .ok_or("Invalid original cloud presentation fields")?;
        Ok(Self {
            properties,
            guid,
```

Replace it with:

```rust
        let properties = crate::stream::cloud_properties(&url, &fields)
            .await
            .ok_or("Invalid original cloud presentation fields")?;
        let total = probe
            .total
            .ok_or("Missing original file size")?;
        let length = total
            .checked_sub(fields.end)
            .ok_or("Cloud exceeds its original file")?;
        if length > 1024 * 1024 {
            return Err("Original source metadata exceeds its range limit".into());
        }
        if length > 0 {
            let raw =
                crate::stream::source_range(&url, fields.end, length, &fields.revision)
                    .await
                    .ok_or("Original source metadata range failed")?;
            crate::range_shape::metadata(&raw, &guid)?;
        }
        Ok(Self {
            properties,
            guid,
```

**Locate this block:**

```rust
    ) -> Result<Self, String> {
        if probe.etag.is_none() && !crate::manifest::immutable_key(&url) {
            return Err("Streaming requires an ETag or immutable source URL".into());
        }
```

Replace it with:

```rust
    ) -> Result<Self, String> {
        if !crate::range_shape::single_cloud(&probe.bytes) {
            return Err("Ranged source must contain one original cloud".into());
        }
        if probe.etag.is_none() && !crate::manifest::immutable_key(&url) {
            return Err("Streaming requires an ETag or immutable source URL".into());
        }
```

## <a id="checkpoint-68g-slice-original-pages-without-losing-attributes"></a>9. Slice pages without losing attributes

Slice original pages without losing attributes.

### 1. `src/range_page.rs`

Keep original attributes and slices with checked page lengths. Copy exact attribute and slice regression fixtures.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            assert!(Page::decode(0..1, 1, &raw, None).is_err());
        }
    }
```

Replace it with:

```rust
            assert!(Page::decode(0..1, 1, &raw, None).is_err());
        }
    }
}

impl crate::page_cache::ByteSize for Page {
    fn bytes(&self) -> usize {
        Page::bytes(self)
    }
}

#[cfg(test)]
mod attribute_tests {
    use super::*;
    #[test]
    fn original_page_attributes_keep_exact_words_and_refuse_partial_columns() {
        let raw = [0.125f64, 0., 0., 0.375, 0., 0.]
            .into_iter()
            .flat_map(f64::to_le_bytes)
            .collect::<Vec<_>>();
        let page = Page::decode(0..2, 2, &raw, None)
            .unwrap()
            .attributes(
                vec![0xff00_00ff, 0x80ff_0000],
                vec![0x1234_5678, 0xffff_0000],
            )
            .unwrap();
        assert_eq!(page.colors, [0xff00_00ff, 0x80ff_0000]);
        assert_eq!(page.normals, [0x1234_5678, 0xffff_0000]);
        assert_eq!(page.coordinates[0], 0.125);
        assert_eq!(page.bytes(), 96);
        let tail = page.slice(1..2).unwrap();
        assert_eq!(tail.coordinates, [0.375, 0., 0.]);
        assert_eq!(tail.ids, [1]);
        assert_eq!(tail.colors, [0x80ff_0000]);
        assert_eq!(tail.normals, [0xffff_0000]);
        assert_eq!(tail.bytes(), 48);
        assert!(page.slice(2..3).is_err());
        assert!(
            Page::decode(0..2, 2, &raw, None)
                .unwrap()
                .attributes(vec![1], vec![])
                .is_err()
        );
        assert!(
            Page::decode(0..2, 2, &raw, None)
                .unwrap()
                .attributes(vec![], vec![1])
                .is_err()
        );
    }
```

**Locate this block:**

```rust
            coordinates,
            display,
            ids,
        })
    }
    pub fn bytes(&self) -> usize {
        self.coordinates.capacity() * 8
            + self.display.capacity() * 4
            + self.ids.capacity() * 4
    }
}
```

Replace it with:

```rust
            coordinates,
            display,
            ids,
            colors: Vec::new(),
            normals: Vec::new(),
        })
    }
    pub fn slice(&self, rows: Range<u32>) -> Result<Self, &'static str> {
        if rows.start < self.rows.start
            || rows.end > self.rows.end
            || rows.start >= rows.end
        {
            return Err("Slice is outside its original page");
        }
        let a = (rows.start - self.rows.start) as usize;
        let b = (rows.end - self.rows.start) as usize;
        Ok(Self {
            rows,
            coordinates: self.coordinates[a * 3..b * 3].to_vec(),
            display: self.display[a * 3..b * 3].to_vec(),
            ids: self.ids[a..b].to_vec(),
            colors: if self.colors.is_empty() {
                Vec::new()
            } else {
                self.colors[a..b].to_vec()
            },
            normals: if self.normals.is_empty() {
                Vec::new()
            } else {
                self.normals[a..b].to_vec()
            },
        })
    }
    pub fn attributes(
        mut self,
        colors: Vec<u32>,
        normals: Vec<u32>,
    ) -> Result<Self, &'static str> {
        let count = (self.rows.end - self.rows.start) as usize;
        if !colors.is_empty() && colors.len() != count
            || !normals.is_empty() && normals.len() != count
        {
            return Err("Source page attributes do not match its point rows");
        }
        self.colors = colors;
        self.normals = normals;
        Ok(self)
    }
    pub fn bytes(&self) -> usize {
        self.coordinates.capacity() * 8
            + self.display.capacity() * 4
            + self.ids.capacity() * 4
            + self.colors.capacity() * 4
            + self.normals.capacity() * 4
    }
}
```

**Locate this block:**

```rust
    pub ids: Vec<u32>,
}
impl Page {
    pub fn decode(
```

Replace it with:

```rust
    pub ids: Vec<u32>,
    pub colors: Vec<u32>,
    pub normals: Vec<u32>,
}
impl Page {
    pub fn decode(
```

## <a id="checkpoint-68h-prepare-shared-remote-source-owners"></a>10. Prepare shared remote source owners

### 1. `src/range_cloud.rs`

Own the complete remote descriptor and precise original bounds.

Create the file and type:

```rust
use crate::{
    bounds::Bounds,
    cloud::LodNode,
    page_cache::{Budget, Cache, Held},
    range_page::Page,
    range_source::Source,
};
use std::{cell::RefCell, rc::Rc};

pub struct Prepared {
    pub source: Rc<Source>,
    pub nodes: Rc<Vec<LodNode>>,
    pub bounds: Bounds,
    pub pages: RefCell<Cache<u32, Page>>,
    pub root: RefCell<Option<Rc<Held<Page>>>>,
    pub generation: RefCell<crate::stream_generation::Generation>,
    pub loaded: std::cell::Cell<u32>,
}
impl Prepared {
    pub fn new(
        source: Rc<Source>,
        budget: Rc<Budget>,
    ) -> Result<Rc<Self>, &'static str> {
        if source.guid.is_empty()
            || source.fields.count == 0
            || !source.lod.valid(source.fields.count)
            || !source.properties.point_px.is_finite()
            || source.properties.point_px <= 0.
            || source.fields.coords_len != u64::from(source.fields.count) * 24
            || source
                .fields
                .coords_at
                .checked_add(source.fields.coords_len)
                .is_none_or(|end| end > source.fields.end)
            || source.fields.revision.is_none()
                && !crate::manifest::immutable_key(&source.url)
        {
            return Err("Invalid streamed original source");
        }
        let n = source.lod.len();
        let mut nodes = Vec::with_capacity(n);
        for i in 0..n {
            let size = source.lod.size[i];
            let center = std::array::from_fn(|axis| {
                (source.lod.min[i * 3 + axis] + size * 0.5) as f32
            });
            nodes.push(LodNode {
                center,
                size: size as f32,
                spacing: source.lod.spacing[i] as f32,
                first: source.lod.first[i] as u32,
                count: source.lod.count[i] as u32,
                children: source.lod.children[i * 8..i * 8 + 8]
                    .try_into()
                    .unwrap(),
            });
        }
        let size = source.lod.size[0];
        let min = std::array::from_fn(|i| source.lod.min[i]);
        let max = std::array::from_fn(|i| min[i] + size);
        Ok(Rc::new(Self {
            source,
            nodes: Rc::new(nodes),
            bounds: Bounds { min, max },
            pages: RefCell::new(Cache::new(budget)),
            root: RefCell::new(None),
            generation: RefCell::new(Default::default()),
            loaded: std::cell::Cell::new(0),
        }))
    }
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod stream_properties;

pub mod range_shape;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod stream_properties;

pub mod range_cloud;

pub mod range_shape;
```

## <a id="checkpoint-68i-keep-independent-streamed-placements"></a>11. Keep independent streamed placements

### 1. `src/range_cloud.rs`

Keep independent owner placements and precise world bounds.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

#[derive(Clone)]
pub struct Row {
    pub id: crate::scene::ObjectId,
    pub guid: String,
    pub prepared: Rc<Prepared>,
    pub model: session_rust::Xform,
    pub point_px: f32,
    pub controls_visible: bool,
}
impl Row {
    pub fn bounds(&self) -> Bounds {
        let mut result = Bounds {
            min: [f64::INFINITY; 3],
            max: [f64::NEG_INFINITY; 3],
        };
        let bounds = &self.prepared.bounds;
        for x in [bounds.min[0], bounds.max[0]] {
            for y in [bounds.min[1], bounds.max[1]] {
                for z in [bounds.min[2], bounds.max[2]] {
                    let p = self
                        .model
                        .transform_point(&session_rust::Point::new(x, y, z));
                    result.include([p[0], p[1], p[2]]);
                }
            }
        }
        result
    }
}
```

## <a id="checkpoint-68j-pin-coarse-pages-and-adopt-checked-ranges"></a>12. Pin coarse pages and adopt checked ranges

### 1. `src/range_cloud.rs`

Adopt checked original pages and retain a coarse fallback. Copy source descriptor, page identity and lifetime fixtures.

**Locate this existing block:**

```rust
    }
}

#[derive(Clone)]
pub struct Row {
    pub id: crate::scene::ObjectId,
    pub guid: String,
    pub prepared: Rc<Prepared>,
    pub model: session_rust::Xform,
    pub point_px: f32,
    pub controls_visible: bool,
}
impl Row {
    pub fn bounds(&self) -> Bounds {
        let mut result = Bounds {
            min: [f64::INFINITY; 3],
            max: [f64::NEG_INFINITY; 3],
        };
        let bounds = &self.prepared.bounds;
        for x in [bounds.min[0], bounds.max[0]] {
            for y in [bounds.min[1], bounds.max[1]] {
                for z in [bounds.min[2], bounds.max[2]] {
                    let p = self
                        .model
                        .transform_point(&session_rust::Point::new(x, y, z));
                    result.include([p[0], p[1], p[2]]);
                }
            }
        }
        result
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn pin_root(
        &self,
        page: Page,
        charge: crate::page_cache::Charge,
    ) -> Result<(), &'static str> {
        let first = self.source.lod.first[0] as u32;
        let end = first + self.source.lod.count[0] as u32;
        if page.rows != (first..end) {
            return Err("Pinned root page does not match its original node");
        }
        let page = self.pages.borrow().hold(page, charge)?;
        self.root.replace(Some(page));
        Ok(())
    }
    pub fn page(&self, first: u32) -> Option<Rc<Held<Page>>> {
        self.pages.borrow_mut().get(&first)
    }
    pub fn pages(&self) -> Vec<Rc<Held<Page>>> {
        let mut cache = self.pages.borrow_mut();
        let keys = cache.keys();
        let mut pages = keys
            .iter()
            .filter_map(|key| cache.get(key))
            .collect::<Vec<_>>();
        if let Some(root) = self.root.borrow().as_ref() {
            if !pages.iter().any(|page| {
                page.value.rows.start <= root.value.rows.start
                    && page.value.rows.end >= root.value.rows.end
            }) {
                pages.push(root.clone());
            }
        }
        pages
    }
    pub fn adopt(
        &self,
        page: Page,
        charge: crate::page_cache::Charge,
    ) -> Result<Rc<Held<Page>>, &'static str> {
        if page.rows.start >= page.rows.end || page.rows.end > self.source.fields.count
        {
            return Err("Page is outside its original source");
        }
        let mut cache = self.pages.borrow_mut();
        let ids = page
            .ids
            .iter()
            .copied()
            .collect::<std::collections::HashSet<_>>();
        for key in cache.keys() {
            if key != page.rows.start {
                if let Some(existing) = cache.get(&key) {
                    if page.rows.start < existing.value.rows.end
                        && existing.value.rows.start < page.rows.end
                    {
                        return Err("Source page ranges overlap");
                    }
                    if existing
                        .value
                        .ids
                        .iter()
                        .any(|id| ids.contains(id))
                    {
                        return Err("Original point IDs repeat across retained pages");
                    }
                }
            }
        }
        let end = page.rows.end;
        let result = cache.insert(page.rows.start, page, charge)?;
        self.loaded.set(self.loaded.get().max(end));
        Ok(result)
    }
    pub fn position(&self, id: u32) -> Option<[f64; 3]> {
        // Look only through the retained pages; on-demand source queries load missing rows separately.
        for page in self.pages() {
            if let Some(i) = page
                .value
                .ids
                .iter()
                .position(|value| *value == id)
            {
                return page.value.coordinates[i * 3..i * 3 + 3]
                    .try_into()
                    .ok();
            }
        }
        None
    }
}

#[derive(Clone)]
pub struct Row {
    pub id: crate::scene::ObjectId,
    pub guid: String,
    pub prepared: Rc<Prepared>,
    pub model: session_rust::Xform,
    pub point_px: f32,
    pub controls_visible: bool,
}
impl Row {
    pub fn bounds(&self) -> Bounds {
        let mut result = Bounds {
            min: [f64::INFINITY; 3],
            max: [f64::NEG_INFINITY; 3],
        };
        let bounds = &self.prepared.bounds;
        for x in [bounds.min[0], bounds.max[0]] {
            for y in [bounds.min[1], bounds.max[1]] {
                for z in [bounds.min[2], bounds.max[2]] {
                    let p = self
                        .model
                        .transform_point(&session_rust::Point::new(x, y, z));
                    result.include([p[0], p[1], p[2]]);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn source() -> Rc<Source> {
        Rc::new(Source {
            properties: Default::default(),
            guid: "original-cloud".into(),
            name: "Remote original".into(),
            url: "points.pb".into(),
            fields: crate::stream::CloudFields {
                end: 100,
                coords_at: 10,
                coords_len: 48,
                colors_at: 58,
                colors_len: 0,
                normals_at: 0,
                normals_len: 0,
                count: 2,
                ids_at: 60,
                ids_len: 8,
                revision: Some("revision1".into()),
            },
            lod: crate::stream::CloudLod {
                min: vec![1_000_000_000.125, 0., 0.],
                size: vec![1.],
                spacing: vec![0.5],
                level: vec![0],
                first: vec![0],
                count: vec![2],
                children: vec![-1; 8],
            },
        })
    }
    fn page() -> Page {
        let raw = [
            1_000_000_000.125f64,
            0.,
            0.,
            1_000_000_000.375,
            0.,
            0.,
        ]
        .into_iter()
        .flat_map(f64::to_le_bytes)
        .collect::<Vec<_>>();
        let ids = [u32::MAX, 7]
            .into_iter()
            .flat_map(u32::to_le_bytes)
            .collect::<Vec<_>>();
        Page::decode(0..2, 2, &raw, Some(&ids)).unwrap()
    }
    #[test]
    fn streamed_rows_keep_original_descriptor_precise_bounds_and_page_identities() {
        let source = source();
        let weak = Rc::downgrade(&source);
        let budget = Budget::new(1024);
        let prepared = Prepared::new(source.clone(), budget.clone()).unwrap();
        let page = page();
        let charge = budget.reserve(page.bytes()).unwrap();
        let held = prepared.adopt(page, charge).unwrap();
        assert_eq!(
            prepared.position(u32::MAX).unwrap()[0],
            1_000_000_000.125
        );
        assert_eq!(
            prepared.position(7).unwrap()[0],
            1_000_000_000.375
        );
        assert_eq!(prepared.bounds.min[0], 1_000_000_000.125);
        assert!(Rc::ptr_eq(&prepared.source, &source));
        prepared.pages.borrow_mut().clear();
        assert!(prepared.position(7).is_none());
        assert_eq!(held.value.ids, [u32::MAX, 7]);
        assert!(budget.used() > 0);
        drop(held);
        assert_eq!(budget.used(), 0);
        drop(source);
        drop(prepared);
        assert!(weak.upgrade().is_none());
    }
    #[test]
    fn invalid_unversioned_sources_and_overlapping_pages_are_refused() {
        let budget = Budget::new(1024);
        let prepared = Prepared::new(source(), budget.clone()).unwrap();
        let original = page();
        let charge = budget.reserve(original.bytes()).unwrap();
        drop(prepared.adopt(original, charge).unwrap());
        let raw = [1_000_000_000.375f64, 0., 0.]
            .into_iter()
            .flat_map(f64::to_le_bytes)
            .collect::<Vec<_>>();
        let overlap = Page::decode(1..2, 2, &raw, None).unwrap();
        let charge = budget.reserve(overlap.bytes()).unwrap();
        assert!(prepared.adopt(overlap, charge).is_err());
        assert_eq!(prepared.pages.borrow().len(), 1);
        let mut unversioned = source();
        Rc::get_mut(&mut unversioned)
            .unwrap()
            .fields
            .revision = None;
        assert!(Prepared::new(unversioned, budget).is_err());
    }
```

## <a id="checkpoint-68k-supply-coarse-and-detailed-page-specimens"></a>13. Supply coarse and detailed page specimens

### 1. `src/range_example.rs`

Copy source-page rendering and ownership specimens.

Create the file and type:

```rust
use crate::{
    page_cache::Budget, range_cloud::Prepared, range_page::Page, range_source::Source,
};
use std::rc::Rc;

pub fn prepared() -> Rc<Prepared> {
    let source = Rc::new(Source {
        properties: crate::stream_properties::Properties::default(),
        guid: "remote-original".into(),
        name: "Streamed source".into(),
        url: "points.pb".into(),
        fields: crate::stream::CloudFields {
            end: 200,
            coords_at: 10,
            coords_len: 96,
            colors_at: 106,
            colors_len: 0,
            normals_at: 0,
            normals_len: 0,
            count: 4,
            ids_at: 120,
            ids_len: 16,
            revision: Some("v1".into()),
        },
        lod: crate::stream::CloudLod {
            min: vec![-1., -1., -1., -1., -1., -1.],
            size: vec![2., 2.],
            spacing: vec![0.1, 0.05],
            level: vec![0, 1],
            first: vec![0, 2],
            count: vec![2, 2],
            children: [vec![1], vec![-1; 15]].concat(),
        },
    });
    let budget = Budget::new(4096);
    let prepared = Prepared::new(source, budget.clone()).unwrap();
    let page = |rows, values: Vec<f64>, ids: Vec<u32>, colors: Vec<u32>| {
        let raw = values
            .into_iter()
            .flat_map(f64::to_le_bytes)
            .collect::<Vec<_>>();
        let ids = ids
            .into_iter()
            .flat_map(u32::to_le_bytes)
            .collect::<Vec<_>>();
        Page::decode(rows, 4, &raw, Some(&ids))
            .unwrap()
            .attributes(colors, vec![])
            .unwrap()
    };
    let root = page(
        0..2,
        vec![-0.5, -0.5, 0.3, 0.5, -0.5, 0.3],
        vec![10, 11],
        vec![0xff00_00ff, 0xff00_00ff],
    );
    let charge = budget.reserve(root.bytes()).unwrap();
    prepared.pin_root(root, charge).unwrap();
    let next = page(
        2..4,
        vec![-0.5, 0.5, 0.3, 0.5, 0.5, 0.3],
        vec![u32::MAX, 16_777_217],
        vec![0xffff_0000, 0xffff_0000],
    );
    let charge = budget.reserve(next.bytes()).unwrap();
    drop(prepared.adopt(next, charge).unwrap());
    prepared
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod range_cloud;

pub mod range_shape;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod range_cloud;

pub mod range_example;

pub mod range_shape;
```

## <a id="checkpoint-68l-insert-streamed-rows-on-the-current-layer"></a>14. Insert streamed rows on the current layer

### 1. `src/scene.rs`

Insert validated streamed rows and release them with the scene.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        self.points.retain(|row| row.id != id);
        self.texts.retain(|row| row.id != id);
        self.clouds.retain(|row| row.id != id);
        if self.extra == Some(id) {
```

Replace it with:

```rust
        self.points.retain(|row| row.id != id);
        self.texts.retain(|row| row.id != id);
        self.clouds.retain(|row| row.id != id);
        self.streamed.retain(|row| row.id != id);
        if self.extra == Some(id) {
```

**Locate this block:**

```rust
        self.view = Default::default();
        self.forest = Default::default();
        self.clouds = Vec::new();
        self.texts = Vec::new();
        self.points = Vec::new();
        self.paths = Vec::new();
```

Replace it with:

```rust
        self.view = Default::default();
        self.forest = Default::default();
        self.clouds = Vec::new();
        self.streamed = Vec::new();
        self.texts = Vec::new();
        self.points = Vec::new();
        self.paths = Vec::new();
```

**Locate this block:**

```rust
            row.controls_visible = Some(row.id) == parent;
        }
    }
    pub fn clouds(&self) -> &[crate::cloud::Row] {
        &self.clouds
    }
```

Replace it with:

```rust
            row.controls_visible = Some(row.id) == parent;
        }
    }
    pub fn streams(&self) -> &[crate::range_cloud::Row] {
        &self.streamed
    }
    pub fn insert_stream(
        &mut self,
        prepared: Rc<crate::range_cloud::Prepared>,
        model: session_rust::Xform,
        point_px: f32,
    ) -> Result<ObjectId, &'static str> {
        if !crate::placement::valid(&model.m) || !point_px.is_finite() || point_px < 0.
        {
            return Err("Invalid streamed source placement or point size");
        }
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        let mut guid = prepared.source.guid.clone();
        while self
            .streamed
            .iter()
            .any(|row| row.guid == guid)
        {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let row = crate::range_cloud::Row {
            id,
            guid: guid.clone(),
            prepared,
            model,
            point_px,
            controls_visible: false,
        };
        if row
            .bounds()
            .min
            .into_iter()
            .chain(row.bounds().max)
            .any(|v| !v.is_finite() || !(v as f32).is_finite())
        {
            return Err("Streamed source placement exceeds display range");
        }
        let name = row.prepared.source.name.clone();
        let properties = row.prepared.source.properties;
        self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id, properties.visible, properties.locked);
        self.streamed.push(row);
        crate::layer_edit::attach_generated(self, id, guid, name)?;
        Ok(id)
    }
    pub fn place_stream(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Invalid streamed source placement");
        }
        let row = self
            .streamed
            .iter_mut()
            .find(|row| row.id == id)
            .ok_or("Streamed source is gone")?;
        let mut candidate = row.clone();
        candidate.model = model;
        if candidate
            .bounds()
            .min
            .into_iter()
            .chain(candidate.bounds().max)
            .any(|v| !v.is_finite() || !(v as f32).is_finite())
        {
            return Err("Streamed source placement exceeds display range");
        }
        *row = candidate;
        Ok(())
    }
    pub fn clouds(&self) -> &[crate::cloud::Row] {
        &self.clouds
    }
```

**Locate this block:**

```rust
            ids: Default::default(),
            view: Default::default(),
            forest: Default::default(),
            clouds: Vec::new(),
            texts: Vec::new(),
            objects: Vec::new(),
```

Replace it with:

```rust
            ids: Default::default(),
            view: Default::default(),
            forest: Default::default(),
            streamed: Vec::new(),
            clouds: Vec::new(),
            texts: Vec::new(),
            objects: Vec::new(),
```

**Locate this block:**

```rust
    pub forest: crate::hierarchy::Forest,
    clouds: Vec<crate::cloud::Row>,
    texts: Vec<crate::scene_label::Row>,
    points: Vec<PointObject>,
```

Replace it with:

```rust
    pub forest: crate::hierarchy::Forest,
    streamed: Vec<crate::range_cloud::Row>,
    clouds: Vec<crate::cloud::Row>,
    texts: Vec<crate::scene_label::Row>,
    points: Vec<PointObject>,
```

## <a id="checkpoint-68m-carry-source-placement-through-scene-operations"></a>15. Carry source placement through scene operations

### 1. `src/scene.rs`

Carry streamed owners through scene identity and placement operations.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            .chain(self.points.iter().map(|r| r.id))
            .chain(self.texts.iter().map(|r| r.id))
            .chain(self.clouds.iter().map(|r| r.id))
            .filter(|id| self.pick_id(id.value()).is_some())
```

Replace it with:

```rust
            .chain(self.points.iter().map(|r| r.id))
            .chain(self.texts.iter().map(|r| r.id))
            .chain(self.clouds.iter().map(|r| r.id))
            .chain(self.streamed.iter().map(|r| r.id))
            .filter(|id| self.pick_id(id.value()).is_some())
```

**Locate this block:**

```rust
            {
                candidate.cloud_point_size(id, point_px)?;
            }
        }
        for doc in &source.forest.documents {
            let mut doc = (**doc).clone();
```

Replace it with:

```rust
            {
                candidate.cloud_point_size(id, point_px)?;
            }
            if source
                .streamed
                .iter()
                .any(|row| row.id == *old)
                && point_px > 0.
            {
                candidate
                    .streamed
                    .iter_mut()
                    .find(|row| row.id == id)
                    .unwrap()
                    .point_px = point_px;
            }
        }
        for doc in &source.forest.documents {
            let mut doc = (**doc).clone();
```

**Locate this block:**

```rust
        rows!(paths);
        rows!(points);
        rows!(clouds);
        rows!(texts);
        for old in &ids {
            let id = remap[old];
```

Replace it with:

```rust
        rows!(paths);
        rows!(points);
        rows!(clouds);
        rows!(streamed);
        rows!(texts);
        for old in &ids {
            let id = remap[old];
```

**Locate this block:**

```rust
                    .map(|row| row.guid.as_str())
            })
            .or_else(|| {
                self.texts
                    .iter()
                    .find(|row| row.id == id)
```

Replace it with:

```rust
                    .map(|row| row.guid.as_str())
            })
            .or_else(|| {
                self.streamed
                    .iter()
                    .find(|row| row.id == id)
                    .map(|row| row.guid.as_str())
            })
            .or_else(|| {
                self.texts
                    .iter()
                    .find(|row| row.id == id)
```

**Locate this block:**

```rust
            row.id = copy;
            row.guid = guid;
            self.points.push(row);
        } else if let Some(row) = self.clouds.iter().find(|row| row.id == id) {
            let mut row = row.clone();
            row.id = copy;
```

Replace it with:

```rust
            row.id = copy;
            row.guid = guid;
            self.points.push(row);
        } else if let Some(row) = self.streamed.iter().find(|row| row.id == id) {
            let mut row = row.clone();
            row.id = copy;
            row.guid = guid;
            row.controls_visible = false;
            self.streamed.push(row);
        } else if let Some(row) = self.clouds.iter().find(|row| row.id == id) {
            let mut row = row.clone();
            row.id = copy;
```

**Locate this block:**

```rust
        for row in &mut self.clouds {
            row.controls_visible = Some(row.id) == parent;
        }
    }
    pub fn streams(&self) -> &[crate::range_cloud::Row] {
        &self.streamed
```

Replace it with:

```rust
        for row in &mut self.clouds {
            row.controls_visible = Some(row.id) == parent;
        }
        for row in &mut self.streamed {
            row.controls_visible = Some(row.id) == parent;
        }
    }
    pub fn streams(&self) -> &[crate::range_cloud::Row] {
        &self.streamed
```

**Locate this block:**

```rust
        if let Some(row) = self.clouds.iter().find(|row| row.id == id) {
            return Some(row.bounds());
        }
        if let Some(row) = self.texts.iter().find(|row| row.id == id) {
            return row.bounds();
        }
```

Replace it with:

```rust
        if let Some(row) = self.clouds.iter().find(|row| row.id == id) {
            return Some(row.bounds());
        }
        if let Some(row) = self.streamed.iter().find(|row| row.id == id) {
            return Some(row.bounds());
        }
        if let Some(row) = self.texts.iter().find(|row| row.id == id) {
            return row.bounds();
        }
```

**Locate this block:**

```rust
            let model = &shift * &row.model;
            return self.place_point(id, model);
        }
        if let Some(row) = self.clouds.iter().find(|r| r.id == id) {
            let model = &shift * &row.model;
            return self.place_cloud(id, model);
```

Replace it with:

```rust
            let model = &shift * &row.model;
            return self.place_point(id, model);
        }
        if let Some(row) = self.streamed.iter().find(|r| r.id == id) {
            let model = &shift * &row.model;
            return self.place_stream(id, model);
        }
        if let Some(row) = self.clouds.iter().find(|r| r.id == id) {
            let model = &shift * &row.model;
            return self.place_cloud(id, model);
```

**Locate this block:**

```rust
            self.place_path(id, model)
        } else if self.clouds.iter().any(|r| r.id == id) {
            self.place_cloud(id, model)
        } else if self.texts.iter().any(|r| r.id == id) {
            self.place_text(id, model)
        } else {
```

Replace it with:

```rust
            self.place_path(id, model)
        } else if self.clouds.iter().any(|r| r.id == id) {
            self.place_cloud(id, model)
        } else if self.streamed.iter().any(|r| r.id == id) {
            self.place_stream(id, model)
        } else if self.texts.iter().any(|r| r.id == id) {
            self.place_text(id, model)
        } else {
```

**Locate this block:**

```rust
                    .map(|r| r.model.clone())
            })
            .or_else(|| {
                self.texts
                    .iter()
                    .find(|r| r.id == id)
```

Replace it with:

```rust
                    .map(|r| r.model.clone())
            })
            .or_else(|| {
                self.streamed
                    .iter()
                    .find(|r| r.id == id)
                    .map(|r| r.model.clone())
            })
            .or_else(|| {
                self.texts
                    .iter()
                    .find(|r| r.id == id)
```

**Locate this block:**

```rust
        for row in self
            .clouds
            .iter()
            .filter(|r| self.visible(r.id))
```

Replace it with:

```rust
        for row in self
            .streamed
            .iter()
            .filter(|r| self.visible(r.id))
        {
            let next = row.bounds();
            if let Some(bounds) = &mut bounds {
                bounds.include(next.min);
                bounds.include(next.max);
            } else {
                bounds = Some(next);
            }
        }
        for row in self
            .clouds
            .iter()
            .filter(|r| self.visible(r.id))
```

### 2. `src/hierarchy.rs`

Name streamed source rows in the original hierarchy.

**Locate this existing block:**

```rust
                .map(|r| (r.id, r.prepared.source.name.as_str())),
        );
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                .map(|r| (r.id, r.prepared.source.name.as_str())),
        )
        .chain(
            scene
                .streams()
                .iter()
                .map(|r| (r.id, r.prepared.source.name.as_str())),
        );
```

## <a id="checkpoint-68n-resolve-compact-picks-to-original-point-ids"></a>16. Resolve compact picks to point IDs

Resolve compact picks to original point IDs.

### 1. `src/cloud_pick.rs`

Connect streamed selection and original identities while refusing incomplete editable Save.

**Locate this existing block:**

```rust
pub fn resolve(scene: &Scene, owner: ObjectId, index: usize) -> Option<Target> {
    let row = scene
        .clouds()
        .iter()
        .find(|r| r.id == owner && r.controls_visible)?;
    if scene.pick_id(owner.value()).is_none()
        || index >= row.prepared.source.point_count()
    {
        return None;
    }
    let target = Target {
        owner,
        part: Part::CloudPoint(row.prepared.source.point_id(index)),
    };
    Some(target)
}

pub fn position(
    scene: &Scene,
    owner: ObjectId,
    id: u32,
) -> Option<session_rust::Point> {
    let row = scene
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn resolve(scene: &Scene, owner: ObjectId, index: usize) -> Option<Target> {
    if let Some(row) = scene
        .streams()
        .iter()
        .find(|r| r.id == owner && r.controls_visible)
    {
        scene.pick_id(owner.value())?;
        let mut address = 0usize;
        for page in row.prepared.pages() {
            let count = page.value.ids.len();
            if index >= address && index - address < count {
                return Some(Target {
                    owner,
                    part: Part::CloudPoint(page.value.ids[index - address]),
                });
            }
            address = address.checked_add(count)?;
        }
        return None;
    }
    let row = scene
        .clouds()
        .iter()
        .find(|r| r.id == owner && r.controls_visible)?;
    if scene.pick_id(owner.value()).is_none()
        || index >= row.prepared.source.point_count()
    {
        return None;
    }
    let target = Target {
        owner,
        part: Part::CloudPoint(row.prepared.source.point_id(index)),
    };
    Some(target)
}

pub fn position(
    scene: &Scene,
    owner: ObjectId,
    id: u32,
) -> Option<session_rust::Point> {
    if let Some(row) = scene
        .streams()
        .iter()
        .find(|r| r.id == owner && r.controls_visible)
    {
        scene.pick_id(owner.value())?;
        let p = row.prepared.position(id)?;
        return Some(
            row.model
                .transform_point(&session_rust::Point::new(p[0], p[1], p[2])),
        );
    }
    let row = scene
```

### 2. `src/pick_sources.rs`

Connect streamed selection and original identities while refusing incomplete editable Save.

**Locate this existing block:**

```rust
            && kind == Kind::Control
            && scene.clouds().iter().any(|r| r.id == owner)
        {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            && kind == Kind::Control
            && (scene.clouds().iter().any(|r| r.id == owner)
                || scene.streams().iter().any(|r| r.id == owner))
        {
```

### 3. `src/editor.rs`

Connect streamed selection and original identities while refusing incomplete editable Save.

**Locate this existing block:**

```rust
pub enum Action {
    CanvasStart {
        from: [f32; 2],
        to: [f32; 2],
        size: [f64; 2],
        modified: bool,
        pointer: i32,
    },
    CanvasMove {
        screen: [f32; 2],
        size: [f64; 2],
    },
    CanvasRelease {
        screen: [f32; 2],
        size: [f64; 2],
        add: bool,
        remove: bool,
    },
    CanvasHit(Option<ObjectId>),
    CanvasCancel,
    SelectRows {
        ids: Vec<ObjectId>,
        add: bool,
        remove: bool,
    },
    PanScreen {
        delta: [f64; 2],
        size: [f64; 2],
    },
    ZoomAt {
        factor: f32,
        screen: [f32; 2],
    },
    Touch(Vec<Action>),
    LayerEdit {
        key: String,
        edit: crate::layer_edit::Edit,
    },
    LayerProperty {
        key: String,
        property: crate::layer_properties::Property,
    },
    TreePick {
        ids: Vec<ObjectId>,
        additive: bool,
    },
    GizmoPress {
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        modified: bool,
    },
    GizmoTouch {
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
    },
    GizmoMove {
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        shift: bool,
    },
    GizmoRelease {
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        shift: bool,
    },
    GizmoCancel,
    GizmoNumber(String),
    Snap(crate::snap::Setting),
    ToolScreen {
        screen: [f32; 2],
        size: [f64; 2],
        commit: bool,
    },
    BeginTool(crate::tool::Shape),
    ToolPoint([f64; 3]),
    ToolHover(Option<[f64; 3]>),
    CancelTool,
    Pick([f32; 2]),
    Picked(Option<ObjectId>),
    PickedTarget(Option<crate::pick_sources::Target>),
    PickedMany {
        target: Option<crate::pick_sources::Target>,
        additive: bool,
    },
    SelectAll,
    PickMode(crate::pick_sources::Mode),
    Live(Box<Scene>),
    Adopt {
        scene: Box<Scene>,
        replace: bool,
    },
    Import(Vec<u8>),
    Replace(Vec<u8>),
    ImportAt(
        Vec<u8>,
        std::rc::Rc<crate::reload_url::ReloadUrl>,
    ),
    ReplaceAt(
        Vec<u8>,
        std::rc::Rc<crate::reload_url::ReloadUrl>,
    ),
    Close,
    UnloadSources,
    Hide,
    Show,
    AddBox,
    AddNested,
    AddCloudLod,
    AddCloud,
    AddCad,
    AddText(crate::text::TextPlacement),
    AddSurface,
    AddLine,
    AddPoint,
    AddCurve,
    AddColours(crate::colour::Mode),
    NormalView(bool),
    Grid(Option<crate::grid::Settings>),
    AddNormals(crate::normal_example::Mode),
    Controls(bool),
    AddPolyline,
    AddArrow,
    Translate([f64; 3]),
    ToggleExtra,
    SelectNext,
    Delete,
    Undo,
    Redo,
    Background,
    Zoom(f32),
    Pan(f32, f32),
    Orbit(f64, f64),
    Isometric,
    ResetView,
    Fit,
    FitSelected,
    Projection(crate::camera::Projection),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Change {
    View,
    Scene,
    Preview,
}

pub struct Editor {
    pub canvas_drag: Option<crate::canvas_drag::Drag>,
    pub gizmo_edit: Option<crate::gizmo_edit::Edit>,
    gizmo_hover: Option<crate::gizmo::Handle>,
    pub edit_preview: Option<crate::edit_preview::Preview>,
    pub snapping: crate::snap::Settings,
    snap_index: Option<crate::snap_index::Index>,
    pub draft: Option<crate::tool::Draft>,
    pub tool_status: String,
    pub scene: Scene,
    pub selected: Option<ObjectId>,
    pub selected_part: Option<crate::pick_sources::Part>,
    pub pick_mode: crate::pick_sources::Mode,
    objects: crate::object_selection::Objects,
    pub camera: Camera,
    pub background: Background,
    pub normal_view: bool,
    history: History,
    last_release: u64,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            canvas_drag: None,
            gizmo_edit: None,
            gizmo_hover: None,
            edit_preview: None,
            snapping: Default::default(),
            snap_index: None,
            draft: None,
            tool_status: String::new(),
            scene: Scene::demo(),
            selected: None,
            selected_part: None,
            pick_mode: Default::default(),
            objects: Default::default(),
            camera: Camera::default(),
            background: Background::default(),
            normal_view: false,
            history: History::default(),
            last_release: 0,
        }
    }
}

impl Editor {
    pub fn display_scene(&self) -> &Scene {
        self.canvas_drag
            .as_ref()
            .and_then(|d| d.preview.as_ref())
            .map_or_else(
                || {
                    self.gizmo_edit.as_ref().map_or_else(
                        || {
                            self.edit_preview
                                .as_ref()
                                .map_or(&self.scene, |p| &p.scene)
                        },
                        |g| g.preview.scene(),
                    )
                },
                |p| &p.scene,
            )
    }

    pub fn gizmo_origin(&self) -> Option<[f64; 3]> {
        if self.draft.is_some() || self.canvas_drag.is_some() {
            return None;
        }
        if self
            .scene
            .clouds()
            .iter()
            .any(|r| r.controls_visible)
        {
            return None;
        }
        if let Some(edit) = self.gizmo_edit.as_ref() {
            return Some(edit.origin());
        }
        if let Some(target) = self
            .selection()
            .filter(|_| self.selected_part.is_some())
        {
            return crate::part_preview::origin(&self.scene, target);
        }
        self.scene
            .bounds_of(&self.selected_objects())
            .map(|b| b.centre())
    }

    pub fn gizmo_hit(
        &self,
        screen: [f32; 2],
        size: [f64; 2],
    ) -> Option<crate::gizmo::Handle> {
        self.gizmo_hit_radius(screen, size, 8.)
    }

    pub fn gizmo_hit_radius(
        &self,
        screen: [f32; 2],
        size: [f64; 2],
        radius: f64,
    ) -> Option<crate::gizmo::Handle> {
        if size
            .iter()
            .any(|v| !v.is_finite() || *v <= 0.)
        {
            return None;
        }
        let origin = self.gizmo_origin()?;
        let scale = crate::widget_layout::world_per_pixel(
            &self.camera.view_projection(),
            origin,
            size[1],
        )?;
        let ray = self.camera.ray(screen)?;
        crate::gizmo::Gizmo::new(session_rust::Point::new(
            origin[0], origin[1], origin[2],
        ))
        .hit_with_radius(&ray.origin, &ray.direction, scale, radius)
    }

    pub fn number_prompt(
        &self,
        size: [f64; 2],
    ) -> Option<crate::number_box::NumberPrompt> {
        let edit = self.gizmo_edit.as_ref()?;
        let handle = edit.gizmo.typing?;
        let origin = edit.origin();
        let matrix = self.camera.view_projection();
        let scale = crate::widget_layout::world_per_pixel(&matrix, origin, size[1])?;
        let point = edit.gizmo.handle_point(handle, scale);
        let at = crate::snap_math::Screen {
            matrix: matrix.m,
            size,
        }
        .point([point[0], point[1], point[2]])?;
        let unit = handle.labels().2;
        Some(crate::number_box::NumberPrompt {
            handle,
            title: handle.title(),
            unit,
            hint: if unit == "factor" {
                "1"
            } else {
                "0"
            },
            at: at.map(|v| v as f32),
        })
    }

    pub fn gizmo_active(&self) -> f32 {
        crate::gizmo_edit::code(
            self.gizmo_edit
                .as_ref()
                .and_then(|e| e.active())
                .or(self.gizmo_hover),
        )
    }

    pub fn hover_gizmo(&mut self, screen: Option<([f32; 2], [f64; 2])>) {
        self.gizmo_hover = if self.gizmo_edit.is_some() {
            None
        } else {
            screen.and_then(|(point, size)| self.gizmo_hit(point, size))
        };
    }

    fn gizmo_press(
        &mut self,
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        modified: bool,
    ) -> Result<Change, &'static str> {
        self.gizmo_press_radius(pointer, screen, size, modified, 8.)
    }

    fn gizmo_press_radius(
        &mut self,
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        modified: bool,
        radius: f64,
    ) -> Result<Change, &'static str> {
        if modified
            || self.draft.is_some()
            || self
                .gizmo_edit
                .as_ref()
                .is_some_and(|g| g.pointer.is_some())
        {
            return Ok(Change::View);
        }
        let Some(handle) = self.gizmo_hit_radius(screen, size, radius) else {
            return Ok(Change::View);
        };
        let origin = self
            .gizmo_origin()
            .ok_or("No editable selection")?;
        let preview = crate::gizmo_edit::Preview::new(
            &self.scene,
            self.selected_objects(),
            self.selection()
                .filter(|_| self.selected_part.is_some()),
        )?;
        let ray = self
            .camera
            .ray(screen)
            .ok_or("Invalid handle ray")?;
        let gizmo = crate::gizmo::Gizmo::new(session_rust::Point::new(
            origin[0], origin[1], origin[2],
        ));
        self.gizmo_edit = crate::gizmo_edit::Edit::begin(
            gizmo, handle, &ray, preview, pointer, screen,
        );
        Ok(Change::View)
    }

    fn gizmo_move(
        &mut self,
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        shift: bool,
    ) -> Result<Change, &'static str> {
        if size
            .iter()
            .any(|v| !v.is_finite() || *v <= 0.)
        {
            return Err("Invalid handle viewport");
        }
        let Some(ray) = self.camera.ray(screen) else {
            return Err("Invalid handle ray");
        };
        let Some(edit) = self.gizmo_edit.as_mut() else {
            return Ok(Change::View);
        };
        Ok(
            if edit.update(pointer, screen, size, &ray, shift)? {
                Change::Preview
            } else {
                Change::View
            },
        )
    }

    fn gizmo_release(
        &mut self,
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        shift: bool,
    ) -> Result<Change, &'static str> {
        if self
            .gizmo_edit
            .as_ref()
            .is_none_or(|g| g.pointer != Some(pointer))
        {
            return Ok(Change::View);
        }
        self.gizmo_move(pointer, screen, size, shift)?;
        let edit = self.gizmo_edit.as_mut().unwrap();
        if edit.click() {
            self.tool_status = format!(
                "{}: type a value in {}, Enter applies, Esc closes",
                edit.active().unwrap().title(),
                edit.active().unwrap().labels().2
            );
            return Ok(Change::Preview);
        }
        let changed = edit
            .preview
            .commit(&mut self.scene, &mut self.history)?;
        self.gizmo_edit = None;
        self.tool_status = "Handle edit completed.".into();
        Ok(if changed {
            Change::Scene
        } else {
            Change::Preview
        })
    }

    fn gizmo_number(&mut self, text: &str) -> Result<Change, &'static str> {
        let edit = self
            .gizmo_edit
            .as_mut()
            .ok_or("No handle number is being requested")?;
        let handle = edit
            .gizmo
            .typing
            .ok_or("No handle number is being requested")?;
        let value = crate::gizmo::typed_value(handle, text)
            .map_err(|_| "Enter a finite number; scale needs a positive factor")?;
        let Some(value) = value else {
            self.gizmo_edit = None;
            return Ok(Change::Preview);
        };
        edit.preview
            .set(edit.gizmo.typed(handle, value))?;
        let changed = edit
            .preview
            .commit(&mut self.scene, &mut self.history)?;
        self.gizmo_edit = None;
        self.tool_status = format!(
            "{} {value} {}",
            handle.title(),
            handle.labels().2
        );
        Ok(if changed {
            Change::Scene
        } else {
            Change::Preview
        })
    }

    fn update_edit_preview(&mut self) -> Result<Change, &'static str> {
        let Some(preview) = self.edit_preview.as_mut() else {
            return Ok(Change::View);
        };
        let offset = self
            .draft
            .as_ref()
            .and_then(|d| d.points.first().zip(d.cursor))
            .map(|(a, b)| std::array::from_fn::<_, 3, _>(|i| b[i] - a[i]))
            .unwrap_or([0.; 3]);
        preview.set(session_rust::Xform::translation(
            offset[0], offset[1], offset[2],
        ))?;
        Ok(Change::Preview)
    }

    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.scene).chain(self.history.scenes())
    }

    pub fn presentation_scenes(&self) -> impl Iterator<Item = &Scene> {
        self.scenes()
            .chain(
                self.canvas_drag
                    .iter()
                    .filter_map(|d| d.preview.as_ref())
                    .flat_map(|p| p.scenes()),
            )
            .chain(
                self.edit_preview
                    .iter()
                    .flat_map(|p| p.scenes()),
            )
            .chain(
                self.gizmo_edit
                    .iter()
                    .flat_map(|g| g.preview.scenes()),
            )
    }

    pub fn reload_keys(&self) -> Vec<crate::rehydrate::ReloadKey> {
        let mut seen = std::collections::HashSet::new();
        self.scene
            .objects()
            .iter()
            .filter_map(crate::rehydrate::ReloadKey::of)
            .filter(|key| seen.insert(key.origin.id))
            .collect()
    }

    fn scenes_mut(&mut self) -> impl Iterator<Item = &mut Scene> {
        std::iter::once(&mut self.scene).chain(self.history.scenes_mut())
    }

    pub fn hydrate(
        &mut self,
        values: Vec<(crate::rehydrate::ReloadKey, Vec<u8>)>,
    ) -> Result<bool, &'static str> {
        if values.is_empty() {
            return Ok(false);
        }
        let mut seen = std::collections::HashSet::new();
        for (key, _) in &values {
            if !seen.insert(key.origin.id)
                || !self
                    .scene
                    .objects()
                    .iter()
                    .any(|row| key.matches(row))
            {
                return Ok(false);
            }
        }
        let prepared: Vec<_> = values
            .into_iter()
            .map(|(key, bytes)| crate::rehydrate::PreparedReload::new(key, &bytes))
            .collect::<Result<_, _>>()?;
        for plan in &prepared {
            if self
                .scenes()
                .flat_map(|scene| scene.objects())
                .filter(|row| plan.key.matches(row))
                .any(|row| !plan.validates(row))
            {
                return Err("Reloaded source GUID or metadata does not match");
            }
        }
        for scene in self.scenes_mut() {
            for plan in &prepared {
                scene.hydrate(plan);
            }
        }
        if self.edit_preview.take().is_some() {
            self.draft = None;
        }
        self.gizmo_edit = None;
        self.snap_index = None;
        Ok(true)
    }

    fn unload_sources(&mut self) -> Result<(), &'static str> {
        let mut origins: std::collections::HashSet<_> = self
            .scenes()
            .flat_map(|scene| scene.objects())
            .filter(|row| row.can_unload())
            .filter_map(|row| row.origin().map(|origin| origin.id))
            .collect();
        for row in self
            .scenes()
            .flat_map(|scene| scene.objects())
        {
            if let Some(origin) = row.origin() {
                if !row.can_unload() {
                    origins.remove(&origin.id);
                }
            }
        }
        if origins.is_empty() {
            return Err("No eligible imported sources to unload");
        }
        let epoch = self
            .last_release
            .checked_add(1)
            .ok_or("Release epochs exhausted")?;
        for scene in self.scenes_mut() {
            scene.unload(&origins, epoch);
        }
        self.last_release = epoch;
        Ok(())
    }

    pub fn move_object(
        &mut self,
        id: ObjectId,
        offset: [f64; 3],
    ) -> Result<Change, &'static str> {
        if !offset.iter().all(|value| value.is_finite()) {
            return Err("Move needs finite coordinates");
        }
        if !self.scene.contains(id) {
            return Err("Object not found");
        }
        let object = self
            .scene
            .objects()
            .iter()
            .find(|row| row.id == id);
        if self.scene.forest.readonly(id) {
            return Err("Object attributes follow their parent and are read only");
        }
        if object.is_some_and(|object| object.geometry().is_none()) {
            return Err("Reload editable sources before Move");
        }
        if offset.iter().all(|&value| value == 0.0) {
            return Ok(Change::View);
        }
        let ids = self.scene.with_descendants(&[id]);
        let delta = session_rust::Xform::translation(offset[0], offset[1], offset[2]);
        self.history
            .try_edit(&mut self.scene, |scene| {
                for id in ids {
                    let matrix = &delta
                        * &scene
                            .placement(id)
                            .ok_or("Object not found")?;
                    scene.place_following(id, matrix)?;
                }
                Ok(())
            })?;
        if self.edit_preview.take().is_some() {
            self.draft = None;
        }
        self.gizmo_edit = None;
        self.snap_index = None;
        Ok(Change::Scene)
    }

    pub fn delete_object(&mut self, id: ObjectId) -> Result<Change, &'static str> {
        if !self.scene.contains(id) {
            return Err("Object not found");
        }
        if self.scene.forest.readonly(id) {
            return Err("Object attributes follow their parent and are read only");
        }
        if self
            .scene
            .objects()
            .iter()
            .find(|row| row.id == id)
            .is_some_and(|row| row.geometry().is_none())
        {
            return Err("Reload editable sources before Delete");
        }
        let ids = self.scene.with_descendants(&[id]);
        self.history.edit(&mut self.scene, |scene| {
            for id in ids {
                scene.remove(id);
            }
        });
        if self.edit_preview.take().is_some() {
            self.draft = None;
        }
        self.objects.retain(&self.scene);
        if self.selected == Some(id) {
            self.selected = self.objects.primary();
            self.selected_part = None;
        }
        self.gizmo_edit = None;
        self.snap_index = None;
        Ok(Change::Scene)
    }

    pub fn selected_objects(&self) -> Vec<ObjectId> {
        if self.selected_part.is_some() {
            return Vec::new();
        }
        if self.objects.primary() == self.selected {
            self.objects.rows().to_vec()
        } else {
            self.selected
                .into_iter()
                .filter(|id| self.scene.pick_id(id.value()).is_some())
                .collect()
        }
    }

    pub fn select_objects(
        &mut self,
        rows: impl IntoIterator<Item = ObjectId>,
        additive: bool,
    ) {
        if additive {
            let old = self.selected_objects();
            self.objects.set(&self.scene, old, false);
        }
        self.selected_part = None;
        self.objects.set(&self.scene, rows, additive);
        self.selected = self.objects.primary();
    }

    pub fn select_rows(&mut self, ids: Vec<ObjectId>, add: bool, remove: bool) {
        if remove {
            let old = self.selected_objects();
            self.objects.set(&self.scene, old, false);
            self.objects.remove(&self.scene, ids);
            self.selected = self.objects.primary();
            self.selected_part = None;
        } else {
            self.select_objects(ids, add);
        }
    }

    fn finish_canvas(&mut self) -> Result<Change, &'static str> {
        let Some(mut drag) = self.canvas_drag.take() else {
            return Ok(Change::View);
        };
        let (add, remove) = drag.released.unwrap_or((false, false));
        if let Some(preview) = drag.preview.take() {
            let changed = preview.commit(&mut self.scene, &mut self.history)?;
            if changed {
                self.snap_index = None;
            }
            return Ok(if changed {
                Change::Scene
            } else {
                Change::Preview
            });
        }
        if drag.missed {
            if let Some(rectangle) = drag.rectangle() {
                let ids = crate::rect_select::found(
                    &self.scene,
                    &self.camera.view_projection().m,
                    drag.size,
                    rectangle,
                );
                self.select_rows(ids, add, remove);
            }
        }
        Ok(Change::View)
    }

    fn repair_selection(&mut self) {
        if self.objects.primary() == self.selected && self.selected_part.is_none() {
            self.objects.retain(&self.scene);
            self.selected = self.objects.primary();
        } else {
            self.objects.clear();
        }
    }

    fn move_selection(&mut self, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|n| n.is_finite()) {
            return Err("Move needs finite coordinates");
        }
        let ids = self.selected_objects();
        if ids.is_empty() {
            return Err("Select an object before Move");
        }
        if offset.iter().all(|&n| n == 0.) {
            return Ok(Change::View);
        }
        let mut preview = crate::edit_preview::Preview::new(&self.scene, ids)?;
        preview.set(session_rust::Xform::translation(
            offset[0], offset[1], offset[2],
        ))?;
        preview.commit(&mut self.scene, &mut self.history)?;
        Ok(Change::Scene)
    }

    fn delete_selection(&mut self) -> Result<Change, &'static str> {
        if self.selected_part.is_some() {
            return Err("Use Select Object before Delete");
        }
        let ids = self.selected_objects();
        if ids.is_empty() {
            return Ok(Change::View);
        }
        if self
            .scene
            .objects()
            .iter()
            .any(|r| ids.contains(&r.id) && r.geometry().is_none())
        {
            return Err("Reload editable sources before Delete");
        }
        let ids = self.scene.with_descendants(&ids);
        self.history.edit(&mut self.scene, |scene| {
            for id in ids {
                scene.remove(id);
            }
        });
        self.objects.clear();
        self.selected = None;
        Ok(Change::Scene)
    }

    pub fn selection(&self) -> Option<crate::pick_sources::Target> {
        self.selected
            .map(|owner| crate::pick_sources::Target {
                owner,
                part: self
                    .selected_part
                    .unwrap_or(crate::pick_sources::Part::Object),
            })
    }

    pub fn whole_selection(&self) -> Option<ObjectId> {
        self.selected
            .filter(|_| self.selected_part.is_none())
    }

    pub fn tool_line(&self, text: &str) -> Option<Result<Action, &'static str>> {
        if self.gizmo_edit.as_ref().is_some_and(|e| {
            e.gizmo.typing.is_some() || text.trim().eq_ignore_ascii_case("Escape")
        }) {
            return Some(Ok(
                if text.trim().eq_ignore_ascii_case("Escape") {
                    Action::GizmoCancel
                } else {
                    Action::GizmoNumber(text.to_owned())
                },
            ));
        }
        self.draft.as_ref()?;
        if text.trim().is_empty() || text.eq_ignore_ascii_case("Escape") {
            return Some(Ok(Action::CancelTool));
        }
        text.contains(',')
            .then(|| crate::tool_coords::parse(text).map(Action::ToolPoint))
    }

    fn place_tool_point(&mut self, point: [f64; 3]) -> Result<Change, &'static str> {
        let draft = self
            .draft
            .as_mut()
            .ok_or("No point tool is running")?;
        let name = draft.tool.name();
        let Some(built) = draft.submit(point)? else {
            self.tool_status = draft.prompt();
            self.snap_index = None;
            return self.update_edit_preview();
        };
        if let crate::tool::Built::Move(offset) = built {
            let preview = self
                .edit_preview
                .as_mut()
                .ok_or("No placement preview is running")?;
            preview.set(session_rust::Xform::translation(
                offset[0], offset[1], offset[2],
            ))?;
            let changed = preview.commit(&mut self.scene, &mut self.history)?;
            if self.edit_preview.take().is_some() {
                self.draft = None;
            }
            self.tool_status = "Move completed.".into();
            return Ok(if changed {
                Change::Scene
            } else {
                Change::Preview
            });
        }
        self.history
            .try_edit(&mut self.scene, |scene| match built {
                crate::tool::Built::Point(point) => {
                    scene.insert_point(point).map(|_| ())
                }
                crate::tool::Built::Line(line) => scene.insert_line(line).map(|_| ()),
                crate::tool::Built::Move(_) => {
                    unreachable!("Move uses its placement preview")
                }
            })?;
        self.draft = None;
        self.tool_status = format!("{name} created.");
        Ok(Change::Scene)
    }

    pub fn snap_point(
        &mut self,
        screen: [f32; 2],
        size: [f64; 2],
    ) -> Option<crate::snap::Hit> {
        let ray = self.camera.ray(screen)?;
        let from = self.draft.as_ref()?.points.last().copied();
        let hit = if self.snapping.objects {
            let exclude = self
                .edit_preview
                .as_ref()
                .filter(|_| from.is_some())
                .map_or_else(Vec::new, |p| p.ids().to_vec());
            let index = self.snap_index.get_or_insert_with(|| {
                crate::snap_index::Index::new(&self.scene).excluding(&exclude)
            });
            let viewport = crate::snap_math::Screen {
                matrix: self.camera.view_projection().m,
                size,
            };
            index.query(
                viewport,
                &ray,
                [
                    (screen[0] as f64 + 1.) * size[0] / 2.,
                    (1. - screen[1] as f64) * size[1] / 2.,
                ],
                self.snapping.modes,
                from,
            )
        } else {
            None
        };
        hit.or_else(|| crate::snap_index::fallback(self.snapping, &self.camera, screen))
    }

    pub fn snap_usage(&self) -> [u64; 2] {
        self.snap_index
            .as_ref()
            .map_or([0, 0], |index| {
                [index.builds, index.visits as u64]
            })
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::Touch(actions) => {
                let mut change = Change::View;
                for action in actions {
                    let next = self.apply(action)?;
                    if next != Change::View {
                        change = next;
                    }
                }
                return Ok(change);
            }
            Action::CanvasStart {
                from,
                to,
                size,
                modified,
                pointer,
            } => {
                self.canvas_drag =
                    crate::canvas_drag::Drag::new(from, to, size, modified);
                if let Some(drag) = self.canvas_drag.as_mut() {
                    drag.pointer = pointer;
                }
                return Ok(Change::View);
            }
            Action::CanvasMove { screen, size } => {
                if let Some(drag) = self.canvas_drag.as_mut() {
                    if drag.released.is_none()
                        && drag.size == size
                        && screen.iter().all(|v| v.is_finite())
                    {
                        drag.to = screen;
                        drag.update(&self.camera)?;
                    }
                    if drag.preview.is_some() {
                        return Ok(Change::Preview);
                    }
                }
                return Ok(Change::View);
            }
            Action::CanvasRelease {
                screen,
                size,
                add,
                remove,
            } => {
                if let Some(drag) = self.canvas_drag.as_mut() {
                    if drag.size != size {
                        self.canvas_drag = None;
                        return Ok(Change::Preview);
                    }
                    drag.to = screen;
                    drag.released = Some((add, remove));
                    drag.update(&self.camera)?;
                    if drag.preview.is_some() || drag.missed {
                        return self.finish_canvas();
                    }
                }
                return Ok(Change::View);
            }
            Action::CanvasHit(hit) => {
                let Some(drag) = self.canvas_drag.as_ref() else {
                    return Ok(Change::View);
                };
                let ids = hit.filter(|id| {
                    !drag.modified && self.scene.pick_id(id.value()).is_some()
                });
                if let Some(id) = ids {
                    let mut rows = self.selected_objects();
                    if !rows.contains(&id) {
                        self.select_objects([id], false);
                        rows = vec![id];
                    }
                    if let Err(error) = self.canvas_drag.as_mut().unwrap().grab(
                        &self.scene,
                        rows,
                        &self.camera,
                    ) {
                        self.canvas_drag = None;
                        return Err(error);
                    }
                } else {
                    self.canvas_drag.as_mut().unwrap().missed = true;
                }
                if self
                    .canvas_drag
                    .as_ref()
                    .unwrap()
                    .released
                    .is_some()
                {
                    return self.finish_canvas();
                }
                return Ok(Change::Preview);
            }
            Action::CanvasCancel => {
                let preview = self
                    .canvas_drag
                    .take()
                    .is_some_and(|d| d.preview.is_some());
                return Ok(if preview {
                    Change::Preview
                } else {
                    Change::View
                });
            }
            Action::SelectRows { ids, add, remove } => {
                self.select_rows(ids, add, remove);
                return Ok(Change::View);
            }
            _ => {}
        }
        self.canvas_drag = None;
        let keep_gizmo = matches!(
            &action,
            Action::GizmoPress { .. }
                | Action::GizmoTouch { .. }
                | Action::GizmoMove { .. }
                | Action::GizmoRelease { .. }
                | Action::GizmoCancel
                | Action::GizmoNumber(_)
                | Action::Zoom(_)
                | Action::ZoomAt { .. }
                | Action::PanScreen { .. }
                | Action::Pan(_, _)
                | Action::Orbit(_, _)
                | Action::Fit
                | Action::FitSelected
                | Action::Projection(_)
                | Action::ResetView
                | Action::Background
                | Action::NormalView(_)
                | Action::Grid(_)
                | Action::Snap(_)
        );
        let keep_edit = matches!(
            &action,
            Action::GizmoPress { .. }
                | Action::GizmoTouch { .. }
                | Action::GizmoMove { .. }
                | Action::GizmoRelease { .. }
                | Action::GizmoCancel
                | Action::GizmoNumber(_)
                | Action::BeginTool(_)
                | Action::ToolPoint(_)
                | Action::ToolScreen { .. }
                | Action::ToolHover(_)
                | Action::CancelTool
                | Action::Snap(_)
                | Action::Zoom(_)
                | Action::ZoomAt { .. }
                | Action::PanScreen { .. }
                | Action::Pan(_, _)
                | Action::Orbit(_, _)
                | Action::Fit
                | Action::FitSelected
                | Action::Projection(_)
                | Action::ResetView
                | Action::Background
                | Action::NormalView(_)
                | Action::Grid(_)
        );
        let mut change = self.apply_action(action)?;
        self.gizmo_hover = None;
        if !keep_gizmo && self.gizmo_edit.take().is_some() && change == Change::View {
            change = Change::Preview;
        }
        if !keep_edit && self.edit_preview.take().is_some() {
            self.draft = None;
            if change == Change::View {
                change = Change::Preview;
            }
        }
        if change == Change::Scene {
            self.snap_index = None;
        }
        Ok(change)
    }

    fn apply_action(&mut self, action: Action) -> Result<Change, &'static str> {
        if matches!(
            &action,
            Action::Close
                | Action::Undo
                | Action::Redo
                | Action::Adopt { .. }
                | Action::Live(_)
                | Action::Import(_)
                | Action::Replace(_)
                | Action::ImportAt(_, _)
                | Action::ReplaceAt(_, _)
        ) {
            self.draft = None;
        }
        match action {
            Action::LayerEdit { key, edit } => {
                if matches!(edit, crate::layer_edit::Edit::Current) {
                    crate::layer_edit::apply(&mut self.scene, &key, edit)?;
                    return Ok(Change::View);
                }
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        crate::layer_edit::apply(scene, &key, edit)
                    })?;
                self.repair_selection();
            }
            Action::LayerProperty { key, property } => {
                crate::layer_properties::apply(&mut self.scene, &key, property)?;
                self.repair_selection();
                if self
                    .selected
                    .is_some_and(|id| self.scene.pick_id(id.value()).is_none())
                {
                    self.selected = None;
                    self.selected_part = None;
                }
                self.snap_index = None;
                return Ok(Change::Preview);
            }
            Action::Hide => {
                let ids = self
                    .scene
                    .with_descendants(&self.selected_objects());
                std::rc::Rc::make_mut(&mut self.scene.view).hide(&ids, true);
                self.select_objects([], false);
                self.snap_index = None;
                return Ok(Change::Preview);
            }
            Action::Show => {
                std::rc::Rc::make_mut(&mut self.scene.view).show_all();
                self.snap_index = None;
                return Ok(Change::Preview);
            }
            Action::TreePick { ids, additive } => {
                let eligible = ids
                    .into_iter()
                    .filter(|id| self.scene.pick_id(id.value()) == Some(*id))
                    .collect::<Vec<_>>();
                if eligible.is_empty() {
                    return Err("This tree row has no visible editable objects");
                }
                self.select_objects(eligible, additive);
                return self.apply(Action::FitSelected);
            }
            Action::GizmoTouch {
                pointer,
                screen,
                size,
            } => return self.gizmo_press_radius(pointer, screen, size, false, 18.),
            Action::GizmoPress {
                pointer,
                screen,
                size,
                modified,
            } => return self.gizmo_press(pointer, screen, size, modified),
            Action::GizmoMove {
                pointer,
                screen,
                size,
                shift,
            } => return self.gizmo_move(pointer, screen, size, shift),
            Action::GizmoRelease {
                pointer,
                screen,
                size,
                shift,
            } => return self.gizmo_release(pointer, screen, size, shift),
            Action::GizmoCancel => {
                self.gizmo_edit = None;
                self.tool_status = "Handle edit cancelled.".into();
                return Ok(Change::Preview);
            }
            Action::GizmoNumber(text) => return self.gizmo_number(&text),
            Action::Snap(setting) => {
                self.tool_status = self.snapping.apply(setting)?;
                if let Some(draft) = self.draft.as_mut() {
                    draft.cursor = None;
                    draft.snapped = None;
                }
                return self.update_edit_preview();
            }
            Action::ToolScreen {
                screen,
                size,
                commit,
            } => {
                let hit = self.snap_point(screen, size);
                if let Some(draft) = self.draft.as_mut() {
                    draft.snapped = hit
                        .filter(|h| {
                            h.owner.is_some() || h.kind == crate::snap::Kind::Grid
                        })
                        .map(|h| h.kind);
                }
                if commit {
                    return hit.map_or(Ok(Change::View), |hit| {
                        self.place_tool_point(hit.point)
                    });
                }
                if let Some(draft) = self.draft.as_mut() {
                    draft.hover(hit.map(|h| h.point))?;
                }
                return self.update_edit_preview();
            }
            Action::BeginTool(shape) => {
                let preview = if matches!(shape, crate::tool::Shape::Move) {
                    if self.selected_part.is_some() {
                        return Err("Use Select Object before Move");
                    }
                    Some(crate::edit_preview::Preview::new(
                        &self.scene,
                        self.selected_objects(),
                    )?)
                } else {
                    None
                };
                let changed = preview.is_some() || self.edit_preview.is_some();
                self.edit_preview = preview;
                self.snap_index = None;
                self.draft = Some(crate::tool::Draft::new(shape));
                self.tool_status = self.draft.as_ref().unwrap().prompt();
                return Ok(if changed {
                    Change::Preview
                } else {
                    Change::View
                });
            }
            Action::ToolPoint(point) => return self.place_tool_point(point),
            Action::ToolHover(point) => {
                if let Some(draft) = self.draft.as_mut() {
                    draft.snapped = None;
                    draft.hover(point)?;
                }
                return self.update_edit_preview();
            }
            Action::CancelTool => {
                let changed = self.edit_preview.take().is_some();
                self.draft = None;
                self.tool_status = "Tool cancelled.".into();
                return Ok(if changed {
                    Change::Preview
                } else {
                    Change::View
                });
            }
            Action::Picked(id) => self.select_objects(id, false),
            Action::PickedMany { target, additive } => {
                if self.pick_mode != crate::pick_sources::Mode::Object {
                    return self.apply(Action::PickedTarget(target));
                }
                self.select_objects(
                    target
                        .filter(|t| {
                            t.part == crate::pick_sources::Part::Object
                                && t.valid(&self.scene)
                        })
                        .map(|t| t.owner),
                    additive,
                );
            }
            Action::SelectAll => {
                let ids: Vec<_> = self
                    .scene
                    .objects()
                    .iter()
                    .map(|r| r.id)
                    .chain(self.scene.lines().iter().map(|r| r.id))
                    .chain(self.scene.paths().iter().map(|r| r.id))
                    .chain(self.scene.points().iter().map(|r| r.id))
                    .chain(self.scene.texts().iter().map(|r| r.id))
                    .chain(self.scene.clouds().iter().map(|r| r.id))
                    .collect();
                self.select_objects(ids, false);
                return Ok(Change::View);
            }
            Action::PickedTarget(target) => {
                let target = target.filter(|target| target.valid(&self.scene));
                self.objects.clear();
                self.selected = target.map(|target| target.owner);
                if let Some(target) =
                    target.filter(|t| t.part == crate::pick_sources::Part::Object)
                {
                    self.objects
                        .set(&self.scene, [target.owner], false);
                }
                self.selected_part = target
                    .map(|target| target.part)
                    .filter(|part| *part != crate::pick_sources::Part::Object);
            }
            Action::PickMode(mode) => {
                if mode != crate::pick_sources::Mode::Control {
                    self.scene.cloud_controls(None);
                }
                self.pick_mode = mode;
                self.selected = None;
                self.selected_part = None;
                self.objects.clear();
            }
            Action::Pick(screen) => {
                let id = self
                    .camera
                    .ray(screen)
                    .and_then(|ray| crate::picking::pick(&self.scene, &ray));
                self.select_objects(id, false);
            }
            Action::UnloadSources => self.unload_sources()?,
            Action::Close => {
                self.scene.clear();
                self.history.clear();
                self.objects.clear();
                self.selected = None;
            }
            Action::AddText(placement) => {
                let source = std::rc::Rc::new(crate::scene_label::example(placement));
                let prepared = crate::scene_label::Prepared::new(source)?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_text(prepared).map(|_| ())
                    })?;
            }
            Action::AddCloudLod => {
                let source = std::rc::Rc::new(crate::cloud::large_example());
                let prepared = crate::cloud::Prepared::new(source)?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_cloud(prepared).map(|_| ())
                    })?;
            }
            Action::AddCloud => {
                let source = std::rc::Rc::new(crate::cloud::example());
                let prepared = crate::cloud::Prepared::new(source)?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_cloud(prepared).map(|_| ())
                    })?;
            }
            Action::AddCad => {
                let source = std::rc::Rc::new(crate::cad_display::example());
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_cad(source).map(|_| ())
                    })?;
            }
            Action::AddSurface => {
                let source = std::rc::Rc::new(crate::surface::example());
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_surface(source).map(|_| ())
                    })?;
            }
            Action::AddNested => {
                let loaded =
                    crate::document::load(&crate::tree_specimen::nested(true))?;
                self.history
                    .try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::Live(ready) => {
                self.history.clear();
                self.scene.restore(*ready);
            }
            Action::Adopt { scene: ready, .. } => {
                self.history.edit(&mut self.scene, |scene| {
                    scene.restore(*ready)
                });
            }
            Action::Replace(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.replace(loaded)
                    })?;
            }
            Action::Import(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history
                    .try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ImportAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history
                    .try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ReplaceAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.replace(loaded)
                    })?;
            }
            Action::AddPolyline | Action::AddArrow => {
                let prepared = if matches!(action, Action::AddPolyline) {
                    let mut line = session_rust::Polyline::new(vec![
                        session_rust::Point::new(-0.6, -0.4, 0.0),
                        session_rust::Point::new(0.0, 0.0, 0.0),
                        session_rust::Point::new(0.6, -0.4, 0.0),
                    ]);
                    line.width = 6.0;
                    line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                    crate::chain::PreparedChain::polyline(std::rc::Rc::new(line))?
                } else {
                    let mut line =
                        session_rust::Line::new(-0.6, 0.3, 0.0, 0.6, 0.3, 0.0);
                    line.width = 5.0;
                    line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                    line.arrowhead = session_rust::Arrowhead::BOTH;
                    crate::chain::PreparedChain::line(std::rc::Rc::new(line))?
                };
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_path(prepared).map(|_| ())
                    })?;
            }
            Action::Controls(visible) => {
                let parent = if visible {
                    self.selected.filter(|id| {
                        self.scene
                            .clouds()
                            .iter()
                            .any(|r| r.id == *id)
                    })
                } else {
                    None
                };
                if parent.is_some()
                    && self.pick_mode == crate::pick_sources::Mode::Control
                    && self
                        .scene
                        .clouds()
                        .iter()
                        .any(|r| Some(r.id) == parent && r.controls_visible)
                {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub enum Action {
    StreamRefresh,
    CanvasStart {
        from: [f32; 2],
        to: [f32; 2],
        size: [f64; 2],
        modified: bool,
        pointer: i32,
    },
    CanvasMove {
        screen: [f32; 2],
        size: [f64; 2],
    },
    CanvasRelease {
        screen: [f32; 2],
        size: [f64; 2],
        add: bool,
        remove: bool,
    },
    CanvasHit(Option<ObjectId>),
    CanvasCancel,
    SelectRows {
        ids: Vec<ObjectId>,
        add: bool,
        remove: bool,
    },
    PanScreen {
        delta: [f64; 2],
        size: [f64; 2],
    },
    ZoomAt {
        factor: f32,
        screen: [f32; 2],
    },
    Touch(Vec<Action>),
    LayerEdit {
        key: String,
        edit: crate::layer_edit::Edit,
    },
    LayerProperty {
        key: String,
        property: crate::layer_properties::Property,
    },
    TreePick {
        ids: Vec<ObjectId>,
        additive: bool,
    },
    GizmoPress {
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        modified: bool,
    },
    GizmoTouch {
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
    },
    GizmoMove {
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        shift: bool,
    },
    GizmoRelease {
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        shift: bool,
    },
    GizmoCancel,
    GizmoNumber(String),
    Snap(crate::snap::Setting),
    ToolScreen {
        screen: [f32; 2],
        size: [f64; 2],
        commit: bool,
    },
    BeginTool(crate::tool::Shape),
    ToolPoint([f64; 3]),
    ToolHover(Option<[f64; 3]>),
    CancelTool,
    Pick([f32; 2]),
    Picked(Option<ObjectId>),
    PickedTarget(Option<crate::pick_sources::Target>),
    PickedMany {
        target: Option<crate::pick_sources::Target>,
        additive: bool,
    },
    SelectAll,
    PickMode(crate::pick_sources::Mode),
    Live(Box<Scene>),
    Adopt {
        scene: Box<Scene>,
        replace: bool,
    },
    Import(Vec<u8>),
    Replace(Vec<u8>),
    ImportAt(
        Vec<u8>,
        std::rc::Rc<crate::reload_url::ReloadUrl>,
    ),
    ReplaceAt(
        Vec<u8>,
        std::rc::Rc<crate::reload_url::ReloadUrl>,
    ),
    Close,
    UnloadSources,
    Hide,
    Show,
    AddBox,
    AddNested,
    AddCloudLod,
    AddCloud,
    AddCad,
    AddText(crate::text::TextPlacement),
    AddSurface,
    AddLine,
    AddPoint,
    AddCurve,
    AddColours(crate::colour::Mode),
    NormalView(bool),
    Grid(Option<crate::grid::Settings>),
    AddNormals(crate::normal_example::Mode),
    Controls(bool),
    AddPolyline,
    AddArrow,
    Translate([f64; 3]),
    ToggleExtra,
    SelectNext,
    Delete,
    Undo,
    Redo,
    Background,
    Zoom(f32),
    Pan(f32, f32),
    Orbit(f64, f64),
    Isometric,
    ResetView,
    Fit,
    FitSelected,
    Projection(crate::camera::Projection),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Change {
    View,
    Scene,
    Preview,
}

pub struct Editor {
    pub canvas_drag: Option<crate::canvas_drag::Drag>,
    pub gizmo_edit: Option<crate::gizmo_edit::Edit>,
    gizmo_hover: Option<crate::gizmo::Handle>,
    pub edit_preview: Option<crate::edit_preview::Preview>,
    pub snapping: crate::snap::Settings,
    snap_index: Option<crate::snap_index::Index>,
    pub draft: Option<crate::tool::Draft>,
    pub tool_status: String,
    pub scene: Scene,
    pub selected: Option<ObjectId>,
    pub selected_part: Option<crate::pick_sources::Part>,
    pub pick_mode: crate::pick_sources::Mode,
    objects: crate::object_selection::Objects,
    pub camera: Camera,
    pub background: Background,
    pub normal_view: bool,
    history: History,
    last_release: u64,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            canvas_drag: None,
            gizmo_edit: None,
            gizmo_hover: None,
            edit_preview: None,
            snapping: Default::default(),
            snap_index: None,
            draft: None,
            tool_status: String::new(),
            scene: Scene::demo(),
            selected: None,
            selected_part: None,
            pick_mode: Default::default(),
            objects: Default::default(),
            camera: Camera::default(),
            background: Background::default(),
            normal_view: false,
            history: History::default(),
            last_release: 0,
        }
    }
}

impl Editor {
    pub fn display_scene(&self) -> &Scene {
        self.canvas_drag
            .as_ref()
            .and_then(|d| d.preview.as_ref())
            .map_or_else(
                || {
                    self.gizmo_edit.as_ref().map_or_else(
                        || {
                            self.edit_preview
                                .as_ref()
                                .map_or(&self.scene, |p| &p.scene)
                        },
                        |g| g.preview.scene(),
                    )
                },
                |p| &p.scene,
            )
    }

    pub fn gizmo_origin(&self) -> Option<[f64; 3]> {
        if self.draft.is_some() || self.canvas_drag.is_some() {
            return None;
        }
        if self
            .scene
            .clouds()
            .iter()
            .any(|r| r.controls_visible)
            || self
                .scene
                .streams()
                .iter()
                .any(|r| r.controls_visible)
        {
            return None;
        }
        if let Some(edit) = self.gizmo_edit.as_ref() {
            return Some(edit.origin());
        }
        if let Some(target) = self
            .selection()
            .filter(|_| self.selected_part.is_some())
        {
            return crate::part_preview::origin(&self.scene, target);
        }
        self.scene
            .bounds_of(&self.selected_objects())
            .map(|b| b.centre())
    }

    pub fn gizmo_hit(
        &self,
        screen: [f32; 2],
        size: [f64; 2],
    ) -> Option<crate::gizmo::Handle> {
        self.gizmo_hit_radius(screen, size, 8.)
    }

    pub fn gizmo_hit_radius(
        &self,
        screen: [f32; 2],
        size: [f64; 2],
        radius: f64,
    ) -> Option<crate::gizmo::Handle> {
        if size
            .iter()
            .any(|v| !v.is_finite() || *v <= 0.)
        {
            return None;
        }
        let origin = self.gizmo_origin()?;
        let scale = crate::widget_layout::world_per_pixel(
            &self.camera.view_projection(),
            origin,
            size[1],
        )?;
        let ray = self.camera.ray(screen)?;
        crate::gizmo::Gizmo::new(session_rust::Point::new(
            origin[0], origin[1], origin[2],
        ))
        .hit_with_radius(&ray.origin, &ray.direction, scale, radius)
    }

    pub fn number_prompt(
        &self,
        size: [f64; 2],
    ) -> Option<crate::number_box::NumberPrompt> {
        let edit = self.gizmo_edit.as_ref()?;
        let handle = edit.gizmo.typing?;
        let origin = edit.origin();
        let matrix = self.camera.view_projection();
        let scale = crate::widget_layout::world_per_pixel(&matrix, origin, size[1])?;
        let point = edit.gizmo.handle_point(handle, scale);
        let at = crate::snap_math::Screen {
            matrix: matrix.m,
            size,
        }
        .point([point[0], point[1], point[2]])?;
        let unit = handle.labels().2;
        Some(crate::number_box::NumberPrompt {
            handle,
            title: handle.title(),
            unit,
            hint: if unit == "factor" {
                "1"
            } else {
                "0"
            },
            at: at.map(|v| v as f32),
        })
    }

    pub fn gizmo_active(&self) -> f32 {
        crate::gizmo_edit::code(
            self.gizmo_edit
                .as_ref()
                .and_then(|e| e.active())
                .or(self.gizmo_hover),
        )
    }

    pub fn hover_gizmo(&mut self, screen: Option<([f32; 2], [f64; 2])>) {
        self.gizmo_hover = if self.gizmo_edit.is_some() {
            None
        } else {
            screen.and_then(|(point, size)| self.gizmo_hit(point, size))
        };
    }

    fn gizmo_press(
        &mut self,
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        modified: bool,
    ) -> Result<Change, &'static str> {
        self.gizmo_press_radius(pointer, screen, size, modified, 8.)
    }

    fn gizmo_press_radius(
        &mut self,
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        modified: bool,
        radius: f64,
    ) -> Result<Change, &'static str> {
        if modified
            || self.draft.is_some()
            || self
                .gizmo_edit
                .as_ref()
                .is_some_and(|g| g.pointer.is_some())
        {
            return Ok(Change::View);
        }
        let Some(handle) = self.gizmo_hit_radius(screen, size, radius) else {
            return Ok(Change::View);
        };
        let origin = self
            .gizmo_origin()
            .ok_or("No editable selection")?;
        let preview = crate::gizmo_edit::Preview::new(
            &self.scene,
            self.selected_objects(),
            self.selection()
                .filter(|_| self.selected_part.is_some()),
        )?;
        let ray = self
            .camera
            .ray(screen)
            .ok_or("Invalid handle ray")?;
        let gizmo = crate::gizmo::Gizmo::new(session_rust::Point::new(
            origin[0], origin[1], origin[2],
        ));
        self.gizmo_edit = crate::gizmo_edit::Edit::begin(
            gizmo, handle, &ray, preview, pointer, screen,
        );
        Ok(Change::View)
    }

    fn gizmo_move(
        &mut self,
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        shift: bool,
    ) -> Result<Change, &'static str> {
        if size
            .iter()
            .any(|v| !v.is_finite() || *v <= 0.)
        {
            return Err("Invalid handle viewport");
        }
        let Some(ray) = self.camera.ray(screen) else {
            return Err("Invalid handle ray");
        };
        let Some(edit) = self.gizmo_edit.as_mut() else {
            return Ok(Change::View);
        };
        Ok(
            if edit.update(pointer, screen, size, &ray, shift)? {
                Change::Preview
            } else {
                Change::View
            },
        )
    }

    fn gizmo_release(
        &mut self,
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        shift: bool,
    ) -> Result<Change, &'static str> {
        if self
            .gizmo_edit
            .as_ref()
            .is_none_or(|g| g.pointer != Some(pointer))
        {
            return Ok(Change::View);
        }
        self.gizmo_move(pointer, screen, size, shift)?;
        let edit = self.gizmo_edit.as_mut().unwrap();
        if edit.click() {
            self.tool_status = format!(
                "{}: type a value in {}, Enter applies, Esc closes",
                edit.active().unwrap().title(),
                edit.active().unwrap().labels().2
            );
            return Ok(Change::Preview);
        }
        let changed = edit
            .preview
            .commit(&mut self.scene, &mut self.history)?;
        self.gizmo_edit = None;
        self.tool_status = "Handle edit completed.".into();
        Ok(if changed {
            Change::Scene
        } else {
            Change::Preview
        })
    }

    fn gizmo_number(&mut self, text: &str) -> Result<Change, &'static str> {
        let edit = self
            .gizmo_edit
            .as_mut()
            .ok_or("No handle number is being requested")?;
        let handle = edit
            .gizmo
            .typing
            .ok_or("No handle number is being requested")?;
        let value = crate::gizmo::typed_value(handle, text)
            .map_err(|_| "Enter a finite number; scale needs a positive factor")?;
        let Some(value) = value else {
            self.gizmo_edit = None;
            return Ok(Change::Preview);
        };
        edit.preview
            .set(edit.gizmo.typed(handle, value))?;
        let changed = edit
            .preview
            .commit(&mut self.scene, &mut self.history)?;
        self.gizmo_edit = None;
        self.tool_status = format!(
            "{} {value} {}",
            handle.title(),
            handle.labels().2
        );
        Ok(if changed {
            Change::Scene
        } else {
            Change::Preview
        })
    }

    fn update_edit_preview(&mut self) -> Result<Change, &'static str> {
        let Some(preview) = self.edit_preview.as_mut() else {
            return Ok(Change::View);
        };
        let offset = self
            .draft
            .as_ref()
            .and_then(|d| d.points.first().zip(d.cursor))
            .map(|(a, b)| std::array::from_fn::<_, 3, _>(|i| b[i] - a[i]))
            .unwrap_or([0.; 3]);
        preview.set(session_rust::Xform::translation(
            offset[0], offset[1], offset[2],
        ))?;
        Ok(Change::Preview)
    }

    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.scene).chain(self.history.scenes())
    }

    pub fn presentation_scenes(&self) -> impl Iterator<Item = &Scene> {
        self.scenes()
            .chain(
                self.canvas_drag
                    .iter()
                    .filter_map(|d| d.preview.as_ref())
                    .flat_map(|p| p.scenes()),
            )
            .chain(
                self.edit_preview
                    .iter()
                    .flat_map(|p| p.scenes()),
            )
            .chain(
                self.gizmo_edit
                    .iter()
                    .flat_map(|g| g.preview.scenes()),
            )
    }

    pub fn reload_keys(&self) -> Vec<crate::rehydrate::ReloadKey> {
        let mut seen = std::collections::HashSet::new();
        self.scene
            .objects()
            .iter()
            .filter_map(crate::rehydrate::ReloadKey::of)
            .filter(|key| seen.insert(key.origin.id))
            .collect()
    }

    fn scenes_mut(&mut self) -> impl Iterator<Item = &mut Scene> {
        std::iter::once(&mut self.scene).chain(self.history.scenes_mut())
    }

    pub fn hydrate(
        &mut self,
        values: Vec<(crate::rehydrate::ReloadKey, Vec<u8>)>,
    ) -> Result<bool, &'static str> {
        if values.is_empty() {
            return Ok(false);
        }
        let mut seen = std::collections::HashSet::new();
        for (key, _) in &values {
            if !seen.insert(key.origin.id)
                || !self
                    .scene
                    .objects()
                    .iter()
                    .any(|row| key.matches(row))
            {
                return Ok(false);
            }
        }
        let prepared: Vec<_> = values
            .into_iter()
            .map(|(key, bytes)| crate::rehydrate::PreparedReload::new(key, &bytes))
            .collect::<Result<_, _>>()?;
        for plan in &prepared {
            if self
                .scenes()
                .flat_map(|scene| scene.objects())
                .filter(|row| plan.key.matches(row))
                .any(|row| !plan.validates(row))
            {
                return Err("Reloaded source GUID or metadata does not match");
            }
        }
        for scene in self.scenes_mut() {
            for plan in &prepared {
                scene.hydrate(plan);
            }
        }
        if self.edit_preview.take().is_some() {
            self.draft = None;
        }
        self.gizmo_edit = None;
        self.snap_index = None;
        Ok(true)
    }

    fn unload_sources(&mut self) -> Result<(), &'static str> {
        let mut origins: std::collections::HashSet<_> = self
            .scenes()
            .flat_map(|scene| scene.objects())
            .filter(|row| row.can_unload())
            .filter_map(|row| row.origin().map(|origin| origin.id))
            .collect();
        for row in self
            .scenes()
            .flat_map(|scene| scene.objects())
        {
            if let Some(origin) = row.origin() {
                if !row.can_unload() {
                    origins.remove(&origin.id);
                }
            }
        }
        if origins.is_empty() {
            return Err("No eligible imported sources to unload");
        }
        let epoch = self
            .last_release
            .checked_add(1)
            .ok_or("Release epochs exhausted")?;
        for scene in self.scenes_mut() {
            scene.unload(&origins, epoch);
        }
        self.last_release = epoch;
        Ok(())
    }

    pub fn move_object(
        &mut self,
        id: ObjectId,
        offset: [f64; 3],
    ) -> Result<Change, &'static str> {
        if !offset.iter().all(|value| value.is_finite()) {
            return Err("Move needs finite coordinates");
        }
        if !self.scene.contains(id) {
            return Err("Object not found");
        }
        let object = self
            .scene
            .objects()
            .iter()
            .find(|row| row.id == id);
        if self.scene.forest.readonly(id) {
            return Err("Object attributes follow their parent and are read only");
        }
        if object.is_some_and(|object| object.geometry().is_none()) {
            return Err("Reload editable sources before Move");
        }
        if offset.iter().all(|&value| value == 0.0) {
            return Ok(Change::View);
        }
        let ids = self.scene.with_descendants(&[id]);
        let delta = session_rust::Xform::translation(offset[0], offset[1], offset[2]);
        self.history
            .try_edit(&mut self.scene, |scene| {
                for id in ids {
                    let matrix = &delta
                        * &scene
                            .placement(id)
                            .ok_or("Object not found")?;
                    scene.place_following(id, matrix)?;
                }
                Ok(())
            })?;
        if self.edit_preview.take().is_some() {
            self.draft = None;
        }
        self.gizmo_edit = None;
        self.snap_index = None;
        Ok(Change::Scene)
    }

    pub fn delete_object(&mut self, id: ObjectId) -> Result<Change, &'static str> {
        if !self.scene.contains(id) {
            return Err("Object not found");
        }
        if self.scene.forest.readonly(id) {
            return Err("Object attributes follow their parent and are read only");
        }
        if self
            .scene
            .objects()
            .iter()
            .find(|row| row.id == id)
            .is_some_and(|row| row.geometry().is_none())
        {
            return Err("Reload editable sources before Delete");
        }
        let ids = self.scene.with_descendants(&[id]);
        self.history.edit(&mut self.scene, |scene| {
            for id in ids {
                scene.remove(id);
            }
        });
        if self.edit_preview.take().is_some() {
            self.draft = None;
        }
        self.objects.retain(&self.scene);
        if self.selected == Some(id) {
            self.selected = self.objects.primary();
            self.selected_part = None;
        }
        self.gizmo_edit = None;
        self.snap_index = None;
        Ok(Change::Scene)
    }

    pub fn selected_objects(&self) -> Vec<ObjectId> {
        if self.selected_part.is_some() {
            return Vec::new();
        }
        if self.objects.primary() == self.selected {
            self.objects.rows().to_vec()
        } else {
            self.selected
                .into_iter()
                .filter(|id| self.scene.pick_id(id.value()).is_some())
                .collect()
        }
    }

    pub fn select_objects(
        &mut self,
        rows: impl IntoIterator<Item = ObjectId>,
        additive: bool,
    ) {
        if additive {
            let old = self.selected_objects();
            self.objects.set(&self.scene, old, false);
        }
        self.selected_part = None;
        self.objects.set(&self.scene, rows, additive);
        self.selected = self.objects.primary();
    }

    pub fn select_rows(&mut self, ids: Vec<ObjectId>, add: bool, remove: bool) {
        if remove {
            let old = self.selected_objects();
            self.objects.set(&self.scene, old, false);
            self.objects.remove(&self.scene, ids);
            self.selected = self.objects.primary();
            self.selected_part = None;
        } else {
            self.select_objects(ids, add);
        }
    }

    fn finish_canvas(&mut self) -> Result<Change, &'static str> {
        let Some(mut drag) = self.canvas_drag.take() else {
            return Ok(Change::View);
        };
        let (add, remove) = drag.released.unwrap_or((false, false));
        if let Some(preview) = drag.preview.take() {
            let changed = preview.commit(&mut self.scene, &mut self.history)?;
            if changed {
                self.snap_index = None;
            }
            return Ok(if changed {
                Change::Scene
            } else {
                Change::Preview
            });
        }
        if drag.missed {
            if let Some(rectangle) = drag.rectangle() {
                let ids = crate::rect_select::found(
                    &self.scene,
                    &self.camera.view_projection().m,
                    drag.size,
                    rectangle,
                );
                self.select_rows(ids, add, remove);
            }
        }
        Ok(Change::View)
    }

    fn repair_selection(&mut self) {
        if self.objects.primary() == self.selected && self.selected_part.is_none() {
            self.objects.retain(&self.scene);
            self.selected = self.objects.primary();
        } else {
            self.objects.clear();
        }
    }

    fn move_selection(&mut self, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|n| n.is_finite()) {
            return Err("Move needs finite coordinates");
        }
        let ids = self.selected_objects();
        if ids.is_empty() {
            return Err("Select an object before Move");
        }
        if offset.iter().all(|&n| n == 0.) {
            return Ok(Change::View);
        }
        let mut preview = crate::edit_preview::Preview::new(&self.scene, ids)?;
        preview.set(session_rust::Xform::translation(
            offset[0], offset[1], offset[2],
        ))?;
        preview.commit(&mut self.scene, &mut self.history)?;
        Ok(Change::Scene)
    }

    fn delete_selection(&mut self) -> Result<Change, &'static str> {
        if self.selected_part.is_some() {
            return Err("Use Select Object before Delete");
        }
        let ids = self.selected_objects();
        if ids.is_empty() {
            return Ok(Change::View);
        }
        if self
            .scene
            .objects()
            .iter()
            .any(|r| ids.contains(&r.id) && r.geometry().is_none())
        {
            return Err("Reload editable sources before Delete");
        }
        let ids = self.scene.with_descendants(&ids);
        self.history.edit(&mut self.scene, |scene| {
            for id in ids {
                scene.remove(id);
            }
        });
        self.objects.clear();
        self.selected = None;
        Ok(Change::Scene)
    }

    pub fn selection(&self) -> Option<crate::pick_sources::Target> {
        self.selected
            .map(|owner| crate::pick_sources::Target {
                owner,
                part: self
                    .selected_part
                    .unwrap_or(crate::pick_sources::Part::Object),
            })
    }

    pub fn whole_selection(&self) -> Option<ObjectId> {
        self.selected
            .filter(|_| self.selected_part.is_none())
    }

    pub fn tool_line(&self, text: &str) -> Option<Result<Action, &'static str>> {
        if self.gizmo_edit.as_ref().is_some_and(|e| {
            e.gizmo.typing.is_some() || text.trim().eq_ignore_ascii_case("Escape")
        }) {
            return Some(Ok(
                if text.trim().eq_ignore_ascii_case("Escape") {
                    Action::GizmoCancel
                } else {
                    Action::GizmoNumber(text.to_owned())
                },
            ));
        }
        self.draft.as_ref()?;
        if text.trim().is_empty() || text.eq_ignore_ascii_case("Escape") {
            return Some(Ok(Action::CancelTool));
        }
        text.contains(',')
            .then(|| crate::tool_coords::parse(text).map(Action::ToolPoint))
    }

    fn place_tool_point(&mut self, point: [f64; 3]) -> Result<Change, &'static str> {
        let draft = self
            .draft
            .as_mut()
            .ok_or("No point tool is running")?;
        let name = draft.tool.name();
        let Some(built) = draft.submit(point)? else {
            self.tool_status = draft.prompt();
            self.snap_index = None;
            return self.update_edit_preview();
        };
        if let crate::tool::Built::Move(offset) = built {
            let preview = self
                .edit_preview
                .as_mut()
                .ok_or("No placement preview is running")?;
            preview.set(session_rust::Xform::translation(
                offset[0], offset[1], offset[2],
            ))?;
            let changed = preview.commit(&mut self.scene, &mut self.history)?;
            if self.edit_preview.take().is_some() {
                self.draft = None;
            }
            self.tool_status = "Move completed.".into();
            return Ok(if changed {
                Change::Scene
            } else {
                Change::Preview
            });
        }
        self.history
            .try_edit(&mut self.scene, |scene| match built {
                crate::tool::Built::Point(point) => {
                    scene.insert_point(point).map(|_| ())
                }
                crate::tool::Built::Line(line) => scene.insert_line(line).map(|_| ()),
                crate::tool::Built::Move(_) => {
                    unreachable!("Move uses its placement preview")
                }
            })?;
        self.draft = None;
        self.tool_status = format!("{name} created.");
        Ok(Change::Scene)
    }

    pub fn snap_point(
        &mut self,
        screen: [f32; 2],
        size: [f64; 2],
    ) -> Option<crate::snap::Hit> {
        let ray = self.camera.ray(screen)?;
        let from = self.draft.as_ref()?.points.last().copied();
        let hit = if self.snapping.objects {
            let exclude = self
                .edit_preview
                .as_ref()
                .filter(|_| from.is_some())
                .map_or_else(Vec::new, |p| p.ids().to_vec());
            let index = self.snap_index.get_or_insert_with(|| {
                crate::snap_index::Index::new(&self.scene).excluding(&exclude)
            });
            let viewport = crate::snap_math::Screen {
                matrix: self.camera.view_projection().m,
                size,
            };
            index.query(
                viewport,
                &ray,
                [
                    (screen[0] as f64 + 1.) * size[0] / 2.,
                    (1. - screen[1] as f64) * size[1] / 2.,
                ],
                self.snapping.modes,
                from,
            )
        } else {
            None
        };
        hit.or_else(|| crate::snap_index::fallback(self.snapping, &self.camera, screen))
    }

    pub fn snap_usage(&self) -> [u64; 2] {
        self.snap_index
            .as_ref()
            .map_or([0, 0], |index| {
                [index.builds, index.visits as u64]
            })
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::StreamRefresh => {
                self.snap_index = None;
                return Ok(Change::Scene);
            }
            Action::Touch(actions) => {
                let mut change = Change::View;
                for action in actions {
                    let next = self.apply(action)?;
                    if next != Change::View {
                        change = next;
                    }
                }
                return Ok(change);
            }
            Action::CanvasStart {
                from,
                to,
                size,
                modified,
                pointer,
            } => {
                self.canvas_drag =
                    crate::canvas_drag::Drag::new(from, to, size, modified);
                if let Some(drag) = self.canvas_drag.as_mut() {
                    drag.pointer = pointer;
                }
                return Ok(Change::View);
            }
            Action::CanvasMove { screen, size } => {
                if let Some(drag) = self.canvas_drag.as_mut() {
                    if drag.released.is_none()
                        && drag.size == size
                        && screen.iter().all(|v| v.is_finite())
                    {
                        drag.to = screen;
                        drag.update(&self.camera)?;
                    }
                    if drag.preview.is_some() {
                        return Ok(Change::Preview);
                    }
                }
                return Ok(Change::View);
            }
            Action::CanvasRelease {
                screen,
                size,
                add,
                remove,
            } => {
                if let Some(drag) = self.canvas_drag.as_mut() {
                    if drag.size != size {
                        self.canvas_drag = None;
                        return Ok(Change::Preview);
                    }
                    drag.to = screen;
                    drag.released = Some((add, remove));
                    drag.update(&self.camera)?;
                    if drag.preview.is_some() || drag.missed {
                        return self.finish_canvas();
                    }
                }
                return Ok(Change::View);
            }
            Action::CanvasHit(hit) => {
                let Some(drag) = self.canvas_drag.as_ref() else {
                    return Ok(Change::View);
                };
                let ids = hit.filter(|id| {
                    !drag.modified && self.scene.pick_id(id.value()).is_some()
                });
                if let Some(id) = ids {
                    let mut rows = self.selected_objects();
                    if !rows.contains(&id) {
                        self.select_objects([id], false);
                        rows = vec![id];
                    }
                    if let Err(error) = self.canvas_drag.as_mut().unwrap().grab(
                        &self.scene,
                        rows,
                        &self.camera,
                    ) {
                        self.canvas_drag = None;
                        return Err(error);
                    }
                } else {
                    self.canvas_drag.as_mut().unwrap().missed = true;
                }
                if self
                    .canvas_drag
                    .as_ref()
                    .unwrap()
                    .released
                    .is_some()
                {
                    return self.finish_canvas();
                }
                return Ok(Change::Preview);
            }
            Action::CanvasCancel => {
                let preview = self
                    .canvas_drag
                    .take()
                    .is_some_and(|d| d.preview.is_some());
                return Ok(if preview {
                    Change::Preview
                } else {
                    Change::View
                });
            }
            Action::SelectRows { ids, add, remove } => {
                self.select_rows(ids, add, remove);
                return Ok(Change::View);
            }
            _ => {}
        }
        self.canvas_drag = None;
        let keep_gizmo = matches!(
            &action,
            Action::GizmoPress { .. }
                | Action::GizmoTouch { .. }
                | Action::GizmoMove { .. }
                | Action::GizmoRelease { .. }
                | Action::GizmoCancel
                | Action::GizmoNumber(_)
                | Action::Zoom(_)
                | Action::ZoomAt { .. }
                | Action::PanScreen { .. }
                | Action::Pan(_, _)
                | Action::Orbit(_, _)
                | Action::Fit
                | Action::FitSelected
                | Action::Projection(_)
                | Action::ResetView
                | Action::Background
                | Action::NormalView(_)
                | Action::Grid(_)
                | Action::Snap(_)
        );
        let keep_edit = matches!(
            &action,
            Action::GizmoPress { .. }
                | Action::GizmoTouch { .. }
                | Action::GizmoMove { .. }
                | Action::GizmoRelease { .. }
                | Action::GizmoCancel
                | Action::GizmoNumber(_)
                | Action::BeginTool(_)
                | Action::ToolPoint(_)
                | Action::ToolScreen { .. }
                | Action::ToolHover(_)
                | Action::CancelTool
                | Action::Snap(_)
                | Action::Zoom(_)
                | Action::ZoomAt { .. }
                | Action::PanScreen { .. }
                | Action::Pan(_, _)
                | Action::Orbit(_, _)
                | Action::Fit
                | Action::FitSelected
                | Action::Projection(_)
                | Action::ResetView
                | Action::Background
                | Action::NormalView(_)
                | Action::Grid(_)
        );
        let mut change = self.apply_action(action)?;
        self.gizmo_hover = None;
        if !keep_gizmo && self.gizmo_edit.take().is_some() && change == Change::View {
            change = Change::Preview;
        }
        if !keep_edit && self.edit_preview.take().is_some() {
            self.draft = None;
            if change == Change::View {
                change = Change::Preview;
            }
        }
        if change == Change::Scene {
            self.snap_index = None;
        }
        Ok(change)
    }

    fn apply_action(&mut self, action: Action) -> Result<Change, &'static str> {
        if matches!(
            &action,
            Action::Close
                | Action::Undo
                | Action::Redo
                | Action::Adopt { .. }
                | Action::Live(_)
                | Action::Import(_)
                | Action::Replace(_)
                | Action::ImportAt(_, _)
                | Action::ReplaceAt(_, _)
        ) {
            self.draft = None;
        }
        match action {
            Action::LayerEdit { key, edit } => {
                if matches!(edit, crate::layer_edit::Edit::Current) {
                    crate::layer_edit::apply(&mut self.scene, &key, edit)?;
                    return Ok(Change::View);
                }
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        crate::layer_edit::apply(scene, &key, edit)
                    })?;
                self.repair_selection();
            }
            Action::LayerProperty { key, property } => {
                crate::layer_properties::apply(&mut self.scene, &key, property)?;
                self.repair_selection();
                if self
                    .selected
                    .is_some_and(|id| self.scene.pick_id(id.value()).is_none())
                {
                    self.selected = None;
                    self.selected_part = None;
                }
                self.snap_index = None;
                return Ok(Change::Preview);
            }
            Action::Hide => {
                let ids = self
                    .scene
                    .with_descendants(&self.selected_objects());
                std::rc::Rc::make_mut(&mut self.scene.view).hide(&ids, true);
                self.select_objects([], false);
                self.snap_index = None;
                return Ok(Change::Preview);
            }
            Action::Show => {
                std::rc::Rc::make_mut(&mut self.scene.view).show_all();
                self.snap_index = None;
                return Ok(Change::Preview);
            }
            Action::TreePick { ids, additive } => {
                let eligible = ids
                    .into_iter()
                    .filter(|id| self.scene.pick_id(id.value()) == Some(*id))
                    .collect::<Vec<_>>();
                if eligible.is_empty() {
                    return Err("This tree row has no visible editable objects");
                }
                self.select_objects(eligible, additive);
                return self.apply(Action::FitSelected);
            }
            Action::GizmoTouch {
                pointer,
                screen,
                size,
            } => return self.gizmo_press_radius(pointer, screen, size, false, 18.),
            Action::GizmoPress {
                pointer,
                screen,
                size,
                modified,
            } => return self.gizmo_press(pointer, screen, size, modified),
            Action::GizmoMove {
                pointer,
                screen,
                size,
                shift,
            } => return self.gizmo_move(pointer, screen, size, shift),
            Action::GizmoRelease {
                pointer,
                screen,
                size,
                shift,
            } => return self.gizmo_release(pointer, screen, size, shift),
            Action::GizmoCancel => {
                self.gizmo_edit = None;
                self.tool_status = "Handle edit cancelled.".into();
                return Ok(Change::Preview);
            }
            Action::GizmoNumber(text) => return self.gizmo_number(&text),
            Action::Snap(setting) => {
                self.tool_status = self.snapping.apply(setting)?;
                if let Some(draft) = self.draft.as_mut() {
                    draft.cursor = None;
                    draft.snapped = None;
                }
                return self.update_edit_preview();
            }
            Action::ToolScreen {
                screen,
                size,
                commit,
            } => {
                let hit = self.snap_point(screen, size);
                if let Some(draft) = self.draft.as_mut() {
                    draft.snapped = hit
                        .filter(|h| {
                            h.owner.is_some() || h.kind == crate::snap::Kind::Grid
                        })
                        .map(|h| h.kind);
                }
                if commit {
                    return hit.map_or(Ok(Change::View), |hit| {
                        self.place_tool_point(hit.point)
                    });
                }
                if let Some(draft) = self.draft.as_mut() {
                    draft.hover(hit.map(|h| h.point))?;
                }
                return self.update_edit_preview();
            }
            Action::BeginTool(shape) => {
                let preview = if matches!(shape, crate::tool::Shape::Move) {
                    if self.selected_part.is_some() {
                        return Err("Use Select Object before Move");
                    }
                    Some(crate::edit_preview::Preview::new(
                        &self.scene,
                        self.selected_objects(),
                    )?)
                } else {
                    None
                };
                let changed = preview.is_some() || self.edit_preview.is_some();
                self.edit_preview = preview;
                self.snap_index = None;
                self.draft = Some(crate::tool::Draft::new(shape));
                self.tool_status = self.draft.as_ref().unwrap().prompt();
                return Ok(if changed {
                    Change::Preview
                } else {
                    Change::View
                });
            }
            Action::ToolPoint(point) => return self.place_tool_point(point),
            Action::ToolHover(point) => {
                if let Some(draft) = self.draft.as_mut() {
                    draft.snapped = None;
                    draft.hover(point)?;
                }
                return self.update_edit_preview();
            }
            Action::CancelTool => {
                let changed = self.edit_preview.take().is_some();
                self.draft = None;
                self.tool_status = "Tool cancelled.".into();
                return Ok(if changed {
                    Change::Preview
                } else {
                    Change::View
                });
            }
            Action::Picked(id) => self.select_objects(id, false),
            Action::PickedMany { target, additive } => {
                if self.pick_mode != crate::pick_sources::Mode::Object {
                    return self.apply(Action::PickedTarget(target));
                }
                self.select_objects(
                    target
                        .filter(|t| {
                            t.part == crate::pick_sources::Part::Object
                                && t.valid(&self.scene)
                        })
                        .map(|t| t.owner),
                    additive,
                );
            }
            Action::SelectAll => {
                let ids: Vec<_> = self
                    .scene
                    .objects()
                    .iter()
                    .map(|r| r.id)
                    .chain(self.scene.lines().iter().map(|r| r.id))
                    .chain(self.scene.paths().iter().map(|r| r.id))
                    .chain(self.scene.points().iter().map(|r| r.id))
                    .chain(self.scene.texts().iter().map(|r| r.id))
                    .chain(self.scene.clouds().iter().map(|r| r.id))
                    .chain(self.scene.streams().iter().map(|r| r.id))
                    .collect();
                self.select_objects(ids, false);
                return Ok(Change::View);
            }
            Action::PickedTarget(target) => {
                let target = target.filter(|target| target.valid(&self.scene));
                self.objects.clear();
                self.selected = target.map(|target| target.owner);
                if let Some(target) =
                    target.filter(|t| t.part == crate::pick_sources::Part::Object)
                {
                    self.objects
                        .set(&self.scene, [target.owner], false);
                }
                self.selected_part = target
                    .map(|target| target.part)
                    .filter(|part| *part != crate::pick_sources::Part::Object);
            }
            Action::PickMode(mode) => {
                if mode != crate::pick_sources::Mode::Control {
                    self.scene.cloud_controls(None);
                }
                self.pick_mode = mode;
                self.selected = None;
                self.selected_part = None;
                self.objects.clear();
            }
            Action::Pick(screen) => {
                let id = self
                    .camera
                    .ray(screen)
                    .and_then(|ray| crate::picking::pick(&self.scene, &ray));
                self.select_objects(id, false);
            }
            Action::UnloadSources => self.unload_sources()?,
            Action::Close => {
                self.scene.clear();
                self.history.clear();
                self.objects.clear();
                self.selected = None;
            }
            Action::AddText(placement) => {
                let source = std::rc::Rc::new(crate::scene_label::example(placement));
                let prepared = crate::scene_label::Prepared::new(source)?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_text(prepared).map(|_| ())
                    })?;
            }
            Action::AddCloudLod => {
                let source = std::rc::Rc::new(crate::cloud::large_example());
                let prepared = crate::cloud::Prepared::new(source)?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_cloud(prepared).map(|_| ())
                    })?;
            }
            Action::AddCloud => {
                let source = std::rc::Rc::new(crate::cloud::example());
                let prepared = crate::cloud::Prepared::new(source)?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_cloud(prepared).map(|_| ())
                    })?;
            }
            Action::AddCad => {
                let source = std::rc::Rc::new(crate::cad_display::example());
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_cad(source).map(|_| ())
                    })?;
            }
            Action::AddSurface => {
                let source = std::rc::Rc::new(crate::surface::example());
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_surface(source).map(|_| ())
                    })?;
            }
            Action::AddNested => {
                let loaded =
                    crate::document::load(&crate::tree_specimen::nested(true))?;
                self.history
                    .try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::Live(ready) => {
                self.history.clear();
                self.scene.restore(*ready);
            }
            Action::Adopt { scene: ready, .. } => {
                self.history.edit(&mut self.scene, |scene| {
                    scene.restore(*ready)
                });
            }
            Action::Replace(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.replace(loaded)
                    })?;
            }
            Action::Import(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history
                    .try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ImportAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history
                    .try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ReplaceAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.replace(loaded)
                    })?;
            }
            Action::AddPolyline | Action::AddArrow => {
                let prepared = if matches!(action, Action::AddPolyline) {
                    let mut line = session_rust::Polyline::new(vec![
                        session_rust::Point::new(-0.6, -0.4, 0.0),
                        session_rust::Point::new(0.0, 0.0, 0.0),
                        session_rust::Point::new(0.6, -0.4, 0.0),
                    ]);
                    line.width = 6.0;
                    line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                    crate::chain::PreparedChain::polyline(std::rc::Rc::new(line))?
                } else {
                    let mut line =
                        session_rust::Line::new(-0.6, 0.3, 0.0, 0.6, 0.3, 0.0);
                    line.width = 5.0;
                    line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                    line.arrowhead = session_rust::Arrowhead::BOTH;
                    crate::chain::PreparedChain::line(std::rc::Rc::new(line))?
                };
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_path(prepared).map(|_| ())
                    })?;
            }
            Action::Controls(visible) => {
                let parent = if visible {
                    self.selected.filter(|id| {
                        self.scene
                            .clouds()
                            .iter()
                            .any(|r| r.id == *id)
                            || self
                                .scene
                                .streams()
                                .iter()
                                .any(|r| r.id == *id)
                    })
                } else {
                    None
                };
                if parent.is_some()
                    && self.pick_mode == crate::pick_sources::Mode::Control
                    && (self
                        .scene
                        .clouds()
                        .iter()
                        .any(|r| Some(r.id) == parent && r.controls_visible)
                        || self
                            .scene
                            .streams()
                            .iter()
                            .any(|r| Some(r.id) == parent && r.controls_visible))
                {
```

### 4. `src/scene_archive.rs`

Connect streamed selection and original identities while refusing incomplete editable Save.

**Locate this existing block:**

```rust
pub fn save(scene: &Scene) -> Result<Vec<u8>, String> {
    let mut namespaces = HashSet::new();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn save(scene: &Scene) -> Result<Vec<u8>, String> {
    if !scene.streams().is_empty() {
        return Err("This scene contains streamed sources. Open complete source documents before saving an editable session.".into());
    }
    let mut namespaces = HashSet::new();
```

## <a id="checkpoint-68o-account-for-original-and-retained-source-data"></a>17. Account for and retained source data

Account for original and retained source data.

### 1. `src/memory.rs`

Count shared streamed sources and retained payload separately.

**Locate this existing block:**

```rust
    usage[1] = seen.len();
    usage
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    usage[1] = seen.len();
    usage
}

pub fn streams<'a>(scenes: impl IntoIterator<Item = &'a Scene>) -> [usize; 5] {
    let mut sources = HashSet::new();
    let mut pages = HashSet::new();
    let mut usage = [0; 5];
    for scene in scenes {
        usage[0] += scene.streams().len();
        for row in scene.streams() {
            if sources.insert(Rc::as_ptr(&row.prepared.source)) {
                usage[2] += row.prepared.source.fields.count as usize;
            }
            for page in row.prepared.pages() {
                if pages.insert(Rc::as_ptr(&page)) {
                    usage[3] += page.bytes();
                    usage[4] += page.value.ids.len();
                }
            }
        }
    }
    usage[1] = sources.len();
    usage
}
```

### 2. `src/stream_row_tests.rs`

Copy streamed identity, placement, accounting and lifetime regressions.

Create the file and type:

```rust
use crate::scene::Scene;
use std::rc::Rc;

use crate::range_example::prepared;

#[test]
fn streamed_owners_share_sources_keep_original_ids_and_refuse_incomplete_save() {
    let prepared = prepared();
    let weak = Rc::downgrade(&prepared.source);
    let mut scene = Scene::empty();
    let first = scene
        .insert_stream(
            prepared.clone(),
            session_rust::Xform::identity(),
            4.,
        )
        .unwrap();
    let second = scene
        .insert_stream(
            prepared.clone(),
            session_rust::Xform::translation(2., 0., 0.),
            7.,
        )
        .unwrap();
    assert_ne!(first, second);
    assert_ne!(scene.row_guid(first), scene.row_guid(second));
    assert!(Rc::ptr_eq(
        &scene.streams()[0].prepared,
        &scene.streams()[1].prepared
    ));
    assert_eq!(scene.streams()[1].point_px, 7.);
    assert_eq!(scene.next(None), Some(first));
    assert_eq!(scene.next(Some(first)), Some(second));
    assert!(
        crate::scene_archive::save(&scene)
            .unwrap_err()
            .contains("complete source")
    );
    let mut editor = crate::editor::Editor::default();
    editor.scene = scene.clone();
    editor
        .apply(crate::editor::Action::SelectAll)
        .unwrap();
    let mut selected = editor.selected_objects();
    selected.sort_by_key(|id| id.value());
    assert_eq!(selected, [first, second]);
    editor
        .apply(crate::editor::Action::Close)
        .unwrap();
    scene.cloud_controls(Some(first));
    let selected = crate::cloud_pick::resolve(&scene, first, 0).unwrap();
    assert_eq!(
        selected.part,
        crate::pick_sources::Part::CloudPoint(u32::MAX)
    );
    assert_eq!(
        crate::cloud_pick::resolve(&scene, first, 2)
            .unwrap()
            .part,
        crate::pick_sources::Part::CloudPoint(10)
    );
    assert_eq!(
        crate::cloud_pick::position(&scene, first, u32::MAX).unwrap()[1],
        0.5
    );
    scene
        .translate(first, [0.25, 0., 0.])
        .unwrap();
    assert_eq!(
        crate::cloud_pick::position(&scene, first, u32::MAX).unwrap()[0],
        -0.25
    );
    assert_eq!(prepared.position(u32::MAX).unwrap()[0], -0.5);
    scene.clear();
    drop(prepared);
    assert!(weak.upgrade().is_none());
}
#[test]
fn pinned_coarse_samples_survive_cache_eviction_and_source_close_releases_them() {
    let prepared = prepared();
    let mut scene = Scene::empty();
    let id = scene
        .insert_stream(
            prepared.clone(),
            session_rust::Xform::identity(),
            4.,
        )
        .unwrap();
    scene.cloud_controls(Some(id));
    prepared.pages.borrow_mut().clear();
    assert_eq!(prepared.pages().len(), 1);
    assert_eq!(
        crate::cloud_pick::resolve(&scene, id, 0)
            .unwrap()
            .part,
        crate::pick_sources::Part::CloudPoint(10)
    );
    assert!(prepared.position(u32::MAX).is_none());
    assert_eq!(prepared.position(10).unwrap()[0], -0.5);
    let root = Rc::downgrade(prepared.root.borrow().as_ref().unwrap());
    scene.clear();
    drop(prepared);
    assert!(root.upgrade().is_none());
}

#[test]
fn shared_streamed_owners_report_original_and_retained_counts_separately() {
    let prepared = prepared();
    let mut scene = Scene::empty();
    let id = scene
        .insert_stream(
            prepared.clone(),
            session_rust::Xform::identity(),
            4.,
        )
        .unwrap();
    scene.copy_row(id, false).unwrap();
    let snapshot = scene.clone();
    let count = crate::memory::streams([&scene, &snapshot]);
    assert_eq!(&count[..3], &[4, 1, 4]);
    assert_eq!(count[4], 4);
    assert_eq!(
        count[3],
        prepared
            .pages()
            .iter()
            .map(|page| page.bytes())
            .sum::<usize>()
    );
    prepared.pages.borrow_mut().clear();
    let count = crate::memory::streams([&scene, &snapshot]);
    assert_eq!(count[2], 4);
    assert_eq!(count[4], 2);
    scene.clear();
    assert_eq!(crate::memory::streams([&scene]), [0; 5]);
}
```

### 3. `src/lib.rs`

Register the source component. Keep regression fixtures outside browser builds.

**Locate this existing block:**

```rust
pub mod range_example;

pub mod range_shape;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod range_example;

#[cfg(test)]
mod stream_row_tests;

pub mod range_shape;
```

## <a id="checkpoint-68p-keep-original-attributes-in-ranged-pages"></a>18. Keep attributes in ranged pages

Keep original attributes in ranged pages.

### 1. `src/stream.rs`

Validate present colour and normal columns before decoding.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            )?,
        )?;
        let raw = source_range(url, at, length, &fields.revision).await?;
        (raw.len() as u64 == length).then(|| normals_from(&raw))
```

Replace it with:

```rust
            )?,
        )?;
        let raw = source_range(url, at, length, &fields.revision).await?;
        if raw
            .chunks_exact(8)
            .any(|bytes| !f64::from_le_bytes(bytes.try_into().unwrap()).is_finite())
        {
            return None;
        }
        (raw.len() as u64 == length).then(|| normals_from(&raw))
```

**Locate this block:**

```rust
                seen[field - 8] = true;
            }

            if field == 5 && length == fields.coords_len {
                normals = (body, length);
            }
```

Replace it with:

```rust
                seen[field - 8] = true;
            }

            if field == 5 {
                if length != fields.coords_len || normals.1 != 0 {
                    return None;
                }
                normals = (body, length);
            }
```

**Locate this block:**

```rust
            i += n;
            *k = (v & 255) as u8;
        }

        out.push(u32::from_le_bytes(rgba));
```

Replace it with:

```rust
            i += n;
            *k = u8::try_from(v).ok()?;
        }

        out.push(u32::from_le_bytes(rgba));
```

### 2. `src/range_source.rs`

Read matching original attributes for each source page.

**Locate this existing block:**

```rust
            .map_err(str::to_owned)
    }
}

#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            .map_err(str::to_owned)
    }
    pub async fn page_attributes(
        &self,
        rows: std::ops::Range<u32>,
        color_at: u64,
    ) -> Result<(crate::range_page::Page, u64), String> {
        let page = self.page(rows.clone()).await?;
        let count = rows.end - rows.start;
        let (colors, next) = if self.fields.colors_len == 0 {
            (Vec::new(), color_at)
        } else {
            let (colors, used) =
                crate::stream::fetch_colors(&self.url, &self.fields, color_at, count)
                    .await
                    .ok_or("Source colour range failed")?;
            (colors, used)
        };
        let normals = if self.fields.normals_len == 0 {
            Vec::new()
        } else {
            crate::stream::fetch_normals(&self.url, &self.fields, rows.start, rows.end)
                .await
                .ok_or("Source normal range failed")?
        };
        Ok((
            page.attributes(colors, normals)
                .map_err(str::to_owned)?,
            next,
        ))
    }
}

#[cfg(test)]
```

## <a id="checkpoint-68q-retain-page-owners-beside-gpu-buffers"></a>19. Retain page owners beside GPU buffers

### 1. `src/cloud_gpu.rs`

Retain charged original page owners with their derived GPU columns.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            ),
            has_normals,
        })
```

Replace it with:

```rust
            ),
            has_normals,
        })
    }
    fn page(
        ctx: &GpuCtx,
        held: Rc<crate::page_cache::Held<crate::range_page::Page>>,
    ) -> Result<Self, &'static str> {
        let page = &held.value;
        let count = page.rows.end - page.rows.start;
        if u64::from(count) * 12
            > u64::from(
                ctx.device
                    .limits()
                    .max_storage_buffer_binding_size,
            )
        {
            return Err("Cloud page exceeds the device storage binding limit");
        }
        let positions = page
            .display
            .iter()
            .flat_map(|value| value.to_ne_bytes())
            .collect::<Vec<_>>();
        let colors = (0..count as usize)
            .flat_map(|i| {
                page.colors
                    .get(i)
                    .copied()
                    .unwrap_or(0xff00_0000)
                    .to_ne_bytes()
            })
            .collect::<Vec<_>>();
        let normals = page
            .normals
            .iter()
            .flat_map(|v| v.to_ne_bytes())
            .collect::<Vec<_>>();
        Ok(Self {
            positions: buffer(
                &ctx.device,
                "streamed source positions",
                &positions,
            ),
            colors: buffer(
                &ctx.device,
                "streamed source colours",
                &colors,
            ),
            normals: buffer(
                &ctx.device,
                "streamed source normals",
                &normals,
            ),
            has_normals: !page.normals.is_empty(),
            _page: Some(held),
        })
```

**Locate this block:**

```rust
            .flat_map(u32::to_ne_bytes)
            .collect::<Vec<_>>();
        Ok(Self {
            positions: buffer(
                &ctx.device,
                "original cloud positions",
```

Replace it with:

```rust
            .flat_map(u32::to_ne_bytes)
            .collect::<Vec<_>>();
        Ok(Self {
            _page: None,
            positions: buffer(
                &ctx.device,
                "original cloud positions",
```

**Locate this block:**

```rust
struct Columns {
    positions: wgpu::Buffer,
    colors: wgpu::Buffer,
    normals: wgpu::Buffer,
```

Replace it with:

```rust
struct Columns {
    _page: Option<Rc<crate::page_cache::Held<crate::range_page::Page>>>,
    positions: wgpu::Buffer,
    colors: wgpu::Buffer,
    normals: wgpu::Buffer,
```

## <a id="checkpoint-68r-describe-resident-and-streamed-draw-rows"></a>20. Describe resident and streamed draw rows

### 1. `src/cloud_gpu.rs`

Describe source ownership and compact retained-page addresses independently.

**Locate this existing block:**

```rust
}
struct Draw {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}
enum Keep {
    Resident(Rc<session_rust::PointCloud>),
    Stream(Rc<crate::range_cloud::Prepared>),
}
impl Keep {
    fn key(&self, first: u32) -> (usize, u32) {
        match self {
            Self::Resident(source) => (Rc::as_ptr(source) as usize, first),
            Self::Stream(source) => (Rc::as_ptr(source) as usize, first),
        }
    }
}
struct RowView {
    keep: Keep,
    id: crate::scene::ObjectId,
    model: session_rust::Xform,
    nodes: Rc<Vec<crate::cloud::LodNode>>,
    descriptor: crate::cloud::Cloud,
    controls_visible: bool,
    point_px: f32,
    rows: std::ops::Range<u32>,
    address: u32,
}
impl RowView {
    fn resident(row: &Row) -> Self {
        Self {
            keep: Keep::Resident(row.prepared.source.clone()),
            id: row.id,
            model: row.model.clone(),
            nodes: row.prepared.nodes.clone(),
            descriptor: row.prepared.descriptor(),
            controls_visible: row.controls_visible,
            point_px: if row.point_px > 0. {
                row.point_px
            } else {
                row.prepared.source.point_size as f32
            },
            rows: 0..row.prepared.source.len() as u32,
            address: 0,
        }
    }
    fn streamed(
        row: &crate::range_cloud::Row,
        rows: std::ops::Range<u32>,
        address: u32,
    ) -> Self {
        Self {
            keep: Keep::Stream(row.prepared.clone()),
            id: row.id,
            model: row.model.clone(),
            nodes: row.prepared.nodes.clone(),
            descriptor: crate::cloud::Cloud {
                spacing: row.prepared.source.lod.spacing[0] as f32,
                node_first: 0,
                node_count: row.prepared.nodes.len() as u32,
                resident: row.prepared.source.fields.count,
            },
            controls_visible: row.controls_visible,
            point_px: if row.point_px > 0. {
                row.point_px
            } else {
                row.prepared.source.properties.point_px
            },
            rows,
            address,
        }
    }
    fn key(&self) -> (usize, u32, u32) {
        let (source, first) = self.keep.key(self.rows.start);
        (source, first, self.rows.end)
    }
}
struct Draw {
```

## <a id="checkpoint-68s-draw-shared-source-pages-at-independent-placements"></a>21. Draw shared source pages at independent placements

### 1. `src/cloud_gpu.rs`

Connect resident and streamed original columns to compact bounded draw records.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            .chain(rot.map(f32::to_bits))
            .chain([
                if draw.columns.has_normals {
                    range.first
                } else {
                    u32::MAX
                },
                instance,
                flags,
                0,
            ]);
```

Replace it with:

```rust
            .chain(rot.map(f32::to_bits))
            .chain([
                if draw.columns.has_normals {
                    range.first - row.rows.start
                } else {
                    u32::MAX
                },
                instance,
                flags,
                row.address,
            ]);
```

**Locate this block:**

```rust
                (px * density * 0.5).max(0.5).to_bits(),
            ])
            .chain([
                range.first,
                range.count,
                cumulative,
                radius_factor(range, px, scale, view.ortho_h).to_bits(),
```

Replace it with:

```rust
                (px * density * 0.5).max(0.5).to_bits(),
            ])
            .chain([
                range.first - row.rows.start,
                range.count,
                cumulative,
                radius_factor(range, px, scale, view.ortho_h).to_bits(),
```

**Locate this block:**

```rust
    let rgb = draw
        .color
        .map_or([1., 1., 1.], |c| c.map(|v| v as f32 / 255.));
    let px = if row.point_px > 0. {
        row.point_px
    } else {
        source.point_size as f32
    };
    let flags = u32::from(draw.selected)
        | if draw.color.is_some() {
            256
```

Replace it with:

```rust
    let rgb = draw
        .color
        .map_or([1., 1., 1.], |c| c.map(|v| v as f32 / 255.));
    let px = row.point_px;
    let flags = u32::from(draw.selected)
        | if draw.color.is_some() {
            256
```

**Locate this block:**

```rust
    ranges: &[crate::cloud_lod::Range],
) -> Result<(Vec<u8>, u32), &'static str> {
    let row = &draw.row;
    let source = &row.prepared.source;
    let model = row.model.to_f32();
    let scale = row.model.uniform_scale();
    let mut total = 0u32;
```

Replace it with:

```rust
    ranges: &[crate::cloud_lod::Range],
) -> Result<(Vec<u8>, u32), &'static str> {
    let row = &draw.row;
    let model = row.model.to_f32();
    let scale = row.model.uniform_scale();
    let mut total = 0u32;
```

**Locate this block:**

```rust
                } else {
                    view.lod_px
                },
                nodes: &row.prepared.nodes,
            };
            walk.select(
                &projection,
                &row.prepared.descriptor(),
                &model,
            );
            records_used += walk.ranges.len();
            if records_used > MAX_RECORDS {
                return Err("Cloud view exceeds the 4096-record budget");
```

Replace it with:

```rust
                } else {
                    view.lod_px
                },
                nodes: &row.nodes,
            };
            walk.select(&projection, &row.descriptor, &model);
            walk.ranges.retain_mut(|range| {
                let start = range.first.max(row.rows.start);
                let end = (range.first + range.count).min(row.rows.end);
                range.first = start;
                range.count = end.saturating_sub(start);
                range.count > 0
            });
            records_used += walk.ranges.len();
            if records_used > MAX_RECORDS {
                return Err("Cloud view exceeds the 4096-record budget");
```

**Locate this block:**

```rust
                );
                (records, group)
            };
            draws.push(Draw {
                row: row.clone(),
                columns,
                records,
                group,
                visible: scene.visible(row.id),
                pickable: scene.pick_id(row.id.value()).is_some(),
                selected: selected.contains(&row.id),
                color: scene.view.faces.get(&row.id).copied(),
                total: 0,
            });
        }
```

Replace it with:

```rust
                );
                (records, group)
            };
            let id = row.id;
            draws.push(Draw {
                row,
                columns,
                records,
                group,
                visible: scene.visible(id),
                pickable: scene.pick_id(id.value()).is_some(),
                selected: selected.contains(&id),
                color: scene.view.faces.get(&id).copied(),
                total: 0,
            });
        }
```

**Locate this block:**

```rust
        scene: &crate::scene::Scene,
        selected: &[crate::scene::ObjectId],
    ) -> Result<(), &'static str> {
        let mut cache: HashMap<usize, Rc<Columns>> = self
            .draws
            .iter()
            .map(|d| {
                (
                    Rc::as_ptr(&d.row.prepared.source) as usize,
                    d.columns.clone(),
                )
            })
            .collect();
        let mut old = std::mem::take(&mut self.draws);
        let mut draws = Vec::with_capacity(scene.clouds().len());
        for row in scene.clouds() {
            let address = Rc::as_ptr(&row.prepared.source) as usize;
            let columns = if let Some(columns) = cache.get(&address) {
                columns.clone()
            } else {
                let columns = Rc::new(Columns::new(ctx, row)?);
                self.uploads += 1;
                cache.insert(address, columns.clone());
                columns
            };
            let (records, group) = if let Some(i) = old
                .iter()
                .position(|d| d.row.id == row.id && Rc::ptr_eq(&d.columns, &columns))
            {
                let d = old.swap_remove(i);
                (d.records, d.group)
            } else {
                let records = ctx
                    .device
```

Replace it with:

```rust
        scene: &crate::scene::Scene,
        selected: &[crate::scene::ObjectId],
    ) -> Result<(), &'static str> {
        let mut cache: HashMap<(usize, u32, u32), Rc<Columns>> = self
            .draws
            .iter()
            .map(|draw| (draw.row.key(), draw.columns.clone()))
            .collect();
        let mut candidates = Vec::new();
        for row in scene.clouds() {
            let view = RowView::resident(row);
            let key = view.key();
            let columns = if let Some(columns) = cache.get(&key) {
                columns.clone()
            } else {
                let columns = Rc::new(Columns::new(ctx, row)?);
                self.uploads += 1;
                cache.insert(key, columns.clone());
                columns
            };
            candidates.push((view, columns));
        }
        for row in scene.streams() {
            let mut address = 0u32;
            for page in row.prepared.pages() {
                let view = RowView::streamed(row, page.value.rows.clone(), address);
                let key = view.key();
                let columns = if let Some(columns) = cache.get(&key) {
                    columns.clone()
                } else {
                    let columns = Rc::new(Columns::page(ctx, page.clone())?);
                    self.uploads += 1;
                    cache.insert(key, columns.clone());
                    columns
                };
                address = address
                    .checked_add(page.value.rows.end - page.value.rows.start)
                    .filter(|value| *value <= crate::sub_id::LIMIT)
                    .ok_or("Streamed pick addresses exceed their source table")?;
                candidates.push((view, columns));
            }
        }
        let mut old = std::mem::take(&mut self.draws);
        let mut draws = Vec::with_capacity(candidates.len());
        for (row, columns) in candidates {
            let (records, group) = if let Some(i) = old.iter().position(|draw| {
                draw.row.id == row.id
                    && draw.row.rows == row.rows
                    && Rc::ptr_eq(&draw.columns, &columns)
            }) {
                let draw = old.swap_remove(i);
                (draw.records, draw.group)
            } else {
                let records = ctx
                    .device
```

**Locate this block:**

```rust
struct Draw {
    row: Row,
    columns: Rc<Columns>,
    records: wgpu::Buffer,
    group: wgpu::BindGroup,
```

Replace it with:

```rust
struct Draw {
    row: RowView,
    columns: Rc<Columns>,
    records: wgpu::Buffer,
    group: wgpu::BindGroup,
```

### 2. `src/cloud_points.wgsl`

Connect resident and streamed original columns to compact bounded draw records.

**Locate this existing block:**

```wgsl
    );
    s.row = i;
    s.instance = table[base + 37u];
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    );
    s.row = i + table[base + 39u];
    s.instance = table[base + 37u];
```

### 3. `src/renderer.rs`

Connect resident and streamed original columns to compact bounded draw records.

**Locate this existing block:**

```rust
    ) -> Result<(), &'static str> {
        if scene.clouds().is_empty() {
            self.clouds.borrow_mut().take();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ) -> Result<(), &'static str> {
        if scene.clouds().is_empty() && scene.streams().is_empty() {
            self.clouds.borrow_mut().take();
```

### 4. `src/pick_gpu_tests.rs`

Copy actual GPU source ID, page ownership, colour, reuse, eviction and cleanup fixtures.

**Locate this existing block:**

```rust
    camera_pixels(&mut renderer, &camera);
    assert_eq!(renderer.cloud_usage()[0], 64);
    assert_eq!(renderer.cloud_usage()[2], uploads);
    scene.clear();
    renderer.set_scene(&scene, None);
    assert_eq!(renderer.cloud_usage(), [0; 5]);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    camera_pixels(&mut renderer, &camera);
    assert_eq!(renderer.cloud_usage()[0], 64);
    assert_eq!(renderer.cloud_usage()[2], uploads);
    scene.clear();
    renderer.set_scene(&scene, None);
    assert_eq!(renderer.cloud_usage(), [0; 5]);
}

#[test]
fn actual_streamed_pages_pick_original_ids_share_uploads_and_release_after_eviction() {
    use crate::pick_sources::{Mode, Part};
    let prepared = crate::range_example::prepared();
    let weak = Rc::downgrade(&prepared.source);
    let mut scene = Scene::empty();
    let owner = scene
        .insert_stream(
            prepared.clone(),
            session_rust::Xform::identity(),
            12.,
        )
        .unwrap();
    scene.cloud_controls(Some(owner));
    let mut renderer = renderer(&scene);
    renderer.set_pick_mode(Mode::Control);
    for (cursor, id) in [
        ([-0.5, 0.5], u32::MAX),
        ([0.5, 0.5], 16_777_217),
        ([-0.5, -0.5], 10),
    ] {
        renderer
            .request_pick(cursor, identity())
            .unwrap();
        let hit = finish(&mut renderer).expect("Actual streamed point coverage");
        let target = renderer.resolve_pick(&scene, hit).unwrap();
        assert_eq!(target.owner, owner);
        assert_eq!(target.part, Part::CloudPoint(id));
    }
    let pixels = colour_pixels(&mut renderer);
    assert!(
        pixels
            .chunks_exact(4)
            .any(|p| p[2] > 160 && p[0] < 10)
    );
    assert!(
        pixels
            .chunks_exact(4)
            .any(|p| p[0] > 160 && p[2] < 10)
    );
    let uploads = renderer.cloud_usage()[2];
    let copy = scene.copy_row(owner, false).unwrap();
    scene.translate(copy, [0.2, 0., 0.]).unwrap();
    renderer.set_scene(&scene, None);
    colour_pixels(&mut renderer);
    assert_eq!(
        renderer.cloud_usage()[2],
        uploads,
        "Placed owners share original page uploads"
    );
    Rc::make_mut(&mut scene.view).lock(&[owner, copy], true);
    renderer.set_scene(&scene, None);
    renderer
        .request_pick([-0.5, 0.5], identity())
        .unwrap();
    assert!(finish(&mut renderer).is_none());
    Rc::make_mut(&mut scene.view).lock(&[owner, copy], false);
    let held = Rc::downgrade(&prepared.page(2).unwrap());
    prepared.pages.borrow_mut().clear();
    assert!(
        held.upgrade().is_some(),
        "GPU page owner retains precise pick identities"
    );
    renderer.set_scene(&scene, None);
    assert!(held.upgrade().is_none());
    renderer
        .request_pick([-0.5, 0.5], identity())
        .unwrap();
    assert!(
        finish(&mut renderer).is_none(),
        "Evicted detailed points are absent"
    );
    renderer
        .request_pick([-0.5, -0.5], identity())
        .unwrap();
    let hit = finish(&mut renderer).unwrap();
    assert_eq!(
        renderer
            .resolve_pick(&scene, hit)
            .unwrap()
            .part,
        Part::CloudPoint(10)
    );
    scene.clear();
    renderer.set_scene(&scene, None);
    drop(prepared);
    assert_eq!(renderer.cloud_usage(), [0; 5]);
    assert!(weak.upgrade().is_none());
}
```

## <a id="checkpoint-68t-validate-encoded-source-metadata"></a>22. Validate encoded source metadata

### 1. `src/source_encoding.rs`

Validate supported source encodings and exact decoded size.

Create the file and type:

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Encoding {
    pub name: String,
    pub decoded: Option<usize>,
}
impl Encoding {
    pub fn new(name: &str, size: u64) -> Result<Self, String> {
        if !matches!(name, "" | "gzip") {
            return Err(format!(
                "Unsupported source encoding: {name}"
            ));
        }
        let decoded = if name.is_empty() {
            None
        } else {
            let size =
                usize::try_from(size).map_err(|_| "Decoded source size overflow")?;
            if size == 0 || size > crate::archive::LIMIT {
                return Err("Decoded source size exceeds its file limit".into());
            }
            Some(size)
        };
        Ok(Self {
            name: name.into(),
            decoded,
        })
    }
    pub fn check(&self, bytes: usize) -> Result<(), String> {
        if bytes > crate::archive::LIMIT
            || self
                .decoded
                .is_some_and(|size| size != bytes)
        {
            return Err("Decoded source size does not match its manifest".into());
        }
        Ok(())
    }
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
mod stream_row_tests;

pub mod range_shape;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod stream_row_tests;

pub mod source_encoding;

pub mod range_shape;
```

### 3. `src/source_encoding.rs`

Copy encoded size and codec refusal fixtures.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn encoded_sources_require_supported_encoding_and_exact_bounded_decoded_bytes() {
        let gzip = Encoding::new("gzip", 24).unwrap();
        assert!(gzip.check(24).is_ok());
        assert!(gzip.check(23).is_err());
        assert!(Encoding::new("gzip", 0).is_err());
        assert!(Encoding::new("gzip", crate::archive::LIMIT as u64 + 1).is_err());
        assert!(Encoding::new("brotli", 24).is_err());
        assert!(
            Encoding::new("", 0)
                .unwrap()
                .check(24)
                .is_ok()
        );
    }
}
```

## <a id="checkpoint-68u-decode-bounded-cancellable-source-streams"></a>23. Decode bounded cancellable source streams

### 1. `Cargo.toml`

Copy the pinned browser stream dependency configuration.

**Locate this existing block:**

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["console", "Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "ErrorEvent", "PromiseRejectionEvent", "Location", "History", "UrlSearchParams", "Navigator", "Storage", "Performance", "PerformanceResourceTiming", "PerformanceEntry", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "RequestMode", "RequestCache", "EventSource", "MessageEvent", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
console_error_panic_hook = "=0.1.7"
```

Edit the existing block to match this code; keep unchanged lines:

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["console", "Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "ErrorEvent", "PromiseRejectionEvent", "Location", "History", "UrlSearchParams", "Navigator", "Storage", "Performance", "PerformanceResourceTiming", "PerformanceEntry", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "ReadableWritablePair", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "RequestMode", "RequestCache", "EventSource", "MessageEvent", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
console_error_panic_hook = "=0.1.7"
```

### 2. `src/source_unpack.rs`

Decode bounded cancellable original source streams.

Create the file and type:

```rust
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name=DecompressionStream)]
    type Unpack;
    #[wasm_bindgen(
        constructor,
        js_class = "DecompressionStream",
        catch
    )]
    fn new(format: &str) -> Result<Unpack, JsValue>;
}

pub async fn decode(
    bytes: Vec<u8>,
    encoding: &crate::source_encoding::Encoding,
    signal: &web_sys::AbortSignal,
) -> Result<Vec<u8>, String> {
    if signal.aborted() {
        return Err("Source decoding cancelled".into());
    }
    if !bytes.starts_with(&[0x1f, 0x8b]) {
        encoding.check(bytes.len())?;
        return Ok(bytes);
    }
    if encoding.name != "gzip" {
        return Err("Packed source lacks its gzip manifest declaration".into());
    }
    let parts = js_sys::Array::of1(&js_sys::Uint8Array::from(bytes.as_slice()));
    let blob = web_sys::Blob::new_with_u8_array_sequence(&parts).map_err(describe)?;
    let unpack = Unpack::new("gzip").map_err(describe)?;
    let stream = blob
        .stream()
        .pipe_through(unpack.unchecked_ref());
    let reader = stream
        .get_reader()
        .dyn_into::<web_sys::ReadableStreamDefaultReader>()
        .map_err(|error| describe(error.into()))?;
    let limit = encoding
        .decoded
        .ok_or("Missing decoded source size")?;
    let result = read(&reader, limit, signal).await;
    if result.is_err() {
        let _ = reader.cancel();
    }
    reader.release_lock();
    let bytes = result?;
    encoding.check(bytes.len())?;
    Ok(bytes)
}
fn describe(error: JsValue) -> String {
    format!("Source decompression failed: {error:?}")
}
async fn read(
    reader: &web_sys::ReadableStreamDefaultReader,
    limit: usize,
    signal: &web_sys::AbortSignal,
) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    loop {
        if signal.aborted() {
            return Err("Source decoding cancelled".into());
        }
        let part = JsFuture::from(reader.read())
            .await
            .map_err(describe)?;
        if js_sys::Reflect::get(&part, &"done".into())
            .map_err(describe)?
            .as_bool()
            == Some(true)
        {
            break;
        }
        let chunk = js_sys::Reflect::get(&part, &"value".into())
            .map_err(describe)?
            .dyn_into::<js_sys::Uint8Array>()
            .map_err(describe)?;
        if chunk.length() as usize > limit - bytes.len() {
            return Err("Decoded source exceeds its declared byte size".into());
        }
        bytes.extend_from_slice(&chunk.to_vec());
    }
    Ok(bytes)
}
```

### 3. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod source_encoding;

pub mod range_shape;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod source_encoding;

#[cfg(target_arch = "wasm32")]
pub mod source_unpack;

pub mod range_shape;
```

## <a id="checkpoint-68v-refuse-conflicting-source-encoding-revisions"></a>24. Refuse conflicting source encoding revisions

### 1. `src/publication_index.rs`

Validate manifest source formats before changing the original inventory.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        assert!(mesh.upgrade().is_none());
        assert!(!index.cached("mesh.pb"));
    }
```

Replace it with:

```rust
        assert!(mesh.upgrade().is_none());
        assert!(!index.cached("mesh.pb"));
    }
    #[test]
    fn refused_encoding_changes_keep_the_existing_manifest_and_original_sources() {
        let mut index = Index::new("files/".into());
        let raw = crate::specimen::bytes();
        index
            .manifest("scene", None, b"items: [{file: mesh.pb}]")
            .unwrap();
        index
            .source("files/mesh.pb", None, &raw)
            .unwrap();
        let scene = index
            .replacement(&Scene::empty())
            .unwrap()
            .unwrap();
        let original = Rc::clone(scene.objects()[0].geometry().unwrap());
        for manifest in [b"items: [{file: mesh.pb, encoding: gzip, size: 2}, {file: mesh.pb, encoding: gzip, size: 3}]".as_slice(),b"items: [{file: mesh.pb, encoding: brotli, size: 24}]".as_slice()] {
            assert!(index.manifest("scene",Some("bad".into()),manifest).is_err());assert_eq!(index.etag("scene"),None);assert_eq!(index.urls(),["files/mesh.pb"]);assert!(index.replacement(&scene).unwrap().is_none());assert!(Rc::ptr_eq(scene.objects()[0].geometry().unwrap(),&original));
        }
        assert!(
            index
                .manifest("scene", None, b"items: [{file: mesh.pb}]")
                .unwrap()
        );
        assert!(index.replacement(&scene).unwrap().is_some());
    }
```

**Locate this block:**

```rust
                return Err(error);
            }
        };
        self.manifest = Some(manifest);
        self.pending = true;
        let used = self
```

Replace it with:

```rust
                return Err(error);
            }
        };
        let mut formats = HashMap::new();
        for item in &manifest.items {
            let format = match crate::source_encoding::Encoding::new(
                &item.encoding,
                item.size,
            ) {
                Ok(format) => format,
                Err(error) => {
                    self.revisions.forget(url);
                    return Err(error);
                }
            };
            let key = crate::publication_route::join(&self.base, &item.file);
            if formats
                .insert(key, format.clone())
                .is_some_and(|previous| previous != format)
            {
                self.revisions.forget(url);
                return Err("Repeated source has conflicting encoding metadata".into());
            }
        }
        self.manifest = Some(manifest);
        self.pending = true;
        let used = self
```

**Locate this block:**

```rust
    }
    pub fn manifest(
        &mut self,
        url: &str,
```

Replace it with:

```rust
    }
    pub fn encoding(
        &self,
        url: &str,
    ) -> Result<crate::source_encoding::Encoding, String> {
        let manifest = self
            .manifest
            .as_ref()
            .ok_or("Source manifest is missing")?;
        let mut encoding = None;
        for item in &manifest.items {
            if crate::publication_route::join(&self.base, &item.file) == url {
                let value =
                    crate::source_encoding::Encoding::new(&item.encoding, item.size)?;
                if encoding
                    .as_ref()
                    .is_some_and(|previous| previous != &value)
                {
                    return Err(
                        "Repeated source has conflicting encoding metadata".into(),
                    );
                }
                encoding = Some(value);
            }
        }
        encoding.ok_or_else(|| "Source URL is absent from its manifest".into())
    }
    pub fn manifest(
        &mut self,
        url: &str,
```

## <a id="checkpoint-68w-adopt-remote-original-revisions-independently"></a>25. Adopt remote revisions independently

Adopt remote original revisions independently.

### 1. `src/publication_index.rs`

Adopt versioned remote originals while retaining complete-source byte identity for whole documents. Copy remote revision, placement and previous-scene retention fixtures.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        );
        assert!(index.replacement(&scene).unwrap().is_some());
    }
```

Replace it with:

```rust
        );
        assert!(index.replacement(&scene).unwrap().is_some());
    }
}

#[cfg(test)]
mod streamed_tests {
    use super::*;
    #[test]
    fn remote_original_revisions_share_placements_and_keep_ids_until_a_changed_validator()
     {
        let mut index = Index::new(String::new());
        index.manifest("scene",None,b"items: [{file: points.pb}, {file: points.pb, at: [2,0,0], point_size: 7}]").unwrap();
        let first = crate::range_example::prepared();
        index
            .stream("points.pb", first.clone())
            .unwrap();
        let scene = index
            .replacement(&Scene::empty())
            .unwrap()
            .unwrap();
        assert_eq!(scene.streams().len(), 2);
        assert!(Rc::ptr_eq(
            &scene.streams()[0].prepared,
            &scene.streams()[1].prepared
        ));
        let ids = scene
            .streams()
            .iter()
            .map(|r| r.id)
            .collect::<Vec<_>>();
        assert!(
            !index
                .stream("points.pb", first.clone())
                .unwrap()
        );
        assert!(index.replacement(&scene).unwrap().is_none());
        assert_eq!(
            scene
                .streams()
                .iter()
                .map(|r| r.id)
                .collect::<Vec<_>>(),
            ids
        );
        let mut changed = crate::range_example::prepared();
        let inner = Rc::get_mut(&mut changed).unwrap();
        Rc::get_mut(&mut inner.source)
            .unwrap()
            .fields
            .revision = Some("v2".into());
        assert!(
            index
                .stream("points.pb", changed.clone())
                .unwrap()
        );
        let next = index.replacement(&scene).unwrap().unwrap();
        assert!(Rc::ptr_eq(
            &next.streams()[0].prepared,
            &changed
        ));
        assert_ne!(next.streams()[0].id, ids[0]);
        assert!(Rc::ptr_eq(
            &scene.streams()[0].prepared,
            &first
        ));
        assert_eq!(next.streams()[1].point_px, 7.);
    }
```

**Locate this block:**

```rust
    }
    pub fn replacement(&mut self, previous: &Scene) -> Result<Option<Scene>, String> {
        if !self.pending {
            return Ok(None);
```

Replace it with:

```rust
    }
    pub fn source_points(&self, url: &str) -> u32 {
        self.sources.get(url).map_or(0, |scene| {
            scene
                .streams()
                .iter()
                .map(|row| row.prepared.loaded.get())
                .sum()
        })
    }
    pub fn streamed_points(&self) -> u32 {
        self.sources.keys().fold(0u32, |sum, url| {
            sum.saturating_add(self.source_points(url))
        })
    }
    pub fn stream(
        &mut self,
        url: &str,
        source: Rc<crate::range_cloud::Prepared>,
    ) -> Result<bool, String> {
        let revision = source.source.fields.revision.clone();
        if self.cached(url) && revision.is_some() && self.etag(url) == revision {
            return Ok(false);
        }
        let mut scene = Scene::empty();
        scene
            .insert_stream(
                source.clone(),
                session_rust::Xform::identity(),
                0.,
            )
            .map_err(str::to_owned)?;
        // Range revisions identify remote originals; sampled page bytes cannot prove whole-file byte equality.
        self.revisions
            .changed(url, revision, source.source.guid.as_bytes());
        self.sources.insert(url.into(), scene);
        self.sizes.remove(url);
        self.pending = true;
        Ok(true)
    }
    pub fn replacement(&mut self, previous: &Scene) -> Result<Option<Scene>, String> {
        if !self.pending {
            return Ok(None);
```

## <a id="checkpoint-68x-prepare-a-coarse-pin-and-bounded-prefix"></a>26. Prepare a coarse pin and bounded prefix

### 1. `src/stream_load.rs`

Prepare a pinned original coarse page and a budgeted initial prefix.

Create the file and type:

```rust
use crate::{
    page_cache::{Budget, Charge},
    range_cloud::Prepared,
    range_source::Source,
};
use std::{cell::Cell, rc::Rc};

pub struct Cursor {
    pub prepared: Rc<Prepared>,
    pub from: u32,
    pub color_at: u64,
    pub loaded: u32,
    pub ceiling: u32,
}
fn reserve(prepared: &Prepared, mut count: u32) -> Result<(u32, Charge), String> {
    while count > 0 {
        let bytes = crate::stream_budget::temporary_page_bytes(count)
            .ok_or("Point page size overflow")?;
        match prepared.pages.borrow_mut().reserve(bytes) {
            Ok(charge) => return Ok((count, charge)),
            Err(_) if count > 1 => count /= 2,
            Err(error) => return Err(error.into()),
        }
    }
    Err("Empty streamed source page".into())
}

pub async fn start(
    source: Rc<Source>,
    budget: Rc<Budget>,
    share: u32,
) -> Result<Cursor, String> {
    let prepared = Prepared::new(source, budget).map_err(str::to_owned)?;
    prepared
        .generation
        .borrow_mut()
        .start()
        .map_err(str::to_owned)?;
    let end = (prepared.source.lod.first[0] as u32)
        .checked_add(prepared.source.lod.count[0] as u32)
        .ok_or("Root range overflow")?;
    if end == 0 || end > crate::stream_budget::PREFIX_POINTS {
        return Err("Original coarse node exceeds the initial page limit".into());
    }
    let (root_count, charge) = reserve(&prepared, end)?;
    if root_count != end {
        return Err("Original coarse node exceeds the page memory budget".into());
    }
    let (root, _) = prepared
        .source
        .page_attributes(0..end, prepared.source.fields.colors_at)
        .await?;
    let root = root
        .slice(prepared.source.lod.first[0] as u32..end)
        .map_err(str::to_owned)?;
    prepared
        .pin_root(root, charge)
        .map_err(str::to_owned)?;
    let requested =
        crate::stream_budget::prefix(prepared.source.fields.count, share, share);
    let (count, charge) = reserve(&prepared, requested)?;
    let (page, color_at) = prepared
        .source
        .page_attributes(0..count, prepared.source.fields.colors_at)
        .await?;
    drop(
        prepared
            .adopt(page, charge)
            .map_err(str::to_owned)?,
    );
    Ok(Cursor {
        prepared,
        from: count,
        color_at,
        loaded: count,
        ceiling: count,
    })
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod source_unpack;

pub mod range_shape;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod source_unpack;

#[cfg(target_arch = "wasm32")]
pub mod stream_load;

pub mod range_shape;
```

## <a id="checkpoint-68y-redraw-before-reusing-evicted-page-addresses"></a>27. Redraw before reusing evicted page addresses

### 1. `src/stream_load.rs`

Synchronize visible eviction before reusing released page memory.

**Locate this existing block:**

```rust
}

pub async fn start(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

fn reserve_visible(
    prepared: &Prepared,
    mut count: u32,
    changed: &dyn Fn(),
) -> Result<(u32, Charge), String> {
    while crate::stream_budget::temporary_page_bytes(count)
        .ok_or("Point page size overflow")?
        > crate::stream_budget::PAGE_BYTES
    {
        count /= 2;
    }
    loop {
        let bytes = crate::stream_budget::temporary_page_bytes(count)
            .ok_or("Point page size overflow")?;
        let result = prepared.pages.borrow_mut().reserve(bytes);
        if let Ok(charge) = result {
            return Ok((count, charge));
        }
        let oldest = prepared.pages.borrow().oldest();
        if let Some(key) = oldest {
            prepared.pages.borrow_mut().remove(&key);
            // Redraw drops the evicted GPU page owner and cancels picks before its compact addresses can be reused.
            changed();
        } else if count > 1 {
            count /= 2;
        } else {
            return Err("Pending readers still hold the point page budget".into());
        }
    }
}

pub async fn start(
```

## <a id="checkpoint-68z-continue-pages-only-for-current-source-owners"></a>28. Continue pages only for current source owners

### 1. `src/stream_load.rs`

Continue checked source pages only while their scene/runtime owner remains current.

**Locate this existing block:**

```rust
    })
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    })
}

pub fn continue_read(
    cursor: Cursor,
    current: Rc<dyn Fn(&Rc<Prepared>) -> bool>,
    changed: Rc<dyn Fn()>,
    error: Rc<dyn Fn(String)>,
    cancelled: Rc<Cell<bool>>,
) {
    wasm_bindgen_futures::spawn_local(async move {
        let Cursor {
            prepared,
            mut from,
            mut color_at,
            mut loaded,
            ceiling,
        } = cursor;
        while current(&prepared) && !cancelled.get() {
            let left = ceiling.saturating_sub(loaded);
            let Some(rows) =
                crate::stream_budget::next(from, prepared.source.fields.count, left)
            else {
                return;
            };
            let (count, charge) =
                match reserve_visible(&prepared, rows.end - rows.start, &*changed) {
                    Ok(value) => value,
                    Err(message) => {
                        error(message);
                        return;
                    }
                };
            let ticket = match prepared.generation.borrow_mut().request() {
                Ok(ticket) => ticket,
                Err(_) => return,
            };
            let result = prepared
                .source
                .page_attributes(from..from + count, color_at)
                .await;
            if !current(&prepared)
                || cancelled.get()
                || !prepared
                    .generation
                    .borrow_mut()
                    .finish(ticket)
            {
                return;
            }
            match result {
                Ok((page, next)) => {
                    match prepared.adopt(page, charge) {
                        Ok(page) => drop(page),
                        Err(message) => {
                            error(message.into());
                            return;
                        }
                    }
                    color_at = next;
                    from += count;
                    loaded += count;
                    changed();
                }
                Err(message) => {
                    error(message);
                    return;
                }
            }
        }
    });
}
```

## <a id="checkpoint-68aa-own-cancellation-for-each-published-source"></a>29. Own cancellation for each published source

### 1. `src/range_fetch.rs`

Cancel active range reads belonging to replaced source URLs.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        });
    }
    let range = crate::range_rules::header(start, length)?;
    let mut read = Read::new()?;
    let init = web_sys::RequestInit::new();
```

Replace it with:

```rust
        });
    }
    let range = crate::range_rules::header(start, length)?;
    let mut read = Read::new(url)?;
    let init = web_sys::RequestInit::new();
```

**Locate this block:**

```rust
        };
        read.touch()?;
        REQUESTS.with(|requests| {
            requests
                .borrow_mut()
                .insert(key, read.controller.clone())
        });
        Ok(read)
    }
```

Replace it with:

```rust
        };
        read.touch()?;
        REQUESTS.with(|requests| {
            requests.borrow_mut().insert(
                key,
                (url.to_owned(), read.controller.clone()),
            )
        });
        Ok(read)
    }
```

**Locate this block:**

```rust
    callback: Closure<dyn FnMut()>,
}
impl Read {
    fn new() -> Result<Self, String> {
        let controller =
            web_sys::AbortController::new().map_err(|error| describe(error.into()))?;
        let key = NEXT.with(|next| {
```

Replace it with:

```rust
    callback: Closure<dyn FnMut()>,
}
impl Read {
    fn new(url: &str) -> Result<Self, String> {
        let controller =
            web_sys::AbortController::new().map_err(|error| describe(error.into()))?;
        let key = NEXT.with(|next| {
```

**Locate this block:**

```rust
    }
}

struct Read {
    key: u64,
    controller: web_sys::AbortController,
```

Replace it with:

```rust
    }
}

pub fn cancel_url(url: &str) {
    REQUESTS.with(|requests| {
        requests
            .borrow_mut()
            .retain(|_, (source, controller)| {
                if source == url {
                    controller.abort();
                    false
                } else {
                    true
                }
            })
    });
}

struct Read {
    key: u64,
    controller: web_sys::AbortController,
```

**Locate this block:**

```rust
thread_local! {
    static REQUESTS:RefCell<BTreeMap<u64,web_sys::AbortController>>=const {RefCell::new(BTreeMap::new())};
    static NEXT:Cell<u64>=const {Cell::new(0)};
}

pub fn cancel() {
    for controller in REQUESTS
        .with(|requests| std::mem::take(&mut *requests.borrow_mut()))
        .into_values()
    {
```

Replace it with:

```rust
thread_local! {
    static REQUESTS:RefCell<BTreeMap<u64,(String,web_sys::AbortController)>>=const {RefCell::new(BTreeMap::new())};
    static NEXT:Cell<u64>=const {Cell::new(0)};
}

pub fn cancel() {
    for (_, controller) in REQUESTS
        .with(|requests| std::mem::take(&mut *requests.borrow_mut()))
        .into_values()
    {
```

### 2. `src/browser_publication.rs`

Own pending source cursors and close every source reader with its publication.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        };
        let mut state = state.borrow_mut();
        state.active = false;
        state.gate.cancel();
```

Replace it with:

```rust
        };
        let mut state = state.borrow_mut();
        state.active = false;
        state.cancelled.set(true);
        for source in state
            .streams
            .drain(..)
            .filter_map(|s| s.upgrade())
        {
            source.generation.borrow_mut().close();
        }
        state.pending_streams.clear();
        crate::stream::reset_range_gate();
        state.gate.cancel();
```

**Locate this block:**

```rust
            active: true,
            inflight: false,
            again: false,
            gate: Default::default(),
            abort: None,
            route,
```

Replace it with:

```rust
            active: true,
            inflight: false,
            again: false,
            pending_streams: Vec::new(),
            streams: Vec::new(),
            cancelled: Rc::new(Cell::new(false)),
            gate: Default::default(),
            abort: None,
            route,
```

**Locate this block:**

```rust
    active: bool,
    inflight: bool,
    again: bool,
    gate: crate::read_gate::ReadGate,
    abort: Option<web_sys::AbortController>,
    route: Route,
```

Replace it with:

```rust
    active: bool,
    inflight: bool,
    again: bool,
    pending_streams: Vec<crate::stream_load::Cursor>,
    streams: Vec<std::rc::Weak<crate::range_cloud::Prepared>>,
    cancelled: Rc<Cell<bool>>,
    gate: crate::read_gate::ReadGate,
    abort: Option<web_sys::AbortController>,
    route: Route,
```

**Locate this block:**

```rust
use std::{
    cell::RefCell,
    collections::HashSet,
    rc::{Rc, Weak},
};
```

Replace it with:

```rust
use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
    rc::{Rc, Weak},
};
```

## <a id="checkpoint-68ab-choose-bounded-range-loading-for-large-originals"></a>30. Choose bounded range loading for large originals

### 1. `src/browser_publication.rs`

Stage original ranged prefixes and keep failed revisions off the current scene.

**Locate this existing block:**

```rust
        }
        let outcome =
            result.and_then(|()| index.replacement(&owner.document.borrow().scene));
        owner.index = Some(index);
        let changed = match outcome {
            Ok(Some(scene)) => {
                owner.last_error = None;
                owner.delivery = Some(Ok(Box::new(scene)));
                true
            }
            Ok(None) => false,
            Err(error) => {
                if owner.last_error.as_ref() == Some(&error) {
                    false
                } else {
                    let _ = crate::browser_report::observe("publication", &error);
                    owner.last_error = Some(error.clone());
                    owner.delivery = Some(Err(error));
                    true
                }
            }
        };
        let again = std::mem::take(&mut owner.again);
        drop(owner);
        if changed {
            if let Some(window) = web_sys::window() {
                if let Ok(event) = web_sys::Event::new("viewer-publication") {
                    let _ = window.dispatch_event(&event);
                }
            }
        }
        if again {
            poll(state);
        }
    });
}

async fn read(
    route: &Route,
    index: &mut Index,
    signal: &web_sys::AbortSignal,
) -> Result<(), String> {
    let started = crate::browser_phase::now();
    let mut response = crate::publication_fetch::get(
        &route.manifest,
        index.etag(&route.manifest),
        4 * 1024 * 1024,
        signal,
    )
    .await?;
    if response.status == 404 && route.manifest.ends_with(".toml") {
        let yaml = format!(
            "{}.yaml",
            route.manifest.trim_end_matches(".toml")
        );
        response = crate::publication_fetch::get(
            &yaml,
            index.etag(&route.manifest),
            4 * 1024 * 1024,
            signal,
        )
        .await?;
    }
    match response.status {
        304 if index.has_manifest() => {}
        200..=299 => {
            index.manifest(
                &route.manifest,
                response.etag,
                &response.bytes,
            )?;
            crate::browser_phase::finish(
                "manifest",
                started,
                response.bytes.len() as u64,
                &route.manifest,
            );
        }
        status => return Err(format!("Manifest HTTP {status}")),
    }
    let mut seen = HashSet::new();
    for url in index.urls() {
        if !seen.insert(url.clone())
            || crate::manifest::immutable_key(&url) && index.cached(&url)
        {
            continue;
        }
        let response = crate::publication_fetch::get(
            &url,
            index.etag(&url),
            crate::archive::LIMIT,
            signal,
        )
        .await?;
        match response.status {
            304 if index.cached(&url) => {}
            200..=299 => {
                let started = crate::browser_phase::now();
                index.source(&url, response.etag, &response.bytes)?;
                crate::browser_phase::finish(
                    "decode/validation/display",
                    started,
                    response.bytes.len() as u64,
                    &url,
                );
            }
            status => {
                index.forget(&url);
                return Err(format!("Source HTTP {status}: {url}"));
            }
        }
    }
    Ok(())
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        }
        let outcome = result.and_then(|cursors| {
            for cursor in &cursors {
                owner
                    .streams
                    .push(Rc::downgrade(&cursor.prepared));
            }
            owner.pending_streams.extend(cursors);
            index.replacement(&owner.document.borrow().scene)
        });
        owner.index = Some(index);
        let changed = match outcome {
            Ok(Some(scene)) => {
                owner.last_error = None;
                owner.delivery = Some(Ok(Box::new(scene)));
                true
            }
            Ok(None) => false,
            Err(error) => {
                if owner.last_error.as_ref() == Some(&error) {
                    false
                } else {
                    let _ = crate::browser_report::observe("publication", &error);
                    owner.last_error = Some(error.clone());
                    owner.delivery = Some(Err(error));
                    true
                }
            }
        };
        let again = std::mem::take(&mut owner.again);
        drop(owner);
        if changed {
            if let Some(window) = web_sys::window() {
                if let Ok(event) = web_sys::Event::new("viewer-publication") {
                    let _ = window.dispatch_event(&event);
                }
            }
        }
        if again {
            poll(state);
        }
    });
}

async fn read(
    route: &Route,
    index: &mut Index,
    signal: &web_sys::AbortSignal,
) -> Result<Vec<crate::stream_load::Cursor>, String> {
    let mut cursors = Vec::new();
    let mut points = index.streamed_points();
    let budget = crate::page_cache::Budget::new(crate::stream_budget::PAGE_BYTES);
    let started = crate::browser_phase::now();
    let mut response = crate::publication_fetch::get(
        &route.manifest,
        index.etag(&route.manifest),
        4 * 1024 * 1024,
        signal,
    )
    .await?;
    if response.status == 404 && route.manifest.ends_with(".toml") {
        let yaml = format!(
            "{}.yaml",
            route.manifest.trim_end_matches(".toml")
        );
        response = crate::publication_fetch::get(
            &yaml,
            index.etag(&route.manifest),
            4 * 1024 * 1024,
            signal,
        )
        .await?;
    }
    match response.status {
        304 if index.has_manifest() => {}
        200..=299 => {
            index.manifest(
                &route.manifest,
                response.etag,
                &response.bytes,
            )?;
            crate::browser_phase::finish(
                "manifest",
                started,
                response.bytes.len() as u64,
                &route.manifest,
            );
        }
        status => return Err(format!("Manifest HTTP {status}")),
    }
    let mut seen = HashSet::new();
    for url in index.urls() {
        if !seen.insert(url.clone())
            || crate::manifest::immutable_key(&url) && index.cached(&url)
        {
            continue;
        }
        let encoding = index.encoding(&url)?;
        if encoding.name.is_empty()
            && url
                .split('?')
                .next()
                .is_some_and(|path| path.ends_with(".pb"))
        {
            if let Some(probe) = crate::stream::probe(&url).await {
                if crate::range_shape::single_cloud(&probe.bytes)
                    && crate::stream::cloud_identity(&probe.bytes).is_some()
                {
                    let source =
                        crate::range_source::Source::from_probe(url.clone(), &probe)
                            .await?;
                    if source.fields.count > crate::stream_budget::PREFIX_POINTS
                        || source.fields.coords_len >= crate::stream_budget::MIN_BYTES
                    {
                        if index.cached(&url)
                            && source.fields.revision.is_some()
                            && index.etag(&url) == source.fields.revision
                        {
                            continue;
                        }
                        if signal.aborted() {
                            return Err("Source read cancelled".into());
                        }
                        points = points.saturating_sub(index.source_points(&url));
                        let remaining =
                            crate::stream_budget::MAX_POINTS.saturating_sub(points);
                        if remaining == 0 {
                            return Err(
                                "Manifest sources exceed the point ceiling".into()
                            );
                        }
                        let cursor = crate::stream_load::start(
                            Rc::new(source),
                            budget.clone(),
                            remaining,
                        )
                        .await?;
                        if signal.aborted() {
                            cursor
                                .prepared
                                .generation
                                .borrow_mut()
                                .close();
                            return Err("Source read cancelled".into());
                        }
                        points = points.saturating_add(cursor.loaded);
                        index.stream(&url, cursor.prepared.clone())?;
                        cursors.push(cursor);
                        continue;
                    }
                }
            }
        }
        let response = crate::publication_fetch::get(
            &url,
            index.etag(&url),
            crate::archive::LIMIT,
            signal,
        )
        .await?;
        match response.status {
            304 if index.cached(&url) => {}
            200..=299 => {
                let started = crate::browser_phase::now();
                let bytes =
                    crate::source_unpack::decode(response.bytes, &encoding, signal)
                        .await?;
                index.source(&url, response.etag, &bytes)?;
                crate::browser_phase::finish(
                    "decode/validation/display",
                    started,
                    bytes.len() as u64,
                    &url,
                );
            }
            status => {
                index.forget(&url);
                return Err(format!("Source HTTP {status}: {url}"));
            }
        }
    }
    let extra = crate::stream_budget::MAX_POINTS.saturating_sub(points)
        / (cursors.len().max(1) as u32);
    for cursor in &mut cursors {
        cursor.ceiling = cursor.loaded + extra;
    }
    Ok(cursors)
}
```

## <a id="checkpoint-68ac-connect-current-pages-to-the-browser-renderer"></a>31. Connect current pages to the browser renderer

### 1. `src/browser_publication.rs`

Connect current streamed page delivery, original source diagnostics and bounded runtime ownership.

**Locate this existing block:**

```rust
        };
        let mut state = state.borrow_mut();
        if !state.active {
            state.delivery = None;
            return Ok(None);
        }
        state
            .delivery
            .take()
            .transpose()
            .map(|scene| scene.map(crate::editor::Action::Live))
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        };
        let mut owner = state.borrow_mut();
        if !owner.active {
            owner.delivery = None;
            return Ok(None);
        }
        let scene = owner.delivery.take().transpose()?;
        if let Some(next) = scene.as_ref() {
            owner.streams.retain(|weak| {
                weak.upgrade().is_some_and(|source| {
                    if next
                        .streams()
                        .iter()
                        .any(|row| Rc::ptr_eq(&row.prepared, &source))
                    {
                        true
                    } else {
                        source.generation.borrow_mut().close();
                        crate::range_fetch::cancel_url(&source.source.url);
                        false
                    }
                })
            });
            let cursors = std::mem::take(&mut owner.pending_streams);
            let cancelled = owner.cancelled.clone();
            drop(owner);
            for cursor in cursors {
                let weak = Rc::downgrade(&state);
                let current = Rc::new(
                    move |source: &Rc<crate::range_cloud::Prepared>| {
                        weak.upgrade().is_some_and(|state| {
                            let owner = state.borrow();
                            owner.active
                                && crate::browser_runtime::owns(&owner.fault)
                                && owner.fault.message().is_none()
                                && owner
                                    .document
                                    .borrow()
                                    .scene
                                    .streams()
                                    .iter()
                                    .any(|row| Rc::ptr_eq(&row.prepared, source))
                        })
                    },
                );
                let changed = Rc::new(|| {
                    if let Some(window) = web_sys::window() {
                        if let Ok(event) = web_sys::Event::new("viewer-stream-page") {
                            let _ = window.dispatch_event(&event);
                        }
                    }
                });
                let error = Rc::new(|message: String| {
                    crate::browser::report(&format!(
                        "Source page read stopped: {message}"
                    ))
                });
                crate::stream_load::continue_read(
                    cursor,
                    current,
                    changed,
                    error,
                    cancelled.clone(),
                );
            }
        }
        Ok(scene.map(crate::editor::Action::Live))
    }
```

### 2. `src/browser.rs`

Connect current streamed page delivery, original source diagnostics and bounded runtime ownership.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            .selection()
            .map_or_else(String::new, |target| target.describe()),
    )?;
    canvas.set_attribute(
```

Replace it with:

```rust
            .selection()
            .map_or_else(String::new, |target| target.describe()),
    )?;
    let stream_usage = crate::memory::streams(editor.scenes());
    canvas.set_attribute(
        "data-range-sources",
        &serde_json::to_string(&stream_usage).unwrap(),
    )?;
    let streamed=editor.scene.streams().iter().map(|row|serde_json::json!({"id":row.id.value(),"guid":row.guid,"source_guid":row.prepared.source.guid,"name":row.prepared.source.name,"url":row.prepared.source.url,"revision":row.prepared.source.fields.revision,"total":row.prepared.source.fields.count,"model":row.model.m,"point_px":row.point_px,"controls":row.controls_visible})).collect::<Vec<_>>();
    canvas.set_attribute(
        "data-range-documents",
        &serde_json::to_string(&streamed).unwrap(),
    )?;

    canvas.set_attribute(
```

**Locate this block:**

```rust
    listeners.listen(document.as_ref(), "cancel")?;
    listeners.listen(window.as_ref(), "viewer-pick")?;
    listeners.listen(window.as_ref(), "viewer-publication")?;
    listeners.listen(window.as_ref(), "viewer-file")?;
    listeners.listen(window.as_ref(), "viewer-file-error")?;
    listeners.listen(window.as_ref(), "viewer-reload")?;
```

Replace it with:

```rust
    listeners.listen(document.as_ref(), "cancel")?;
    listeners.listen(window.as_ref(), "viewer-pick")?;
    listeners.listen(window.as_ref(), "viewer-publication")?;
    listeners.listen(window.as_ref(), "viewer-stream-page")?;
    listeners.listen(window.as_ref(), "viewer-file")?;
    listeners.listen(window.as_ref(), "viewer-file-error")?;
    listeners.listen(window.as_ref(), "viewer-reload")?;
```

**Locate this block:**

```rust
                None
            } else if event.type_() == "viewer-publication" {
                match publication_control.take() {
                    Ok(action) => action,
```

Replace it with:

```rust
                None
            } else if event.type_() == "viewer-stream-page" {
                renderer.cancel_pick();
                Some(Action::StreamRefresh)
            } else if event.type_() == "viewer-publication" {
                match publication_control.take() {
                    Ok(action) => action,
```

## Build & Run

![Recorded visual output for Lesson 12.4 (68ac-connect-current-pages-to-the-browser-renderer)](../../../screenshots/journey/68ac-connect-current-pages-to-the-browser-renderer-browser.png)

```sh
npm --prefix ../session_tests run course -- dependencies 12.4
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 12.4
npm --prefix ../session_tests run course -- save 12.4
```

Continue with [13.1 · Shared original definitions and independently placed instances](../../consolidation/69-validate-original-definition-references.md).
