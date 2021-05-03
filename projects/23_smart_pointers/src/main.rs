//use crate::smart_pointers::box_contrived_example;
//use smart_pointers;

fn main() {
    smart_pointers::box_contrived_example();
    smart_pointers::test_con_list();
    //box_contrived_example();
    smart_pointers::test_rc_list();
    smart_pointers::test_multiple_owners_of_mutable();
}

