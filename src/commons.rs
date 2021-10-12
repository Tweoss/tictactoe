pub mod commons {
    //useless change

pub(crate) use std::sync::{MutexGuard, Mutex, Arc};
// pub(crate) use std::borrow::BorrowMut;
pub(crate) use std::cell::{RefCell};
pub(crate) use std::rc::Rc;
// pub(crate) use std::cell::{RefCell, RefMut};
    pub(crate) use stdweb::web::{
        html_element::CanvasElement, 
        self, 
        CanvasRenderingContext2d, 
        IEventTarget, 
        INonElementParentNode,
        // Window,
        window,
        TextAlign, TextBaseline
    };
    pub(crate) use stdweb::unstable::TryInto;
    pub(crate) use stdweb::web::event::{
        //IEvent, IKeyboardEvent, KeyDownEvent, KeyUpEvent, KeyboardLocation, MouseButton,
        MouseDownEvent, //MouseMoveEvent, MouseUpEvent,TouchStart,TouchEnd,TouchMove
    };
    pub(crate) use stdweb::traits::*;
#[derive(Default)]
#[derive(Copy, Clone)]
pub struct Selection {
    pub x: u8,		//? the loc of the cell of the subcell
    pub y: u8,		//? the loc of the cell of the subcell
    pub m: u8		//? the loc of the subcell within the cell
}

pub struct ExtendedSelection {
    pub x: u8,		//? the loc of the cell of the subcell
    pub y: u8,		//? the loc of the cell of the subcell
    pub m: u8,		//? the loc of the subcell within the cell
    pub m_next: u8 	//? the linking subcell for cycles
}

// pub struct 

#[derive(Default)]
#[derive(Copy, Clone)]
pub struct SubCell {
    pub value: u16,		//? the value of the subcell
    pub loc: Selection, //? the loc of the subcell
    pub link: Selection //? the linked subcell
}
//TODO: MAKE EVERYTHING A METHOD FOR GAMESTRUCT
//TODO: also, should refactor make things better
#[derive(Default)]
#[derive(Clone)]
    pub struct GameStruct {
        pub state: [[[SubCell; 9]; 3]; 3],  //? the state of the board
        pub waiting_for_second_click: bool, //? is there already a matching uncertain
        pub previous_select: SubCell,       //? the previous selection (an uncertain)
        pub turn: u16,				        //? the current turn number
        pub is_waiting_for_selection: bool, //? if the click is supposed to be processed as a collapsing selection
        // pub has_a_selection: bool,	    //? if there is a selection to be processed
        // pub collapse_result: Selection,  //? the user-selected collapsing
        pub collapse_possib: Selection,     //? stores the selection that closes the loop,
        pub x_score: f32, 				    //? the current score for X
        pub o_score: f32, 				    //? the current score for O
        pub recent_msg: String,             //? the displaying recent msg
        pub new_game: bool,                 //? if waiting for a click to start another game
    }
    pub(crate) const SMALL_BLANK: u16 = 0;
    pub(crate) const BIG_BLANK: u16 = 1;
    pub(crate) const BIG_X: u16 = 300; // * 300->329
    pub(crate) const BIG_O: u16 = 400; // * 400->429
    pub(crate) const UNCERTAIN_X: u16 = 4;
    pub(crate) const UNCERTAIN_O: u16 = 5;
    pub(crate) const SMALL_X: u16 = 100; // * 100->129
    pub(crate) const SMALL_O: u16 = 200; // * 200->229

    pub fn state_test(value: u16, match_value: u16) -> bool {
        match match_value {
            SMALL_BLANK if value == SMALL_BLANK => {
                return true;
            },
            BIG_BLANK if value == BIG_BLANK => {
                return true;
            },
            UNCERTAIN_X if value == UNCERTAIN_X => {
                return true;
            },
            UNCERTAIN_O if value == UNCERTAIN_O => {
                return true;
            },
            SMALL_X if (value > SMALL_X) && (value <= SMALL_O) => {
                return true;
            },
            SMALL_O if (value > SMALL_O) && (value <= BIG_X) => {
                return true;
            },
            BIG_X if (value > BIG_X) && (value <= BIG_O) => {
                return true;
            },
            BIG_O if (value > BIG_O) && (value <= BIG_O+100) => {
                return true;
            },
            _ => {
                return false;
            }
        }
    }

    pub(crate) fn ze_console(msg: &str) {
        js! {
            console.log(@{msg});
        }
    }
}