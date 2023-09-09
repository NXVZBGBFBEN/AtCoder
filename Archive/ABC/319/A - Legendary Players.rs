use proconio::input;

fn main() {
    let player_list = [
        ("tourist", 3858),
        ("ksun48", 3679),
        ("Benq", 3658),
        ("Um_nik", 3648),
        ("apiad", 3638),
        ("Stonefeang", 3630),
        ("ecnerwala", 3613),
        ("mnbvmar", 3555),
        ("newbiedmy", 3516),
        ("semiexp", 3481)
    ];

    input! {
        s: String,
    }

    for i in 0..10 {
        if s == player_list[i].0 {
            println!("{}", player_list[i].1);
        }
    }
}
