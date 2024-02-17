pub mod domain {
    pub mod models {
        pub mod tasks;
    }
    pub mod commands {
        pub mod tasks;
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
