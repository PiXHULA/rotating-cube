pub fn print_color(color_str: &str, color: [f32; 4]) {
        println!("{}: {:.3}, {:.3}, {:.3}", color_str, color[0], color[1], color[2]);
    }

pub fn reset_color() -> [f32; 4] {
    let temp_color: [f32; 4] = [0.2, 0.0, 0.0, 1.0];
    temp_color
}

pub fn rainbow_effect(color: [f32; 4]) -> [f32; 4] {
    let mut temp_color: [f32; 4] = color;

    if temp_color[0] >= 0.0  && temp_color[1] <= 0.0 && temp_color[2] <= 0.0 { //RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        temp_color[0] += 0.005;     
        temp_color[1] = 0.0;     
        temp_color[2] = 0.0;
        // print_color("red", temp_color);     
    }
    if temp_color[0] >= 1.0 && temp_color[1] <= 1.0 && temp_color[2] <= 0.0 { //ORANGE: [f32; 4] = [1.0, 0.65, 0.0, 1.0];
        temp_color[0] = 1.0;   
        temp_color[1] += 0.005;   
        temp_color[2] = 0.0;   
        // print_color("orange", temp_color);     
    } 
    if temp_color[0] >= 1.0 && temp_color[1] >= 0.65 && temp_color[2] <= 0.0 { //YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
        temp_color[0] = 1.0;   
        temp_color[1] += 0.005;   
        temp_color[2] = 0.0;
        // print_color("yellow", temp_color);     

    }
    if temp_color[0] <= 1.0 && temp_color[1] >= 1.0 && temp_color[2] <= 0.0 { //GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        temp_color[0] -= 0.005;
        temp_color[1] = 1.0;   
        temp_color[2] = 0.0;     
        // print_color("green", temp_color);
    }
    if temp_color[0] <= 0.0 && temp_color[1] <= 1.0 && temp_color[2] < 1.0 { //CYAN: [f32; 4] = [0.0, 1.0, 1.0, 1.0]; 
        temp_color[0] = 0.0;
        temp_color[1] = 1.0;   
        temp_color[2] += 0.005; 
        // print_color("cyan", temp_color);
    }
    if temp_color[0] <= 0.0 && temp_color[1] >= 0.0 && temp_color[2] >= 1.0 { //BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0]; 
        temp_color[0] = 0.0;
        temp_color[1] -= 0.005;   
        temp_color[2] = 1.0;
        // print_color("blue", temp_color);
    }
    if temp_color[0] <= 0.55 && temp_color[1] <= 0.0 && temp_color[2] <= 1.0 { //VIOLET: [f32; 4] = [0.55, 0.0, 0.9, 1.0]; 
        temp_color[0] += 0.005;
        temp_color[1] = 0.0;   
        temp_color[2] -= 0.001; 
        // print_color("purple", temp_color);
    }
    if temp_color[0] >= 0.55 && temp_color[1] <= 0.0 && temp_color[2] <= 0.9 { //BACK TO RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0]; 
        temp_color[0] += 0.005;
        temp_color[1] = 0.0;   
        temp_color[2] -= 0.01; 
        // print_color("back to red", temp_color);
    }
    temp_color
}