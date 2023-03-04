use rand::Rng;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn generate_list(size:i32)->Vec<i32>{
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();
    for _ in 0..size {
        vec.push(rng.gen_range(0..size));
    }
    return vec;
}

fn insertion_sort(list:Vec<T>)->Vec<T>{
    let mut new:Vec<T> = Vec::new();
    list.for_each(|i|{
        if new.is_empty() {
            new.push(i);
        }else{
            for k in new.len()..0{
                if i > { }
            }
        }
    });
    return new;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn its_sorted(){
        let list = generate_list(1_000_000);

    }
}
