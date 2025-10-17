#[macro_export]
macro_rules! make_token {
    ($c:expr, $self:ident, $pattern:pat $(if $guard:expr)? $(,)?) => ({
        let mut acc = String::new();
        acc.push($c);
        while let Some(&next) = $self.peek() {
            match next {
                $pattern $(if $guard)? => acc.push($self.next().unwrap()),
                _ => break,
            }
        }
        acc
    })
}