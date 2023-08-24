struct Box {
    dimensions :Dimensions,
    weight:i32,
    color:Color,
}

enum Color {
    Brown,
    White,
}

struct Dimensions {

    lenght:i32,
    width:i32,
    height:i32,
}

impl Dimensions {
    fn print(&self) {
        println!("width : {:?}",self.width);
        println!("lenght : {:?}",self.lenght);
        println!("height : {:?}",self.height);
    }
}



impl Color {
    fn print(&self){
        match self {
            Color::Brown => println!("Color is brown"),
            _ => println!("Color is white"),
            
        }
    }
}



impl Box {
    fn new(weight:i32,color:Color,dimensions:Dimensions) -> Self {
        Self{
            weight,
            color,
            dimensions,
        }
    }
    fn print(&self){
        self.color.print();
        self.dimensions.print();
        println!("weight : {:?}",self.weight);
    }
}


fn main() {
    let small_dimensions = Dimensions{
        lenght : 10,
        width :12,
        height : 12,
    };


    let small_box = Box::new(50,Color::Brown, small_dimensions);
    small_box.print();

    }
