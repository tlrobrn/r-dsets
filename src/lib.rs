/// Collection of disjoint sets.
///
/// Facilitates finding the root of the set containing any element
/// as well as merging the sets containing any two elements.
///
/// # Example
///
/// ```rust
/// use disjoint_sets::DisjointSets;
///
/// let mut dsets = DisjointSets::new(10);
/// assert_eq!(10, dsets.size());
///
/// dsets.set_union(0, 9);
/// dsets.set_union(0, 8);
/// dsets.set_union(1, 2);
/// dsets.set_union(2, 8);
///
/// match dsets.find_root(1) {
///     Ok(root) => assert_eq!(0, root),
///     Err(e) => panic!(e),
/// }
/// ```
pub struct DisjointSets { sets: Vec<i64> }

impl DisjointSets {
    /// Create a new DisjointSets object with the given number of sets.
    ///
    /// # Example
    ///
    /// ```rust
    /// use disjoint_sets::DisjointSets;
    ///
    /// let dsets = DisjointSets::new(10);
    /// assert_eq!(10, dsets.size());
    /// ```
    pub fn new(size: usize) -> DisjointSets {
        DisjointSets { sets: vec![-1; size] }
    }

    /// Add the given number of sets to this collection.
    ///
    /// # Example
    ///
    /// ```rust
    /// use disjoint_sets::DisjointSets;
    ///
    /// let mut dsets = DisjointSets::new(0);
    /// assert_eq!(0, dsets.size());
    ///
    /// dsets.add_sets(10);
    /// assert_eq!(10, dsets.size());
    /// ```
    pub fn add_sets(&mut self, count: usize) {
        for _ in 0..count { self.sets.push(-1); }
    }

    /// Find the root of the set containing the given element.
    ///
    /// # Example
    ///
    /// ```rust
    /// use disjoint_sets::DisjointSets;
    ///
    /// let mut dsets = DisjointSets::new(3);
    /// match dsets.find_root(0) {
    ///     Ok(root) => assert_eq!(0, root),
    ///     Err(e) => panic!(e),
    /// }
    /// ```
    pub fn find_root(&mut self, element: usize) -> Result<usize, &'static str> {
        if element > self.size() { return Err("Out of bounds") }

        if self.sets[element] < 0 { return Ok(element) }

        let parent:usize = self.sets[element] as usize;
        self.sets[element] = try!(self.find_root(parent)) as i64;

        Ok(self.sets[element] as usize)
    }

    /// Union the two sets containing the given arguments.
    ///
    /// # Example
    ///
    /// ```rust
    /// use disjoint_sets::DisjointSets;
    ///
    /// let mut dsets = DisjointSets::new(3);
    /// dsets.set_union(0, 2);
    /// match dsets.find_root(2) {
    ///     Ok(root) => assert_eq!(0, root),
    ///     Err(e) => panic!(e),
    /// }
    /// ```
    pub fn set_union(&mut self, a: usize, b: usize) {
        match (self.find_root(a), self.find_root(b)) {
            (Ok(root_a), Ok(root_b)) => {
                if root_a == root_b { return }

                if self.sets[root_a] <= self.sets[root_b] {
                    self.sets[root_a] += self.sets[root_b];
                    self.sets[root_b] = root_a as i64;
                }
                else {
                    self.sets[root_b] += self.sets[root_a];
                    self.sets[root_a] = root_b as i64;
                }
            },
            _ => panic!("Unable to get roots in order to merge sets."),
        }
    }

    /// Return the number of sets contained by this instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use disjoint_sets::DisjointSets;
    ///
    /// let dsets = DisjointSets::new(0);
    /// assert_eq!(0, dsets.size());
    /// ```
    pub fn size(&self) -> usize {
        self.sets.len()
    }
}
