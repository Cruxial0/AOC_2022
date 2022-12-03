pub(crate) use std::{fs::{self}, io::{BufReader, BufRead}};

/// ### Container for a Rock/Paper/Scissors Shape
/// 
/// * 'letter_representation' - Letter represenation of Rock, Paper or Scissors
/// * 'beats' - Letter representation that this shape beats
/// * 'points' - Points are awarded from playing this shape
#[derive(Default)]
struct Shape{
    /// What would this do?
    letter_representation: String, /// Letter represenation of Rock, Paper or Scissors
    beats: String, // Letter representation that this shape beats
    points: i32, // How many points are awarded from playing this shape
    name: String
}

impl Shape {
    /// ### Returns new instance of Shape
    /// 
    /// * 'letter_representation' - Letter represenation of Rock, Paper or Scissors
    /// * 'beats' - Letter representation that this shape beats
    /// * 'points' - Points are awarded from playing this shape
    /// 
    ///  ```
    /// let shape = Shape::new("A".to_owned(), "B".to_owned(), 1);
    /// ```
    pub fn new(letter_representation: String, beats: String, points: i32, name: String) -> Self{
        Self{letter_representation, beats, points, name}
    }

    pub fn empty() -> Self{ 
        Default::default()
     }

    /// Checks if Shape beats letter
    /// 
    /// * 'letter' - Letter to check
    pub fn beats(&self, letter: String) -> bool{
        letter == self.beats
    }

    pub fn is_even(&self, letter: String) -> bool{
        let l = &letter;
        // 'Decrypt' instuctions
        let opposite = match letter.to_lowercase().as_str(){
            "a" => "x".to_owned(),
            "b" => "y".to_owned(),
            "c" => "z".to_owned(),
            &_ => l.to_owned()
        };
        // Return if letter equals letter representation of instance
        opposite == self.letter_representation
    }

    pub fn get_opposite(letter: String) -> &'static str{
        let opposite = match letter.to_lowercase().as_str(){
            "a" => "x",
            "b" => "y",
            "c" => "z",
            &_ => todo!()
        };

        opposite
    }

    pub fn get_losing(&self, letter: String) -> &str{
        let opposite = Shape::get_opposite(letter);

        let losing = match opposite.to_lowercase().as_str(){
            "x" => "z",
            "y" => "x",
            "z" => "y",
            &_ => todo!()
        };

        losing
    }

    pub fn get_winning(&self, letter: String) -> &str{
        let opposite = Shape::get_opposite(letter);

        let winning = match opposite.to_lowercase().as_str(){
            "x" => "y",
            "y" => "z",
            "z" => "x",
            &_ => todo!()
        };

        winning
    }
}

/// Returns list of shape instances
fn init_shapes() -> Vec<Shape>{
    // Generate empty Vector
    let mut output: Vec<Shape> = Vec::new();
    // Generate an instance of each individual shape
    output.push(Shape::new("Y".to_lowercase().to_owned(), "A".to_lowercase().to_owned(), 2, "Paper".to_owned()));
    output.push(Shape::new("X".to_lowercase().to_owned(), "C".to_lowercase().to_owned(), 1, "Rock".to_owned()));
    output.push(Shape::new("Z".to_lowercase().to_owned(), "B".to_lowercase().to_owned(), 3, "Scissors".to_owned()));

    output // Return output
}

/// Returns a Vec with tuples
fn get_file(file_path: &str) -> Vec<(String, String)>{
    let mut output: Vec<(String, String)> = Vec::new();

    let file = fs::File::open(file_path).expect("no such file.");

    for x in BufReader::new(file).lines() {
        let text = x.expect("msg");
        let split: Vec<&str> = (text).split_whitespace().collect();
        output.push((split[0].to_owned(), split[1].to_owned()))
    }

    output
}

fn calculate_points_part_1(input: Vec<(String, String)>, shapes: &Vec<Shape>) -> i32{
    let mut i: i32 = 0;
    
    for x in &input{
        let mut iter = shapes.into_iter(); // Get iterator
        // Get shape instance matching string value, and proceed if the value is not None
        if let Some(instance) = iter.find(| &y| y.letter_representation == x.1.to_lowercase()){
            if instance.beats(x.0.to_lowercase()){ i = i + 6} // If instance gets beaten by x, add 6 points
            if instance.is_even(x.0.to_lowercase()) { i = i + 3 } // If instance gets drawn by x, add 3 points
            i = i + instance.points; // Add shape points
        }
    }

    i
}

fn calculate_points_part_2(input: Vec<(String, String)>, shapes: &Vec<Shape>) -> i32{
    let mut total_points: i32 = 0;
    let mut instance: &Shape = &Shape::empty();
    for x in &input{
        let mut iter = shapes.into_iter(); // Get iterator
        // Get shape instance matching string value, and proceed if the value is not None

        // Check if second line at position = x
        if x.1.to_lowercase() == "x"{
            // Get losing Shape
            instance = iter.find(| &y| y.letter_representation == Shape::get_losing(y, x.0.to_lowercase().to_owned())).unwrap();
            println!("{}", instance.name)
        }
        
        // Check if second line at position = y
        if x.1.to_lowercase() == "y"{
            // Get shape that draws
            let opposite = Shape::get_opposite(x.0.to_lowercase().to_owned());
            instance = iter.find(| &y| y.letter_representation == opposite).unwrap();
            println!("{}", instance.name)
        }

        // Check if second line at position = z
        if x.1.to_lowercase() == "z"{
            // Get winning shape
            instance = iter.find(| &y| y.letter_representation == Shape::get_winning(y, x.0.to_lowercase().to_owned())).unwrap();
            println!("{}", instance.name)
        }

        if instance.beats(x.0.to_lowercase()){
            println!("+ 6 (win)");
             total_points = total_points + 6
            } // If instance gets beaten by x, add 6 points
        else if instance.is_even(x.0.to_lowercase()) {
            println!("+ 3 (draw)");
             total_points = total_points + 3 
            } // If instance gets drawn by x, add 3 points
        else {
            println!("+ 0 (lose)"); // Visualize a loss
        }

        // Add shape points
        println!("+ {} ({} points)", instance.points, instance.name);
        total_points = total_points + instance.points; 

        println!("Total: {}", total_points);
    }

    total_points // Return points
}

fn main() {
    let file_path = "src\\input.txt";
    let points_1 = calculate_points_part_1(get_file(file_path), &init_shapes());
    let points_2 = calculate_points_part_2(get_file(file_path), &init_shapes());

    println!("\nPart 1: {}", points_1);
    println!("Part 2: {}", points_2);
}

