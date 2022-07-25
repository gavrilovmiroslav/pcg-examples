use std::fmt::{Display, Formatter};
use std::ops::AddAssign;
use std::slice;

use cellular_automaton::ruleset::*;
use cellular_automaton::life_like::*;
use notan::prelude::*;

use jammdb::{DB, Data, Error};

#[derive(Clone)]
pub struct StateConfig {
    pub size: (usize, usize),
    pub birth: Vec<bool>,
    pub survival: Vec<bool>,
    pub generations: usize,
}

pub fn bools_to_indices(input: &[bool]) -> Vec<u8> {
    let indices = input.iter()
        .zip(1..9)
        .filter(|(&state, _)| state)
        .map(|(_, index)| index)
        .collect();
    indices
}

pub fn bools_to_string(input: &[bool]) -> String {
    input.iter()
        .zip(1..9)
        .map(|(&state, index)| if state { format!("{}", index) } else { "x".to_string() })
        .collect::<Vec<String>>()
        .join("")
}


impl Display for StateConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "B{} ", bools_to_string(&self.birth));
        write!(f, "S{} ", bools_to_string(&self.survival));
        write!(f, "G{}", self.generations)
    }
}

impl StateConfig {
    pub fn from_string(s: &str) -> StateConfig {
        println!("{}", s);
        // 0123456789abcdef012345
        // Bxxxx5678 Sxxx45678 G4
        let sb = s[1..=8].to_string();
        let ss = s[11..=19].to_string();
        let sg = s[21..].to_string();

        let birth: Vec<bool> = sb.chars().map(|c| c != 'x').collect();
        let survival: Vec<bool> = ss.chars().map(|c| c != 'x').collect();
        let generations: usize = sg.parse().expect("Cannot parse number");

        StateConfig{
            size: (50, 50),
            birth: birth.into(),
            survival: survival.into(),
            generations
        }
    }

    pub fn set_to(&mut self, config: &StateConfig) {
        self.generations = config.generations;
        self.birth = config.birth.clone();
        self.survival = config.survival.clone();
        self.size = config.size;
    }
}

#[derive(AppState, Clone)]
pub struct State {
    pub config: StateConfig,
    pub gallery: Vec<StateConfig>,
    pub automaton: Automaton,
}

impl Default for State {
    fn default() -> Self {
        let size = (50, 50);
        let gen = 4;
        let mut state = State::new(size, gen,
                   [ false, false, false, false, true, true, true, true ].to_vec(),
                   [ false, false, false, false, true, true, true, true ].to_vec());

        state.load_gallery_from_file();
        state
    }
}

impl State {
    pub fn new_from_config(config: &StateConfig) -> State {
        Self::new(config.size, config.generations, config.birth.clone(), config.survival.clone())
    }

    pub fn create_automaton_from_config(config: &StateConfig) -> Automaton {
        let births: Vec<u8> = bools_to_indices(&config.birth);
        let survival: Vec<u8> = bools_to_indices(&config.survival);

        let rules = BSC::new(births.as_slice(), survival.as_slice(), 1);
        let mut ca = Automaton::new(config.size.0, config.size.1);
        ca.rules = rules;

        ca.randomize_cells(0.5);
        for _ in 0..config.generations {
            ca.step();
        }

        ca
    }

    pub fn new(size: (usize, usize), gen: usize, birth: Vec<bool>, survival: Vec<bool>) -> State {
        let config = StateConfig { size, birth, survival, generations: gen };
        let automaton = Self::create_automaton_from_config(&config);

        State { config, automaton, gallery: vec![] }
    }

    pub fn update(&mut self) {
        self.automaton = Self::create_automaton_from_config(&self.config);
    }

    pub fn get_matrix(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.automaton.cells_ptr(), self.config.size.0 * self.config.size.1) }
    }

    pub fn load_gallery_from_file(&mut self) {
        let db = DB::open("gallery.db").expect("Cannot open db");
        let mut tx = db.tx(false).expect("Cannot start transaction");
        let gallery_bucket = tx.get_bucket("gallery").expect("Cannot get bucket");

        for x in gallery_bucket.cursor() {
            let str = std::str::from_utf8(x.kv().value()).expect("Cannot parse gallery string");
            self.gallery.push(StateConfig::from_string(str));
        }
    }

    pub fn save_gallery_to_file(&self) {
        let db = DB::open("gallery.db").expect("Cannot open db");;
        let mut tx = db.tx(true).expect("Cannot start transaction");

        tx.delete_bucket("gallery");
        let mut gallery_bucket = tx.create_bucket("gallery").expect("Creating bucket");
        let mut n = 0;

        for item in &self.gallery {
            gallery_bucket.put(n.to_string(), item.to_string());
            n += 1;
        }
        tx.commit();
    }

    pub fn save_to_gallery(&mut self) {
        self.gallery.push(self.config.clone());
        self.save_gallery_to_file();
    }
}
