use concrete::*;
use std::time::{Instant};

mod lib;
use lib::Net;

fn main() {

    // #### ---- FHE STUFF ---- ####

    let lwe_dim = 1024;//512, 1024, 2048];
    let lwe_noise = -40;//-19, -40, -62];
    
    let rlwe_dim = 1024; //512, 1024, 2048];
    let rlwe_noise = -40; //-19, -40, -62];

    let base_log = 6;
    let lvl = 6;
    
    //let lwe_params: LWEParams = LWEParams::new(lwe_dim, lwe_noise);
    let rlwe_params: RLWEParams = RLWEParams{polynomial_size: rlwe_dim, dimension: 1, log2_std_dev: rlwe_noise};

    let sk_rlwe = RLWESecretKey::new(&rlwe_params);
    let sk = sk_rlwe.to_lwe_secret_key();
    let bsk = LWEBSK::new(&sk, &sk_rlwe, base_log, lvl);
    //let ksk = LWEKSK::new(&sk, &sk_rlwe, base_log, lvl);

    let input_size = 16;
    let data = vec![1.; input_size];

    let prec = 4;
    let enc = Encoder::new(0., 4., prec, prec+5).unwrap();

    // #### ---- FHE STUFF ---- ####

    println!("done");
    
    let hidden_size = vec![8,16,32,64,128,256];
    let output_size = 6;

    for h in hidden_size.iter(){
        println!("\n\n###---- HIDDEN SIZE: {} ----####", h);

        let net = Net::new(input_size, *h, output_size);

        let input = VectorLWE::encode_encrypt(&sk, &data, &enc).unwrap();
        let (mu, sig) = net.forward(input.clone(), &bsk);

        //for _ in 0..10{
        let now = Instant::now();
        for i in 0..1{
            let (mu, sig) = net.forward(input.clone(), &bsk);
            //println!("mu = {:?}, sig = {:?}", mu.decrypt_decode(&sk).unwrap(), sig.decrypt_decode(&sk).unwrap());
        
        }
        println!("Sequential: {:?}", now.elapsed());
        let single = now.elapsed().as_millis() as f64;

        let mut input = vec![];
        for d in data.iter(){
            input.push(LWE::encode_encrypt(&sk, *d, &enc).unwrap());
        }

        let now = Instant::now();
        for i in 0..1{
            let (mu, sig) = net.forward_par(input.clone(), &bsk);
            //println!("mu = {}, sig = {}", mu[0].decrypt_decode(&sk).unwrap(), sig[0].decrypt_decode(&sk).unwrap());
        }
        println!("Concurrent: {:?}", now.elapsed());
        let multi = now.elapsed().as_millis() as f64;

        println!("{}", single/multi);
        //}
    }
}
