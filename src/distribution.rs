use rand::Rng;

use generator::Generator;
// distribution of bytes of given iterator
fn dist20(iter: &mut Iterator<Item=u8>, result: &mut [Box<[i32; 256]>]){
    for i in 0..20 {
        match iter.next() {
            Some(v) => {
                let ref mut b = result[i];
                b[v as usize] += 1;
            }
            None => {
                break;
            }
        }
    }
}

pub fn one_dist<T>(r: &mut T, result: &mut [Box<[i32; 256]>])
    where T: Rng + Sized {
    // generate random key
    let key = r.gen::<[u8; 16]>();

    let mut g = Generator::new(&key[..]);

    dist20(&mut g, result);
}

fn dist_at(n: u32, iter: &mut Iterator<Item=u8>, result: &mut [i32; 256]){
    for _ in 0..(n-1) {
        if let None = iter.next() {
                return;
        }
    }
    if let Some(v) = iter.next() {
        result[v as usize] += 1;
    }
}

pub fn one_dist_at<T>(n: u32, r: &mut T, result: &mut [i32; 256])
    where T: Rng + Sized {
    let key = r.gen::<[u8; 16]>();

    let mut g = Generator::new(&key[..]);

    dist_at(n, &mut g, result);
}
