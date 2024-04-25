fn powtorki(v: Vec<u32>) -> Vec<u32> {
    let mut out = Vec::new();
    //for element in v {}
    let mut last_pushed = false;
    for i in 1..v.len() {
        if v[i] == v[i - 1] {
            out.push(v[i]);
            last_pushed = true;
        } else if last_pushed {
            out.push(v[i - 1]);
            last_pushed = false;
        }
    }
    out
}

fn unikalne_gorzej(v: Vec<u32>) -> Vec<u32> {
    let mut out = Vec::new();
    for element in v {
        if !out.contains(&element) {
            out.push(element);
        }
    }
    out
}

fn no_occurences(v: &Vec<u32>, e: u32) -> usize {
    /*let wektor: Vec<u32> = v.iter()
                            .filter(|x| **x == e)
                            .map(|x| *x)
                            .collect();
    wektor.len()*/
    v.iter().filter(|x| **x == e).count()
}

fn unikalne(v: Vec<u32>) -> Vec<u32> {
    let mut out: Vec<u32> = Vec::new();

    for element in &v {
        if no_occurences(&v, *element) == 1 {
            out.push(*element);
        }
    }
    out
}

fn main() {
    let wektor: Vec<u32> = vec![1, 5, 4, 3, 3, 3, 3, 4, 1, 1, 6];
    println!("{:?}", powtorki(wektor.clone()));
    println!("{:?}", unikalne_gorzej(wektor.clone()));
    println!("{:?}", unikalne(wektor.clone()));
}
