pub mod modle{
   pub struct  User{
      pub name:String,
      pub age:i32
   }
   impl User{
       pub fn login(&self,name:String,age:i32)->String{
           "userlogindemo".to_string()
       }
   }
}