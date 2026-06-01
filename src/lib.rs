#![deny(unsafe_code)]

use wasm_bindgen::prelude::*;

/// A story held in living memory.
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Story {
    name: String,
    weight: f64,
    tell_count: u32,
    parent_id: Option<usize>,
}

#[wasm_bindgen]
impl Story {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, weight: f64, parent_id: Option<usize>) -> Story {
        Story {
            name,
            weight,
            tell_count: 0,
            parent_id,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn weight(&self) -> f64 {
        self.weight
    }

    #[wasm_bindgen(getter)]
    pub fn tell_count(&self) -> u32 {
        self.tell_count
    }

    #[wasm_bindgen(getter)]
    pub fn parent_id(&self) -> Option<usize> {
        self.parent_id
    }
}

/// A griot — keeper of oral tradition and living memory.
#[wasm_bindgen]
#[derive(Debug)]
pub struct Griot {
    stories: Vec<Story>,
    decay_rate: f64,
    capacity: usize,
}

#[wasm_bindgen]
impl Griot {
    #[wasm_bindgen(constructor)]
    pub fn new(decay_rate: f64, capacity: usize) -> Griot {
        Griot {
            stories: Vec::new(),
            decay_rate,
            capacity,
        }
    }

    /// Add a story to the griot's memory. Returns the story id.
    pub fn add_story(
        &mut self,
        name: String,
        weight: f64,
        parent_id: Option<usize>,
    ) -> usize {
        if self.stories.len() >= self.capacity {
            // Evict the lowest-weight story
            if let Some(pos) = self.stories.iter().enumerate().min_by(|a, b| a.1.weight.partial_cmp(&b.1.weight).unwrap()).map(|(i, _)| i) {
                self.stories.remove(pos);
            }
        }
        let story = Story::new(name, weight, parent_id);
        self.stories.push(story);
        self.stories.len() - 1
    }

    /// Find a story by exact name.
    pub fn find_story(&self, name: String) -> Option<Story> {
        self.stories.iter().find(|s| s.name == name).cloned()
    }

    /// Tell (recount) a story, increasing its tell count and weight. Returns the new tell count.
    pub fn tell_story(&mut self, name: String) -> Option<u32> {
        for story in &mut self.stories {
            if story.name == name {
                story.tell_count += 1;
                story.weight += 1.0;
                return Some(story.tell_count);
            }
        }
        None
    }

    /// Apply exponential decay to all story weights over elapsed time.
    pub fn apply_decay(&mut self, elapsed: f64) {
        for story in &mut self.stories {
            story.weight *= (-self.decay_rate * elapsed).exp();
        }
    }

    /// Overall tradition score: sum of all story weights.
    pub fn tradition_score(&self) -> f64 {
        self.stories.iter().map(|s| s.weight).sum()
    }

    /// Return all memory strengths (weights) as a vector.
    pub fn memory_strengths(&self) -> Vec<f64> {
        self.stories.iter().map(|s| s.weight).collect()
    }

    /// Number of stories stored.
    pub fn story_count(&self) -> usize {
        self.stories.len()
    }

    /// Get a story by index.
    pub fn get_story(&self, id: usize) -> Option<Story> {
        self.stories.get(id).cloned()
    }
}

/// A praise name generated from shared stories.
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct PraiseName {
    name: String,
    story_count: usize,
    combined_weight: f64,
}

#[wasm_bindgen]
impl PraiseName {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn story_count(&self) -> usize {
        self.story_count
    }

