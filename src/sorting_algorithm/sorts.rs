//the below is selection sort:::
pub fn selection_sort(arr : &mut Vec<i32>)
{
  let n =  arr.len();

  for i in 0..n
  {
    let mut min_idx =  i;
    for j in i+1..n{
        if arr[min_idx] > arr[j]
        {
            min_idx  =  j;
        }
    }
    (arr[i], arr[min_idx]) = (arr[min_idx],arr[i]);
  }
}



pub fn merge(left : usize, mid:usize, right:usize, arr: &mut Vec<i32>)
{
    let mut my_arr= Vec::new();
    let mut l =  left;

    let mut r = mid+1;
   
    while l <= mid && r <= right{
        if arr[l] < arr[r]
        {
            my_arr.push(arr[l]);
            l+=1;
        }
        else{
            my_arr.push(arr[r]);
            r+=1;
        }
    }

    while l <= mid
    {
        my_arr.push(arr[l]);
        l+=1;
    }
    while r <=right
    {
        my_arr.push(arr[r]);
        r+=1;

    }
    let m  = my_arr.len();
    

    l = left;
    

    for i in 0..m{
        arr[l] = my_arr[i];
        l+=1; 
    }


}


pub fn merge_sort(left: usize, right:usize, arr: &mut Vec<i32> )
{
    if right <= left
    {
        return;

    }

    let mid = (left+right)/2;
   

    merge_sort(left,mid,arr);
    merge_sort(mid+1,right,arr);
    merge(left, mid, right, arr);

}