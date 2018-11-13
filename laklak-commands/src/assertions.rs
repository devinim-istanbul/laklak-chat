#[macro_export]
macro_rules! id_like {
    ($token:expr) => {
        if $token.len() != 16 {
            panic!("Invalid token: {}", $token);
        } else {
            $token
        };
    }
}
