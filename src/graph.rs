use std::collections::{HashMap, HashSet, VecDeque};
use std::default::Default;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
struct Vertex<T> {
    id: String,
    value: T,
}

impl<T> fmt::Display for Vertex<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.value)
    }
}

impl<T> Vertex<T>
where
    T: Default + Clone,
{
    fn new(id: &String, value: &T) -> Self {
        Vertex {
            id: id.clone(),
            value: value.clone(),
        }
    }
}

use i32 as Weight;

#[derive(Debug, Clone)]
struct Edge {
    src: String,
    dst: String,
    weight: Weight,
}

impl Edge {
    fn new(src: &String, dst: &String) -> Self {
        Edge {
            src: src.clone(),
            dst: dst.clone(),
            weight: 1,
        }
    }
    fn with_weight(mut self, weight: Weight) -> Self {
        self.weight = weight;
        self
    }
    fn contains(&self, vert_id: &String) -> bool {
        self.src.eq(vert_id) || self.dst.eq(vert_id)
    }
}

impl std::cmp::PartialEq for Edge {
    fn eq(&self, rhs: &Edge) -> bool {
        format!("{}{}", self.src, self.dst) == format!("{}{}", rhs.src, rhs.dst)
    }
    fn ne(&self, rhs: &Edge) -> bool {
        format!("{}{}", self.src, self.dst) != format!("{}{}", rhs.src, rhs.dst)
    }
}

impl std::cmp::Eq for Edge {}

impl std::hash::Hash for Edge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{}{}", self.src, self.dst).hash(state);
    }
}

// adjacency list implementation
#[derive(Debug)]
pub struct Graph<T> {
    verts: HashMap<String, Vertex<T>>, // vertex id to vertex. vertex stores information such as a payload
    edges: HashSet<Edge>,              // set of edges in the graph
    adjacency_lists: HashMap<String, HashSet<String>>, // vertex id to list of adjacent vertices by vertex id
}

