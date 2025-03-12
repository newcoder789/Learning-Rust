struct Rect {
    length: i32,
    width:  i32
}

impl Rect{
    // IN rust , a struct's self is like this in class 
    fn area(&self) -> i32{
        self.width*self.length
    }

    // &self and something else as input
    fn fancing_cost(&self, num: i32) -> i32{
        2* self.length* self.width * num
    }

    // without &self,  work like static function 
    fn debug(){
        println!("This is Debug Function");
    }
}


fn main(){
    let rect = Rect{
        length: 3,
        width: 3
    };
    println!("The area of the rectangle is: {}", rect.area());
    println!("The fancing cost of the rectangle is: {}", rect.fancing_cost(2));
    
    // LIKE STATIC FUNCITON IN CLASS WE CANT CALL THIS FUNCTION WITH OBJECT
    // println!("{}", rect.Debug());
    println!("{:?}", Rect::debug());


    // println! cause it do not return anything also gives () to only get output things
    Rect::debug();
}