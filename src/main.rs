struct Mutator {}
impl Mutator {
    fn mutate<'a> (x :&'a mut String, ch: char ) {
        x.push(ch);
    }
    fn mutate_mutatee<'a> (x: &'a mut Mutatee<'a>, s: &'a String) {
        x.x = s;
    }
    fn mutate_mutatee_and_return_ownership<'a> (x: &'a mut Mutatee<'a>, s: &'a String) -> &'a mut Mutatee<'a> {
        x.x = s;
        x
    }
}

struct Mutatee<'a> {
    x: &'a String,
}

struct SelfMutator<'a> {
    x: &'a mut String,
}
impl<'a> SelfMutator<'a> {
    fn self_mutate(&mut self, ch: char) {
        self.x.push(ch);
    }
}


fn main () {
    one();
    two();
    three(); // why doesn't this work, when `two()` does? How are they different?
    four();
    five();
    six();
    seven();
    eight();
}

// in this example, I can call a self-mutating method on a struct multiple times
fn one() {
    // built in
    let mut x = String::from("foo");
    x.clear();
    x.push('h');
    x.push('i');

    // or with our own struct
    let mut self_mutator = SelfMutator { x: &mut x };
    self_mutator.self_mutate('h');
    self_mutator.self_mutate('i');
}

// in this example, I can call a function which mutates a parameter more than once
fn two() {
    let mut goo = String::from("goo");
    Mutator::mutate(&mut goo, 'h');
    Mutator::mutate(&mut goo, 'i');
}

// but I can't have a third-party mutate my own struct?
fn three() {
    let goo = String::from("goo");
    let foo = String::from("foo");
    let mut mutatee = Mutatee { x: &goo };

    Mutator::mutate_mutatee(&mut mutatee, &foo);
    // uncomment for error
    // Mutator::mutate_mutatee(&mut mutatee, &foo);
}

// in this example, now I can call a function which mutates a parameter multiple times, because each time we return ownership of the parameter
fn four() {
    let goo = String::from("goo");
    let foo = String::from("foo");
    let mut mutatee = &mut Mutatee { x: &goo };

    let mut mutatee = Mutator::mutate_mutatee_and_return_ownership(&mut mutatee, &foo);
    let mut mutatee = Mutator::mutate_mutatee_and_return_ownership(&mut mutatee, &foo);
    Mutator::mutate_mutatee_and_return_ownership(&mut mutatee, &foo);
}

// in this example, we mutate every item in a collection
fn five() {
    let mut strings_to_mutate = vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
        String::from("d"), 
    ];
    for s in &mut strings_to_mutate {
        Mutator::mutate(s, 'a');
    }
}

// in this example, I mutate every item in a collection, multiple times
fn six() {
    let mut strings_to_mutate = vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
        String::from("d"), 
    ];

    let chars_to_push = vec!['a', 'b', 'c', 'd'];

    for c in chars_to_push {
        for s in &mut strings_to_mutate {
            Mutator::mutate(s, c);
        }
    }

    println!("{:?}", strings_to_mutate);
}

// But I can't with my own structs. Maybe this is failing for the same reason that `three()` fails
fn seven() {
    let goo = String::from("goo");
    let mut mutatees = vec![
        Mutatee { x: &goo },
        Mutatee { x: &goo },
        Mutatee { x: &goo },
        Mutatee { x: &goo },
    ];

    let replacement_strings = vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
        String::from("d"),
    ];

    for s in &replacement_strings {
    // uncomment for error
    //     for m in &mut mutatees {
    //         Mutator::mutate_mutatee(m, &s);
    //     }
    }
}

// so let's do it using the function which returns ownership, which solved `two()`.
// but....... how????
fn eight() {
    let goo = String::from("goo");
    let mut mutatees = vec![
        Mutatee { x: &goo },
        Mutatee { x: &goo },
        Mutatee { x: &goo },
        Mutatee { x: &goo },
    ];

    let mut replacement_strings = vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
        String::from("d"),
    ];

    // let last_string = replacement_strings.pop().unwrap();

    // let mut mutated_mutatees: Vec<&mut Mutatee> = Vec::new();
    // for m in &mut mutatees {
    //     mutated_mutatees.push(Mutator::mutate_mutatee_and_return_ownership(m, &last_string));
    // }
    // for s in replacement_strings {
    //     for m in mutated_mutatees {
    //         mutated_mutatees.push(Mutator::mutate_mutatee_and_return_ownership(m, &last_string));
    //     }
    // }

}