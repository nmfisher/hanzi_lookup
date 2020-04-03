#![allow(dead_code)]
#![allow(unused_imports)]

use std::ffi::{CString, CStr};

extern crate wasm_bindgen;
extern crate serde_derive;
extern crate bincode;

mod analyzed_character;
mod cubic_curve_2d;
mod entities;
mod match_collector;
mod matcher;

use serde_derive::{Deserialize, Serialize};
use std::cell::RefCell;
use std::io;           
use std::slice;
use std::cmp;

use wasm_bindgen::prelude::*;

use match_collector::*;
use analyzed_character::*;
use match_collector::*;
use matcher::*;

#[derive(Serialize, Deserialize)]
struct Action {
    action: String,
    points: Vec<Vec<u8>>,
}

#[derive(Serialize, Deserialize)]
struct Input {
    char: String,
    ix: i64,
    duration: i64,
    actions: Vec<Action>,
}


#[wasm_bindgen]
pub fn lookup(input: &JsValue, limit: usize) -> String {
    // android_logger::init_once(
    //     Config::default().with_min_level(Level::Trace),
    // );
    // debug!("this is a debug {}", "message");
    // can't get logging to work 

    // Input is vector of vector of vector of numbers - how strokes and their points are represented in JS
    let input: Vec<Vec<Vec<f32>>> = input.into_serde().unwrap();
    // Convert to typed form: vector of strokes
    let mut strokes: Vec<Stroke> = Vec::with_capacity(input.len());
    for i in 0..input.len() {
        let mut stroke = Stroke {
            points: Vec::with_capacity(input[i].len()),
        };
        for j in 0..input[i].len() {
            stroke.points.push(Point {
                x: input[i][j][0].round() as u8,
                y: input[i][j][1].round() as u8,
            });
        }
        strokes.push(stroke);
    }
    let lookup_res = match_typed(&strokes, limit);
    serde_json::to_string(&lookup_res).unwrap()
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Match {
    pub hanzi: char,
    pub score: f32,
}

impl Match {    
    fn set_score(&mut self, new_val:f32 ) {
        self.score = new_val;
    }
    fn set_hanzi(&mut self, new_val:char ) {
        self.hanzi = new_val;
    }
}

#[repr(C)]
pub struct FFIPoint {
    pub x: f64,
    pub y: f64,
}

#[repr(C)]
pub struct FFIStroke {
    pub num_points:u8,
    pub point:*mut FFIPoint,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug)]
pub struct Stroke {
    pub points: Vec<Point>,
}

#[no_mangle]
pub extern "C" fn lookupFFI<'a>(
    mut strokes_ffi: *mut FFIStroke, num_strokes:u8, matches:*mut Match, limit: u8
) -> u8 {  
    let mut count = 0;
    unsafe {
        
        let mut strokes: Vec<Stroke> = Vec::with_capacity(num_strokes as usize);

        let _strokes_ffi_arr: &mut [FFIStroke] = slice::from_raw_parts_mut(strokes_ffi, num_strokes as usize);

        for i in 0..num_strokes {

            let _stroke_ffi: & FFIStroke = &_strokes_ffi_arr[i as usize];

            let _points_ffi_arr: &mut [FFIPoint] = slice::from_raw_parts_mut(_stroke_ffi.point, _stroke_ffi.num_points as usize);
        
            let mut points: Vec<Point> = Vec::with_capacity(_stroke_ffi.num_points as usize);

            for j in 0.._stroke_ffi.num_points as usize {

                let point_ffi: &FFIPoint = &_points_ffi_arr[j as usize];

                let point : Point = Point { 
                    x: (*point_ffi).x.round() as u8, 
                    y:(*point_ffi).y.round() as u8
                };
                points.push(point);            
                count = count + 1;
                
            }

            strokes.push(Stroke { 
                points:points
            });
        }
        
        let _lookup_res: Vec<Match> = match_typed(&strokes, limit as usize);
        
        let array: &mut [Match] = slice::from_raw_parts_mut(matches, limit as usize);
        if(_lookup_res.len() > 0) {
            for i in 0..cmp::min(limit as usize, _lookup_res.len()) {
                array[i as usize].set_score(_lookup_res[i as usize].score.into());
                array[i as usize].set_hanzi(_lookup_res[i as usize].hanzi);
                println!("{}", _lookup_res[i as usize].hanzi as u32);
            }
        }
        _lookup_res.len() as u8
    }
}



thread_local!(static MATCHER: RefCell<Matcher> = RefCell::new(Matcher::new()));

pub fn match_typed(strokes: &Vec<Stroke>, limit: usize) -> Vec<Match> {
    let mut res: Vec<Match> = Vec::with_capacity(limit);
    let mut collector = MatchCollector::new(&mut res, limit);
    MATCHER.with(|matcher| {
        matcher.borrow_mut().lookup(strokes, &mut collector);
    });
    res
}

