pub fn both<A, B, E>(a: Result<A, E>, b: Result<B, E>) -> Result<(A, B), E> {
    a.and_then(|a| b.map(|b| (a, b)))
}