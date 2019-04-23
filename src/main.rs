use raylib::prelude::*;
use rand::Rng;

fn main() {
    
        let screen_width: i32 = 1920;
        let screen_height: i32 = 800;
        let rl = raylib::init().size(screen_width, screen_height).title("Bubble Sort").msaa_4x().build();
        let mut rng = rand::thread_rng();
        let mut values: Vec<i32> = Vec::new();
    
        for _ in 0..screen_width {
        
            values.push(rng.gen_range(1, screen_height));
    }
    
    let length = values.len();
    
        /*
for _ in 0..length{

for j in 0.. length - 1 {

let mut temp = values[j + 1];
let mut a = values[j];
let mut b = values[j + 1 ];

if a > b {

// swap vals
values[j + 1] = values[j];
values[j] = temp;
}
}
}
*/
    
        let mut n = 0;
    
        rl.set_target_fps(60);
    
        while !rl.window_should_close() {
        
            if n <= length - 1{
            
                for j in 0..length - n - 1{
                
                    let temp = values[j + 1];
                    let a = values[j];
                    let b = values[j + 1 ];
                
                    if a > b {
                    
                        values[j + 1] = values[j];
                        values[j] = temp;
                }
            }
            n+= 1;
        }
        
        rl.begin_drawing();
            rl.clear_background(Color::new(10, 10, 10, 255));
        
            let mut iter = 0;
        
            for i in &values {
            
                rl.draw_line(iter, screen_height, iter, screen_height - i, Color::new(180, 180, 180, 255));
                iter += 1;
        }
        rl.end_drawing();
    }
    println!("Hello, world!");
}