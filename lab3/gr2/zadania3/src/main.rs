// fn print_ascii(){
//     for i in 33..127{
//         println!("{} : {}", i , i as u8 as char);
//     }
// }

fn collatz_steps(mut i:u128)->u128{
    let mut step = 0;
    while i!=1{
        if i%2 ==0{
            step +=1;
            i = i/2;
        }
        else{
            step +=1;
            i = 3*i+1;
        }
    }
    step

    // if i!=1{
    //     if i%2==0{
    //         step+=1;
    //         collatz_steps(i/2,step)
    //     }
    //     else{
    //         step+=1;
    //         collatz_steps(3*i+1,step)
    //     }
    // }
    // else{
    //     println!("{step}");
    //     return 1;
    // }
}

fn main(){
    //print_ascii();
    println!("{:?}",collatz_steps(12));
}
