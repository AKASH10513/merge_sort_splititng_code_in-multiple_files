mod sorting_algorithm;
use crate::sorting_algorithm::sorts;
fn main()
{
  let mut arr =  vec![10,3,8,7,5];
  println!("previous array content is : {:?}", arr);
  let r = arr.len()-1;

  sorts::merge_sort(0,r,&mut arr);
  println!("After sorting content is : {:?}",arr);

}