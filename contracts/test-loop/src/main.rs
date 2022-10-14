use cosmwasm_std::Uint128;

fn main() {
    let mut test = Uint128::zero();
    let mut i = 0u64;
    loop {
        if i > 10 {
            break;
        }
        i += 1;
        test += Uint128::from(i);
    }
    println!("{}", test);
}
