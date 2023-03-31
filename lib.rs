#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod increamenter1 {

//importing mapping from ink storage
    use ink::storage::Mapping;

//module that provides a data structure so that state variable can store values...
    #[ink(storage)]
    //creating a type increamenter that has a value and a map that maps each accountId 
    // to a value
    //accountId can be an address
   pub struct Increamenter1{
        value: i32,
        my_map: Mapping<AccountId, i32>,
    }


// implementation of our Increamenter1 smart contract
    impl Increamenter1{
        //this attribute macro shows that the code snippet is a constructor
        #[ink(constructor)]
        //a function that takes an init_value and returns Self(meaning it returns the type of the struct)
        pub fn new(init_value: i32)-> Self{
            //creating a new variable and assigning it to its default value
            // its mutable hence can be modified later on
            let mut my_map = Mapping::default();
            //getting the accoundId of the one deployed the contract
            let caller = Self::env().caller();
            //inserting new key pair of which is the accountId of the caller and the value to 0
            my_map.insert(&caller, &0);
            //creating a new instance of our struct of which value is assigned to value we passed as our parameter
            // and my_map to to the one we just created
            Self{
                 value: init_value,
                 my_map,
            }
        }

        #[ink(constructor)]
        //this code snippet once called creates an instance of our struct and initializes
        //our value to zero and our mapping to default
        pub fn default()-> Self{
            Self{
                value: 0, 
                my_map:Mapping::default(),
            }
        }
    

#[ink(message)]
    //a public function that reference to self(instance of our contract)
    //it then returns an i32
    pub fn get(&self)-> i32{
        //&self borrows reference from our instance and returns the value
        self.value
    }

    #[ink(message)]
    pub fn get_mine(&self) -> i32 {
        //retrieves the user AccountId and check there corresponding values
        // or if there is no value in them
        let caller = self.env().caller();
        self.my_map.get(&caller).unwrap_or_default()
    }

    #[ink(message)]
    //a function that takes ownership and incraments the value of the caller of the function
    pub fn inc(&mut self, by: i32){
   
        self.value += by;
    }

    #[ink(message)]
    
    pub fn inc_mine(&mut self, by: i32){
        //retrieving the address of the caller
        let caller = self.env().caller();
        //retrieving the value associated with the caller by the the use of the get_mine() function
        //the function returns the value(which can be value defaulted to 0) and the associated address
        let my_value = self.get_mine();
        //this one increases  the value of the caller  by parameter passed and then update it in the map
        self.my_map.insert(caller, &(my_value + by));
    
    }

    #[ink(message)]
    //this code snippet shows how the user can be able to remove his own keypair
    pub fn remove_mine(&self){
        let caller = self.env().caller();
        self.my_map.remove(&caller);
    }

  }
    #[ink::test]
//to test the default working of our smart contract
    fn default_works(){
        let contract = Increamenter1::default();
        //value expected should be equal to zero
        assert_eq!(contract.get(), 0);
    }

    #[ink::test]
    fn it_works(){
        let mut contract = Increamenter1::new(42);
        assert_eq!(contract.get(), 42);
        contract.inc(5);
        assert_eq!(contract.get(), 47);
        contract.inc(-50);
        assert_eq!(contract.get(), -3);
    }

    #[ink::test]
    fn map_works(){
        let contract = Increamenter1::new(11);
        assert_eq!(contract.get(), 11);
        assert_eq!(contract.get_mine(), 0);
    }
   
   #[ink::test]
   pub fn inc_mine_works(){
    let mut contract = Increamenter1::new(5);
    assert_eq!(contract.get_mine(), 0);
    contract.inc_mine(5);
    assert_eq!(contract.get_mine(), 5);
    contract.inc_mine(5);
    assert_eq!(contract.get_mine(), 10);
   }

   #[ink::test]
   pub fn remove_mine_works(){
    let mut contract = Increamenter1::new(11);
    assert_eq!(contract.get_mine(), 0);
    contract.inc_mine(5);
    assert_eq!(contract.get_mine(),5);
    contract.remove_mine();
    assert_eq!(contract.get_mine(), 0);
   }
    










