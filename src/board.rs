pub mod board {

    //useless change
    
use crate::commons::commons::*;
use crate::graph::graph::select_draw;

// !pub const BIG_BLANK: u8 = 0;
// !pub const SMALL_BLANK: u8 = 3;
// !pub const BIG_X: u8 = 4;
// !pub const BIG_O: u8 = 5;
// !pub const SMALL_X: u8 = 1;
// !pub const SMALL_O: u8 = 2;

pub fn state_change(x: u8, y:u8, game_board_original: MutexGuard<GameStruct>){
    let x_not_o;
    let game_board_rc = Rc::new(RefCell::new(game_board_original));
    // let game_board_arc = Arc::clone(&game_board_original);
    // let game_board = game_board_original.lock().unwrap();
	if (*game_board_rc).borrow_mut().turn%2 == 0 {
		x_not_o = false;
	}
	else {
		x_not_o = true;
    }

	
    for m in 0..9 {
        // let mut game_board = (*game_board_rc).borrow_mut();
        let temp_board_1 = (*game_board_rc).borrow();
        let temp_board = temp_board_1.clone();
        drop(temp_board_1);
        match temp_board.state[usize::from(x)][usize::from(y)][m].value {
            cell_state if state_test(cell_state, BIG_X) || state_test(cell_state, BIG_O) => {
                break;
            },
            cell_state if state_test(cell_state, SMALL_X) || state_test(cell_state, SMALL_O) => {
                continue;
            },
            cell_state if state_test(cell_state, SMALL_BLANK) || state_test(cell_state, BIG_BLANK)=> {
                if temp_board.waiting_for_second_click == true {
                    if (x == temp_board.previous_select.loc.x) & (y == temp_board.previous_select.loc.y) {
                        (*game_board_rc).borrow_mut().state[usize::from(x)][usize::from(y)][m-1].value = SMALL_BLANK;
                        (*game_board_rc).borrow_mut().waiting_for_second_click = false;
                        (*game_board_rc).borrow().redraw();
                        break;
                    }
                    (*game_board_rc).borrow_mut().turn += 1;
                }
                else {
                    (*game_board_rc).borrow_mut().previous_select.loc.x = x;
                    (*game_board_rc).borrow_mut().previous_select.loc.y = y;
                    (*game_board_rc).borrow_mut().previous_select.loc.m = m as u8;
                }
                let tempx = usize::from(temp_board.previous_select.loc.x);
                let tempy = usize::from(temp_board.previous_select.loc.y);
                let tempm = usize::from(temp_board.previous_select.loc.m);
                // let tempx = usize::from((*game_board_rc).borrow_mut().previous_select.loc.x);
                // let tempy = usize::from((*game_board_rc).borrow_mut().previous_select.loc.y);
                // let tempm = usize::from((*game_board_rc).borrow_mut().previous_select.loc.m);
                match x_not_o {
                    true => {
                        if temp_board.waiting_for_second_click {
                            // js! {
                            //     console.log("Was waiting for second click");
                            // }
                            (*game_board_rc).borrow_mut().state[tempx][tempy][tempm].value = SMALL_X+temp_board.turn;
                            (*game_board_rc).borrow_mut().state[usize::from(x)][usize::from(y)][m].value = SMALL_X+temp_board.turn;
                            (*game_board_rc).borrow_mut().state[usize::from(x)][usize::from(y)][m].link.x = tempx as u8;
                            (*game_board_rc).borrow_mut().state[tempx][tempy][tempm].link.x = x;
                            (*game_board_rc).borrow_mut().state[usize::from(x)][usize::from(y)][m].link.y = tempy as u8;
                            (*game_board_rc).borrow_mut().state[tempx][tempy][tempm].link.y = y;
                            (*game_board_rc).borrow_mut().state[usize::from(x)][usize::from(y)][m].link.m = tempm as u8;
                            (*game_board_rc).borrow_mut().state[tempx][tempy][tempm].link.m = m as u8;
                            (*game_board_rc).borrow_mut().recent_msg = "O's TURN".to_string();
                        }
                        else {
                            // // js! {
                                // //     console.log("HERE: UNCERTAIN_X");
                                // // }
                            (*game_board_rc).borrow_mut().state[usize::from(x)][usize::from(y)][m].value = UNCERTAIN_X;
                        }
                    },
                    false => {
                        // js! {
                        //     console.log("Was waiting for second click");
                        // }
                        if temp_board.waiting_for_second_click {
                            (*game_board_rc).borrow_mut().state[tempx][tempy][tempm].value = SMALL_O+temp_board.turn;
                            (*game_board_rc).borrow_mut().state[usize::from(x)][usize::from(y)][m].value = SMALL_O+temp_board.turn;
                            (*game_board_rc).borrow_mut().state[usize::from(x)][usize::from(y)][m].link.x = tempx as u8;
                            (*game_board_rc).borrow_mut().state[tempx][tempy][tempm].link.x = x;
                            (*game_board_rc).borrow_mut().state[usize::from(x)][usize::from(y)][m].link.y = tempy as u8;
                            (*game_board_rc).borrow_mut().state[tempx][tempy][tempm].link.y = y;
                            (*game_board_rc).borrow_mut().state[usize::from(x)][usize::from(y)][m].link.m = tempm as u8;
                            (*game_board_rc).borrow_mut().state[tempx][tempy][tempm].link.m = m as u8;
                            (*game_board_rc).borrow_mut().recent_msg = "X's TURN".to_string();
                        }
                        else {
                            (*game_board_rc).borrow_mut().state[usize::from(x)][usize::from(y)][m].value = UNCERTAIN_O;
                        }
                    }
                }
                (*game_board_rc).borrow_mut().waiting_for_second_click = !temp_board.waiting_for_second_click;
                // let (*game_board_rc).borrow_mut()_temp = (*game_board_rc).borrow_mut().clone();
                // // let temp_board_1 = (*game_board_rc).borrow();
                // // let temp_board = temp_board_1.clone();
                // // drop(temp_board_1);
                let temp_board_1 = (*game_board_rc).borrow();
                let temp_board_view = temp_board_1.clone();
                drop(temp_board_1);
                
                (*game_board_rc).borrow_mut().redraw();
                // // if temp_board_view.state[usize::from(x)][usize::from(y)][m].value >= 100 {
                // ? If something created a loop
                match recurse_travel(x, y, m as u8, x, y, Rc::clone(&game_board_rc), true) {
                    //? if the change created a loop
                    Some(vec) if vec.len() > 0 => {
                        //? then ask for a selection and set was_waiting
                        (*game_board_rc).borrow_mut().is_waiting_for_selection = true;
                        (*game_board_rc).borrow_mut().collapse_possib.x = x;
                        (*game_board_rc).borrow_mut().collapse_possib.y = y;
                        (*game_board_rc).borrow_mut().collapse_possib.m = m as u8;
                        js! {
                            console.log("M IS: ",@{(m as u8).clone()});
                        }
                        request_x_not_o(x, y, temp_board_view.state[usize::from(x)][usize::from(y)][m].link.x, temp_board_view.state[usize::from(x)][usize::from(y)][m].link.y);
                    },
                    //? if the change did not create a loop
                    None => (),
                    _ => ()
                }
                // // }
                js! {
                    console.log(@{(*game_board_rc).borrow_mut().state[0][0][0].value.clone()})
                }
                
                break;
            }
            _ => ()
        }
    }
	// // for i in 0..std::cmp::min(x.len(),2) {
	// // 	for m in 0..9 {
    // //         let mut game_board = (*game_board_rc).borrow_mut();
    // //         let temp_board_1 = (*game_board_rc).borrow();
    // //         let temp_board = temp_board_1.clone();
    // //         drop(temp_board_1);
	// // 		match temp_board.state[usize::from(x[i])][usize::from(y[i])][m].value {
	// // 			cell_state if (cell_state == BIG_X) || (cell_state == BIG_O) => {
	// // 				break;
	// // 			},
	// // 			cell_state if (cell_state == SMALL_X) || (cell_state == SMALL_O) => {
	// // 				continue;
	// // 			},
	// // 			cell_state if (cell_state == SMALL_BLANK) || (cell_state == BIG_BLANK)=> {
	// // 				if temp_board.waiting_for_second_click == true {
	// // 					if (x[i] == temp_board.previous_select.loc.x) & (y[i] == temp_board.previous_select.loc.y) {
    // //                         (*game_board_rc).borrow_mut().state[usize::from(x[i])][usize::from(y[i])][m-1].value = SMALL_BLANK;
    // //                         (*game_board_rc).borrow_mut().waiting_for_second_click = false;
    // //                         (*game_board_rc).borrow().redraw();
	// // 						break;
    // //                     }
	// // 					(*game_board_rc).borrow_mut().turn += 1;
	// // 				}
	// // 				else {
    // //                     (*game_board_rc).borrow_mut().previous_select.loc.x = x[i];
	// // 					(*game_board_rc).borrow_mut().previous_select.loc.y = y[i];
	// // 					(*game_board_rc).borrow_mut().previous_select.loc.m = m as u8;
    // //                 }
    // //                 let tempx = usize::from(temp_board.previous_select.loc.x);
    // //                 let tempy = usize::from(temp_board.previous_select.loc.y);
    // //                 let tempm = usize::from(temp_board.previous_select.loc.m);
    // //                 let tempx = usize::from((*game_board_rc).borrow_mut().previous_select.loc.x);
    // //                 let tempy = usize::from((*game_board_rc).borrow_mut().previous_select.loc.y);
    // //                 let tempm = usize::from((*game_board_rc).borrow_mut().previous_select.loc.m);
	// // 				match x_not_o {
    // //                     true => {
    // //                         if temp_board.waiting_for_second_click {
    // //                             js! {
    // //                                 console.log("Was waiting for second click");
    // //                             }
    // //                             (*game_board_rc).borrow_mut().state[tempx][tempy][tempm].value = SMALL_X+temp_board.turn;
    // //                             (*game_board_rc).borrow_mut().state[usize::from(x[i])][usize::from(y[i])][m].value = SMALL_X+temp_board.turn;
    // //                             (*game_board_rc).borrow_mut().state[usize::from(x[i])][usize::from(y[i])][m].link.x = tempx as u8;
    // //                             (*game_board_rc).borrow_mut().state[tempx][tempy][tempm].link.x = x[i];
    // //                             (*game_board_rc).borrow_mut().state[usize::from(x[i])][usize::from(y[i])][m].link.y = tempy as u8;
    // //                             (*game_board_rc).borrow_mut().state[tempx][tempy][tempm].link.y = y[i];
    // //                             (*game_board_rc).borrow_mut().state[usize::from(x[i])][usize::from(y[i])][m].link.m = tempm as u8;
    // //                             (*game_board_rc).borrow_mut().state[tempx][tempy][tempm].link.m = m as u8;
	// // 						}
	// // 						else {
    // //                             js! {
    // //                                     console.log("HERE: UNCERTAIN_X");
    // //                                 }
    // //                             (*game_board_rc).borrow_mut().state[usize::from(x[i])][usize::from(y[i])][m].value = UNCERTAIN_X;
    // //                         }
    // //                     },
    // //                     false => {
    // //                         js! {
    // //                             console.log("Was waiting for second click");
    // //                         }
    // //                         if temp_board.waiting_for_second_click {
    // //                             (*game_board_rc).borrow_mut().state[tempx][tempy][tempm].value = SMALL_O+temp_board.turn;
    // //                             (*game_board_rc).borrow_mut().state[usize::from(x[i])][usize::from(y[i])][m].value = SMALL_O+temp_board.turn;
    // //                             (*game_board_rc).borrow_mut().state[usize::from(x[i])][usize::from(y[i])][m].link.x = tempx as u8;
    // //                             (*game_board_rc).borrow_mut().state[tempx][tempy][tempm].link.x = x[i];
    // //                             (*game_board_rc).borrow_mut().state[usize::from(x[i])][usize::from(y[i])][m].link.y = tempy as u8;
    // //                             (*game_board_rc).borrow_mut().state[tempx][tempy][tempm].link.y = y[i];
    // //                             (*game_board_rc).borrow_mut().state[usize::from(x[i])][usize::from(y[i])][m].link.m = tempm as u8;
    // //                             (*game_board_rc).borrow_mut().state[tempx][tempy][tempm].link.m = m as u8;
    // //                         }
    // //                         else {
    // //                             (*game_board_rc).borrow_mut().state[usize::from(x[i])][usize::from(y[i])][m].value = UNCERTAIN_O;
    // //                         }
    // //                     }
	// // 				}
    // //                 (*game_board_rc).borrow_mut().waiting_for_second_click = !temp_board.waiting_for_second_click;
    // //                 let (*game_board_rc).borrow_mut()_temp = (*game_board_rc).borrow_mut().clone();
    // //                 let temp_board_1 = (*game_board_rc).borrow();
    // //                 let temp_board = temp_board_1.clone();
    // //                 drop(temp_board_1);
    // //                 let temp_board_1 = (*game_board_rc).borrow();
    // //                 let temp_board_view = temp_board_1.clone();
    // //                 drop(temp_board_1);
                    
    // //                 (*game_board_rc).borrow_mut().redraw();
    // //                 if temp_board_view.state[usize::from(x[i])][usize::from(y[i])][m].value >= 100 {
    // //                 ? If something created a loop
    // //                 match recurse_travel(x, y, m, x, y, temp_board_view, true) {
    // //                     ? if the change created a loop
    // //                     Some(vec) if vec.len() > 0 => {
    // //                         ? then ask for a selection and set was_waiting
    // //                         game_board.borrow_mut().is_waiting_for_selection = true;
    // //                         game_board.borrow_mut().collapse_possib.x = x;
    // //                         game_board.borrow_mut().collapse_possib.y = y;
    // //                         game_board.borrow_mut().collapse_possib.m = m;
    // //                         request_x_not_o(x,y);
    // //                     },
    // //                     ? if the change did not create a loop
    // //                     None => ()
    // //                 }
    // //                 }
    // //                 js! {
    // //                     console.log(@{(*game_board_rc).borrow_mut().state[0][0][0].value.clone()})
    // //                 }
                    
	// // 				break;
	// // 			}
	// // 			_ => ()
	// // 		}
	// // 	}
    // // }
}

fn raster_to_board_x(x: f64, width_or_height: u32) -> u8 {
    ((x/width_or_height as f64 *3.0)%3.0 )as u8
}
fn raster_to_board_y(y: f64, width_or_height: u32) -> u8 {
    ((((y/width_or_height as f64) - 0.1) *3.0)%3.0 )as u8
}

fn recurse_travel(x: u8, y: u8, m: u8, x_original: u8, y_original: u8, game_board: Rc<RefCell<MutexGuard<GameStruct>>>/*RefMut<MutexGuard<GameStruct>>*/ /*Arc<Mutex<GameStruct>>*/, mut ignore_first: bool) -> Option<Vec<ExtendedSelection>> {
    // let &mut game_board = (*game_board_original).borrow_mut();//.lock().unwrap();
    let game_board_rc = Rc::clone(&game_board);
    // let game_board = (*game_board).borrow_mut();
    let temp_board_1 = (*game_board_rc).borrow();
    let temp_board = temp_board_1.clone();
    drop(temp_board_1);
    js! {
        console.log("recurse: x ", @{x}, ", y ", @{y}, ", m ", @{m}, ", x_origin ", @{x_original}, ", y_origin ", @{y_original}, ", ignore_first ", @{ignore_first});
    }
    if ignore_first && !((x == x_original) && (y == y_original)) {
        // js! {
            // console.log("MOVED OUT");
        // }
        ignore_first = !ignore_first;
    }
    if (x == x_original) && (y == y_original) && !ignore_first {
        js! {
            console.log("Returning with ignore = ", @{ignore_first}, ", x and y and m = ", @{x}, @{y}, @{m});
        }
        return Some(Vec::new());    
    }
    let mut i = 0;
    while (&temp_board.state[usize::from(x)][usize::from(y)][i].value != &SMALL_BLANK) && (&temp_board.state[usize::from(x)][usize::from(y)][i].value != &BIG_BLANK) {
        // std::mem::drop(temp_board);

        // js! {
            // console.log("loop reached i=", @{i as u8});
        // }
        if i as u8 != m {
            match recurse_travel(temp_board.state[usize::from(x)][usize::from(y)][i].link.x.clone(), temp_board.state[usize::from(x)][usize::from(y)][i].link.y.clone(), temp_board.state[usize::from(x)][usize::from(y)][i].link.m.clone(), x_original, y_original, /*Arc::clone(&*/Rc::clone(&game_board_rc)/*)*/, ignore_first) {
                Some(mut vector) => {
                    // js! {
                    //     console.log("LINKING TO: x, ", @{temp_board.state[usize::from(x)][usize::from(y)][i].link.x.clone()}, ", y", @{temp_board.state[usize::from(x)][usize::from(y)][i].link.y.clone()})
                    // }    
                    vector.push(ExtendedSelection {
                        x,
                        y, 
                        m,
                        m_next: i as u8
                    });
                    js! {
                        console.log("M_NEXT I IS: ", @{(i as u8).clone()});
                    }
                    return Some(vector);
                } ,
                None => ()
            }
            
        }
        // let game_board = game_board_original.lock().unwrap();
        i+=1;
    }
    return None;
}

fn _vec_collapse(loop_vec: Vec<ExtendedSelection>, x_not_o: bool, game_board: Rc<RefCell<MutexGuard<GameStruct>>>/*RefMut<MutexGuard<GameStruct>>*//*: Arc<Mutex<GameStruct>>*/) {
    //? The 
    
    // let mut game_board = game_board_original;//.lock().unwrap();

    // let &mut game_board = (*game_board_original).borrow_mut();//.lock().unwrap();
    let mut game_board = (*game_board).borrow_mut();
    // // loop_vec.reverse();
    let mut x_or_o = x_not_o;
    for cell in loop_vec {
        js! {
            console.log("OPERATING ON CELL ", @{cell.x}, @{cell.y});
        }
        for i in 0..9 {
            if (i != cell.m) && (i != cell.m_next) {
                let linkedx = game_board.state[usize::from(cell.x)][usize::from(cell.y)][usize::from(i)].link.x;
                let linkedy = game_board.state[usize::from(cell.x)][usize::from(cell.y)][usize::from(i)].link.y;
                let value   = game_board.state[usize::from(cell.x)][usize::from(cell.y)][usize::from(i)].value;
                match value {
                    subcellstate if SMALL_X<=subcellstate && subcellstate<SMALL_O  => {
                        js! {
                            console.log("Collapsing ", @{linkedx}, @{linkedy}, @{game_board.state[usize::from(cell.x)][usize::from(cell.y)][usize::from(i)].link.m}, "into an X");
                        }
                        game_board.state[usize::from(linkedx)][usize::from(linkedy)][0].value = BIG_X;
                    },
                    subcellstate if SMALL_O<=subcellstate => {
                        js! {
                            console.log("Collapsing ", @{linkedx}, @{linkedy}, @{game_board.state[usize::from(cell.x)][usize::from(cell.y)][usize::from(i)].link.m}, "into an O");
                        }
                        game_board.state[usize::from(linkedx)][usize::from(linkedy)][0].value = BIG_O;
                    },
                    _ => ()
                } 
            }
            else if i == cell.m {
                if x_or_o {
                    js! {
                        console.log("iCollapsing ", @{cell.x.clone()}, @{cell.y.clone()}, @{game_board.state[usize::from(cell.x)][usize::from(cell.y)][usize::from(i)].loc.m}, "into an X");
                    }
                    game_board.state[usize::from(cell.x)][usize::from(cell.y)][0].value = BIG_X;
                }
                else {
                    js! {
                        console.log("iCollapsing ", @{cell.x.clone()}, @{cell.y.clone()}, @{game_board.state[usize::from(cell.x)][usize::from(cell.y)][usize::from(i)].loc.m}, "into an O");
                    }
                    game_board.state[usize::from(cell.x)][usize::from(cell.y)][0].value = BIG_O;
                }
            }
        }
        // if x_or_o {
        //     game_board.state[usize::from(cell.x)][usize::from(cell.y)][usize::from(cell.m)].value = BIG_X;
        // }
        // else {
        //     game_board.state[usize::from(cell.x)][usize::from(cell.y)][usize::from(cell.m)].value = BIG_O;
        // }
        x_or_o = !x_or_o;
    }
}

fn request_x_not_o(x1: u8, y1: u8, x2: u8, y2: u8) -> bool {
    //? HIGHLIGHT THE TWO CELLS THAT CHANGED
    // let canvas: CanvasElement = web::document()
    //     .get_element_by_id("viewport")
    //     .unwrap()
    //     .try_into()
    //     .unwrap();
    // let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();
    // ctx.fill_rect(set_to_raster(0,canvas.height(), set_to_raster(0,canvas.height()),set_to_raster(.3333,canvas.height()), set_to_raster(.3333,canvas.height()));
    select_draw(x1, y1, x2, y2);
    return true;
}

fn handle_request_x_not_o(x: u8, y: u8, game_board: Rc<RefCell<MutexGuard<GameStruct>>>) {

    let game_board_view = game_board.borrow();
    //? if the selection matches with the possible choices
    if (x == game_board_view.collapse_possib.x) && (y == game_board_view.collapse_possib.y) {
        let x_not_o = game_board_view.state[usize::from(game_board_view.collapse_possib.x)][usize::from(game_board_view.collapse_possib.y)][usize::from(game_board_view.collapse_possib.m)].value;
        // if x_not_o >= UNCERTAIN_O {
        //     let x_not_o = false;
        // }
        // else if x_not_o >= UNCERTAIN_X {
        //     let x_not_o = true;
        // }
        let x_not_o = (x_not_o >= UNCERTAIN_X) && (x_not_o < UNCERTAIN_O);
        let m = game_board_view.collapse_possib.m;
        drop(game_board_view);
        game_board.borrow_mut().is_waiting_for_selection = false;
        //? TIME TO COLLAPSE
        collapse(x, y, m, Rc::clone(&game_board), x_not_o);
        win_check(Rc::clone(&game_board));
    }
    else if (x == game_board_view.state[usize::from(game_board_view.collapse_possib.x)][usize::from(game_board_view.collapse_possib.y)][usize::from(game_board_view.collapse_possib.m)].link.x) && (y == game_board_view.state[usize::from(game_board_view.collapse_possib.x)][usize::from(game_board_view.collapse_possib.y)][usize::from(game_board_view.collapse_possib.m)].link.y) {
        let m = game_board_view.state[usize::from(game_board_view.collapse_possib.x)][usize::from(game_board_view.collapse_possib.y)][usize::from(game_board_view.collapse_possib.m)].link.m;
        let x_not_o = game_board_view.state[usize::from(game_board_view.state[usize::from(game_board_view.collapse_possib.x)][usize::from(game_board_view.collapse_possib.y)][usize::from(game_board_view.collapse_possib.m)].link.x)][usize::from(game_board_view.state[usize::from(game_board_view.collapse_possib.x)][usize::from(game_board_view.collapse_possib.y)][usize::from(game_board_view.collapse_possib.m)].link.y)][usize::from(m)].value;
        let x_not_o = (x_not_o >= UNCERTAIN_X) && (x_not_o < UNCERTAIN_O);
        drop(game_board_view);
        game_board.borrow_mut().is_waiting_for_selection = false;
        //? TIME TO COLLAPSE
        collapse(x, y, m, Rc::clone(&game_board), x_not_o);
        win_check(Rc::clone(&game_board));
    }


}

fn collapse(x: u8, y: u8, m: u8, game_board: Rc<RefCell<MutexGuard<GameStruct>>>, _x_not_o: bool) {
    // // ? if was waiting for input, then read the flag
    // // ? if was not waiting for input / !was_collapsing, then draw the request and set was_collapsing
    
    // // let game_board_rc = Rc::clone(&game_board);
    
    // // let mut loop_vec = match recurse_travel(x, y, m, x, y, Rc::clone(&game_board_rc), true) {
    // //     Some(vector) => vector,
    // //     None => Vec::new() //? This will happen if there is no cycle
    // // };
    // // loop_vec.reverse();
    // // let tempx: Vec<u8> = loop_vec.iter().map(|s| s.x).collect();
    // // let tempy: Vec<u8> = loop_vec.iter().map(|s| s.y).collect();
    // // let tempm: Vec<u8> = loop_vec.iter().map(|s| s.m).collect();
    // // let tempmm: Vec<u8> = loop_vec.iter().map(|s| s.m_next).collect();
    // // js! {
    // //     console.log("CYCLE X: ",@{tempx});
    // //     console.log("CYCLE Y: ",@{tempy});
    // //     console.log("CYCLE M: ",@{tempm});
    // //     console.log("CYCLE MNEXT: ",@{tempmm});
    // // }
    // // let x_not_o;
    // // if loop_vec.len() > 0 {
    // //     x_not_o = request_x_not_o(loop_vec[0].x, loop_vec[0].y, Rc::clone(&game_board_rc));
    // //     if (game_board.was_collapsing) {
    // //         let mut game_board = (*game_board).borrow_mut();
            
    // //         vec_collapse(loop_vec, handle_request_x_not_o(loop_vec[0].x, loop_vec[0].y, game_board_rc), game_board_rc);
    // //     }
    // //     else {
    // //         (*game_board).borrow_mut().was_collapsing = true;
    // //         request_x_not_o(loop_vec[0].x, loop_vec[0].y, game_board_rc);
    // //     }
    // // }

    //? NEW/OLD STUFF just straight up collapse
    let mut game_board = game_board.borrow_mut();
    let mut collapsible_links = Vec::new();
    // // let mut not_cycled = true;
    //? creates a vector of links to follow from the first base cell
    for i in 0..9 {
        if state_test(game_board.state[usize::from(x)][usize::from(y)][i].value, SMALL_BLANK) {
            break;
        }
        if i as u8 != m {
            collapsible_links.push(game_board.state[usize::from(x)][usize::from(y)][i].link);
        }
    }
    let mut x_or_o = BIG_X;
    if state_test(game_board.state[usize::from(x)][usize::from(y)][usize::from(m)].value, SMALL_O) {
        x_or_o = BIG_O + (game_board.state[usize::from(x)][usize::from(y)][usize::from(m)].value)%200;
    }
    if state_test(game_board.state[usize::from(x)][usize::from(y)][usize::from(m)].value, SMALL_X) {
        x_or_o = BIG_X + (game_board.state[usize::from(x)][usize::from(y)][usize::from(m)].value)%100;
    }
    // if state_test(game_board.state[usize::from(x)][usize::from(y)][usize::from(m)].value, SMALL_O) {
    //     x_or_o = BIG_O;
    // }
    js! {
        console.log("Collapsing ", @{x}, @{y}, "with value ", @{x_or_o});
    }
    game_board.state[usize::from(x)][usize::from(y)][0].value = x_or_o;
    //? for each linked cell, push links from those cells then collapse the cell
    //? if it reaches the base cell, it stops
    loop {
        let checkable_links_temp = collapsible_links.clone();//drain(..).cloned();
        
        if checkable_links_temp.len() == 0 {
            break;
        }
        collapsible_links = Vec::new();
        for link in checkable_links_temp {
            if (link.x == x) && (link.y == y) {
                // // not_cycled = false;
                continue;
            }
            for i in 0..9 {
                if state_test(game_board.state[usize::from(link.x)][usize::from(link.y)][i].value, SMALL_BLANK) {
                    break;
                }
                if i as u8 != link.m {
                    collapsible_links.push(game_board.state[usize::from(link.x)][usize::from(link.y)][i].link);
                }
            }
            let mut x_or_o = 0;// = BIG_X;
            if state_test(game_board.state[usize::from(link.x)][usize::from(link.y)][usize::from(link.m)].value, SMALL_O) {
                x_or_o = BIG_O + (game_board.state[usize::from(link.x)][usize::from(link.y)][usize::from(link.m)].value)%200;
            }
            if state_test(game_board.state[usize::from(link.x)][usize::from(link.y)][usize::from(link.m)].value, SMALL_X) {
                x_or_o = BIG_X + (game_board.state[usize::from(link.x)][usize::from(link.y)][usize::from(link.m)].value)%100;
            }
            //? SETTING THE VALUE
            js! {
                console.log("Collapsing ", @{link.x}, @{link.y}, "with value ", @{x_or_o});
            }
            game_board.state[usize::from(link.x)][usize::from(link.y)][0].value = x_or_o;
            game_board.state[usize::from(link.x)][usize::from(link.y)][1].value = SMALL_BLANK;
        }
    }
    game_board.redraw();


}

fn win_check(game_board: Rc<RefCell<MutexGuard<GameStruct>>>) {
    let game_board_rc = Rc::clone(&game_board);
    let temp_board_1 = (*game_board_rc).borrow();
    let game_board_view = temp_board_1.clone();
    drop(temp_board_1);

    let mut x_won = false; let mut o_won = false; let mut draw = false;

    //? In a draw logic, can be overidden
    let mut counter = 0;
    for i in 0..3 {
        for j in 0..3 {
            if !state_test(game_board_view.state[i][j][0].value, BIG_O) && !state_test(game_board_view.state[i][j][0].value, BIG_X) {
                counter+=1;
            }
        }
    }
    if counter == 1 {
        draw = true;
    }

    if 
        state_test(game_board_view.state[0][0][0].value,BIG_X) && state_test(game_board_view.state[0][1][0].value,BIG_X) && state_test(game_board_view.state[0][2][0].value,BIG_X) ||
        state_test(game_board_view.state[1][0][0].value,BIG_X) && state_test(game_board_view.state[1][1][0].value,BIG_X) && state_test(game_board_view.state[1][2][0].value,BIG_X) ||
        state_test(game_board_view.state[2][0][0].value,BIG_X) && state_test(game_board_view.state[2][1][0].value,BIG_X) && state_test(game_board_view.state[2][2][0].value,BIG_X)
            ||
        state_test(game_board_view.state[0][0][0].value,BIG_X) && state_test(game_board_view.state[1][0][0].value,BIG_X) && state_test(game_board_view.state[2][0][0].value,BIG_X) ||
        state_test(game_board_view.state[0][1][0].value,BIG_X) && state_test(game_board_view.state[1][1][0].value,BIG_X) && state_test(game_board_view.state[2][1][0].value,BIG_X) ||
        state_test(game_board_view.state[0][2][0].value,BIG_X) && state_test(game_board_view.state[1][2][0].value,BIG_X) && state_test(game_board_view.state[2][2][0].value,BIG_X)
            ||
        state_test(game_board_view.state[0][0][0].value,BIG_X) && state_test(game_board_view.state[1][1][0].value,BIG_X) && state_test(game_board_view.state[2][2][0].value,BIG_X) ||
        state_test(game_board_view.state[2][0][0].value,BIG_X) && state_test(game_board_view.state[1][1][0].value,BIG_X) && state_test(game_board_view.state[0][2][0].value,BIG_X)
    {
        js! {
            console.log("WOW X WINS");
        }
        x_won = true;
    }
    
    if 
        state_test(game_board_view.state[0][0][0].value,BIG_O) && state_test(game_board_view.state[0][1][0].value,BIG_O) && state_test(game_board_view.state[0][2][0].value,BIG_O) ||
        state_test(game_board_view.state[1][0][0].value,BIG_O) && state_test(game_board_view.state[1][1][0].value,BIG_O) && state_test(game_board_view.state[1][2][0].value,BIG_O) ||
        state_test(game_board_view.state[2][0][0].value,BIG_O) && state_test(game_board_view.state[2][1][0].value,BIG_O) && state_test(game_board_view.state[2][2][0].value,BIG_O)
            ||
        state_test(game_board_view.state[0][0][0].value,BIG_O) && state_test(game_board_view.state[1][0][0].value,BIG_O) && state_test(game_board_view.state[2][0][0].value,BIG_O) ||
        state_test(game_board_view.state[0][1][0].value,BIG_O) && state_test(game_board_view.state[1][1][0].value,BIG_O) && state_test(game_board_view.state[2][1][0].value,BIG_O) ||
        state_test(game_board_view.state[0][2][0].value,BIG_O) && state_test(game_board_view.state[1][2][0].value,BIG_O) && state_test(game_board_view.state[2][2][0].value,BIG_O)
            ||
        state_test(game_board_view.state[0][0][0].value,BIG_O) && state_test(game_board_view.state[1][1][0].value,BIG_O) && state_test(game_board_view.state[2][2][0].value,BIG_O) ||
        state_test(game_board_view.state[2][0][0].value,BIG_O) && state_test(game_board_view.state[1][1][0].value,BIG_O) && state_test(game_board_view.state[0][2][0].value,BIG_O)
    {
        js! {
            console.log("WOW O WINS");
        }
        o_won = true;
        // game_board.borrow();

    }

    if x_won && o_won {
        // game_board.borrow_mut().recent_msg = "O Won".to_string();
        game_board.borrow_mut().recent_msg = "TIED".to_string();
        // ***ALWAYS FOLD THIS; IT IS ANNOYING
        if tie_calc(Rc::clone(&game_board_rc)) {
            game_board.borrow_mut().x_score += 1.0;
            game_board.borrow_mut().o_score += 0.5;
        }
        else {
            game_board.borrow_mut().x_score += 0.5;
            game_board.borrow_mut().o_score += 1.0;
        }
        game_board.borrow_mut().new_game = true;
    }
    else if x_won {
        game_board.borrow_mut().x_score += 1.0;
        game_board.borrow_mut().recent_msg = "X Won".to_string();
        game_board.borrow_mut().new_game = true;
    }
    else if o_won {
        game_board.borrow_mut().o_score += 1.0;
        game_board.borrow_mut().recent_msg = "O Won".to_string();
        game_board.borrow_mut().new_game = true;
    }
    else if draw {
        game_board.borrow_mut().recent_msg = "DRAW".to_string();
        game_board.borrow_mut().new_game = true;
    }

    game_board.borrow().redraw();

}

fn tie_calc(game_board: Rc<RefCell<MutexGuard<GameStruct>>>) -> bool{
    let game_board_rc = Rc::clone(&game_board);
    let temp_board_1 = (*game_board_rc).borrow();
    let game_board_view = temp_board_1.clone();
    drop(temp_board_1);
    let mut x_max = 0; let mut o_max = 0;
    if state_test(game_board_view.state[0][0][0].value,BIG_X) && state_test(game_board_view.state[0][1][0].value,BIG_X) && state_test(game_board_view.state[0][2][0].value,BIG_X) {
        x_max = std::cmp::max(std::cmp::max(game_board_view.state[0][0][0].value, game_board_view.state[0][1][0].value),game_board_view.state[0][2][0].value);
    }
    else if state_test(game_board_view.state[1][0][0].value, BIG_X) && state_test(game_board_view.state[1][1][0].value, BIG_X) && state_test(game_board_view.state[1][2][0].value, BIG_X) {
        x_max = std::cmp::max(std::cmp::max(game_board_view.state[1][0][0].value, game_board_view.state[1][1][0].value), game_board_view.state[1][2][0].value);
    }
    else if state_test(game_board_view.state[2][0][0].value, BIG_X) && state_test(game_board_view.state[2][1][0].value, BIG_X) && state_test(game_board_view.state[2][2][0].value, BIG_X) {
        x_max = std::cmp::max(std::cmp::max(game_board_view.state[2][0][0].value, game_board_view.state[2][1][0].value), game_board_view.state[2][2][0].value);
    }
    else if state_test(game_board_view.state[0][0][0].value, BIG_X) && state_test(game_board_view.state[1][0][0].value, BIG_X) && state_test(game_board_view.state[2][0][0].value, BIG_X) {
        x_max = std::cmp::max(std::cmp::max(game_board_view.state[0][0][0].value, game_board_view.state[1][0][0].value), game_board_view.state[2][0][0].value);
    }
    else if state_test(game_board_view.state[0][1][0].value, BIG_X) && state_test(game_board_view.state[1][1][0].value, BIG_X) && state_test(game_board_view.state[2][1][0].value, BIG_X) {
        x_max = std::cmp::max(std::cmp::max(game_board_view.state[0][1][0].value, game_board_view.state[1][1][0].value), game_board_view.state[2][1][0].value);
    }
    else if state_test(game_board_view.state[0][2][0].value, BIG_X) && state_test(game_board_view.state[1][2][0].value, BIG_X) && state_test(game_board_view.state[2][2][0].value, BIG_X) {
        x_max = std::cmp::max(std::cmp::max(game_board_view.state[0][2][0].value, game_board_view.state[1][2][0].value), game_board_view.state[2][2][0].value);
    }
    else if state_test(game_board_view.state[0][0][0].value, BIG_X) && state_test(game_board_view.state[1][1][0].value, BIG_X) && state_test(game_board_view.state[2][2][0].value, BIG_X) {
        x_max = std::cmp::max(std::cmp::max(game_board_view.state[0][0][0].value, game_board_view.state[1][1][0].value), game_board_view.state[2][2][0].value);
    }
    else if state_test(game_board_view.state[2][0][0].value, BIG_X) && state_test(game_board_view.state[1][1][0].value, BIG_X) && state_test(game_board_view.state[0][2][0].value, BIG_X) {
        x_max = std::cmp::max(std::cmp::max(game_board_view.state[2][0][0].value, game_board_view.state[1][1][0].value), game_board_view.state[0][2][0].value);
    }
    if state_test(game_board_view.state[0][0][0].value,BIG_O) && state_test(game_board_view.state[0][1][0].value,BIG_O) && state_test(game_board_view.state[0][2][0].value,BIG_O) {
        o_max = std::cmp::max(std::cmp::max(game_board_view.state[0][0][0].value, game_board_view.state[0][1][0].value),game_board_view.state[0][2][0].value);
    }
    else if state_test(game_board_view.state[1][0][0].value, BIG_O) && state_test(game_board_view.state[1][1][0].value, BIG_O) && state_test(game_board_view.state[1][2][0].value, BIG_O) {
        o_max = std::cmp::max(std::cmp::max(game_board_view.state[1][0][0].value, game_board_view.state[1][1][0].value), game_board_view.state[1][2][0].value);
    }
    else if state_test(game_board_view.state[2][0][0].value, BIG_O) && state_test(game_board_view.state[2][1][0].value, BIG_O) && state_test(game_board_view.state[2][2][0].value, BIG_O) {
        o_max = std::cmp::max(std::cmp::max(game_board_view.state[2][0][0].value, game_board_view.state[2][1][0].value), game_board_view.state[2][2][0].value);
    }
    else if state_test(game_board_view.state[0][0][0].value, BIG_O) && state_test(game_board_view.state[1][0][0].value, BIG_O) && state_test(game_board_view.state[2][0][0].value, BIG_O) {
        o_max = std::cmp::max(std::cmp::max(game_board_view.state[0][0][0].value, game_board_view.state[1][0][0].value), game_board_view.state[2][0][0].value);
    }
    else if state_test(game_board_view.state[0][1][0].value, BIG_O) && state_test(game_board_view.state[1][1][0].value, BIG_O) && state_test(game_board_view.state[2][1][0].value, BIG_O) {
        o_max = std::cmp::max(std::cmp::max(game_board_view.state[0][1][0].value, game_board_view.state[1][1][0].value), game_board_view.state[2][1][0].value);
    }
    else if state_test(game_board_view.state[0][2][0].value, BIG_O) && state_test(game_board_view.state[1][2][0].value, BIG_O) && state_test(game_board_view.state[2][2][0].value, BIG_O) {
        o_max = std::cmp::max(std::cmp::max(game_board_view.state[0][2][0].value, game_board_view.state[1][2][0].value), game_board_view.state[2][2][0].value);
    }
    else if state_test(game_board_view.state[0][0][0].value, BIG_O) && state_test(game_board_view.state[1][1][0].value, BIG_O) && state_test(game_board_view.state[2][2][0].value, BIG_O) {
        o_max = std::cmp::max(std::cmp::max(game_board_view.state[0][0][0].value, game_board_view.state[1][1][0].value), game_board_view.state[2][2][0].value);
    }
    else if state_test(game_board_view.state[2][0][0].value, BIG_O) && state_test(game_board_view.state[1][1][0].value, BIG_O) && state_test(game_board_view.state[0][2][0].value, BIG_O) {
        o_max = std::cmp::max(std::cmp::max(game_board_view.state[2][0][0].value, game_board_view.state[1][1][0].value), game_board_view.state[0][2][0].value);
    }
    if (x_max-BIG_X) < (o_max-BIG_O) {
        return true;
    }
    else {
        return false;
    }
}

pub(crate) fn click_handler(mut x: f64, mut y: f64, game_board: MutexGuard<GameStruct>/*, x_not_o: bool*/) {
    //? if was_collapsing, then the click should set the collapse_result flag
    //? if was not collapsing, then continue with normal events

    let canvas: CanvasElement = web::document()
        .get_element_by_id("viewport")
        .unwrap()
        .try_into()
        .unwrap();
    let x_offset = canvas.get_bounding_client_rect().get_left() + window().page_x_offset() as f64;
    let y_offset = canvas.get_bounding_client_rect().get_top()  + window().page_y_offset() as f64;
    x = x-x_offset;
    y = y-y_offset;
    let x = raster_to_board_x(x, canvas.width());
    let y = raster_to_board_y(y, canvas.width());
    // // js! {
    // // 	console.log("xoff: ",@{x_offset.clone()});
    // // 	console.log("yoff: ",@{y_offset.clone()});
    // // }


    //? if was waiting for selection, 
    if game_board.is_waiting_for_selection {
        //? process selection
        let game_board_rc = Rc::new(RefCell::new(game_board));
        handle_request_x_not_o(x, y, game_board_rc);
    }
    else if game_board.new_game {
        // let temp_game: GameStruct;
        // let game_board: MutexGuard<GameStruct> = Default::default();
        // game_board = temp_game.clone();
        //? pub state: [[[SubCell; 9]; 3]; 3],  //? the state of the board
        //? pub waiting_for_second_click: bool, //? is there already a matching uncertain
        //? pub previous_select: SubCell,       //? the previous selection (an uncertain)
        //? pub turn: u16,				        //? the current turn number
        //? pub is_waiting_for_selection: bool, //? if the click is supposed to be processed as a collapsing selection
        //? // pub has_a_selection: bool,	    //? if there is a selection to be processed
        //? // pub collapse_result: Selection,  //? the user-selected collapsing
        //? pub collapse_possib: Selection,     //? stores the selection that closes the loop,
        //? pub x_score: f32, 				    //? the current score for X
        //? pub o_score: f32, 				    //? the current score for O
        //? pub recent_msg: String,             //? the displaying recent msg
        //? pub new_game: bool,  
        // [[commons::commons::SubCell; 9]; 3]
        // game_board.state.iter_mut().for_each(|x| *x = {0);
            // game_board.state
            // drop(temp_game);let game_board: Arc<Mutex<GameStruct>> = Arc::new(Mutex::new(Default::default()));
        let game_board_rc = Rc::new(RefCell::new(game_board));
        let temp_board: Arc<Mutex<GameStruct>> = Arc::new(Mutex::new(Default::default()));
        let temp_board = temp_board.lock().unwrap().clone();
        (*Rc::clone(&game_board_rc)).borrow_mut().state = temp_board.state;
        (*Rc::clone(&game_board_rc)).borrow_mut().waiting_for_second_click = temp_board.waiting_for_second_click;
        (*Rc::clone(&game_board_rc)).borrow_mut().previous_select = temp_board.previous_select;
        // (*Rc::clone(&game_board_rc)).borrow_mut().turn = temp_board.turn;
        (*Rc::clone(&game_board_rc)).borrow_mut().is_waiting_for_selection = temp_board.is_waiting_for_selection;
        (*Rc::clone(&game_board_rc)).borrow_mut().collapse_possib = temp_board.collapse_possib;
        // (*Rc::clone(&game_board_rc)).borrow_mut().x_score = temp_board.x_score;
        // (*Rc::clone(&game_board_rc)).borrow_mut().o_score = temp_board.o_score;
        (*Rc::clone(&game_board_rc)).borrow_mut().recent_msg = temp_board.recent_msg;
        (*Rc::clone(&game_board_rc)).borrow_mut().new_game = temp_board.new_game;
        (*Rc::clone(&game_board_rc)).borrow_mut().turn = 1;
        (*Rc::clone(&game_board_rc)).borrow_mut().previous_select.loc.x = 4;
        (*Rc::clone(&game_board_rc)).borrow_mut().previous_select.loc.y = 4;
        // game_board.redraw();
        (*Rc::clone(&game_board_rc)).borrow().redraw();

    // {
    //     let game_board_mut = Arc::clone(&game_board);
    //     let mut mut_game_board = game_board_mut.lock().unwrap();
        
    //  
    }
    //? if was not
    else {
        // js! {
            //     console.log("x: ",@{x.clone()});
            //     console.log("y: ",@{y.clone()});
            // }
     

        //? state change
        state_change(x, y, game_board);
    }
}

}