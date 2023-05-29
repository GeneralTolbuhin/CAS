fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    let mut swapped;
    
    for i in 0..n-1 {
        swapped = false;
        
        for j in 0..n-i-1 {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
                swapped = true;
            }
        }
        
        if !swapped {
            break;
        }
    }
}

fn main() {
    let mut array = [64, 34, 25, 12, 22, 11, 90];
    
    bubble_sort(&mut array);
    
    println!("Sorted array: {:?}", array);
}
