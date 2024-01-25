#![cfg_attr(not(feature = "std"), no_main)]

pub type TyAlias1 = u32;
pub type TyAlias2 = TyAlias1;

#[ink::contract]
pub mod divide_before_multiply {

    const CONST_1: u32 = 1;
    const CONST_2: u32 = 2;
    const CONST_3: u32 = 3;

    #[ink(storage)]
    pub struct DivideBeforeMultiply {}

    impl DivideBeforeMultiply {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        fn return_1(&self) -> u32 {
            1
        }

        fn return_2(&self) -> u32 {
            2
        }

        #[ink(message)]
        pub fn test_lint(&mut self) {
            // Constants
            let _ = CONST_1 / CONST_2 * CONST_3;

            // Variables
            let a: u32 = 1;
            let b: u32 = 2;
            let _ = a / b * (4 + 4);
            let _ = a / b * CONST_3;
            let _ = a / b * a;

            // Function calls
            let _ = self.return_1() / self.return_2() * self.return_1();
            let _ = a / self.return_2() * 2;
            let _ = self.return_1() / CONST_1 * b;

            // Type aliases
            let c: crate::TyAlias2 = 4;
            let _ = c / b * 4;
            let _ = a / c * CONST_3;
            let _ = a / b * c;
        }
    }
}

fn main() {}
