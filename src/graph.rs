pub mod graph {

    //useless change

    use crate::commons::commons::*;
    
    pub fn init_canvas() {
        let canvas: CanvasElement = web::document()
            .get_element_by_id("viewport")
            .unwrap()
            .try_into()
            .unwrap();
        
        canvas.set_width((std::cmp::min(window().inner_width(),window().inner_height())*4/5) as u32);
        canvas.set_height((1.1 * canvas.width() as f64) as u32);
        let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();
        ctx.clear_rect(0.0, 0.0 - canvas.width() as f64 * 0.1, canvas.width() as f64, canvas.width() as f64);
        ctx.begin_path();
        ctx.set_stroke_style_color("#505050");
        for i in 1..9 {
            ctx.move_to(set_to_raster_x(i as f64 / 9.0, canvas.width()), set_to_raster_y(           0.0, canvas.width())); ctx.line_to(set_to_raster_x(i as f64 / 9.0, canvas.width()), set_to_raster_y(           1.0, canvas.width()));
            ctx.move_to(set_to_raster_x(           0.0, canvas.width()), set_to_raster_y(i as f64 / 9.0, canvas.width())); ctx.line_to(set_to_raster_x(           1.0, canvas.width()), set_to_raster_y(i as f64 / 9.0, canvas.width()));
        }
        ctx.stroke();
        
        ctx.begin_path();
        ctx.set_stroke_style_color("#000030");
            for i in 0..4 {
                ctx.move_to(set_to_raster_x(i as f64 / 3.0, canvas.width()), set_to_raster_y(           0.0, canvas.width())); ctx.line_to(set_to_raster_x(i as f64 / 3.0, canvas.width()), set_to_raster_y(           1.0, canvas.width()));
                ctx.move_to(set_to_raster_x(           0.0, canvas.width()), set_to_raster_y(i as f64 / 3.0, canvas.width())); ctx.line_to(set_to_raster_x(           1.0, canvas.width()), set_to_raster_y(i as f64 / 3.0, canvas.width()));
            }
        ctx.stroke();

        // // let t = format!("{}px Arial", canvas.height()*30/500);
        // // ctx.set_font(&t);
        // // ctx.set_text_align(TextAlign::Center);
        // // ctx.set_text_baseline(TextBaseline::Middle);
        // // ctx.set_stroke_style_color("#3B99FC"); //? X
        // // ctx.stroke_text("X: 0", set_to_raster_x(0.25,canvas.width()), set_to_raster_y(-0.05,canvas.width()), None);
        // // ctx.set_stroke_style_color("#AA71C6"); //? O
        // // ctx.stroke_text("O: 0", set_to_raster_x(0.75,canvas.width()), set_to_raster_y(-0.05,canvas.width()), None);

    }
    
    fn set_to_raster_x(x: f64, width: u32) -> f64 {
        //Assumes written with 1x1 pixels
        x/*/500.0*/*width as f64
    }    
    fn set_to_raster_y(y: f64, width: u32) -> f64 {
        //Assumes written with 1x1 pixels
        (y + 0.1)/*/500.0*/*width as f64
    }    
    
    pub fn select_draw(x1: u8, y1: u8, x2: u8, y2: u8) {
        let canvas: CanvasElement = web::document()
            .get_element_by_id("viewport")
            .unwrap()
            .try_into()
            .unwrap();
        let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();
        ctx.set_fill_style_color("#AAB1FF60");
        // ctx.set_fill_style_color("#000000A0");
        ctx.fill_rect(set_to_raster_x(x1 as f64/3.0,canvas.width()), set_to_raster_y(y1 as f64/3.0,canvas.width()), set_to_raster_x(1.0/3.0,canvas.width()), set_to_raster_y((1.0/3.0) - 0.1,canvas.width()));
        ctx.fill_rect(set_to_raster_x(x2 as f64/3.0,canvas.width()), set_to_raster_y(y2 as f64/3.0,canvas.width()), set_to_raster_x(1.0/3.0,canvas.width()), set_to_raster_y((1.0/3.0) - 0.1,canvas.width()));
    }

    impl GameStruct {
        pub fn redraw(&self) {
            init_canvas();
            let canvas: CanvasElement = web::document()
            .get_element_by_id("viewport")
            .unwrap()
            .try_into()
            .unwrap();
            let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();
            ctx.set_text_align(TextAlign::Center);
            ctx.set_text_baseline(TextBaseline::Middle);
            for i in 0..=2 {
                for j in 0..=2 {
                    match self.state[i][j][0].value {
                        cell if state_test(cell, BIG_BLANK) => {
                            continue;
                        },
                        cell if state_test(cell, BIG_X) => {
                            ctx.set_stroke_style_color("#3B99FC");
                            ctx.begin_path();
                            ctx.move_to(set_to_raster_x(i as f64 /3.0 +     1.0/18.0,canvas.width()), set_to_raster_y(j as f64 /3.0 +     1.0/18.0,canvas.width()));
                            ctx.line_to(set_to_raster_x(i as f64 /3.0 + 5.0*1.0/18.0,canvas.width()), set_to_raster_y(j as f64 /3.0 + 5.0*1.0/18.0,canvas.width()));
                            ctx.move_to(set_to_raster_x(i as f64 /3.0 +     1.0/18.0,canvas.width()), set_to_raster_y(j as f64 /3.0 + 5.0*1.0/18.0,canvas.width()));
                            ctx.line_to(set_to_raster_x(i as f64 /3.0 + 5.0*1.0/18.0,canvas.width()), set_to_raster_y(j as f64 /3.0 +     1.0/18.0,canvas.width()));
                            ctx.stroke();
                            let t = format!("{}px Arial", canvas.height()*30/500);
                            ctx.set_font(&t);
                            ctx.stroke_text(&format!("{}", cell%300), set_to_raster_x(i as f64 /3.0 + 1.0/6.0,canvas.width()), set_to_raster_y(j as f64 /3.0 + 1.0/6.0,canvas.width()), None);
                        },
                        cell if state_test(cell, BIG_O) => {
                            ctx.set_stroke_style_color("#AA71C6");
                            ctx.begin_path();
                            ctx.arc(set_to_raster_x(i as f64 /3.0 + 1.0/6.0,canvas.width()), set_to_raster_y(j as f64 /3.0 + 1.0/6.0,canvas.width()), set_to_raster_x(1.0/6.0 * 3.0/4.0,canvas.width()), 0.0, 2.0 * 3.141592653589, true);
                            ctx.stroke();
                            
                            // let value = value;
                            let t = format!("{}px Arial", canvas.height()*30/500);
                            ctx.set_font(&t);
                            // // js! {
                            // //     console.log("REE", @{cell})
                            // // }
                            ctx.stroke_text(&format!("{}", cell%400), set_to_raster_x(i as f64 /3.0 + 1.0/6.0,canvas.width()), set_to_raster_y(j as f64 /3.0 + 1.0/6.0,canvas.width()), None);
                        },
                        //? Uses the sketchy round-off for integer division to avoid having to take floor and subtract (this way is shorter)
                        cell if state_test(cell, SMALL_O)||state_test(cell, SMALL_X)||state_test(cell, UNCERTAIN_O)||state_test(cell, UNCERTAIN_X) => {
                            let mut k = 0;
                            while (k<9) && (state_test(self.state[i][j][k].value, SMALL_O) || state_test(self.state[i][j][k].value, SMALL_X) || state_test(self.state[i][j][k].value, UNCERTAIN_O) || state_test(self.state[i][j][k].value, UNCERTAIN_X)) {
                                
                                if state_test(self.state[i][j][k].value, SMALL_O) || state_test(self.state[i][j][k].value, UNCERTAIN_O) {
                                    //draw a circle centered at i*500/3+k%3*500/9+500/18
                                    ctx.set_stroke_style_color("#AA71C6");
                                    if state_test(self.state[i][j][k].value, UNCERTAIN_O) {
                                        ctx.set_stroke_style_color("#634771");
                                    }
                                    ctx.begin_path();
                                    ctx.arc(set_to_raster_x(i as f64 /3.0 + (k%3) as f64 /9.0 + 1.0/18.0,canvas.width()), set_to_raster_y(j as f64 /3.0 + (k/3) as f64 /9.0 + 1.0/18.0,canvas.width()), set_to_raster_x(1.0/18.0 * 3.0/4.0,canvas.width()), 0.0, 2.0 * 3.141592653589, true);
                                    ctx.stroke();
                                    
                                    
                                }
                                else if state_test(self.state[i][j][k].value, SMALL_X) || state_test(self.state[i][j][k].value, UNCERTAIN_X) {
                                    if state_test(self.state[i][j][k].value, UNCERTAIN_X) {
                                    // // js! {
                                    // //     console.log("HERE: UNCERTAIN_X",@{self.state[i][j][k].clone()})
                                    // // }

                                    }
                                    ctx.set_stroke_style_color("#3B99FC");
                                    if state_test(self.state[i][j][k].value, UNCERTAIN_X) {
                                        ctx.set_stroke_style_color("#475b71");
                                    }
                                    ctx.begin_path();
                                    ctx.move_to(set_to_raster_x(i as f64 /3.0 + (k%3) as f64 * 1.0/9.0 +     1.0/54.0,canvas.width()), set_to_raster_y(j as f64 /3.0 + (k/3) as f64 /9.0 +     1.0/54.0,canvas.width()));
                                    ctx.line_to(set_to_raster_x(i as f64 /3.0 + (k%3) as f64 * 1.0/9.0 + 5.0*1.0/54.0,canvas.width()), set_to_raster_y(j as f64 /3.0 + (k/3) as f64 /9.0 + 5.0*1.0/54.0,canvas.width()));
                                    ctx.move_to(set_to_raster_x(i as f64 /3.0 + (k%3) as f64 * 1.0/9.0 +     1.0/54.0,canvas.width()), set_to_raster_y(j as f64 /3.0 + (k/3) as f64 /9.0 + 5.0*1.0/54.0,canvas.width()));
                                    ctx.line_to(set_to_raster_x(i as f64 /3.0 + (k%3) as f64 * 1.0/9.0 + 5.0*1.0/54.0,canvas.width()), set_to_raster_y(j as f64 /3.0 + (k/3) as f64 /9.0 +     1.0/54.0,canvas.width()));
                                    ctx.stroke();
                                }
                                let t = format!("{}px Arial", canvas.height()*30/500);
                                ctx.set_font(&t);
                                ctx.set_text_align(TextAlign::Center);
                                ctx.set_text_baseline(TextBaseline::Middle);
                                // let mut value: String = (self.state[i][j][k]%100).to_string();
                                let mut value: String = "".to_string();// = value;
                                if state_test(self.state[i][j][k].value, SMALL_O) || state_test(self.state[i][j][k].value, SMALL_X) {
                                    value = (self.state[i][j][k].value%100).to_string();
                                }
                                else if state_test(self.state[i][j][k].value, UNCERTAIN_O) || state_test(self.state[i][j][k].value, UNCERTAIN_X){
                                    value = "".to_string();
                                }
                                // let value = value;
                                ctx.stroke_text(&value, set_to_raster_x(i as f64 /3.0 + (k%3) as f64 /9.0 + 1.0/18.0,canvas.width()), set_to_raster_y(j as f64 /3.0 + (k/3) as f64 /9.0 + 1.0/18.0,canvas.width()), None);
                                k+=1;
                            }
                        } 
                        _ => ()
                    }
                }
            }
            let t = format!("{}px Arial", canvas.height()*30/500);
            ctx.set_font(&t);
            // ctx.set_text_align(TextAlign::Center);
            // ctx.set_text_baseline(TextBaseline::Middle);
            ctx.set_stroke_style_color("#3B99FC"); //? X
            ctx.stroke_text(&format!("X: {}", self.x_score), set_to_raster_x(0.25,canvas.width()), set_to_raster_y(-0.05,canvas.width()), None);
            ctx.set_stroke_style_color("#AA71C6"); //? O
            ctx.stroke_text(&format!("O: {}", self.o_score), set_to_raster_x(0.75,canvas.width()), set_to_raster_y(-0.05,canvas.width()), None);
            let t = format!("{}px Arial", ((canvas.height()*30/500) as f64/ 1.1) as u32);
            ctx.set_font(&t);
            ctx.set_stroke_style_color("#606060");
            ctx.stroke_text(&self.recent_msg, set_to_raster_x(0.5,canvas.width()), set_to_raster_y(-0.05,canvas.width()), None);
        }
    }
}
