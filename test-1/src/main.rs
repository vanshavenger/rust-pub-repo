
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
   let p = Person {
        name: "John".to_string(),
        age: 25,
   };

   let mut vc = Vec::<u8>::new();

    p.serialize(&mut vc).unwrap();



    

    for i in 0..vc.len() {
        vc[i] = vc[i] + 1;
    }

    println!("Encoded: {:?}",vc);




    let mut vc_slice: &mut &[u8] = &mut &vc[..];
    let c = Person::try_from_slice(vc_slice).unwrap();
    
    println!("Decoded: {:?}",c);
   


}





