mod decimal;

fn main() {
    {
    use std::str::FromStr;
    let x = decimal::UnsignedInteger::from_str("6106370327537124369917353116076419185226836738092151283090256592406883297278429111051221535933437500");
    let y = decimal::UnsignedInteger::from_str("1551324303876771241884343113259599609378653143111280111834187291909203932606583041853156452338126705");

    let out_str = x
        .and_then(|a| y.map(|b| (a, b)))
        .map(|(a, b)| format!(" {}\n+{}\n====================================================================================================================================\n {}", a.clone(), b.clone(), a + b))
        .unwrap_or(String::new());
    println!("{}", out_str);
}
{
use std::str::FromStr;
let x = decimal::UnsignedInteger::from_str("6");
let y = decimal::UnsignedInteger::from_str("1551324303876771241884343113259599609378653143111280111834187291909203932606583041853156452338126705");

let out_str = x
    .and_then(|a| y.map(|b| (a, b)))
    .map(|(a, b)| format!(" {}\n+{}\n====================================================================================================================================\n {}", a.clone(), b.clone(), a + b))
    .unwrap_or(String::new());
println!("{}", out_str);
}
}
