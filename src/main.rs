#[macro_use]
extern crate serde_derive;

use std::io;
use std::process;

mod blockchain;


fn main() {
  let mut miner_addr = String::new();
  let mut choice = String::new(); 
  let mut difficulty = String::new();

  println!("input a miner address: ");
  io::stdin().read_line(&mut miner_addr).expect("Error reading miner_addr");

  println!("input a difficulty: ");
  io::stdin().read_line(&mut difficulty).expect("Error reading difficulty");
  let diff: u32 = difficulty.trim().parse::<u32>().expect("Difficulty must be an integer"); 

  println!("GENERATING GENESIS BLOCK!");
  let mut chain = blockchain::BlockChain::new(miner_addr.trim().to_string(), diff);

  loop {
    println!("Menu:");
    println!("1 -> New tx");
    println!("2 -> Mine Block");
    println!("3 -> Change difficulty");
    println!("4 -> Change Reward");
    println!("0 -> Exit");
    choice.clear();
    io::stdin().read_line(&mut choice).expect("Error reading choice");

    match choice.trim().parse().unwrap() {
      0 => {
        let mut conf = String::new();
        println!("exiting the program will kill the process and all data, are you sure? => y/n");
        io::stdin().read_line(&mut conf).expect("Error reading confirmation");

        match conf.trim() {
          "y" => {
            println!("exiting process");
            process::exit(0);
          },
          "Y" => {
            println!("exiting process");
            process::exit(0);
          },
          "n" => {
            break;
          },
          "N" => {
            break;
          },
          _ => println!("invalid choice, please use 'y' or 'n' ")
        }

      },
      1 => {
        let mut sender = String::new();
        let mut receiver = String::new();
        let mut amount = String::new();
        println!("type the sender");
        io::stdin().read_line(&mut sender).expect("Error reading sender");
        println!("type the receiver");
        io::stdin().read_line(&mut receiver).expect("Error reading receiver");
        println!("type the amount");
        io::stdin().read_line(&mut amount).expect("Error reading amount");

        let res = chain.new_transaction(
          sender.trim().to_string(), 
          receiver.trim().to_string(), 
          amount.trim().parse().unwrap()
        );

        match res {
          true => println!("Tx added"),
          false => println!("Tx failed"),
        }
      },
      2 => {
        println!("Mining block");
        let res = chain.generate_new_block();
        println!("{:#?}", chain);
        match res {
          true => println!("Block appended to the Blockchain"),
          false => println!("mining the block failed"),
        }
      },
      3 => {
        let mut difficulty = String::new();
        println!("input a difficulty: ");
        io::stdin().read_line(&mut difficulty).expect("Error reading new difficulty");
        let diff: u32 = difficulty.trim().parse().unwrap();
        chain.update_difficulty(diff);
      },
      4 => {
        let mut reward = String::new();
        println!("input a reward:");
        io::stdin().read_line(&mut reward).expect("Error reading reward");
        let rew: u128 = reward.trim().parse().unwrap();
        chain.update_reward(rew);
      },
      _ => println!("invalid choice")
    }

  }

}
