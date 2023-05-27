mod block;

use rand::Rng;
use sha2::{Digest, Sha256};

//se usassi block::Blockchain ogni volta questo use non sarebbe necessario perchè block è un sottomodulo di blockchain
use crate::blockchain::block::Block;

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    pub fn return_head(&self) -> Block {
        return self.blocks[0].clone();
    }

    pub fn add_block(&mut self, data: String) {
        let previous_block = self.blocks.last().unwrap();
        let new_block = Block::new(
            previous_block.index + 1,
            data,
            previous_block.hash.clone(),
        );
        self.blocks.push(new_block);
    }

    pub fn print_blocks(&self) {
        for block in &self.blocks {
            println!("{:#?}", block);
        }
    }

    pub fn tamper_block(&mut self) -> usize{
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(1..=self.blocks.len()-1);
        println!("tampering block {}", random_index);
        self.blocks[random_index].data = "TAMPERED_DATA".to_string();


        let input = format!("{}{}{}{}", self.blocks[random_index].index, 
                                        self.blocks[random_index].data, 
                                        self.blocks[random_index].previous_hash, 
                                        self.blocks[random_index].timestamp);


        let mut hasher = Sha256::new();
        hasher.update(input);
        let new_hash = format!("{:x}", hasher.finalize());

        self.blocks[random_index].hash = new_hash;

        return random_index;

    }

    pub fn tamper_multiple_block(&mut self) {
        let index = self.tamper_block();
        for i in (1..index).rev() {
            self.blocks[i-1].previous_hash = self.blocks[i].hash.clone();

            let input = format!("{}{}{}{}", self.blocks[i-1].index, 
            self.blocks[i-1].data, 
            self.blocks[i-1].previous_hash, 
            self.blocks[i-1].timestamp);

            let mut hasher = Sha256::new();
            hasher.update(input);
            let new_hash = format!("{:x}", hasher.finalize());
            
            self.blocks[i-1].hash = new_hash;
            
        }

    }

    pub fn reveal_tamper_in_head(&self, head: Block) {
       if self.blocks[0].hash.clone() != head.hash {
            println!("blockchain tamper revealed in head");
       }else {
            println!("blockchain multitamper healtcheck passed");
       }
    }  

  
    pub fn reveal_tamper(&self) {
        for i in (1..self.blocks.len()).rev() {
            if self.blocks[i].previous_hash != self.blocks[i-1].hash{
                println!("block {} has been tampered", i-1);
                return;
            }
        } 
        println!("blockchain healtcheck passed");
    }

  

}
