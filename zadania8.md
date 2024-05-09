1. Zdefiniuj typ strukturalny, modelujący proces w systemie Linux. Poniższa
   funkcja powinna działać z Twoim kodem.
```
fn main() {
    let command = "toilet";
    let script = "echo mleko | toilet & ls;";
    let mut p1 = Process::from_command(command);
    let mut p2 = Process::shell();
    let p3 = Process::from_script(script);
    let mut p4 = Process::from_command(command);
    p1.stop();
    p2.start();
    p2.finish();
    println!("{}", !(p1 == p2));
    println!("{}", !(p1 == p4));
    println!("{}", p3.len() == 3);
    for process in p3 {
        println!("{}: {}", process.pid, process.command);
    }
}
```

2. Zdefiniuj typ strukturalny do przechowywania koloru w modelu RGB z
   dokładnością do 8 bitów na składową. Poniższa funkcja testująca powinna
   działać z Twoim typem (i wyświetlać 8 razy true):
```
fn main() {
    let szary1 = Rgb::from_3u8(127, 127, 127);
    let szary2 = Rgb::from_3percent(50.0, 50.0, 50.0).unwrap();
    let szary3 = Rgb::gray(50.0).unwrap();
    let fiolet = Rgb::from_3u8(100, 35, 120);
    let bialy1 = Rgb::white();
    let bialy2 = Rgb::from_3u8(255, 255, 255);
    let mut czarny1 = Rgb::black();
    let czarny2 = Rgb::from_3u8(0, 0, 0);
    println!("{} {}", szary1 == szary2, szary1 == szary3);
    println!("{} {}", bialy1 == bialy2, czarny1 == czarny2);
    czarny1.invert();
    println!("{}", bialy1 == czarny1);
    println!("{}", fiolet.intensity() == 1.0/3.0);
    println!("{}", fiolet.as_rgb_u8tuple() == (100, 35, 120));
    println!("{}", fiolet.as_cmy_u8tuple() == (155, 220, 135));
}
```
