use crate::belief::BeliefState;
use petgraph::algo::dijkstra;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use rand::prelude::*;

const SIZE: f64 = 40.0;

#[derive(Clone)]
pub struct Game {
    pub board: Graph<u32, i32, Undirected>,
    pub target: NodeIndex,
    pub agent: NodeIndex,
}

impl Game {
    pub fn new() -> Game {
        let size = SIZE as u32;
        let random_connections = SIZE as i32 / 4;
        let mut graph = Graph::new_undirected();

        // create the graph with the edges making a circle
        graph.add_node(0);
        for index in 1..size {
            graph.add_node(index);
            graph.add_edge((index - 1).into(), index.into(), 1);
        }
        graph.add_edge(0.into(), (size - 1).into(), 1);

        // create random edges on that graph
        let mut rng = rand::thread_rng();
        for _ in 0..random_connections {
            // choose one node, and make sure it doesn't already have the max amount of connections
            let mut start_node: u32 = rng.gen_range(0..(size - 1).into());
            while graph.neighbors(start_node.into()).count() > 3 {
                start_node = rng.gen_range(0..(size - 1).into());
            }

            // choose another node, making sure it is
            // 1: not the same as the first node chosen
            // 2: does not correspond to an existing edge
            // 3: does not already have the max amount of connections
            let mut end_node = rng.gen_range(0..(size - 1).into());
            while start_node == end_node
                || graph.contains_edge(start_node.into(), end_node.into())
                || graph.neighbors(end_node.into()).count() > 2
            {
                end_node = rng.gen_range(0..(size - 1).into());
            }
            graph.add_edge(start_node.into(), end_node.into(), 1);
        }
        let target = rng.gen_range(0..(size - 1).into());
        Game {
            board: graph,
            target: target.into(),
            agent: 0.into(),
        }
    }

    pub fn move_target(&mut self) {
        let neighbors = self.board.neighbors(self.target);

        // this is guarenteed to have at least one neighbor, but we will error handle anyway
        let mut rng = rand::thread_rng();
        let chosen_neighbor = neighbors.choose(&mut rng);
        match chosen_neighbor {
            Some(neighbor) => self.target = neighbor,
            None => (),
        }
    }

    pub fn target_distance(&self) -> i32 {
        let node_map = dijkstra(&self.board, self.agent, Some(self.target), |_| 1);
        let distance = node_map.get(&self.target);

        *distance.unwrap()
    }

    pub fn agent_0(&mut self) -> i32 {
        let mut count = 0;

        while self.target != self.agent {
            self.move_target();
            count = count + 1;
        }
        count
    }

    pub fn agent_1(&mut self) -> i32 {
        let mut count = 0;
        let mut rng = rand::thread_rng();

        while self.target_distance() != 0 {
            self.move_target();

            // move the agent closer
            let node_map = dijkstra(&self.board, self.agent, Some(self.target), |_| 1);
            let agent_distance = node_map.get(&self.target).unwrap();
            let neighbors: Vec<NodeIndex<_>> = self
                .board
                .neighbors(self.agent)
                .filter(|possible_move| {
                    node_map.get(&possible_move).unwrap_or(agent_distance) < agent_distance
                })
                .collect();
            if neighbors.len() == 0 {
                continue;
            }
            let choice = neighbors.choose(&mut rng);
            match choice {
                Some(neighbor) => self.agent = *neighbor,
                None => (),
            }
            count = count + 1;
        }
        count
    }

    // this is where the agent will choose the best move available
    pub fn agent_2(&mut self) -> i32 {
        let mut count = 0;
        let mut rng = rand::thread_rng();

        while self.target_distance() != 0 {
            self.move_target();

            // move the agent closer
            let node_map = dijkstra(&self.board, self.agent, Some(self.target), |_| 1);
            let agent_distance = node_map.get(&self.target).unwrap();
            let neighbors: Vec<NodeIndex<_>> = self
                .board
                .neighbors(self.agent)
                .filter(|possible_move| {
                    node_map.get(&possible_move).unwrap_or(agent_distance) < agent_distance
                })
                .collect();
            if neighbors.len() == 0 {
                continue;
            }
            let mut choice = neighbors.choose(&mut rng).unwrap();
            for neighbor in &neighbors {
                if node_map.get(neighbor).unwrap_or(&(SIZE as i32)) < node_map.get(choice).unwrap_or(&(SIZE as i32 - 1)) {
                    choice = neighbor;
                }
            }
            self.agent = *choice;
            count = count + 1;
        }
        count
    }

    // observe a random spot on the grid and see if the target is there
    pub fn agent_3(&mut self) -> i32 {
        let mut count = 0;
        let mut rng = rand::thread_rng();
        self.agent = rng.gen_range(0..39).into();

        while self.target != self.agent {
            self.move_target();
            count = count + 1;
        }
        count
    }

