fn with_capacity() -> Vec<usize> {
    let mut col = Vec::with_capacity(1000);
    for i in 0..(1000*1000){
        if col.capacity() == 0 {
            col.reserve(1000);
        }
        col.push(i);
    }
    col
}

fn without_capacity() -> Vec<usize> {
    let mut col = Vec::new();
    for i in 0..(1000*1000){
        col.push(i);
    }
    col
}
