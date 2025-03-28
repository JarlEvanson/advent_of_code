
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct UnionFind {
    parents: Box<[usize]>,
    sizes: Box<[usize]>,
}


impl UnionFind {
    pub fn new(node_count: usize) -> Self {
        Self {
            parents: (0..node_count).collect::<Box<[usize]>>(),
            sizes: (0..node_count).collect::<Box<[usize]>>(),
        }
    }

    pub fn find_const(&self, mut node: usize) -> usize {
        while self.parents[node] != node {
            node = self.parents[node];
        }

        node
    }

    pub fn find(&mut self, mut node: usize) -> usize {
        let mut root = node;
        while self.parents[root] != root {
            root = self.parents[root];
        }

        while self.parents[node] != node {
            let tmp = self.parents[node];
            self.parents[node] = root;
            node = tmp;
        }

        root
    }

    pub fn union(&mut self, node_0: usize, node_1: usize) {
        let root_0 = self.find(node_0);
        let root_1 = self.find(node_1);

        if root_0 != root_1 {
            if self.sizes[root_0] < self.sizes[root_1] {
                self.sizes[root_0] += self.sizes[root_1];
                self.parents[root_0] = root_1;
            } else {
                self.sizes[root_1] += self.sizes[root_0];
                self.parents[root_1] = root_0;
            }
        }
    }
}
