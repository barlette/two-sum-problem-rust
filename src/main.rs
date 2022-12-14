use std::collections::HashMap;

fn main() {
    let nums: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 9;

    // let nums: Vec<i32> = vec![3, 2, 4];
    // let target: i32 = 6;

    // let nums: Vec<i32> = vec![3, 3];
    // let target: i32 = 6;

    //println!("{:?}", two_sum_first_version(nums, target))
    println!("{:?}", two_sum_second_version(nums, target));
}

fn two_sum_second_version(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut diffs = HashMap::new();
    for x in 0..nums.len() {
        let diff = target - nums[x];
        if !diffs.get(&nums[x]).is_none() && (*diffs.get(&nums[x]).unwrap_or(&-1) != x as i32) {
            return vec![*diffs.get(&nums[x]).unwrap(), x.try_into().unwrap()];
        }
        diffs.insert(diff, x as i32);
    }
    return vec![];
}


// fn two_sum_first_version(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     let mut output: Vec<i32> = vec![];

//     for x in 1..nums.len() {

//         let index = match nums
//             .iter()
//             .position(|&element| element == (target - nums[x]))
//         {
//             Some(value) => value as i32,
//             None => -1 as i32,
//         };

//         if index > -1 && x != index as usize {
//             output.push(x as i32);
//             output.push(index);
//             return output;
//         }
//     }

//     return output;
// }


