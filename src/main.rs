use pipey::Pipey;
use std::thread;
use std::time;
const x: &[u8] = include_bytes!("x.txt");
// comments dont count so:
// xs = prgram array, pi = program index, i = array index, si = index on loop call, m = main array
// every layer no has their own pi

fn p(_:&mut usize,_:&mut [u8]){}
const NRET_FN_TABLE: &[fn(&mut usize,&mut [u8])] = &[
     |i,m| m[*i] = m[*i] + 1, // +
     p, // pad
     |i,m| m[*i] = m[*i] - 1, // -
     |i,m| print!("{}",m[*i] as char),
     p, p, p, p,p, p, p, p, p, p , p ,p , p, // pad
     |i,_| *i = *i - 1, // <
     |_,_| {},
     |i,_| *i = *i + 1, // >
];

fn 
f(xs: &[u8], mut pi: usize, i: &mut usize, si: usize, m: &mut [u8]) 
-> usize                 {
     if pi > xs.len() - 1 {
        return 
          pi;
                                                                   };
    println!("b{:?}, pi:{pi}, xs[pi]: {:?}, si: {si} ,i: {i}", m, xs[pi] as char);
    thread::sleep(time::Duration::from_millis(100));
    pi = match xs[pi] {
     // requires current layer returns
        b'[' => {
                 f(xs, pi + 1, i, pi, m)
                                },
        b']' => if 0 < m[*i]    {
                    si          } else 
                                { return
                    pi +  1     }
        
        // index update issue
           _ => (xs[pi].wrapping_sub(43) as usize).poof(|c| if 43 > c {
                       NRET_FN_TABLE[c](i, m);}).pipe(|_| pi + 1)
    };
    f(xs, pi, i, si, m)}
fn main() {
    f(x, 0, &mut 0, 0, &mut [0u8; 30]);}