    pub fn agent_4(&mut self) -> i32 {
        let mut belief_state = BeliefState::new(self.board.clone());
        let mut count = 0;
        let mut rng = rand::thread_rng();

        while self.agent != self.target {
            self.move_target();

            // use the belief state to randomly select a node
            let choices = belief_state.get_most_likely();
            let choice = choices.choose(&mut rng).unwrap();

            // this means we are inspecting the board at the most likely spot for the target to be
            self.agent = *choice;

            // see if the target is there (this could just be false but for thouroughness we will check)
            let result = self.agent == self.target;

            // update the boards probabilities with the observation data
            belief_state.update_with_observation(self.agent.index(), result);
            // update the boards probabilities with the transitional model
            belief_state.update_with_transition();
            
            count = count + 1;
        }
        count
    }

    pub fn agent_5(&mut self) -> i32 {
        let mut belief_state = BeliefState::new(self.board.clone());
        let mut count = 0;
        let mut rng = rand::thread_rng();

        while self.agent != self.target {
            self.move_target();

            // use the belief state to randomly select a node
            // first we get a list of all the nodes that we can choose from
            let choices = belief_state.get_most_likely();

            // we will consider the nodes that are the most connected
            let mut choice = choices.choose(&mut rng).unwrap();
            for potential_choice in choices.iter() {
                if self.board.neighbors(*potential_choice).count() > self.board.neighbors(*choice).count() {
                    choice = potential_choice;
                }
            }

            // this means we are inspecting the board at the most likely spot for the target to be
            self.agent = *choice;

            // see if the target is there (this could just be false but for thouroughness we will check)
            let result = self.agent == self.target;

            // update the boards probabilities with the observation data
            belief_state.update_with_observation(self.agent.index(), result);
            // update the boards probabilities with the transitional model
            belief_state.update_with_transition();
            
            count = count + 1;
        }
        count
    }

    pub fn agent_6(&mut self) -> i32 {
        let mut belief_state = BeliefState::new(self.board.clone());
        let mut count = 0;
        let mut rng = rand::thread_rng();

        while self.agent != self.target {
            self.move_target();

            // use the belief state to randomly select a node
            let choices = belief_state.get_most_likely();
            let likely_target = choices.choose(&mut rng).unwrap();

            // move the agent closer to that node with the highest probability
            // move the agent closer
            let node_map = dijkstra(&self.board, self.agent, Some(*likely_target), |_| 1);
            let agent_distance = node_map.get(likely_target).unwrap();
            let neighbors: Vec<NodeIndex<_>> = self
                .board
                .neighbors(self.agent)
                .filter(|possible_move| {
                    node_map.get(&possible_move).unwrap_or(agent_distance) < agent_distance
                })
                .collect();
            if neighbors.len() == 0 {
                continue;
            }
            let mut choice = neighbors.choose(&mut rng).unwrap();
            for neighbor in &neighbors {
                if node_map.get(neighbor).unwrap_or(&(SIZE as i32)) < node_map.get(choice).unwrap_or(&(SIZE as i32 - 1)) {
                    choice = neighbor;
                }
            }

            // move the agent to the spot that is closer to that likely square
            self.agent = *choice;

            // see if the target is there (this could just be false but for thouroughness we will check)
            let result = *likely_target == self.target;

            // update the boards probabilities with the observation data
            belief_state.update_with_observation(likely_target.index(), result);
            // update the boards probabilities with the transitional model
            belief_state.update_with_transition();

            count = count + 1;
        }
        count
    }

    pub fn agent_7(&mut self) -> i32 {
        let mut belief_state = BeliefState::new(self.board.clone());
        let mut count = 0;
        let mut rng = rand::thread_rng();

        while self.agent != self.target {
            self.move_target();

            // use the belief state to randomly select a node
            let choices = belief_state.get_most_likely();
            // we will consider the nodes that are the most connected
            let mut likely_target = choices.choose(&mut rng).unwrap();
            for potential_choice in choices.iter() {
                if self.board.neighbors(*potential_choice).count() > self.board.neighbors(*likely_target).count() {
                    likely_target = potential_choice;
                }
            }
            

            // move the agent closer to that node with the highest probability
            // move the agent closer
            let node_map = dijkstra(&self.board, self.agent, Some(*likely_target), |_| 1);
            let agent_distance = node_map.get(likely_target).unwrap();
            let neighbors: Vec<NodeIndex<_>> = self
                .board
                .neighbors(self.agent)
                .filter(|possible_move| {
                    node_map.get(&possible_move).unwrap_or(agent_distance) < agent_distance
                })
                .collect();
            if neighbors.len() == 0 {
                continue;
            }
            let mut choice = neighbors.choose(&mut rng).unwrap();
            for neighbor in &neighbors {
                if node_map.get(neighbor).unwrap_or(&(SIZE as i32)) < node_map.get(choice).unwrap_or(&(SIZE as i32 - 1)) {
                    choice = neighbor;
                }
            }

            // move the agent to the spot that is closer to that likely square
            self.agent = *choice;

            // see if the target is there (this could just be false but for thouroughness we will check)
            let result = *likely_target == self.target;

            // update the boards probabilities with the observation data
            belief_state.update_with_observation(likely_target.index(), result);
            // update the boards probabilities with the transitional model
            belief_state.update_with_transition();

            count = count + 1;
        }
        count
    }
}
