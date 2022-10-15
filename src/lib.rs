use concrete::*;
use rayon::prelude::*;

fn sum_ct_VectorLWE(mut c: VectorLWE, new_min: f64) -> VectorLWE{
    let lenght = c.nb_ciphertexts;
    let mut ct_min = 0.;
    let mut min = 0.;
    let mut ct_min_arr = vec![0.; lenght];
    
    for i in 0..lenght{
        min = f64::abs(f64::min(0., c.encoders[i].get_min() as f64));
        ct_min += min;
        ct_min_arr[i] = min;
    }
    
    c.add_constant_static_encoder_inplace(&ct_min_arr).unwrap();
    let mut ct = c.sum_with_new_min(ct_min+new_min).unwrap();
    ct.add_constant_dynamic_encoder_inplace(&[-1.*ct_min]).unwrap();
    
    return ct;
}

fn sum_N_VectorLWE(x: &VectorLWE) -> VectorLWE{
    let mut y = x.clone();
    let mut number = x.nb_ciphertexts as f64;
    let mut n = 0;
    while number/2. == f64::floor(number/2.){
        n += 1;
        number /= 2.;
    }
    let padd = x.encoders[0].nb_bit_padding;
    let mut ct_1: VectorLWE;
    let mut ct_2: VectorLWE;
    

    for i in 0..(n as usize){
        //y.pp();
        if ((padd as i32) - (n as i32) <= 0) && (y.encoders[0].nb_bit_padding == 1){
            //println!("Not enough padding!");
            return y;
        }else{
            let N = u32::pow(2, (n-i-1) as u32) as usize;
            let mut tmpVec = VectorLWE::zero(x.dimension, N).unwrap();
            for j in 0..N{
                ct_1 = y.extract_nth(2*j).unwrap();
                ct_2 = y.extract_nth(2*j+1).unwrap();

                ct_1.add_with_padding_inplace(&ct_2).unwrap();

                tmpVec.copy_in_nth_nth_inplace(j, &ct_1, 0).unwrap();
            }
            y = tmpVec.clone();
            //y.pp();
        }
    }
    if y.nb_ciphertexts > 1{
        y = sum_ct_VectorLWE(y, 0.);
    }
    return y;    
}

/*
fn sum_ct_LWE(mut c: Vec<LWE>, new_min: f64) -> Vec<LWE>{
    let lenght = c.len();
    let mut ct_min = 0.;
    let mut min = 0.;
    let mut ct_min_arr = vec![0.; lenght];
    
    for i in 0..lenght{
        min = f64::abs(f64::min(0., c[i].encoder.get_min() as f64));
        ct_min += min;
        ct_min_arr[i] = min;
    }
    
    c.add_constant_static_encoder_inplace(&ct_min_arr).unwrap();
    let mut ct = c.sum_with_new_min(ct_min+new_min).unwrap();
    ct.add_constant_dynamic_encoder_inplace(&[-1.*ct_min]).unwrap();
    
    return ct;
}*/

fn sum_N_LWE(x: &Vec<LWE>) -> Vec<LWE>{
    let mut y = x.clone();
    let mut number = x.len() as f64;
    let mut n = 0;
    while number/2. == f64::floor(number/2.){
        n += 1;
        number /= 2.;
    }
    let padd = x[0].encoder.nb_bit_padding;
    let mut ct_1: LWE;
    let mut ct_2: LWE;
    

    for i in 0..(n as usize){
        //y.pp();
        if ((padd as i32) - (n as i32) <= 0) && (y[0].encoder.nb_bit_padding == 1){
            //println!("Not enough padding!");
            return y;
        }else{
            let N = u32::pow(2, (n-i-1) as u32) as usize;
            let mut tmpVec = vec![x[0].clone(); N];
            for j in 0..N{
                ct_1 = y[2*j].clone();
                ct_2 = y[2*j+1].clone();

                ct_1.add_with_padding_inplace(&ct_2).unwrap();

                tmpVec[j] = ct_1;
            }
            y = tmpVec.clone();
        }
    }
    //Vec<LWE> got no "sum_with_new_min"
    /*if y.len() > 1{
        y = sum_ct_VectorLWE(y, 0.);
    }*/
    return y;
}

fn relu(x: f64) -> f64{
    return f64::max(x, 0.);
}

fn elu_plus_one(x: f64) -> f64{
    if x >= 0. {
        return x+1.;
    }
    else {
        return f64::exp(x)
    }
}