impl<T> Graph<T>
where
    T: Default + Clone,
{
    pub fn new() -> Graph<T> {
        Graph {
            verts: HashMap::new(),
            edges: HashSet::new(),
            adjacency_lists: HashMap::new(),
        }
    }

    // adds the vertex x, if it is not there
    pub fn add_vertex(&mut self, id: &String, value: &T) -> bool {
        let v = Vertex::new(id, value);
        if let None = self.verts.get(&v.id) {
            self.verts.insert(v.id.clone(), v);
            true
        } else {
            false
        }
    }

    // removes the vertex x and all of its edges
    pub fn remove_vertex(&mut self, id: &String) -> Option<T> {
        let mut edges_to_remove: Vec<Edge> = Vec::new();
        for e in &self.edges {
            if e.contains(id) {
                edges_to_remove.push(e.clone());
            }
        }
        for e in edges_to_remove {
            self.edges.remove(&e);
        }

        let result = self.verts.remove(id);
        self.adjacency_lists.remove(id);

        for (_, adjacency_list) in &mut self.adjacency_lists {
            adjacency_list.remove(id);
        }

        if let Some(vertex) = result {
            return Some(vertex.value);
        }
        None
    }

    // dds the edge from the vertex x to the vertex y, if it is not there;
    pub fn add_weighted_edge(&mut self, src: &String, dst: &String, weight: Weight) -> bool {
        if let None = self.verts.get(src) {
            return false;
        }

        if let None = self.verts.get(dst) {
            return false;
        }

        if !self.edges.insert(Edge::new(src, dst).with_weight(weight)) {
            return false;
        }

        if let Some(adjacency_list) = self.adjacency_lists.get_mut(src) {
            adjacency_list.insert(dst.clone())
        } else {
            let mut adjacency_list = HashSet::new();
            let result = adjacency_list.insert(dst.clone());
            self.adjacency_lists.insert(src.clone(), adjacency_list);
            result
        }
    }

    pub fn add_edge(&mut self, src: &String, dst: &String) -> bool {
        self.add_weighted_edge(src, dst, 1)
    }

    // removes the edge from the vertex x to the vertex y, if it is there
    pub fn remove_edge(&mut self, src: &String, dst: &String) -> bool {
        self.edges.remove(&Edge::new(src, dst));

        if let Some(adjacency_list) = self.adjacency_lists.get_mut(src) {
            return adjacency_list.remove(dst);
        }
        false
    }

    // lists all vertices y such that there is an edge from the vertex x to the vertex y
    pub fn get_adjacent_verts(&self, id: &String) -> Option<HashSet<String>> {
        if let Some(adjacency_list) = self.adjacency_lists.get(id) {
            return Some(adjacency_list.clone());
        }
        None
    }

    // tests whether there is an edge from the vertex x to the vertex y
    pub fn is_adjacent(&self, src: &String, dst: &String) -> bool {
        if let Some(adjacency_list) = self.adjacency_lists.get(src) {
            for adjacent_vert in adjacency_list {
                if adjacent_vert == dst {
                    return true;
                }
            }
        }
        false
    }

    //  returns the value associated with the vertex x;
    pub fn get_value(&self, id: &String) -> Option<T> {
        if let Some(v) = self.verts.get(id) {
            Some(v.value.clone())
        } else {
            None
        }
    }

    // sets the value associated with the vertex x to v.
    pub fn set_value(&mut self, id: &String, value: &T) -> Option<T> {
        if let Some(v) = self.verts.get_mut(id) {
            let previous = v.value.clone();
            v.value = value.clone();
            Some(previous)
        } else {
            None
        }
    }

    // checks if the edge src->dst exists
    pub fn has_edge(&self, src: &String, dst: &String) -> bool {
        if let Some(_) = self.edges.get(&Edge::new(src, dst)) {
            true
        } else {
            false
        }
    }

    // 'dst' could be some other criteria
    // breadth first search.
    // as it stands this function is useless but can be modified to find a given vertex in the graph
    pub fn search(&self, src: &String, dst: &String) -> Option<String> {
        let mut queue: VecDeque<String> = VecDeque::new();
        let mut marked_verts: HashSet<String> = HashSet::new();
        queue.push_back(src.clone());
        marked_verts.insert(src.clone());

        while queue.len() > 0 {
            let w = queue.pop_front().unwrap();

            if w.eq(dst) {
                return Some(w);
            }

            if let Some(a) = self.get_adjacent_verts(&w) {
                for x in &a {
                    if !marked_verts.contains(x) {
                        marked_verts.insert(x.clone());
                        queue.push_back(x.clone());
                    }
                }
            }
        }
        None
    }

    // implementation of dijkstra's algorithm to find the shortest path between two vertices
    pub fn shortest_path(&self, src: &String, dst: &String) -> Vec<String> {
        if let None = self.verts.get(src) {
            return vec![];
        }

        if let None = self.verts.get(dst) {
            return vec![];
        }

        // dist[u] is the current distance from the source to vertex u
        let mut dist: HashMap<String, Weight> = HashMap::new();

        // prev contains pointers to previous hop nodes on the shortest path
        //   or the next hp on the path from the given vertex to the source
        let mut prev: HashMap<String, String> = HashMap::new();

        // Q is a set of vertices that has the least dist[u] value
        let mut q: Vec<String> = Vec::new();

        for (v, _) in &self.verts {
            dist.entry(v.clone()).or_insert(Weight::MAX);
            prev.entry(v.clone()).or_insert("".to_string());
            q.push(v.clone());
        }
        *dist.entry(src.clone()).or_insert(Weight::MAX) = 0;

        while q.len() > 0 {
            // vertex in Q with min dist[u]
            let mut u: String = q.get(0).unwrap().clone();
            let mut min: Weight = *dist.get(&u).unwrap();
            let mut rmpos = 0;
            for (pos, v) in q.iter().enumerate() {
                let v_min = *dist.get(v).unwrap();
                if v_min < min {
                    u = v.clone();
                    min = v_min;
                    rmpos = pos;
                }
            }

            if u.eq(dst) || min == Weight::MAX {
                break;
            }

            q.remove(rmpos);

            for v in &q {
                if !self.is_adjacent(&u, v) {
                    continue;
                }
                let w = self.edges.get(&Edge::new(&u, v)).unwrap().weight;
                let alt: Weight = dist.get(&u).unwrap() + w;
                if alt < *dist.get(v).unwrap() {
                    *dist.entry(v.clone()).or_insert(Weight::MAX) = alt;
                    *prev.entry(v.clone()).or_insert("".to_string()) = u.clone();
                }
            }
        }

        let mut s: Vec<String> = Vec::new();
        let mut u: String = dst.clone();
        if prev.get(&u).unwrap().ne("") || u.eq(src) {
            while u.ne("") {
                s.push(u.clone());
                u = prev.get(&u).unwrap().clone();
            }
        }

        s.reverse();

        return s;
    }
}

