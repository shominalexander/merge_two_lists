#[derive(Clone, Debug, PartialEq)]
struct Item { next: Option<Box<Item>>, value: i32 }

impl Item {
 fn create(list: Vec<i32>) -> Self {
  let mut item: Item = Item { next: None, value: 0 };

  for (index, value) in list.iter().rev().enumerate() {
   if index == 0 {
    item.next = None;

   } else {//if index == 0 {
    item.next = Some(Box::new(Item { next: item.next, value: item.value }));

   }//} else {//if index == 0 {

   item.value = *value;
  }//for (index, value) in list.iter().enumerate() {

  item
 }//fn create(list: Vec<i32>) -> Self {

 fn merge(&mut self, mut item: Item) {
  let mut merge : Vec<i32> = self.vector();
  let     vector: Vec<i32> = item.vector();

  for value in vector {
   merge.push(value);

  }//for value in vector {

  merge.sort();

  *self = Item::create(merge);
 }//fn merge(&mut self, mut item: Item) {

 fn vector(&mut self) -> Vec<i32> {
  if let Some(ref mut item) = self.next {
   let mut vector: Vec<i32> = item.vector();
   
   vector.push(self.value);

   vector

  } else {//if let Some(ref mut item) = self.next {
   let mut vector: Vec<i32> = vec![];

   vector.push(self.value);

   vector
  }//} else {//if let Some(ref mut item) = self.next {
 }//fn vector(&mut self) -> Vec<i32> {
}//impl List {

fn request() -> String {
 let mut value: String = String::new();

 std::io::stdin().read_line(&mut value).expect("Input failed");

 value = value.trim().to_string();
 value = value.replace("\n", "") ;
 value = value.replace("\r", "") ;

 value
}//fn request() -> String {

fn main() {
 'first: loop {
  println!("\r\n\r\nfirst vector:");

  let input: String = request();
 
  match &input[..] {
   "back" | "exit" => { break 'first; }
   _ => {
    match serde_json::from_str::<Vec<i32>>(&input[..]) {
     Ok(list) => {
      let mut first: Item = Item::create(list); println!("first: {:?}", first);

      'second: loop {
       println!("\r\nsecond vector:");

       let input: String = request();

       match &input[..] {
        "back" => { break 'second; }
        "exit" => { break 'first ; }
        _ => {
         match serde_json::from_str::<Vec<i32>>(&input[..]) {
          Ok(list) => {
           let second: Item = Item::create(list);

           println!("second: {:?}", second);

           first.merge(second);

           println!("\r\nfirst: {:?}", first);
          }//Ok(list) => {

          Err(error) => {
           println!("Error: {:?}", error);

          }//Err(error) => {
         }//match serde_json::from_str::<Vec<i32>>(&input[..]) {
        }//_ => {
       }//match &input[..] {
      }//'second: loop {
     }//Ok(list) => {

     Err(error) => {
      println!("Error: {:?}", error);

     }//Err(error) => {
    }//match serde_json::from_str::<Vec<i32>>(&input[..]) {
   }//_ => {
  }//match &input[..] {
 }//'first: loop {
}//fn main() {
