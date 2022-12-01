pub fn part1(input: String) {
    let vec: Vec<Vec<u32>> = input
        .trim() // nightmare
        .split("\n\n") // vec of `"1\n1\n1"`
        .map(|c|
            c.split('\n') // vec of `"1"`
                .map(|c| {
                    c.parse() // parses each thing
                        .unwrap() // unwraps the result
                })
                .collect()) // vec of `1`
        .collect(); // vec of vecs of `1`

    let mut addedvec = vec![];

    for cl in vec {
        let mut num: u32 = 0;
        for c in cl {
            num += c;
        };
        addedvec.push(num);
    };

    println!("{:?}", addedvec.iter().max().expect("vec was empty, how did you manage that one"));
}
