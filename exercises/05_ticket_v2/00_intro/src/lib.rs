fn intro() -> &'static str {
    // TODO: fix me 👇
    let var_name = "I'm ready to refine the `Ticket` type!";
    var_name
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(intro(), "I'm ready to refine the `Ticket` type!");
    }
}
