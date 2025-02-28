use std::iter::Map;
use std::time::SystemTime;

#[derive(PartialEq)]
enum NodeType {
    THOUGHT,
    EVENT,
    CHAT,
}
struct Node {
    node_id: i64,
    node_type: NodeType,
    node_count: i64,
    depth: i64,
    creation: SystemTime,
    expiration: SystemTime,
    last_accessed: SystemTime,
    subject: String,
    predicate: String,
    object: String,
    description: String,
    embedding_key: i64,
    pognancy: f32,
    keywords: Vec<String>,
    filling: String,
}
impl Node {
    fn summary(&self) -> String {
        format!("{} {} {}", self.subject, self.predicate, self.object)
    }
}
struct AssociativeMemory {
    id_node: Map<i64, Node>,
    seq_nodes: Vec<Node>,
    embeddings: Vec<i64>,
    // keyword_nodes:
}
impl AssociativeMemory {
    fn select_nodes(&self, node_type: NodeType) -> Vec<&Node> {
        self.seq_nodes
            .iter()
            .filter(|p| p.node_type == node_type)
            .collect()
    }
    fn keyword_nodes(&self, keyword: String) -> Vec<&Node> {
        self.seq_nodes
            .iter()
            .filter(|p| p.keywords.contains(&keyword))
            .collect()
    }
}