    #[wasm_bindgen(getter)]
    pub fn combined_weight(&self) -> f64 {
        self.combined_weight
    }
}

/// Generate a praise name from selected stories.
#[wasm_bindgen]
pub fn generate_praise(griot: &Griot, story_ids: Vec<usize>, name: String) -> PraiseName {
    let mut combined_weight = 0.0;
    let mut count = 0usize;
    for &id in &story_ids {
        if let Some(story) = griot.stories.get(id) {
            combined_weight += story.weight;
            count += 1;
        }
    }
    PraiseName {
        name,
        story_count: count,
        combined_weight,
    }
}

/// Result of a call-and-response between two griots.
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct CallResponse {
    story_name: String,
    caller_tell_count: u32,
    responder_tell_count: u32,
    harmony: bool,
}

#[wasm_bindgen]
impl CallResponse {
    #[wasm_bindgen(getter)]
    pub fn story_name(&self) -> String {
        self.story_name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn caller_tell_count(&self) -> u32 {
        self.caller_tell_count
    }

    #[wasm_bindgen(getter)]
    pub fn responder_tell_count(&self) -> u32 {
        self.responder_tell_count
    }

    #[wasm_bindgen(getter)]
    pub fn harmony(&self) -> bool {
        self.harmony
    }
}

/// Perform a call-and-response between two griots on a named story.
#[wasm_bindgen]
pub fn call_and_response(
    caller: &Griot,
    responder: &Griot,
    story_name: String,
) -> CallResponse {
    let caller_story = caller.find_story(story_name.clone());
    let responder_story = responder.find_story(story_name.clone());

    let caller_count = caller_story.as_ref().map_or(0, |s| s.tell_count);
    let responder_count = responder_story.as_ref().map_or(0, |s| s.tell_count);

    // Harmony: both know the story and have told it at least once
    let harmony = caller_story.is_some()
        && responder_story.is_some()
        && caller_count > 0
        && responder_count > 0;

    CallResponse {
        story_name,
        caller_tell_count: caller_count,
        responder_tell_count: responder_count,
        harmony,
    }
}

/// Trace the genealogy (ancestor chain) of a story by following parent_id links.
#[wasm_bindgen]
pub fn genealogy(griot: &Griot, story_id: usize) -> Vec<usize> {
    let mut chain = Vec::new();
    let mut current = Some(story_id);
    while let Some(id) = current {
        chain.push(id);
        current = griot.stories.get(id).and_then(|s| s.parent_id);
    }
    chain
}

/// A federation of griots sharing stories across a network.
#[wasm_bindgen]
#[derive(Debug)]
pub struct Federation {
    griots: Vec<Griot>,
    decay_rate: f64,
}

#[wasm_bindgen]
impl Federation {
    #[wasm_bindgen(constructor)]
    pub fn new(count: usize, decay_rate: f64) -> Federation {
        let griots = (0..count)
            .map(|_| Griot::new(decay_rate, 1000))
            .collect();
        Federation { griots, decay_rate }
    }

    /// Sync a story from one griot to another by name.
    pub fn sync_story(&mut self, from: usize, to: usize, name: String) {
        if from == to || from >= self.griots.len() || to >= self.griots.len() {
            return;
        }
        if let Some(story) = self.griots[from].find_story(name) {
            self.griots[to].add_story(story.name, story.weight, story.parent_id);
        }
    }

    /// Coverage: fraction of griots that have at least one story.
    pub fn coverage(&self) -> f64 {
        if self.griots.is_empty() {
            return 0.0;
        }
        let with_stories = self.griots.iter().filter(|g| g.story_count() > 0).count();
        with_stories as f64 / self.griots.len() as f64
    }

    /// Get a reference to a griot by index (returns a clone for wasm).
    pub fn get_griot(&self, index: usize) -> Option<Griot> {
        // We need a mutable version for tests; return a clone
        // Actually for wasm_bindgen we can't easily return &mut, so we provide a helper
        self.griots.get(index).map(|g| Griot {
            stories: g.stories.clone(),
            decay_rate: g.decay_rate,
            capacity: g.capacity,
        })
    }

    /// Add a story to a specific griot in the federation.
    pub fn add_to_griot(
        &mut self,
        index: usize,
        name: String,
        weight: f64,
        parent_id: Option<usize>,
    ) -> usize {
        if index < self.griots.len() {
            self.griots[index].add_story(name, weight, parent_id)
        } else {
            usize::MAX
        }
    }

