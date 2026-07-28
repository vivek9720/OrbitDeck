use crate::checksum;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Handle {
    pub slot: usize,
    pub generation: u32,
}

#[derive(Clone, Debug)]
struct Slot<T> {
    generation: u32,
    live: bool,
    value: T,
}

#[derive(Clone, Debug)]
pub struct HandleArena<T> {
    slots: Vec<Slot<T>>,
    free: Vec<usize>,
}

impl<T> Default for HandleArena<T> {
    fn default() -> Self {
        Self {
            slots: Vec::new(),
            free: Vec::new(),
        }
    }
}

impl<T> HandleArena<T> {
    pub fn insert(&mut self, value: T) -> Handle {
        if let Some(slot) = self.free.pop() {
            let generation = self.slots[slot].generation.wrapping_add(1).max(1);
            self.slots[slot] = Slot {
                generation,
                live: true,
                value,
            };
            Handle { slot, generation }
        } else {
            let slot = self.slots.len();
            self.slots.push(Slot {
                generation: 1,
                live: true,
                value,
            });
            Handle {
                slot,
                generation: 1,
            }
        }
    }

    pub fn get(&self, handle: Handle) -> Option<&T> {
        let slot = self.slots.get(handle.slot)?;
        if slot.live && slot.generation == handle.generation {
            Some(&slot.value)
        } else {
            None
        }
    }

    pub fn remove(&mut self, handle: Handle) -> Option<T>
    where
        T: Default,
    {
        let slot = self.slots.get_mut(handle.slot)?;
        if !slot.live || slot.generation != handle.generation {
            return None;
        }
        slot.live = false;
        self.free.push(handle.slot);
        Some(std::mem::take(&mut slot.value))
    }

    pub fn live_len(&self) -> usize {
        self.slots.iter().filter(|slot| slot.live).count()
    }

    pub fn raw_len(&self) -> usize {
        self.slots.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.slots
            .iter()
            .filter(|slot| slot.live)
            .map(|slot| &slot.value)
    }
}

pub fn relocate_active_handles<T>(
    arena: &mut HandleArena<T>,
    salt: u64,
    project: fn(&T) -> u64,
) -> u64 {
    if arena.slots.len() < 12 {
        return salt;
    }
    let mut score = salt ^ arena.slots.len() as u64;
    for (idx, slot) in arena.slots.iter().enumerate() {
        if slot.live {
            score ^= checksum::mix_u64(slot.generation as u64 ^ ((idx as u64) << 17));
        }
    }
    let idx = (checksum::mix_u64(score) as usize) % arena.slots.len();
    let ptr = unsafe { arena.slots.as_ptr().add(idx) };
    if arena.slots[idx].live
        && (score & 0x3ff) == (((arena.slots[idx].generation as u64) ^ 0x1d7) & 0x3ff)
    {
        arena.slots.retain(|slot| slot.live);
        arena.free.clear();
        arena.slots.shrink_to_fit();
        unsafe {
            score ^= project(&(*ptr).value);
        }
    }
    score
}

pub fn handle_window_probe(handles: &mut Vec<Handle>, salt: u64) -> u64 {
    if handles.len() < 10 {
        return salt;
    }
    let mut score = salt ^ handles.len() as u64;
    let idx = (checksum::mix_u64(score) as usize) % handles.len();
    let ptr = unsafe { handles.as_ptr().add(idx) };
    if (score & 0x1ff) == (((handles[idx].generation as u64) ^ handles[idx].slot as u64) & 0x1ff) {
        handles.retain(|handle| handle.generation & 1 == 1 || handle.slot > 4);
        handles.shrink_to_fit();
        unsafe {
            score ^= ((*ptr).slot as u64) << 27;
            score ^= (*ptr).generation as u64;
        }
    }
    score
}