//     use ink::primitives::AccountId;


//     #[ink(storage)]
//     pub struct Increamenter1{
//         //storage of our bool value
//         my_bool: bool,
//         //storage of a number
//         my_number: i32,
//     }

//     //another example of how one can use AccountId, Balance and hash(excluded) as if they were primitive types

//     #[ink(storage)]
//     pub struct MyContract{
//         //storing the accountId
//         my_account: AccountId,
//         //storing the balance of the account
//         my_balance: Balance,
//     }

// #[ink(storage)]
//     struct MyContract{
//         number: u32,
//     }

//     impl MyContract {
// #[ink(constructor)]
//         pub fn new(unit_value: u32) -> Self{
//             Self{
//                 number: unit_value,
//             }
//         }

// #[ink(constructor)]
//         pub fn default()->{
//             Self{
//                 number: Default::default(),
//             }
//         }
//     }



    // /// Defines the storage of your contract.
    // /// Add new fields to the below struct in order
    // /// to add new static storage fields to your contract.
    // #[ink(storage)]
    // pub struct Increamenter1 {
    //     /// Stores a single `bool` value on the storage.
    //     value: bool,
    // }

    // impl Increamenter1 {
    //     /// Constructor that initializes the `bool` value to the given `init_value`.
    //     #[ink(constructor)]
    //     pub fn new(init_value: bool) -> Self {
    //         Self { value: init_value }
    //     }

    //     /// Constructor that initializes the `bool` value to `false`.
    //     ///
    //     /// Constructors can delegate to other constructors.
    //     #[ink(constructor)]
    //     pub fn default() -> Self {
    //         Self::new(Default::default())
    //     }

    //     /// A message that can be called on instantiated contracts.
    //     /// This one flips the value of the stored `bool` from `true`
    //     /// to `false` and vice versa.
    //     #[ink(message)]
    //     pub fn flip(&mut self) {
    //         self.value = !self.value;
    //     }

    //     /// Simply returns the current value of our `bool`.
    //     #[ink(message)]
    //     pub fn get(&self) -> bool {
    //         self.value
    //     }
    // }

    // /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    // /// module and test functions are marked with a `#[test]` attribute.
    // /// The below code is technically just normal Rust code.
    // #[cfg(test)]
    // mod tests {
    //     /// Imports all the definitions from the outer scope so we can use them here.
    //     use super::*;

    //     /// We test if the default constructor does its job.
    //     #[ink::test]
    //     fn default_works() {
    //         let increamenter1 = Increamenter1::default();
    //         assert_eq!(increamenter1.get(), false);
    //     }

    //     /// We test a simple use case of our contract.
    //     #[ink::test]
    //     fn it_works() {
    //         let mut increamenter1 = Increamenter1::new(false);
    //         assert_eq!(increamenter1.get(), false);
    //         increamenter1.flip();
    //         assert_eq!(increamenter1.get(), true);
    //     }
    // }
}



// ////////////////////////ESCROW CODES TO RECONSIDER///////////////////////////
// #![cfg_attr(not(feature = "std"), no_std)]

// #[ink::contract]
// mod my_contract {

//     #[ink(storage)]
//     pub struct MyContract {
//         // Store a contract owner
//         owner: AccountId,
//     }

//     impl MyContract {
//         #[ink(constructor)]
//         pub fn new() -> Self {
//             Self {
//                 owner: Self::env().caller(),
//             }
//         }
//         /* --snip-- */
//     }
//HERE THE CONTRACT CALLER HAS BEEN SAVED AS OWNER 
// THEREFORE FUNCTIONS CAN LATER BE MADE MAKING SURE THAT THE CALLER OF THE CONTRACT IS OWNER



//