    /// Tell a story on a specific griot.
    pub fn tell_on_griot(&mut self, index: usize, name: String) -> Option<u32> {
        if index < self.griots.len() {
            self.griots[index].tell_story(name)
        } else {
            None
        }
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Story tests ──

    #[test]
    fn test_story_new() {
        let s = Story::new("Sundiata".into(), 5.0, None);
        assert_eq!(s.name, "Sundiata");
        assert!((s.weight - 5.0).abs() < f64::EPSILON);
        assert_eq!(s.tell_count, 0);
        assert_eq!(s.parent_id, None);
    }

    #[test]
    fn test_story_with_parent() {
        let s = Story::new("Mansa Musa".into(), 10.0, Some(0));
        assert_eq!(s.parent_id, Some(0));
    }

    // ── Griot tests ──

    #[test]
    fn test_griot_new() {
        let g = Griot::new(0.01, 100);
        assert_eq!(g.story_count(), 0);
        assert!((g.tradition_score() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_add_story_returns_id() {
        let mut g = Griot::new(0.01, 100);
        let id = g.add_story("Sundiata".into(), 5.0, None);
        assert_eq!(id, 0);
        assert_eq!(g.story_count(), 1);
    }

    #[test]
    fn test_find_story_found() {
        let mut g = Griot::new(0.01, 100);
        g.add_story("Sundiata".into(), 5.0, None);
        let found = g.find_story("Sundiata".into());
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Sundiata");
    }

    #[test]
    fn test_find_story_not_found() {
        let g = Griot::new(0.01, 100);
        let found = g.find_story("Nonexistent".into());
        assert!(found.is_none());
    }

    #[test]
    fn test_tell_story_increments() {
        let mut g = Griot::new(0.01, 100);
        g.add_story("Sundiata".into(), 5.0, None);
        let count = g.tell_story("Sundiata".into());
        assert_eq!(count, Some(1));
        let count2 = g.tell_story("Sundiata".into());
        assert_eq!(count2, Some(2));
    }

    #[test]
    fn test_tell_story_increases_weight() {
        let mut g = Griot::new(0.01, 100);
        g.add_story("Sundiata".into(), 5.0, None);
        g.tell_story("Sundiata".into());
        let s = g.find_story("Sundiata".into()).unwrap();
        assert!((s.weight - 6.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_tell_nonexistent() {
        let mut g = Griot::new(0.01, 100);
        assert_eq!(g.tell_story("Ghost".into()), None);
    }

    #[test]
    fn test_apply_decay() {
        let mut g = Griot::new(0.1, 100);
        g.add_story("Sundiata".into(), 10.0, None);
        g.apply_decay(1.0);
        let weights = g.memory_strengths();
        // weight = 10 * exp(-0.1 * 1) ≈ 9.048
        assert!(weights[0] < 10.0);
        assert!(weights[0] > 9.0);
    }

    #[test]
    fn test_tradition_score() {
        let mut g = Griot::new(0.01, 100);
        g.add_story("A".into(), 3.0, None);
        g.add_story("B".into(), 7.0, None);
        assert!((g.tradition_score() - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_memory_strengths() {
        let mut g = Griot::new(0.01, 100);
        g.add_story("A".into(), 2.0, None);
        g.add_story("B".into(), 4.0, None);
        let strengths = g.memory_strengths();
        assert_eq!(strengths.len(), 2);
        assert!((strengths[0] - 2.0).abs() < f64::EPSILON);
        assert!((strengths[1] - 4.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_capacity_eviction() {
        let mut g = Griot::new(0.01, 2);
        g.add_story("A".into(), 5.0, None); // id 0
        g.add_story("B".into(), 1.0, None); // id 1
        g.add_story("C".into(), 10.0, None); // evicts B (lowest weight)
        assert_eq!(g.story_count(), 2);
        // B should be evicted
        assert!(g.find_story("B".into()).is_none());
        assert!(g.find_story("A".into()).is_some());
        assert!(g.find_story("C".into()).is_some());
    }

    // ── generate_praise tests ──

    #[test]
    fn test_generate_praise() {
        let mut g = Griot::new(0.01, 100);
        g.add_story("Sundiata".into(), 5.0, None); // id 0
        g.add_story("Mansa Musa".into(), 10.0, None); // id 1
        let praise = generate_praise(&g, vec![0, 1], "Lion King".into());
        assert_eq!(praise.name, "Lion King");
        assert_eq!(praise.story_count, 2);
        assert!((praise.combined_weight - 15.0).abs() < 1e-10);
    }

    #[test]
    fn test_generate_praise_invalid_ids() {
        let g = Griot::new(0.01, 100);
        let praise = generate_praise(&g, vec![99], "Empty".into());
        assert_eq!(praise.story_count, 0);
        assert!((praise.combined_weight - 0.0).abs() < 1e-10);
    }

    // ── call_and_response tests ──

    #[test]
    fn test_call_and_response_harmony() {
        let mut caller = Griot::new(0.01, 100);
        let mut responder = Griot::new(0.01, 100);
        caller.add_story("Epic".into(), 5.0, None);
        caller.tell_story("Epic".into());
        responder.add_story("Epic".into(), 3.0, None);
        responder.tell_story("Epic".into());
        let cr = call_and_response(&caller, &responder, "Epic".into());
        assert!(cr.harmony);
        assert_eq!(cr.caller_tell_count, 1);
        assert_eq!(cr.responder_tell_count, 1);
    }

    #[test]
    fn test_call_and_response_no_harmony_unknown() {
        let caller = Griot::new(0.01, 100);
        let responder = Griot::new(0.01, 100);
        let cr = call_and_response(&caller, &responder, "Missing".into());
        assert!(!cr.harmony);
    }

    #[test]
    fn test_call_and_response_no_harmony_untold() {
        let mut caller = Griot::new(0.01, 100);
        let mut responder = Griot::new(0.01, 100);
        caller.add_story("Epic".into(), 5.0, None);
        // caller hasn't told it
        responder.add_story("Epic".into(), 3.0, None);
        responder.tell_story("Epic".into());
        let cr = call_and_response(&caller, &responder, "Epic".into());
        assert!(!cr.harmony);
    }

    // ── genealogy tests ──

    #[test]
    fn test_genealogy_linear() {
        let mut g = Griot::new(0.01, 100);
        g.add_story("Root".into(), 5.0, None);       // id 0
        g.add_story("Child".into(), 3.0, Some(0));    // id 1
        g.add_story("Grandchild".into(), 1.0, Some(1)); // id 2
        let chain = genealogy(&g, 2);
        assert_eq!(chain, vec![2, 1, 0]);
    }

    #[test]
    fn test_genealogy_root() {
        let mut g = Griot::new(0.01, 100);
        g.add_story("Root".into(), 5.0, None);
        let chain = genealogy(&g, 0);
        assert_eq!(chain, vec![0]);
    }

    #[test]
    fn test_genealogy_invalid() {
        let g = Griot::new(0.01, 100);
        let chain = genealogy(&g, 99);
        assert_eq!(chain, vec![99]);
    }

    // ── Federation tests ──

    #[test]
    fn test_federation_new() {
        let f = Federation::new(5, 0.01);
        assert!((f.coverage() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_federation_coverage() {
        let mut f = Federation::new(4, 0.01);
        f.add_to_griot(0, "A".into(), 5.0, None);
        f.add_to_griot(2, "B".into(), 3.0, None);
        // 2 out of 4 have stories
        assert!((f.coverage() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_federation_sync_story() {
        let mut f = Federation::new(3, 0.01);
        f.add_to_griot(0, "Epic".into(), 10.0, None);
        f.sync_story(0, 1, "Epic".into());
        let g1 = f.get_griot(1).unwrap();
        assert!(g1.find_story("Epic".into()).is_some());
    }

    #[test]
    fn test_federation_sync_nonexistent() {
        let mut f = Federation::new(3, 0.01);
        f.sync_story(0, 1, "Ghost".into());
        let g1 = f.get_griot(1).unwrap();
        assert_eq!(g1.story_count(), 0);
    }

    #[test]
    fn test_federation_full_coverage() {
        let mut f = Federation::new(3, 0.01);
        f.add_to_griot(0, "A".into(), 1.0, None);
        f.add_to_griot(1, "B".into(), 1.0, None);
        f.add_to_griot(2, "C".into(), 1.0, None);
        assert!((f.coverage() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_decay_multiple_stories() {
        let mut g = Griot::new(0.5, 100);
        g.add_story("A".into(), 10.0, None);
        g.add_story("B".into(), 20.0, None);
        g.apply_decay(1.0);
        let w = g.memory_strengths();
        // Both decay by exp(-0.5) ≈ 0.6065
        assert!(w[0] < 10.0);
        assert!(w[1] < 20.0);
        assert!(w[0] > 5.0);
        assert!(w[1] > 10.0);
    }

    #[test]
    fn test_tradition_score_after_tell_and_decay() {
        let mut g = Griot::new(0.1, 100);
        g.add_story("Epic".into(), 10.0, None);
        g.tell_story("Epic".into()); // weight becomes 11.0
        g.apply_decay(1.0); // weight becomes 11 * exp(-0.1) ≈ 9.953
        let score = g.tradition_score();
        assert!(score < 11.0);
        assert!(score > 9.0);
    }

    #[test]
    fn test_get_story_by_id() {
        let mut g = Griot::new(0.01, 100);
        g.add_story("First".into(), 1.0, None);
        g.add_story("Second".into(), 2.0, None);
        let s = g.get_story(1).unwrap();
        assert_eq!(s.name, "Second");
    }
}
