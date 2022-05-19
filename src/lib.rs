use solana_sdk::pubkey::Pubkey;
use anchor_lang::prelude::*;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signer::keypair::{read_keypair_file, Keypair};
use solana_sdk::signature::Signature;
use std::time::{UNIX_EPOCH, Duration};
use chrono::prelude::DateTime;
use chrono::Utc;
use anchor_client::Client;
use  anchor_client::Cluster;
use std::rc::Rc;
use solana_sdk::signer::Signer;
use solana_sdk::system_program;
use std::io;
use voting::{VoteTopic,Organisation};



pub const URL: &str = "https://api.testnet.solana.com";
const LAMPORTS_PER_SOL: f64 = 1000000000.0;



pub fn request_air_drop( pub_key: &Pubkey, amount_sol: f64) -> Result<Signature> {
    let rpc_client = RpcClient::new(URL);
    let sig = rpc_client.request_airdrop(&pub_key, (amount_sol * LAMPORTS_PER_SOL) as u64).unwrap();
    loop {
        let confirmed = rpc_client.confirm_transaction(&sig).unwrap();
        if confirmed {
            break;
        }
    }
    Ok(sig)
}

pub fn create_vote_topic(key:Keypair){
    println!("topic:");
    let mut topic = String::new();
    io::stdin()
        .read_line(&mut topic)
        .expect("Failed to read line");

    println!("application period:");
    let mut a_period = String::new();
    io::stdin()
        .read_line(&mut a_period)
        .expect("Failed to read line");

        let a_period: i32 = a_period.trim().parse().expect("Please type a number!");
    println!("voting period:");
        let mut v_period = String::new();
        io::stdin()
            .read_line(&mut v_period)
            .expect("Failed to read line");
    
            let v_period: i32 = v_period.trim().parse().expect("Please type a number!");




    println!("Sending transaction...");

    let r = Rc::from(key);
    let client = Client::new(Cluster::Testnet, r.clone());
    let k = voting::ID;
    let program = client.program(k.clone());

    let vaccount = Keypair::new();

    let res =program
    .request()
    .signer(&vaccount)
    .accounts(voting::accounts::Create {
        vote_account: vaccount.pubkey(),
        user: program.payer(),
        system_program: system_program::ID,
    })
    .args(voting::instruction::Create { topic_:topic, applications_deadline: a_period, voting_deadline: v_period })
    .send();
    println!("{:?}",res);
}

pub fn create_organisation(key:Keypair){ 
    println!("organisation name:");
    let mut o_name = String::new();
    io::stdin()
        .read_line(&mut o_name)
        .expect("Failed to read line");


    println!("Sending transaction...");

    let r = Rc::from(key);
    let client = Client::new(Cluster::Testnet, r.clone());
    let k = voting::ID;
    let program = client.program(k.clone());

    let oaccount = Keypair::new();

    let res =program
    .request()
    .signer(&oaccount)
    .accounts(voting::accounts::CreateOrganisation {
        organisation_account: oaccount.pubkey(),
        user: program.payer(),
        system_program: system_program::ID,
    })
    .args(voting::instruction::CreateOrganisation { name:o_name })
    .send();
    println!("{:?}",res);
}

pub fn get_program(keypair_path: &str) -> Result<Keypair> {
    let program_keypair = read_keypair_file(keypair_path).unwrap();
    Ok(program_keypair)
}

pub fn get_accounts<'a>()-> Vec<(Pubkey,solana_sdk::account::Account)>{
    let rpc_client = RpcClient::new(URL);
    let pkey = voting::ID;
    let accounts=rpc_client.get_program_accounts(&pkey).unwrap();
    accounts
}
pub fn get_vote_topics(accounts:Vec<(Pubkey,solana_sdk::account::Account)>, program:&anchor_client::Program)->Vec<(Pubkey,VoteTopic)>{
    let mut accounts_data:Vec<(Pubkey,VoteTopic)>=Vec::new();


    for account in accounts{
        let (key,_acc)=account;
        let accdata = program.account::<VoteTopic>(key);
        match accdata {
            Ok(v) => accounts_data.push((key,v)), 
            Err(_e) => ()
        }
       
    }
    accounts_data
}


pub fn get_organisations(accounts:Vec<(Pubkey,solana_sdk::account::Account)>, program: &anchor_client::Program)->Vec<(Pubkey,Organisation)>{
    let mut accounts_data:Vec<(Pubkey,Organisation)>=Vec::new();


    for account in accounts{
        let (key,_acc)=account;
        let accdata = program.account::<Organisation>(key);
        match accdata {
            Ok(v) => accounts_data.push((key,v)), 
            Err(_e) => ()
        }
       
    }
    accounts_data
}
pub fn unix_to_data(unix:u64)-> String{

    let d = UNIX_EPOCH + Duration::from_secs(unix);
    // Create DateTime from SystemTime
    let datetime = DateTime::<Utc>::from(d);
    // Formats the combined date and time with the specified format string.
    let applications_deadline = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
    applications_deadline

}
pub fn show_topics(topics:&Vec<(Pubkey,VoteTopic)>){
    for topic in topics {
        let (_pkey, t)=topic;

       
        let applications_deadline =unix_to_data(t.applications_deadline as u64);
        let voting_deadline = unix_to_data(t.voting_deadline as u64);



        println!("Topic: {:?}\n
        apllications deadline: {:?} \n
        voting deadline: {:?} \n
        only for organisation: {:?}",
        t.topic, applications_deadline, voting_deadline,t.use_organisation);
    }
}
pub fn show_organisations(organisations:&Vec<(Pubkey,Organisation)>){
    for organisation in organisations {
        let (_pkey, org)=organisation;
        println!("Organisation: \n
        name: {:?} \n
        participants: {:?} \n
        authority: {:?}",org.name, org.participants, org.authority);
    }
}

