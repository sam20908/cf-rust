pub struct DSU<T: Clone> {
    size: Vec<usize>,
    par: Vec<usize>,
    data: Vec<T>,
}
impl<T: Clone> DSU<T> {
    pub fn new(n: usize, data_init: T) -> Self {
        DSU {
            size: vec![1; n],
            par: (0..n).collect(),
            data: vec![data_init; n],
        }
    }
    pub fn find(&mut self, x: usize) -> usize {
        if self.par[x] != x {
            self.par[x] = self.find(self.par[x]);
        }
        self.par[x]
    }
    pub fn unite<F: FnOnce(&T, &T) -> T>(&mut self, x: usize, y: usize, data_merge_fn: F) {
        let mut px = self.find(x);
        let mut py = self.find(y);
        if px == py {
            return;
        }
        if self.size[px] < self.size[py] {
            (px, py) = (py, px);
        }
        self.data[px] = data_merge_fn(&self.data[px], &self.data[py]);
        self.size[px] += self.size[py];
        self.par[py] = px;
    }
    pub fn size(&mut self, x: usize) -> usize {
        let par = self.find(x);
        self.size[par]
    }
    pub fn data(&mut self, x: usize) -> &T {
        let par = self.find(x);
        &self.data[par]
    }
}
