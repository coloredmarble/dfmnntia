use pipey::Pipey;
use std::thread;
use std::time;
use std::time::Duration;
const x: &[u8] = include_bytes!("x.txt");
// comments dont count so:
// xs = prgram array, pi = program index, i = array index, si = index on loop call
fn f(xs: &[u8], pi: &mut usize, i: &mut usize, si: usize, m: &mut [u8]) -> usize {
    if *pi > xs.len() - 1 {
        return *pi;
    };
    println!("{:?}, pi:{pi}, xs[pi]: {:?}, si: {si}", m, xs[*pi] as char);
    match xs[pi.clone()] {
        b'[' => return pi.tap_mut(|p| **p = **p + 1).pipe(|p| f(xs, p, i, p.clone(), m)),
        b']' => if 0 < m[*i] {
                        *pi = si } else { return *pi.tap_mut(|p| **p = **p + 1) }
        
        // index update issue
            _ => match xs[pi.clone()] {
                b'-' => m[*i] 
                     =  m[*i] - 1,
                b'+' => m[*i] 
                     =  m[*i] + 1,
                b'<' => *i 
                     =  i.wrapping_sub(1),
                b'>' => *i 
                     =  i.wrapping_add(1),
                _ => {}
            }
                .btw(|| *pi = *pi + 1),
    }
    thread::sleep(time::Duration::from_millis(1000));
    f(xs, pi, i, si, m)
}
fn main() {
    f(x, &mut 0, &mut 0, 0, &mut [0u8; 30]);
}
