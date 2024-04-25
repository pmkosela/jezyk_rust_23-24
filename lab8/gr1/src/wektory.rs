fn powtorki(v: Vec<u32>) -> Vec<u32> {
    //1, 3, 4, 3, 3, 3, 3, 4, 1, 1, 6
    //         ^^^^^^^^^^     ^^^^
    /*for element in v {
        let x = *element;
    }*/
    let mut out = Vec::new();
    let mut last_pushed = false;
    for i in 1..v.len() {
        if v[i] == v[i - 1] {
            out.push(v[i]);
            last_pushed = true;
        } else if last_pushed {
            last_pushed = false;
            out.push(v[i - 1]);
        }
    }
    out
}

fn zlicz_wiele(a: Vec<u32>, b: Vec<u32>) -> usize {
    let mut out = 0;
    let mut longer = a;
    let mut shorter = b;
    if shorter.len() > longer.len() {
        let tmp = longer;
        longer = shorter;
        shorter = tmp;
    }
    for element in longer {
        if shorter.contains(&element) {
            out += 1;
        }
    }
    out
}

fn main() {
    let wektor: Vec<_> = vec![1, 3, 4, 3, 3, 3, 3, 4, 1, 1, 6];
    let maly_wektor: Vec<_> = vec![1, 3];
    println!("{:?}", powtorki(wektor.clone()));
    println!("{:?}", zlicz_wiele(wektor.clone(),
                                 maly_wektor.clone()));
    println!("{:?}", zlicz_wiele(maly_wektor.clone(),
                                 wektor.clone()));
}
