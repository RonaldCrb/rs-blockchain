extern crate sha2;
extern crate chrono;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
use chrono::{Utc};
use sha2::{Sha256, Digest};
use std::fmt::Write;

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
  sender: String,
  receiver: String,
  amount: u128,
}

#[derive(Debug, Serialize)]
pub struct BlockHeader {
  timestamp: i64,
  nonce: u32,
  pre_hash: String,
  merkle: String,
  difficulty: u32,
}

#[derive(Debug, Serialize)]
pub struct Block {
  header: BlockHeader,
  count: u32,
  tx: Vec<Transaction>,
}

#[derive(Debug, Serialize)]
pub struct BlockChain {
  chain: Vec<Block>,
  mempool_tx: Vec<Transaction>,
  difficulty: u32,
  miner_addr: String,
  reward: u128,
}

impl BlockChain {
  pub fn new(miner_addr: String, difficulty: u32) -> BlockChain {
    let mut blockchain = BlockChain {
      chain: Vec::new(),
      mempool_tx: Vec::new(),
      difficulty,
      miner_addr,
      reward: 100
    };

    blockchain.generate_new_block();

    blockchain
  }

  pub fn generate_new_block(&mut self) -> bool {
    let header = BlockHeader {
      timestamp: Utc::now().timestamp(),   // ().to_timesec().sec,
      nonce: 0,
      pre_hash: self.last_hash(),
      merkle: String::new(),
      difficulty: self.difficulty,
    };

    let reward_trans = Transaction {
      sender: String::from("root"),
      receiver: self.miner_addr.clone(),
      amount: self.reward,
    };

    let mut block = Block {
      header,
      count: 0,
      tx: vec![],
    };

    block.tx.push(reward_trans);
    block.tx.append(&mut self.mempool_tx);
    block.count = block.tx.len() as u32;
    block.header.merkle = BlockChain::get_merkle(block.tx.clone());
    BlockChain::proof_of_work(&mut block.header);

    println!("{:#?}", &block);
    self.chain.push(block);

    true
  }

  pub fn new_transaction(&mut self, sender: String, receiver: String, amount: u128) -> bool {
    self.mempool_tx.push(Transaction{
      sender,
      receiver,
      amount,
    });
    true
  }

  pub fn last_hash(&self) -> String {
    let block = match self.chain.last() {
      Some(block) => block,
      None => return String::from_utf8(vec![48;64]).unwrap()
    };
    BlockChain::hash(&block.header)
  }

  pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
    self.difficulty = difficulty;
    true
  }

  pub fn update_reward(&mut self, reward: u128) -> bool {
    self.reward = reward;
    true
  }

  pub fn get_merkle(mempool_tx: Vec<Transaction>) -> String {
    let mut merkle = Vec::new();
    for t in mempool_tx {
      let hash = BlockChain::hash(&t);
      merkle.push(hash);
    }

    if merkle.len() % 2 == 1 {
      let last = merkle.last().cloned().unwrap();
      merkle.push(last)
    }
    
    while merkle.len() > 1 {
      let mut h1 = merkle.remove(0);
      let mut h2 = merkle.remove(0);
      h1.push_str(&mut h2);
      let nh = BlockChain::hash(&h1);
      merkle.push(nh);
    }

    merkle.pop().unwrap()
  }

  pub fn proof_of_work(header: &mut BlockHeader) {
    loop {
      let hash = BlockChain::hash(header);
      let slice: &str = &hash[..header.difficulty as usize];
      match slice.parse::<u32>() {
        Ok(val) => {
          if val != 0 {
            header.nonce += 1;
          } else {
            println!("Block hash: {}", hash);
            break;
          }
        },
        Err(_) => {
          header.nonce += 1;
          continue
        }
      }
    }
  }

  pub fn hash<T: serde::Serialize>(item: &T) -> String {
    let input = serde_json::to_string(&item).unwrap();
    let mut hasher = Sha256::default();
    hasher.update(input.as_bytes());
    let res = hasher.finalize();
    let vec_res = res.to_vec();
    BlockChain::hex_to_string(vec_res.as_slice())
  }

  pub fn hex_to_string(vec_res: &[u8]) -> String {
    let mut s = String::new();
    
    for b in vec_res {
      write!(&mut s, "{:x}", b).expect("unable to write");
    }

    return s
  }
}

