
use rust_spp::*;


fn leak_test() {
    let pipeline = pipeline![
        parallel!(|item: i32| { 
            Some(item * 2) 
        }, 20),
        parallel!(|item: i32| { 
            if item % 5 == 0 {
                None
            } else {
                Some(item * 2) 
            }
        }, 20),
        sequential!(|item: i32| {
            println!("number: {:?}", item);
        })
    ];

    for i in 1..1000 {
        pipeline.post(i).unwrap()
    }
}


fn main() {

    leak_test();
}
