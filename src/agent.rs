use crate::nn::NeuralNetwork;
use rand::seq::SliceRandom;
use std::collections::VecDeque;
use std::fmt;

const MAX_MEMORY: usize = 100_000;
const BATCH_SIZE: usize = 1000;

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Action::Up => write!(f, "Up"),
            Action::Down => write!(f, "Down"),
            Action::Left => write!(f, "Left"),
            Action::Right => write!(f, "Right"),
        }
    }
}

pub struct Agent {
    pub neural_network: NeuralNetwork,
    pub gamma: f64,
    pub epsilon: f64,
    pub epsilon_decay: f64,
    pub min_epsilon: f64,
    pub memory: VecDeque<(Vec<i32>, Action, f64, Vec<i32>, bool)>,
}

impl Agent {
    pub fn new() -> Self {
        Self {
            neural_network: NeuralNetwork::new(12, 64, 4),
            gamma: 0.9,
            epsilon: 1.0,
            epsilon_decay: 0.9999,
            min_epsilon: 0.00,
            memory: VecDeque::with_capacity(MAX_MEMORY),
        }
    }

    pub fn select_action(&mut self, state: &[i32; 12]) -> Action {
        let random_float: f64 = rand::random::<f64>();
        self.epsilon = (self.epsilon * self.epsilon_decay).max(self.min_epsilon);
        //println!("Epsilon: {}, Float: {}", self.epsilon, random_float);
        let action = if random_float < self.epsilon {
            //println!("random");
            vec![Action::Up, Action::Down, Action::Left, Action::Right]
                .choose(&mut rand::thread_rng())
                .unwrap()
                .clone()
        } else {
            //println!("not random");

            let q_values = self.neural_network.forward(&state);
            let max_q_value_action = q_values
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0;
            match max_q_value_action {
                0 => Action::Up,
                1 => Action::Down,
                2 => Action::Left,
                3 => Action::Right,
                _ => panic!("Unexpected action index"),
            }
        };
        // println!("Action Taken :{}", action);
        action
    }

    pub fn remember(
        &mut self,
        state: &[i32; 12],
        action: Action,
        reward: f64,
        next_state: &[i32; 12],
        done: bool,
    ) {
        // If the memory is full, remove the oldest entry.
        if self.memory.len() == MAX_MEMORY {
            self.memory.pop_front();
        }
    
        // Create the data tuple.
        let data = (state.to_vec(), action, reward, next_state.to_vec(), done);
    
        // If the reward is positive, push the data 100 times.
        if reward > 0.0 {
            for _ in 0..10 {
                // Check for overflow in each iteration
                if self.memory.len() == MAX_MEMORY {
                    self.memory.pop_front();
                }
                self.memory.push_back(data.clone());
            }
        } else {
            self.memory.push_back(data);
        }
    }    

    pub fn train_long_memory(&mut self) {
        let sample_size = BATCH_SIZE.min(self.memory.len());
        let mini_sample: Vec<_> = self
            .memory
            .make_contiguous()
            .choose_multiple(&mut rand::thread_rng(), sample_size)
            .cloned()
            .collect();
        let states: Vec<_> = mini_sample.iter().map(|item| &item.0).collect();
        let actions: Vec<_> = mini_sample.iter().map(|item| &item.1).collect();
        let rewards: Vec<_> = mini_sample.iter().map(|item| item.2).collect();
        let next_states: Vec<_> = mini_sample.iter().map(|item| &item.3).collect();
        let dones: Vec<_> = mini_sample.iter().map(|item| item.4).collect();

        // Now the `train` function will need to be updated to accept batches instead of individual items.
        for i in 0..sample_size {
            let state_array: [i32; 12] = states[i].clone().try_into().expect("Wrong length");
            let next_state_array: [i32; 12] =
                next_states[i].clone().try_into().expect("Wrong length");
            self.train(
                &state_array,
                actions[i].clone(),
                rewards[i],
                &next_state_array,
                dones[i],
            );
        }
    }

    pub fn train(
        &mut self,
        state: &[i32; 12],
        action: Action,
        reward: f64,
        next_state: &[i32; 12],
        done: bool,
    ) {
        //println!("current_q_values_forward\n\n");
        let current_q_values = self.neural_network.forward(&state);
        // println!("current state {:?}", state);
        // println!("current q {:?}", current_q_values);
        //println!("next_q_values_forward\n\n");
        let next_q_values = self.neural_network.forward(&next_state);
        // println!("next state{:?}", next_state);
        // println!("next q {:?}", next_q_values);
        let mut target_q_values = current_q_values.clone();
        let next_max_q_value = *next_q_values
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let action_idx = match action {
            Action::Up => 0,
            Action::Down => 1,
            Action::Left => 2,
            Action::Right => 3,
        };
        let q_new = if done {
            reward
        } else {
            reward + self.gamma * next_max_q_value
        };

        target_q_values[action_idx] = q_new;

        self.neural_network
            .backward(&state, target_q_values, current_q_values);
    }
}
