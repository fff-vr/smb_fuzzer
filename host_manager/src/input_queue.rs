use std::collections::VecDeque;
use rand::Rng;
use rand::rngs::ThreadRng;
pub struct InputQueue{
    nego_queue :VecDeque<Vec<u8>>,
    setup_queue : VecDeque<Vec<u8>>,
    gettree_queue : VecDeque<Vec<u8>>,
    rng :  ThreadRng
}
impl InputQueue{
    fn get_nego(&mut self)->Vec<u8>{
        let idx = self.rng.gen_range(0..self.nego_queue.len());
        self.nego_queue[idx].clone()
    }
    fn get_setup(&mut self)->Vec<u8>{
        let idx = self.rng.gen_range(0..self.setup_queue.len());
        self.setup_queue[idx].clone()
    }
    fn get_gettree(&mut self)->Vec<u8>{
        let idx = self.rng.gen_range(0..self.gettree_queue.len());
        self.gettree_queue[idx].clone()
    }
    fn insert_nego(&mut self, new_input : Vec<u8>){
        self.nego_queue.push_back(new_input);
    }
    fn insert_setup(&mut self, new_input : Vec<u8>){
        self.setup_queue.push_back(new_input);
    }
    fn insert_gettree(&mut self, new_input : Vec<u8>){
        self.gettree_queue.push_back(new_input);
    }
    fn get_input(&mut self, command : u8)->Vec<u8>{
        match command{
            0=>self.get_nego(),
            1=>self.get_setup(),
            2=>self.get_gettree(),
            _=>panic!("Unknown Command")
        }
    }

}
