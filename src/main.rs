#[macro_use]
extern crate stdweb;
// // extern crate web_sys;
// // use std::panic;
// // extern crate console_error_panic_hook;

mod board;
// // use stdweb::JsSerialize;
pub use crate::board::board::*;
mod graph;
pub use crate::graph::graph::*;
mod commons;
pub use crate::commons::commons::*;
// use stdweb::print_error_panic;

// // pub fn set_panic_hook() {
    // // std::panic::set_hook(Box::new(|info| {
        // // print_error_panic(&info.to_string());
    // // }));
// // }

// // #[inline]
// // pub fn print_error_panic< A: JsSerialize >( value: A ) -> ! {
    // // js! { @(no_return)
        // // console.error( @{value} );
    // // }
    // // panic!();
// // }

fn main() {
    // // useless change
    // // set_panic_hook();
    // // console_error_panic_hook::set_once();
    // // stdweb::print_error_panic(&info.to_string())
    // // std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    stdweb::initialize();
    init_canvas();
    let game_board: Arc<Mutex<GameStruct>> = Arc::new(Mutex::new(Default::default()));
    {
        let game_board_mut = Arc::clone(&game_board);
        let mut mut_game_board = game_board_mut.lock().unwrap();
        
        mut_game_board.turn = 1;
        mut_game_board.previous_select.loc.x = 4;
        mut_game_board.previous_select.loc.y = 4;
        
        // ! DEBUGGING 
        // // mut_game_board.previous_select.loc.x = 0;
        // // mut_game_board.previous_select.loc.y = 0;
        // // mut_game_board.previous_select.loc.m = 0;
        // // mut_game_board.state[0][0][0].value = UNCERTAIN_O;
        // // mut_game_board.waiting_for_second_click = true;
        // ! END DEBUGGING
        // mut_game_board.state[0][0][3] = SMALL_X;
        // mut_game_board.state[0][0][4] = SMALL_X;
        // mut_game_board.state[0][0][5] = SMALL_X;
        // mut_game_board.state[0][0][1] = SMALL_O;
        // mut_game_board.state[0][0][6] = SMALL_X;
        // mut_game_board.state[0][0][7] = SMALL_X;
        // mut_game_board.state[0][0][8] = SMALL_O;
        // mut_game_board.state[0][1][0] = SMALL_O;
        // mut_game_board.state[0][1][1] = SMALL_BLANK;
        // mut_game_board.state[0][2][0] = BIG_O;
        mut_game_board.redraw();
    }


    let canvas: CanvasElement = web::document()
        .get_element_by_id("viewport")
        .unwrap()
        .try_into()
        .unwrap();

    let temph = canvas.height();
    let tempw = canvas.width();
    js! {
        console.log("Height:", @{temph});
        console.log("Width:", @{tempw});
    };

    let extra_game_board = Arc::clone(&game_board);
    canvas.add_event_listener(move |e: MouseDownEvent| {
        let game_board_1 = Arc::clone(&extra_game_board);
        let game_board_mut = game_board_1.lock().unwrap();
        click_handler(e.client_x() as f64,e.client_y() as f64,game_board_mut/*,true*/);
    });
    // canvas.add_event_listener(|e: TouchStart| {
    //     click_handler(e.changed_touches().iter().map(|t| t.client_x()).collect(), e.changed_touches().iter().map(|t| t.client_y()).collect(),true);
    // });
    // // let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();
}








