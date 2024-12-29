mod front_of_house;

fn deliver_order() {}

mod back_of_house {
    fn cook_order(){}
    fn fix_incorrect_order(){
        super::deliver_order();
    }
    mod dishwasher {
        fn wash_dishes(){
        }
    }
    mod cook{
        fn make_food(){
        }
    }
}
pub fn eat_at_resaurant() {
    crate::front_of_house::hosting::add_to_waitlist(); // this can be better if you might move the
                                                       // definition and the calling code
                                                       // separately. This outcome is generally
                                                       // more likely and is therefore preferred
                                                       // generally. 
    front_of_house::hosting::add_to_waitlist(); // relative paths can be good if you
                                                // move this code and the code it calls seperately
                                                // into a new
                                                // module, this one doesn't need to be updated,
                                                // while the absolute path would need updated. 
}
