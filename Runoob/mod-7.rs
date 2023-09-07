mod nation{
    pub mod government{
        pub fn govern(){}
    }

    pub use government::govern;
}


fn main(){
    nation::govern();
}
