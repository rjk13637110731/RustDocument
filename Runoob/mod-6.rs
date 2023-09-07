mod nation{
    pub mod government{
        pub fn govern(){}
    }

    pub fn govern(){}
}

use crate::nation::government::govern;
use crate::nation::govern as nation_govern;

fn main(){
    govern();
    nation_govern();
}
