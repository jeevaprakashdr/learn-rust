
// defining the module
enum SpreadSheetCell<'a> {
    Int(i32),
    Float(f64),
    Text(String),
    Rawstring(&'a str),
}

pub fn demo_enum_inside_vector(){
    
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(2.34),
        SpreadSheetCell::Text(String::from("hello i am a cell")),
        SpreadSheetCell::Rawstring("i am a raw string")
    ];

    match &row[0] {
        SpreadSheetCell::Int(i) => println!("{}", i),
        _=> println!("not an integer")
    };
} 
