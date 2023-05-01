fn main(){
    if let Some(pos) = binary_search(69, &mut [69,420,1,2,3]){
        println!("{}", pos);
    } else {
        println!("Not in the given array");
    }
}

fn binary_search(num:i32, arr:&mut [i32])->Option<usize>{
    arr.sort();
    let mut lo = 0;
    let mut hi = arr.len();
    while hi>lo{
        let mid = ((lo + hi)/2)-1;
        if arr[mid]<num{
            lo+=mid; 
        } else if arr[mid]>num{
            hi=hi-mid-1;
        } else {
            return Some(mid);
        }
    }
    return None;
}
