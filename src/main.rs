mod blockchain;
mod test;


//pub negli elementi del modulo li rende visibili solo al padre
//pub mod rende il modulo accessibile a tutte le gerarchie in alto

//module tree

//crate (main)
    //blockchain
        //block
    //test

fn main() {
    let mut blockchain = blockchain::Blockchain::new();

    let blockchain_head = blockchain.return_head();
    blockchain.add_block("Block 1 data".to_string());
    blockchain.add_block("Block 2 data".to_string());
    blockchain.add_block("Block 3 data".to_string());
    blockchain.print_blocks();
    // blockchain.reveal_tamper();
    // blockchain.tamper_block();
    // blockchain.reveal_tamper();
    // blockchain.print_blocks();
    blockchain.tamper_multiple_block();
    blockchain.reveal_tamper_in_head(blockchain_head);
    blockchain.print_blocks();
}