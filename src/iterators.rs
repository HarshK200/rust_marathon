pub fn vec_iterator() {
    let mut v1 = vec![12, 3, 1, 4];

    // WARN: this is immutable iterator cannot mutate the variables
    let it1 = v1.iter(); // NOTE: this iterator doesn't affect performace at all since it don't do
                         // anyting for now that is why ITERATORS IN RUST ARE LAZY
    for i in it1 {
        println!("{} ", i);
    }

    println!("{:?}", v1);

    // WARN: this is mutable iterator values can be mutated when iterating through
    let it2 = v1.iter_mut();

    for i in it2 {
        *i += 1; // NOTE: has to use * to dereference the variable
    }

    println!("{:?}", v1);

    // WARN: ONLY USE THIS WHEN YOU DON'T NEED THE VECTOR ANYMORE since it moves it instead of borrowing
    let it3 = v1.into_iter();

    for i in it3 {
        print!("{} ", i);
    }
}

pub fn iterator_adpater() {
    let v = vec![12, 3, 12];

    let it1 = v.iter();

    // WARN: it2 points to another vector with value set as x + 1
    let it2 = it1.map(|x| x + 1); // NOTE: the vec iter1 gets consumed/moved here

    for i in it2 {
        print!("{} ", i);
    }
    println!();

    println!("{:?}", v);

    let it3 = v.iter().filter(|x| *x % 2 == 0);

    for i in it3 {
        print!("{} ", i);
    }
    println!();
}

pub fn iterator_adpater_assignment() {
    let v1 = vec![12, 1, 3, 5, 4];

    let it1 = v1.iter();
    // NOTE:  filtering out all odd values
    let it2 = it1.filter(|x| *x % 2 != 0);
    let it3 = it2.map(|x| x * 2);

    let v2: Vec<i32> = it3.collect();

    println!("{:?}", v1);
    println!("{:?}", v2);
}
