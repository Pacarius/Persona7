use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::path::Path;
use std::{collections::HashMap, iter::Map};
// use tiff::decoder::{Decoder, DecodingResult};
use std::fs::File;
use std::io::BufReader;

use xcf::{RgbaPixel, Xcf};
#[derive(Debug)]
pub struct MapHelper(pub Vec<Layer>);
struct RGBA(RgbaPixel);
impl Display for RGBA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self.0))
    }
}
impl MapHelper {
    pub fn new(json_file_path: &Path) -> Self {
        let xcf_path = json_file_path.with_extension("xcf");
        let xcf_data = Self::load_xcf(&xcf_path).expect("Failed to load XCF file");
        Self(Self::load_json(json_file_path, &xcf_data).expect("Failed to load JSON file"))
    }
    fn load_xcf(file_path: &Path) -> Result<Vec<Vec<Vec<Option<u32>>>>, Box<dyn Error>> {
        let xcf = Xcf::open(file_path)?;
        let (width, length) = xcf.dimensions();
        let mut values: Vec<Vec<Vec<Option<u32>>>> = vec![];
        for layer in xcf.layers {
            let mut v: Vec<Vec<Option<u32>>> = vec![vec![None; (width - 1).try_into().unwrap()]; (length - 1).try_into().unwrap()];
            let mut keys = HashMap::new();
            //Load Keys
            let mut x = 0;
            while let Some(pixel) = layer.pixel(x, 0) {
                if pixel.eq(&RgbaPixel {
                    0: [0;4],
                })  || pixel.eq(&RgbaPixel{
                    0: [255; 4]
                }){
                    break;
                }
                keys.insert(format!("{:?}", pixel), x);
                println!("{} : {:?}", x, pixel);
                x += 1;
            }
            for x in 1..width - 1 {
                for y in 1..length - 1 {
                    match layer.pixel(x, y) {
                        None => {}
                        Some(p) => {
                            let p = RGBA(p);
                            if keys.contains_key(&p.to_string()) {
                                v[x as usize][y as usize] = Some(keys[&p.to_string()]);
                            }
                        }
                    }
                }
            }
            values.push(v);
        }
        Ok(values)
    }
    fn load_json( file_path: &Path, xcf: &Vec<Vec<Vec<Option<u32>>>>) -> Result<Vec<Layer>, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let json_data: serde_json::Value = serde_json::from_reader(reader)?;

        let mut layers = Vec::new();
        
        if let Some(object) = json_data.get("object") {
            let layer = Layer {
                name: "object".to_string(),
                content: vec![vec![None; xcf[0][0].len()]; xcf[0].len()],
                pairs: object
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap().to_string())
                .collect(),
            };
            layers.push(layer);
        }
        if let Some(layout) = json_data.get("layout") {
            let layer = Layer {
                name: "layout".to_string(),
                content: vec![vec![None; xcf[0][0].len()]; xcf[0].len()],
                pairs: layout
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap().to_string())
                .collect(),
            };
            layers.push(layer);
        }
        if let Some(background) = json_data.get("background") {
            let layer = Layer {
                name: "background".to_string(),
                content: vec![vec![None; xcf[0][0].len()]; xcf[0].len()],
                pairs: background
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_str().unwrap().to_string())
                    .collect(),
            };
            layers.push(layer);
        }

        for (i, layer) in layers.iter_mut().enumerate() {
            for x in 0..xcf[i].len() {
                for y in 0..xcf[i][x].len() {
                    let value = xcf[i][x][y];
                    match value {
                        Some(v) => {
                            // println!("{}", i);
                            layer.content[x][y] = Some(layer.pairs[v as usize].clone());
                        }
                        None => {}
                    }
                }
            }
        }
        Ok(layers)
    }
}
#[derive(Debug)]
pub struct Layer {
    name: String,
    content: Vec<Vec<Option<String>>>,
    pairs: Vec<String>,
}
impl MapHelper{
        pub fn find_path(&self, start: (usize, usize), end: (usize, usize), walls: &Vec<(usize, usize)>) -> Option<Vec<(usize, usize)>> {
            let mut open_set = VecDeque::new();
            let mut came_from = HashMap::new();
            let mut g_score = HashMap::new();
            let mut f_score = HashMap::new();
    
            open_set.push_back(start);
            g_score.insert(start, 0);
            f_score.insert(start, Self::heuristic(start, end));
    
            while let Some(current) = open_set.pop_front() {
                if current == end {
                    return Some(Self::reconstruct_path(came_from, current));
                }
    
                for neighbor in Self::get_neighbors(current) {
                    if walls.contains(&neighbor) {
                        continue;
                    }
    
                    let tentative_g_score = g_score.get(&current).unwrap_or(&usize::MAX) + 1;
    
                    if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&usize::MAX) {
                        came_from.insert(neighbor, current);
                        g_score.insert(neighbor, tentative_g_score);
                        f_score.insert(neighbor, tentative_g_score + Self::heuristic(neighbor, end));
    
                        if !open_set.contains(&neighbor) {
                            open_set.push_back(neighbor);
                        }
                    }
                }
            }
    
            None
        }
    
        fn heuristic(a: (usize, usize), b: (usize, usize)) -> usize {
            let dx = (a.0 as isize - b.0 as isize).abs();
            let dy = (a.1 as isize - b.1 as isize).abs();
            (dx + dy) as usize
        }
    
        fn get_neighbors(pos: (usize, usize)) -> Vec<(usize, usize)> {
            let mut neighbors = Vec::new();
            let (x, y) = pos;
    
            if x > 0 {
                neighbors.push((x - 1, y));
            }
            if x < usize::MAX {
                neighbors.push((x + 1, y));
            }
            if y > 0 {
                neighbors.push((x, y - 1));
            }
            if y < usize::MAX {
                neighbors.push((x, y + 1));
            }
    
            neighbors
        }
    
        fn reconstruct_path(mut came_from: HashMap<(usize, usize), (usize, usize)>, mut current: (usize, usize)) -> Vec<(usize, usize)> {
            let mut total_path = vec![current];
            while let Some(&next) = came_from.get(&current) {
                current = next;
                total_path.push(current);
            }
            total_path.reverse();
            total_path
        }
    }