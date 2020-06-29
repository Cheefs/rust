mod outermost {
    pub fn middle_fn() {
        middle_secret_fn()
    }
    fn middle_secret_fn() {
        println!("secret middle")
    }

    pub mod inside {
        fn secret_inner_fn() {
            println!("secret inner");
        }
        pub fn inner_fn() {
            secret_inner_fn();
        }
    }
}

pub fn try_me() {
    outermost::middle_fn();
    outermost::inside::inner_fn();
}