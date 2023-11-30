use rand::Rng;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Clone,Debug)]
pub struct Fragment {
    pub offset: usize,
    pub value: u8,
}
#[derive(Clone,Debug)]
pub struct Fragments {
    datas: Vec<Fragment>,
}
impl Fragments {
    pub fn new() -> Self {
        Self {
            datas: Default::default(),
        }
    }
    pub fn len(self) -> usize {
        self.datas.len()
    }
    pub fn push(&mut self, fragment: Fragment) {
        self.datas.push(fragment);
    }
    pub fn iter(&self) -> std::slice::Iter<'_, Fragment> {
        self.datas.iter()
    }
}

pub struct InputQueue {
    queue: VecDeque<HashMap<u32, Fragments>>,
}
impl InputQueue {
    pub fn new() -> Self {
        Self {
            queue: Default::default(),
        }
    }
    pub fn len(&self)->usize{
        self.queue.len()
    }

    pub fn get_input(&mut self, packet_count: u32) -> Fragments {
        let valid_corquses: VecDeque<&HashMap<u32, Fragments>> = self
            .queue
            .iter()
            .filter(|&hashmap| hashmap.contains_key(&packet_count))
            .collect();
        match valid_corquses.len(){
            0=> Fragments::new(),
            1=> {
                let corpus = valid_corquses[0];
                corpus.get(&packet_count).expect("fail to get fragment").clone()
            }
            _=>{
                let idx = rand::thread_rng().gen_range(0..valid_corquses.len()-1);
                let corpus = valid_corquses[idx];
                corpus.get(&packet_count).expect("fail to get fragment").clone()
            }
        }
        
    }
    pub fn insert_input(&mut self, new_input: HashMap<u32, Fragments>) {
        self.queue.push_back(new_input);
    }
}