impl<T> fmt::Display for Graph<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for (vert, edges) in &self.adjacency_lists {
            for (idx, conn) in edges.iter().enumerate() {
                s += format!("{}->{}", vert, conn).as_str();
                if idx < edges.len() - 1 {
                    s += ","
                }
            }
            s += ";"
        }
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod test {
    use super::Graph;

    #[derive(Debug, PartialEq, Clone)]
    struct Foo {
        property: i32,
    }

    impl std::default::Default for Foo {
        fn default() -> Self {
            Foo { property: 0 }
        }
    }

    impl Foo {
        fn new() -> Self {
            Foo { property: 0 }
        }
    }
    #[test]
    fn modify_graph() {
        let mut g: Graph<Foo> = Graph::new();
        let invalid_id = String::from("foo");
        let id1 = String::from("a");
        let id2 = String::from("b");
        let f1 = Foo::new();
        let f2 = Foo::new();

        // insert vertex
        assert_eq!(g.add_vertex(&id1, &f1), true);
        assert_eq!(g.add_vertex(&id1, &f1), false);
        assert_eq!(g.add_vertex(&id2, &f2), true);

        // connect vertices
        assert_eq!(g.add_edge(&id1, &id2), true);
        assert_eq!(g.add_edge(&id1, &id2), false);
        assert_eq!(g.add_edge(&id1, &invalid_id), false);

        assert_eq!(g.add_edge(&id2, &id1), true);

        // check connections
        assert_eq!(g.get_adjacent_verts(&id1).unwrap().len(), 1);
        assert_eq!(g.get_adjacent_verts(&id1).unwrap().contains(&id2), true);
        assert_eq!(
            g.get_adjacent_verts(&id1).unwrap().contains(&invalid_id),
            false
        );

        assert_eq!(g.get_adjacent_verts(&id2).unwrap().len(), 1);
        assert_eq!(g.get_adjacent_verts(&id2).unwrap().contains(&id1), true);
        assert_eq!(
            g.get_adjacent_verts(&id2).unwrap().contains(&invalid_id),
            false
        );

        assert_eq!(g.get_adjacent_verts(&invalid_id), None);

        // remove vertex
        assert_eq!(g.remove_vertex(&id1), Some(f1.clone()));
        assert_eq!(g.get_adjacent_verts(&id1), None);
        assert_eq!(g.get_adjacent_verts(&id2).unwrap().len(), 0);

        // remove edge
        g.add_vertex(&id1, &f1);
        g.add_edge(&id1, &id2);
        g.add_edge(&id2, &id1);

        assert_eq!(g.get_adjacent_verts(&id1).unwrap().len(), 1);
        assert_eq!(g.get_adjacent_verts(&id1).unwrap().contains(&id2), true);

        assert_eq!(g.get_adjacent_verts(&id2).unwrap().len(), 1);
        assert_eq!(g.get_adjacent_verts(&id2).unwrap().contains(&id1), true);

        g.remove_edge(&id1, &id2);
        assert_eq!(g.get_adjacent_verts(&id1).unwrap().len(), 0);
        assert_eq!(g.get_adjacent_verts(&id2).unwrap().len(), 1);
        assert_eq!(g.get_adjacent_verts(&id2).unwrap().contains(&id1), true);
    }

    #[test]
    fn test_remove() {
        let mut g: Graph<Foo> = Graph::new();

        let id1 = String::from("a");
        let id2 = String::from("b");
        let id3 = String::from("c");
        let f1 = Foo::new();
        let f2 = Foo::new();
        let f3 = Foo::new();

        g.add_vertex(&id1, &f1);
        g.add_vertex(&id2, &f2);
        g.add_vertex(&id3, &f3);

        g.add_edge(&id1, &id2);
        g.add_edge(&id2, &id1);
        g.add_edge(&id3, &id1);

        g.remove_vertex(&id1);

        assert_eq!(g.get_adjacent_verts(&id1), None);
        assert_eq!(g.get_adjacent_verts(&id2).unwrap().len(), 0);
        assert_eq!(g.get_adjacent_verts(&id3).unwrap().len(), 0);
        assert_eq!(g.has_edge(&id1, &id2), false);
        assert_eq!(g.has_edge(&id2, &id1), false);
        assert_eq!(g.has_edge(&id3, &id1), false);
    }

    #[test]
    fn get_set_value() {
        let mut g: Graph<Foo> = Graph::new();
        let id1 = String::from("a");
        let id2 = String::from("b");
        let f1 = Foo { property: 1 };
        let f2 = Foo::new();

        g.add_vertex(&id1, &f1);
        g.add_vertex(&id2, &f2);
        g.add_edge(&id1, &id2);

        assert_eq!(g.get_value(&id1), Some(f1.clone()));

        let f1 = Foo { property: 2 };
        g.set_value(&id1, &f1);
        assert_eq!(g.get_value(&id1), Some(f1.clone()));
    }

    #[test]
    fn test_graph_fmt() {
        let mut g: Graph<Foo> = Graph::new();
        let id1 = String::from("a");
        let id2 = String::from("b");
        let id3 = String::from("c");
        let f1 = Foo::new();
        let f2 = Foo::new();
        let f3 = Foo::new();

        g.add_vertex(&id1, &f1);
        g.add_vertex(&id2, &f2);
        g.add_vertex(&id3, &f3);

        g.add_edge(&id1, &id2);
        g.add_edge(&id1, &id3);
        g.add_edge(&id2, &id1);

        let s = format!("{}", g);
        // assert_eq!("a->b,a->c;b->a;", format!("{}", g)) // ; this doesnt work because order isn't guaranteed iterating over hash map
        assert_eq!(s.contains("a->b"), true);
        assert_eq!(s.contains("a->c"), true);
        assert_eq!(s.contains("b->a;"), true);
    }

    #[test]
    fn search_graph() {
        let mut g: Graph<Foo> = Graph::new();
        let id1 = String::from("a");
        let id2 = String::from("b");
        let id3 = String::from("c");
        let id4 = String::from("d");
        let id5 = String::from("e");
        let f1 = Foo::new();
        let f2 = Foo::new();
        let f3 = Foo::new();
        let f4 = Foo::new();
        let f5 = Foo::new();

        g.add_vertex(&id1, &f1);
        g.add_vertex(&id2, &f2);
        g.add_vertex(&id3, &f3);
        g.add_vertex(&id4, &f4);
        g.add_vertex(&id5, &f5);

        g.add_edge(&id1, &id2);
        g.add_edge(&id1, &id5);
        g.add_edge(&id2, &id3);
        g.add_edge(&id2, &id1);
        g.add_edge(&id3, &id1);

        assert_eq!(g.search(&id1, &id3), Some(id3.clone()));
        assert_eq!(g.search(&id1, &id4), None);
    }

    #[test]
    fn find_shortest_path() {
        let mut g: Graph<Foo> = Graph::new();
        let id1 = String::from("a");
        let id2 = String::from("b");
        let id3 = String::from("c");
        let id4 = String::from("d");
        let id5 = String::from("e");
        let f1 = Foo::new();
        let f2 = Foo::new();
        let f3 = Foo::new();
        let f4 = Foo::new();
        let f5 = Foo::new();

        g.add_vertex(&id1, &f1);
        g.add_vertex(&id2, &f2);
        g.add_vertex(&id3, &f3);
        g.add_vertex(&id4, &f4);
        g.add_vertex(&id5, &f5);

        g.add_edge(&id1, &id2);
        g.add_edge(&id1, &id5);
        g.add_edge(&id2, &id3);
        g.add_edge(&id2, &id1);
        g.add_edge(&id3, &id1);

        // valid path through the graph
        assert_eq!(g.shortest_path(&id1, &id2), ["a", "b"]);
        assert_eq!(g.shortest_path(&id1, &id3), ["a", "b", "c"]);

        // no path from vertex 1 to vertex 2
        assert_eq!(g.shortest_path(&id5, &id1).len(), 0);

        // just for sanity make sure the path id1->id5 we added works
        assert_eq!(g.shortest_path(&id1, &id5), ["a", "e"]);

        // path to a vertex that is not connected
        assert_eq!(g.shortest_path(&id1, &id4).len(), 0);

        // path to nonexistant vertex
        assert_eq!(g.shortest_path(&id1, &"foo".to_string()).len(), 0);
    }
}
