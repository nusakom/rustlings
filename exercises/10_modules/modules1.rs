mod sausage_factory {
    // This function remains private.
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // This function is now private and can access the private function.
    fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }

    // This is a public function that can be called from outside the module.
    pub fn public_make_sausage() {
        make_sausage();
    }
}

fn main() {
    // Call the public function which in turn calls the private function.
    sausage_factory::public_make_sausage();
}
