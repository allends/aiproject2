use petgraph::Undirected;
use petgraph::graph::{Graph, NodeIndex};

const SIZE: f64 = 40.0;

#[derive (Clone, Default)]
struct Node {
  probability: f64,
  neighbor_count: usize,
  neighbor_indicies: Vec<usize>,
}

impl Node {
  pub fn _new() -> Node {
    Node {
      probability: 0.0,
      neighbor_count: 0,
      neighbor_indicies: vec![],
    }
  }

  
}


pub struct BeliefState {
  graph: Vec<Node>,
}

impl BeliefState {
  pub fn new(board: Graph<u32, i32, Undirected>) -> BeliefState {
    let mut graph = vec![];
    for (_index, node_index) in board.node_indices().enumerate() {
      let neighbors = board.neighbors(node_index).count();
      let node = Node {
        probability: 1.0 / SIZE,
        neighbor_count: neighbors,
        neighbor_indicies: board.neighbors(node_index).map(|neighbor| neighbor.index()).collect(),
      };
      graph.push(node);
    }

    BeliefState { graph }
  }

  pub fn update_with_observation(&mut self, location: usize, target_present: bool) {
    
    // we see the target so we can set its location with 100% certainty
    if target_present {
      for index in 0..SIZE as usize {
        self.graph[index].probability = 0.0;
      }
      self.graph[location].probability = 1.0;
      return;
    }

    // here we will weight the rest of the graph more heavily because we can elimate one of the locations
    // equally distribute the probability of the node that was found empty
    self.graph[location].probability = 0.0;
    self.normalize();
  }

  pub fn update_with_transition(&mut self) {
    let mut new_board = self.graph.clone();
    for index in 0..SIZE as usize {
      // starting our sum here for each of the nodes
      let mut probability = 0.0;
      for neighbor_index in self.graph[index].neighbor_indicies.iter() {
        probability += self.graph[*neighbor_index].probability * (1.0 / self.graph[*neighbor_index].neighbor_count as f64);
      }
      new_board[index].probability = probability;
    }
    self.graph = new_board;
  }

  pub fn get_most_likely(&self) -> Vec<NodeIndex> {
    let mut highest_probability = 0.0;
    let mut result = vec![];
    for (index, node) in self.graph.iter().enumerate() {
      if node.probability > highest_probability {
        result = vec![(index as u32).into()];
        highest_probability = node.probability;
      } else if node.probability == highest_probability {
        result.push((index as u32).into());
      }
    }
    result
  }

  pub fn normalize(&mut self) {
    let mut total = 0.0;
    for node in self.graph.iter() {
      total = total + node.probability;
    }

    let beta = 1.0 / total;
    for (index, node) in self.graph.clone().iter().enumerate() {
      self.graph[index].probability = node.probability * beta;
    }
  }

  pub fn _check_validity(&self) {
    let mut total = 0.0;
    for node in self.graph.iter() {
      total = total + node.probability;
    }
    println!("the total is {}", total);
  }


}