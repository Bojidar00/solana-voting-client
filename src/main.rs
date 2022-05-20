use solana_sdk::signer::keypair:: Keypair;
use client::{get_organisations, get_vote_topics, get_accounts,show_topics,show_organisations,create_organisation,join_organisation,create_vote_topic,vote,request_air_drop};
use std::io;
use  anchor_client::Cluster;
use std::rc::Rc;
use anchor_client::Client;
use solana_sdk::signer::Signer;
use solana_sdk::pubkey::Pubkey;
use voting::{VoteTopic,Organisation};

fn main() {

    let key = Keypair::new();
    let pkey = key.pubkey();
    let key_bytes=key.to_bytes();
    let r = Rc::from(key);
    let client = Client::new(Cluster::Testnet, r.clone());
    let k = voting::ID;
    let program = client.program(k.clone());


    let accounts=get_accounts();
    let mut topics:Vec<(Pubkey,VoteTopic)>=get_vote_topics(accounts.clone(), &program);
    let mut organisations:Vec<(Pubkey,Organisation)>=get_organisations(accounts.clone(), &program);
    
    request_air_drop(&pkey, 1 as f64);

    let mut menu=true;
    
    
while menu{
    println!("======== solana voting ========\n");
    println!("============= menu ============\n");
    println!("0 - exit\n");
    println!("1 - show vote topics\n");
    println!("2 - show organisations\n");
    println!("3 - create vote topic\n");
    println!("4 - create organisation\n");
    println!("5 - vote\n");
    println!("6 - join organisation\n");

    
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

        let choice: u32 = choice.trim().parse().expect("Please type a number!");

    match choice {
        0=>{menu = false;},
        1=>{
           // topics = get_vote_topics(accounts.clone(), &program);
            show_topics(&topics);
        },
        2=>{
            let organisations = get_organisations(accounts.clone(), &program);
            show_organisations(&organisations);
        },
        3=>{
            create_vote_topic(Keypair::from_bytes(&key_bytes).unwrap());
        },
        4=>{
            create_organisation(Keypair::from_bytes(&key_bytes).unwrap());
        },
        5=>{
            vote(&topics, &program);
        },
        6 =>{
            join_organisation(&organisations, &program);
        }
        _=>{println!("no match");}
    }
    
}

   
}

