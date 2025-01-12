
#![forbid(unsafe_code)]

fn main() {


    fn generate_next_num(mut my_vec: Vec<u32>, mut quantity_to_push: u32) -> Vec<u32> {


        if my_vec[my_vec.len() - 1] == 0 {
            my_vec.push(1);
        } else if my_vec[my_vec.len() - 1] == 1 && my_vec[my_vec.len() - 2] == 0 {
            my_vec.push(1);
        } else {
            my_vec.push(my_vec[my_vec.len() - 1] + my_vec[my_vec.len() - 2])
        }

        quantity_to_push -= 1;

        return generate_next_num(my_vec, quantity_to_push);

    }


    generate_next_num(vec![0], 5_u32);



}