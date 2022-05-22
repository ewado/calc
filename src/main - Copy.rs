extern crate rand;
//use rand::prelude::*;
use std::io;
use std::io::Write;
use std::io::Read;
use std::fs::File;
//use std::io::prelude::*;
use std::fmt;
use std::collections::HashMap;
//use std::cmp::Reverse;

//use std::str::FromStr;
//use std::num::ParseIntError;

enum Action {
	Input(Input),
	Quit,		// call fn equal every time in loop, without clear_all.
	Undo,
	ClearAll,
	Copy,
	Help,
	ShowVar,
	ShowHistory,
	ShowHistoryH,
	Replace, // Replace value in history
	Refresh,
	ToVariableX, // assing to x
	ToVariableI, // tab initial
	ToVariableF, // tab final
	ToVariableN, // tab number of iterations
	Tabulate, // tabulate(i,f,n)
	Graphs,
	GraphVertical,
	ProcessFile,
	ReprocessFile,
}

#[derive(Clone)]
enum Input {
	Number(f64),
	String(String),
	Add,
	Subtract,
	Multiply,
	Divide,
	Sqrt,
	Pow,
	X,
	Rnd,
	Sin,
}
// let a: Input = Input::Number(123);

enum Instruction {
	Dsz,
	Sto,
	Rcl,
	Prt,
	Prl,
	Prf,
	Wrt,
	Ltt,
	Lte,
	Gtt,
	Gte,
	Eqq,
	Neq,
	Gto,
	Read,
	Other,
	//others
}

impl fmt::Display for Input {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			//Input::String(st) => write!(f, "\"{}\"", st),
			Input::String(st) => write!(f, "{}", st),
			Input::Number(n) => write!(f, "{}", n),
			Input::Add => write!(f, "+"),
			Input::Subtract => write!(f, "-"),
			Input::Multiply => write!(f, "*"),
			Input::Divide => write!(f, "/"),
			Input::Sqrt => write!(f, "sqrt"),
			Input::Pow => write!(f, "pow"),
			Input::X => write!(f, "x"),
			Input::Rnd => write!(f, "rnd"),
			Input::Sin => write!(f, "sin"),
		}
	}
}

const CSI: &str = "\x1B["; //escape
const MAXY: f64 = 65.0;  //width of screen for graphs, max i want to show
const XPARTITIONS: i32 = 80; // width of screen for vertical graph, x axe
const YVERTICAL: f64 = 50.0; // highth of y axe. highth of screen for graphs, max i want to show

struct AltScreen;

impl Drop for AltScreen {
	fn drop(&mut self) {
		print!("{csi}?1049l", csi = CSI)
	}
}

impl AltScreen {
	fn enable() -> Self {
		print!("{csi}?1049h", csi = CSI);
		Self{}
	}
	
	#[allow(dead_code)]
	fn disable(self) {
		drop(self);
	}
}

///Clear screen and position cursor
fn clear_screen() {
	print!("{csi}H{csi}J", csi = CSI);
}

///Clear_prev go to previous line and clear it
/* fn clear_prev() {
	print!("{csi}F{csi}2K", csi = CSI);
} */

fn parse(input_line: &str) -> Action {
	match input_line.trim() {
		"quit" | "q" => Action::Quit,
		"undo" | "u" => Action::Undo,
		"clr" | "c" => Action::ClearAll, // clears x, and all history
		"cpy" => Action::Copy,
		"hlp" | "h" => Action::Help,
		"shw" => Action::ShowVar,
		"hst" => Action::ShowHistory,
		"hsth" => Action::ShowHistoryH,
		"rpl" => Action::Replace,
		"" => Action::Refresh, //refresh screen
		"tox" => Action::ToVariableX,  // assign last value to x
		"toi" => Action::ToVariableI, // assign last value to i
		"tof" => Action::ToVariableF, // assign last value to f
		"ton" => Action::ToVariableN, // assign last value to n
		"tab" => Action::Tabulate, // tabulate based on i, f, n
		"grph" => Action::Graphs, // Graphs based on i, f, n
		"grphv" => Action::GraphVertical, // x horizontal y vertical 
		"pfile" => Action::ProcessFile,
		"." => Action::ReprocessFile,

		// Inputs
		"x" => Action::Input(Input::X), // enter parameter x
		"+" | "pl" | "pls" => Action::Input(Input::Add),
		"-" | "mn" | "mns" | "min" => Action::Input(Input::Subtract),
		"*" | "tms" | "tim" | "mlt" | "prd" => Action::Input(Input::Multiply),
		"/" | "div" | "dv" => Action::Input(Input::Divide),
		"sqr" | "sqrt" | "sq" => Action::Input(Input::Sqrt),
		"pwr" | "pow" | "pw" => Action::Input(Input::Pow),
		"rnd" => Action::Input(Input::Rnd),
		"sin" => Action::Input(Input::Sin),
		/* i => Action::Input(
				Input::Number(i.parse::<f64>().expect("Algo no anduvo"))),  */
		i => 
			{
			let value = i.parse::<f64>();
			match value 
				{
				Ok(i) => Action::Input(Input::Number(i)),
					//Action::Input(Input::Number(i.parse::<f64>().expect("Algo no anduvo"))),
					
				Err(_er) => Action::Input(Input::String(i.to_string())),
				}
			
			}
	}
}
/* fn parse_tof64(value: &str) -> value_f64 {
	match value.trim() {
		
	}
} */

fn parse_instruction(instruction: &str) -> Instruction {
	match instruction.trim() {
		"dsz" => Instruction::Dsz,
		"sto" => Instruction::Sto,
		"rcl" => Instruction::Rcl,
		"prt" => Instruction::Prt,
		"prl" => Instruction::Prl,
		"prf" | "" => Instruction::Prf,
		"wrt" => Instruction::Wrt,
		"ltt" => Instruction::Ltt,
		"lte" => Instruction::Lte,
		"gtt" => Instruction::Gtt,
		"gte" => Instruction::Gte,
		"eqq" => Instruction::Eqq,
		"neq" => Instruction::Neq,
		"gto" => Instruction::Gto,
		"read" => Instruction::Read,
		&_ => Instruction::Other,
	}
}	
//
//----------------------------------------------------------------------------------------------
//
struct CalState {
	history: Vec<Input>,
	expresion: Vec<String>,
	resultado: Vec<f64>,
	variable_x: f64,
	variable_i: f64,
	variable_f: f64,
	variable_n: i32,
	min_y: f64,
	max_y: f64,
	do_quit: bool,
	vars: HashMap<String, f64>,
	next_step: f64,
	show_results_expressions: bool,
	save_line: String,
	save_file_contents: String,
	// points: Vec<(i32, i32)>,
}

impl CalState {
	
	fn display_results_expressions(&self) {
		
		clear_screen();
		
		let mut printing_x: String;	
		
		let iter = self.resultado
			.iter()
			.zip(self.expresion.iter())
			.enumerate();
			
		for (i, (r, e)) in iter {
			if e.find('x') != None {
				printing_x = format!("x={}",self.variable_x.to_string());
			}
			else {
				printing_x = String::from(" ");
			}
			println!("#{}\t{}\t{}\t\t{}",i, e, printing_x, r)
		}
		print!("\n> ");
		io::stdout().flush().unwrap();
	}
	// shows tabulation
	fn display_tab(&self) {

		let iter = self.resultado
			.iter()
			.zip(self.expresion.iter())
			.enumerate();
			
		for (_i, (r, e)) in iter {		
			println!("y={}\t\tx={}\t\ty={}",e, self.variable_x.to_string(), r);
		}
		
		//io::stdout().flush().unwrap();
	}

	
	fn calculate_min_max(&mut self) {
		// find max_y and min_y
		let iter1 = self.resultado
			.iter()
			//.zip(self.expresion.iter())
			.enumerate();

		for (_i, r) in iter1 {
			if r < &self.min_y && r.is_infinite() == false {
				self.min_y = *r;
				
			}
			if r > &self.max_y && r.is_infinite() == false {
				self.max_y = *r;
				
			}
		}
	}
	
	// action tab 
	fn tabulate(&mut self) {
		// Needs the 3 parameters: i, n, f -- valor x inicio, n veces (+1), valor x final
		let delta = (self.variable_f - self.variable_i) / ((self.variable_n) as f64);

		for i in 0..self.variable_n+1
		{
			self.variable_x = self.variable_i + (delta * i as f64);			
			self.recreate();
			self.display_tab();
		}
		print!("\n> ");
		io::stdout().flush().unwrap(); 
	}

	fn process_graph(&mut self) {
		// Needs the 3 parameters: i, n, f -- valor x inicio, n veces (+1), valor x final
		let delta = (self.variable_f - self.variable_i) / ((self.variable_n) as f64);

		self.min_y = f64::MAX;  //99999999.0;
		self.max_y = f64::MIN;  // -99999999.0;
		
		for i in 0..self.variable_n+1
		{
			self.variable_x = self.variable_i + (delta * i as f64);
			self.recreate();
			self.calculate_min_max();
		}
		println!("min y {}, max y {}", &self.min_y, &self.max_y);
		
		for i in 0..self.variable_n+1
		{
			self.variable_x = self.variable_i + (delta * i as f64);			
			self.recreate();
			self.display_graph();
		}
		print!("\n> ");
		io::stdout().flush().unwrap(); 
		
		//self.display_graph2();
	}
	
	fn process_graph_vertical(&mut self) {
		// Needs the 3 parameters: i, n, f -- valor x inicio, n veces (+1), valor x final
		// In this case n (XPARTITIONS)will be a constant aprox 80.
		let delta = (self.variable_f - self.variable_i) / (XPARTITIONS as f64);

		self.min_y = f64::MAX;  //99999999.0;
		self.max_y = f64::MIN;  // -99999999.0;
		
		for i in 0..XPARTITIONS+1
		{
			self.variable_x = self.variable_i + (delta * i as f64);
			self.recreate();
			self.calculate_min_max(); // calculates y boundaries
		}
		println!("min y {}, max y {}", &self.min_y, &self.max_y);

		let mut points = Vec::<(i32, i32)>::new(); // save for vertical graph

		for i in 0..XPARTITIONS+1
		{
			self.variable_x = self.variable_i + (delta * i as f64);			
			self.recreate();
			//self.display_graph();
			self.calculate_graph_vertical(&mut points);
		}
		
		//points.sort_by_key(|k| Reverse(k.2));
		//self.points.sort_by_key(|k| k.1);
		
		points.sort_by(|a, b| if a.1 == b.1 {a.0.cmp(&b.0)} else {b.1.cmp(&a.1)});
		
		self.display_graph_vertical(&points);
		print!("\n> ");
		io::stdout().flush().unwrap(); 

	}
	// draw graph
	fn display_graph(&mut self) {

		//let mut yy: f64 = 0.0;

		// draw
		let iter = self.resultado
			.iter()
			//.zip(self.expresion.iter())
			.enumerate();


			
		for (_ii, mut r) in iter {
			// give proportion
			if r.is_infinite() == true {
				if r.is_sign_negative() == true {
					r = &self.max_y;
				}
				else {
					r = &self.min_y;
				}
			};
			
			let yy = (((r - &self.min_y)/(&self.max_y - &self.min_y)) * &MAXY).floor();
			let zz = ((( - &self.min_y)/(&self.max_y - &self.min_y)) * &MAXY).floor();

			//println!("{}",&zz);
			
			// print horizontal grph
			let mut i = 1.0;
			while &i <= &MAXY {
				if &i == &zz {
					print!("|");
				} else if &i <= &yy {
					print!("*");
				} else {
					print!(" ");
				}

				i += 1.0 // add one more dot
			} //end while for one line
			
			print!("\n");
			
			
			
			//io::stdout().flush().unwrap(); 
		} // end for
	}

	fn calculate_graph_vertical(&mut self, points: &mut Vec<(i32, i32)>) {
		
		let iter = self.resultado
			.iter()
			//.zip(self.expresion.iter())
			.enumerate();
			
		for (_ii, mut r) in iter {
			// give proportion to y axe
			if r.is_infinite() == true {
				if r.is_sign_negative() == true {
					r = &self.max_y;
				}
				else {
					r = &self.min_y;
				}
			};
			
			let yy = ((((r   - &self.min_y)/(&self.max_y - &self.min_y)) * &YVERTICAL)/1.214285).floor();
			let zz = ((((0.0 - &self.min_y)/(&self.max_y - &self.min_y)) * &YVERTICAL)/1.214285).floor();

			//println!("{}",&zz);
			
			let x1 = (((self.variable_x - self.variable_i + 1.0) / (self.variable_f - self.variable_i)) * XPARTITIONS as f64).floor() as i32;  // proportion to 80 chars per line max for x
			
			points.push((x1, yy as i32));
		}

	}
	fn display_graph_vertical(&mut self, points: &Vec<(i32, i32)>) {
		
		let mut line = " ".to_string();
		
		let mut yi = YVERTICAL as i32;   // max lines for y vertical
		//let mut first_time = true;
		
		let mut y_anterior: i32 = 999;
		let mut x_anterior: i32 = -999;


		// for debugging
		for (x, y) in points.iter() {
			print!("{} {}, ", x, y);
		}
		print!("\n");
		// end debugging

		for (x, y) in points.iter() {
			
			if yi > *y {
				//print last line
				let yaux = yi-1;
				for _i in *y..yaux {
					println!("{}",line);
				} 
				yi = *y;
			// continues processing
			} 
			if *y == y_anterior {
				//add to prev string
				if *x != x_anterior {
					line = format!("{}{}{}", line, " ".repeat((*x-x_anterior-1) as usize),"*");
					x_anterior = *x;
				}
			} else {
				//print last line, start new string
				println!("{}",line);
				line = format!("{}{}", " ".repeat((*x-1) as usize),"*");
				x_anterior = *x;
				y_anterior = *y;
				yi = *y;
			}
		} // end for
		
		//print last line
		println!("{}",line);
		

	}

	// action undo
	fn undo(&mut self) {
		self.history.pop().expect("Algo no anduvo en undo");
		self.recreate();
	}

	// action toi -- assign to variable x
	fn to_x(&mut self){
		self.history.pop().expect("Algo no anduvo"); // remove last value from history		
		self.variable_x = self.resultado.pop().expect("Algo no anduvo"); // latest value will be x
		
		//self.vars.insert("x".to_string(), self.variable_x); //hashmap
		self.recreate();
	}

	fn to_i(&mut self) {
		self.history.pop().expect("Algo no anduvo"); // remove last value from history		
		self.variable_i = self.resultado.pop().expect("Algo no anduvo"); // latest value will be x
		self.recreate();
	}	

	fn to_f(&mut self) {
		self.history.pop().expect("Algo no anduvo"); // remove last value from history		
		self.variable_f = self.resultado.pop().expect("Algo no anduvo"); // latest value will be x
		self.recreate();
	}

	fn to_n(&mut self) {
		self.history.pop().expect("Algo no anduvo"); // remove last value from history		
		self.variable_n = self.resultado.pop().expect("Algo no anduvo") as i32; // latest value will be x
		self.recreate();
	}

	fn recreate(&mut self) {
		self.resultado.clear();
		self.expresion.clear();
		for action in self.history.clone().iter() {
			self.do_step(&action);
		}
	}
	
	fn retrieve_last(&mut self) -> f64 {
		let last_result = self.resultado.pop().expect("Algo no anduvo");
		let last_result = last_result as usize;  //  replace with f64 usize function
		return *self.resultado.get(last_result).unwrap();
	}

	//action copy
	fn copy(&mut self){
		let y = self.retrieve_last();
		//let y = self.resultado[last_result].clone(); // option Some(_) or None
		
		self.resultado.push(y);
		
		self.expresion.pop().expect("Algo no anduvo");
		let z = y.to_string();
		self.expresion.push(z);
		
		
		self.history.pop().expect("Algo no anduvo en copy");
		self.history.push(Input::Number(y));
	}
	
	// action help
	fn help(&mut self) {
		println!(
"Commands\n
	quit | q => End program
	undo | u => Undo last entry
	+ | pls | add => Add previous to last [+(prev,last)]
	- | mns => Subtract last from previous [-(prev,last)]
	* | tms | mlt | prd => Multiply previous to last [*(prev,last)]
	/ | div | dv => Divide previous into last [/(prev,last)]
	sqrt | sqr | sq => Square root applied to last [sqrt(last)]
	pow | pwr => Power using previous as base and last as exponent [pow(prev,last)]
	c | clr => Clear history
	cpy => Copy value from line number [copy(last)] 
	h | hlp => Show list of commands
	hst => Show history
	hsth => Show history horizontal
	<enter> => Refresh screen
	tab [tabulate y(x) (i,f,n)]
	grph [graph y(x) (i,f,n)]
	tox_= [x=(last)] | toi [i=(last)] | tof [f=(last)] | ton [n=(last)]
	x [enter independent variable x]"
		);

		print!("\n> ");
		io::stdout().flush().unwrap();
	}

//	Action show variables
	fn show_variables(&mut self) {
		println!("i={}",self.variable_i);
		println!("f={}",self.variable_f);
		println!("n={}",self.variable_n);
		println!("x={}",self.variable_x);
		let dsz_index = *(self.vars.get(&"dsz_index".to_string()).unwrap()); //hashmap get
		println!("dsz_index={}", dsz_index);
		println!("command_line={}", self.save_line);
		println!("file_contents={}", self.save_file_contents);
		print!("\n> ");
		io::stdout().flush().unwrap();
	}

	// Action hst
	fn show_history(&mut self) {
		println!("Item\tCommand");
		for (i, x) in self.history.iter().enumerate() {
			println!("#{}\t{}", i, x);
		}
		
		print!("\n> ");
		io::stdout().flush().unwrap();
	}
	
	// Action hsth -- horizontal
	fn show_history_h(&mut self) {
		//println!("Item\tCommand");
		for (_i, x) in self.history.iter().enumerate() {
			print!("{} ", x);
		}
		
		print!("\n> ");
		io::stdout().flush().unwrap();
	}
	
	// action Replace
	fn replace(&mut self) {
		// take previous as index and last as value to update vector history
		let value = self.history.pop().expect("Algo no anduvo");

 		if let Some(Input::Number(item)) = Some(self.history.pop().expect("Algo no anduvo")) {
			let i = item as usize;
			if let Some(element) = self.history.get_mut(i) {
				*element = value;
				self.recreate()	 
			}
		}
		else {
		}; 
	}

	// action clear
	fn clear_all(&mut self){
		self.resultado.clear();
		self.expresion.clear();
		self.history.clear();
		self.variable_x = 1.0;
	}

	fn do_step(&mut self, action: &Input){
		self.step_resultado(&action); 
		self.step_expresion(&action);
	}

	fn step_resultado(&mut self, act: &Input) {
		match act {
			&Input::Add => {
				let x = self.resultado.pop().expect("Algo no anduvo");
				let y = self.resultado.pop().expect("Algo no anduvo");
				self.resultado.push(y+x);
			},
			&Input::Subtract => {
				let x = self.resultado.pop().expect("Algo no anduvo");
				let y = self.resultado.pop().expect("Algo no anduvo");
				self.resultado.push(y-x);
			},
			&Input::Multiply => {
				let x = self.resultado.pop().expect("Algo no anduvo");
				let y = self.resultado.pop().expect("Algo no anduvo");
				self.resultado.push(y*x);
			},
			&Input::Divide => {
				let x = self.resultado.pop().expect("Algo no anduvo");
				let y = self.resultado.pop().expect("Algo no anduvo");
				self.resultado.push(y/x);
			}
			&Input::Sqrt => {
				let x = self.resultado.pop().expect("Algo no anduvo");
				self.resultado.push(x.sqrt());
			}
			&Input::Pow => {
				let x = self.resultado.pop().expect("Algo no anduvo");
				let y = self.resultado.pop().expect("Algo no anduvo");
				self.resultado.push(y.powf(x)); // y elvated to x 
			}
			&Input::Sin => {
				let x = self.resultado.pop().expect("Algo no anduvo");
				self.resultado.push(x.sin());
			}
			&Input::X => {
				self.resultado.push(self.variable_x);
			}
			// random de 1 a 100
			&Input::Rnd => {
				//self.resultado.push((rand::random::<u32>() % 100) + 1);
				//self.resultado.push(((rand::random::<i32>() % 100) + 1) as f64);
				self.resultado.push(((rand::random::<u32>() % 100) + 1) as f64);
			}
			&Input::Number(i) => {
				self.resultado.push(i);
			},
			
			Input::String(_st) => {},
		}
	}
	
	fn step_expresion(&mut self, act: &Input){
		match act {
			&Input::Add => {
				let last = self.expresion.pop().expect("Algo no anduvo");
				let prev = self.expresion.pop().expect("Algo no anduvo");
				self.expresion.push(format!("({}+{})", &prev, &last));
			},
			&Input::Subtract => {
				let last = self.expresion.pop().expect("Algo no anduvo");
				let prev = self.expresion.pop().expect("Algo no anduvo");
				self.expresion.push(format!("({}-{})", &prev, &last));
			},
			&Input::Multiply => {
				let last = self.expresion.pop().expect("Algo no anduvo");
				let prev = self.expresion.pop().expect("Algo no anduvo");
				self.expresion.push(format!("({}*{})", &prev, &last));
			},
			&Input::Divide => {
				let last = self.expresion.pop().expect("Algo no anduvo");
				let prev = self.expresion.pop().expect("Algo no anduvo");
				self.expresion.push(format!("({}/{})", &prev, &last));
			},
			&Input::Sqrt => {
				let last = self.expresion.pop().expect("Algo no anduvo");
				self.expresion.push(format!("sqrt({})", &last));
			},
			&Input::Pow => {
				let last = self.expresion.pop().expect("Algo no anduvo");
				let prev = self.expresion.pop().expect("Algo no anduvo");
				self.expresion.push(format!("pow({},{})", &prev, &last));
			},
						&Input::Sin => {
				let last = self.expresion.pop().expect("Algo no anduvo");
				self.expresion.push(format!("sin({})", &last));
			},
			&Input::X => {
				self.expresion.push(format!("x"));
			},
			&Input::Rnd => {
				self.expresion.push(format!("rnd"));
			},
			&Input::Number(i) => {
				self.expresion.push(i.to_string());
			},
			Input::String(_st) => {
				//self.expresion.push(st);
			},
		}
	}
	
		// special commands. Here last or prev, when it is a string it refers to the history, not 'resultado'.
		// "sto" => Action::StoreToVar, // store last into previous (as variable name)
		//			sto,varname
		// "rcl" => retrieves variable 'last' to the stack
		//			rcl,varname
		// "prt" ==> Print // prints a variable, var name is in 'last'
		//			prt,varname
		//			prx,text // print text
		// "gto" => Action::GoTo, // change control next step to label in 'last' value 
		//			gto,lbl
		// "lte" => Action::GreaterThan // compares previous values in stack, for true changes control to label in last 
		//			lte,lbl
		// "lbl" => Action::Label // set step with Label name in last
		//			lbl,:labelname
		// "stp" or... use gto (end of file) => Action::Stop // stop processing file
		//			stp
		// "dsz" => // subtract 1 from last and change control to lbl if equal to zero
		//			dsz,labelname
		
		// "prf" -- Future: process file given a file name in last
		//			prf,filename
	
	
	fn write_file(&self, filename: &str) {		

		let mut file = File::create(filename).expect("Algo no anduvo");

		writeln!(&mut file, "{}", &self.save_line).expect("Algo no anduvo");
		
		print!("File {} saved\n> ", filename);
		io::stdout().flush().unwrap();
	}	
	
	// Action process file
	fn process_file(&mut self, filename: &str){
		//let mut input_file = std::fs::File::open("data.txt").unwrap();
		let mut input_file = std::fs::File::open(filename).unwrap();
		let mut contents = String::new();
		input_file.read_to_string(&mut contents).unwrap();
		//print!("{}", contents);
		contents = contents.replace("\n"," ");
		contents = contents.replace("\r"," ");
		contents = contents.replace("\t"," ");
		
		self.save_file_contents = contents.clone();
		
		let commands: Vec<&str> = contents.trim().split(' ').collect();
		
		self.process_line(&commands);
	}
	
	fn process_line(&mut self, commands: &Vec<&str>) {
		
		self.vars.insert("dsz_index".to_string(), 4.0); //hashmap insert default value
		//self.vars.insert("test".to_string(), 4.0); //hashmap insert for testing
		let commands_length = &commands.len();
		let mut change_control = true;
		self.next_step = 0.0;

		
		while change_control == true {		
			let from_i = self.next_step as usize;		
			for command_i in from_i..*commands_length  {
				change_control = false;

				if let Some(command) = &commands.get(command_i) {

					if command.find(',') == None {
						self.process_command(command);
					}
					else {
						
						let instruction: Vec<&str> = command.split(',').collect();
						
						if let Some(instruction_name) = &instruction.get(0) {
							match parse_instruction(&instruction_name.to_string()) {
								//------------------------------
								// dsz,labelname			//default varname is dsz_index
								// dsz,labelname,varname 
								//------------------------------
								Instruction::Dsz => {
									let mut dsz_value: f64;
									let varname_string: String;
									if let Some(varname) = &instruction.get(2) {
										varname_string = varname.to_string();
									}
									else {
										varname_string = "dsz_index".to_string();
									}
									dsz_value = *(self.vars.get(&varname_string).unwrap()); //hashmap get	
									
									dsz_value -= 1.0;
									if dsz_value > 0.0 {
										self.find_label(&instruction, &commands, &mut change_control);										
										self.vars.insert(varname_string, dsz_value); //hashmap insert
									}
									/* print!("\n> ");
									io::stdout().flush().unwrap(); */
								},
								// ltt,labelname    prev less-than last
								Instruction::Ltt => {
									let value2 = self.resultado.pop().expect("Algo no anduvo");
									let value1 = self.resultado.pop().expect("Algo no anduvo");
									if value1 < value2 {
										self.find_label(&instruction, &commands, &mut change_control);
										self.recreate();
									}
								},
								Instruction::Lte => {
									let value2 = self.resultado.pop().expect("Algo no anduvo");
									let value1 = self.resultado.pop().expect("Algo no anduvo");
									if value1 <= value2 {
										self.find_label(&instruction, &commands, &mut change_control);
										self.recreate();
									}
								},
								Instruction::Gtt => {
									let value2 = self.resultado.pop().expect("Algo no anduvo");
									let value1 = self.resultado.pop().expect("Algo no anduvo");
									if value1 > value2 {
										self.find_label(&instruction, &commands, &mut change_control);
										self.recreate();
									}
								},
								Instruction::Gte => {
									let value2 = self.resultado.pop().expect("Algo no anduvo");
									let value1 = self.resultado.pop().expect("Algo no anduvo");
									if value1 >= value2 {
										self.find_label(&instruction, &commands, &mut change_control);
										self.recreate();
									}
								},
								Instruction::Eqq => {
									let value2 = self.resultado.pop().expect("Algo no anduvo");
									let value1 = self.resultado.pop().expect("Algo no anduvo");
									if value1 == value2 {
										self.find_label(&instruction, &commands, &mut change_control);
										self.recreate();
									}
								},
								Instruction::Neq => {
									let value2 = self.resultado.pop().expect("Algo no anduvo");
									let value1 = self.resultado.pop().expect("Algo no anduvo");
									if value1 != value2 {
										self.find_label(&instruction, &commands, &mut change_control);
										self.recreate();
									}
								},
								Instruction::Gto => {
										self.find_label(&instruction, &commands, &mut change_control);
								},
								// sto,varname,value
								// sto,varname			//stores last value in stack
								Instruction::Sto => {
									if let Some(varname) = &instruction.get(1) {
										if let Some(strvalue) = &instruction.get(2) {
											let value = strvalue.parse::<f64>().expect("Algo no anduvo");
											self.vars.insert(varname.to_string(), value); //.expect("algo no anda por aca"); // insert hashmap
											
										}
										else {
											self.history.pop().expect("Algo no anduvo"); // remove last value from history
											let value = self.resultado.pop().expect("Algo no anduvo"); // latest value will be x
											self.vars.insert(varname.to_string(), value); //.expect("algo no anda por aca"); // insert hashmap
											self.recreate();
										}
									}
									/* print!("\n> ");
									io::stdout().flush().unwrap(); */
								},
								// rcl,varname
								Instruction::Rcl => {
									if let Some(varname) = &instruction.get(1) {
										let varname_string = varname.to_string();
										let var_content = *(self.vars.get(&varname_string).unwrap()); //hashmap get	
										self.history.push(Input::Number(var_content));
										self.show_results_expressions = true;

										self.recreate();
									}
								},
								
								//prt,varname - print variable
								Instruction::Prt => {
									if let Some(varname) = &instruction.get(1) {
										let varname_string = varname.to_string();
										let var_content = *(self.vars.get(&varname_string).unwrap()); //hashmap get	
										println!("{}={}", varname_string, var_content);
										/* print!("\n> ");										
										io::stdout().flush().unwrap(); */
										self.show_results_expressions = false;

									}
								}, 
								//prl,literal - print literal
								Instruction::Prl => {
									if let Some(literal) = &instruction.get(1) {
										//let varname_string = varname.to_string();
										println!("{}", literal);
										self.show_results_expressions = false;

									}
								},
								Instruction::Read => {
									if let Some(varname) = &instruction.get(1) {
										let value = self.read_value();
											self.vars.insert(varname.to_string(), value);
									}
								},
								// prcess file prf,filename    
								// extension is always txt but doesn't have to be entered
								Instruction::Prf => {
									if let Some(filename) = &instruction.get(1) {
										let filename_str = format!("{}.txt",filename.to_string());
										self.process_file(&filename_str);
										self.show_results_expressions = false; // cancels all possible trues
									}
								},
								Instruction::Wrt => {
									if let Some(filename) = &instruction.get(1) {
										let filename_str = format!("{}.txt",filename.to_string());
										self.write_file(&filename_str);
										self.show_results_expressions = false;
									}
								},
								Instruction::Other => {},  // future
							} // end match known instructions using comma
						} // has read instruction(0)
					} // command with a comma was processed
				} // command processed
				if change_control == true {break;};  // break for loop

			} // for
			
		} // while
	} // end function

	fn find_label(&mut self, instruction: &Vec<&str>, commands: &Vec<&str>, change_control: &mut bool) {
		if let Some(label) = &instruction.get(1) { 
			let tofind:String = format!("{}{}", "lbl,", label.trim());
			let le = &commands.len();
			for i in 0..*le  {
				if let Some(element) = &commands.get(i) {
					let found:String = element.trim().to_string();
					if found.eq(&tofind) {
						self.next_step = (i + 1) as f64;
						*change_control = true;
						break; // exit for-loop
					}
				}
			} // for loop
		}
	}
	
	fn process_command(&mut self, command: &str){
			match parse(&command.to_string()) {
				Action::Quit => {
					self.do_quit = true;
					return;
				},
				Action::Undo => self.undo(),
				Action::ClearAll => {
					self.clear_all();
					self.show_results_expressions = true;
				},
				Action::Copy => self.copy(),
				Action::Tabulate => {

					self.tabulate();
					self.show_results_expressions = false;
				},
				Action::Graphs => {
					self.process_graph();
					self.show_results_expressions = false;
				},
				Action::GraphVertical => {
					self.process_graph_vertical();
					self.show_results_expressions = false;
				},				
				Action::ProcessFile => {
					self.process_file("data.txt");
					self.show_results_expressions = false; //false; //true;
				},
				Action::ReprocessFile => {					
					let contents = self.save_file_contents.clone();
					let commands: Vec<&str> = contents.trim().split(' ').collect();					
					self.process_line(&commands);
					self.show_results_expressions = false
				}
				Action::ShowVar => {
					self.show_variables();
					self.show_results_expressions = false;
				},
				Action::Help => {
					self.help();
					self.show_results_expressions = false;
				},				
				Action::ShowHistory => {
					self.show_history();
					self.show_results_expressions = false;
				}
				Action::ShowHistoryH => {
					self.show_history_h();
					self.show_results_expressions = false;
				}
				Action::Replace => self.replace(),
				Action::Refresh => {
					self.show_results_expressions = true;
				}
				Action::ToVariableX => self.to_x(),
				Action::ToVariableI => self.to_i(),
				Action::ToVariableF => self.to_f(),
				Action::ToVariableN => self.to_n(),
				Action::Input(input) => {
					self.do_step(&input); 		
					self.history.push(input);
					self.show_results_expressions = true;
				}
			};
			
	} // end function
	
	// read numeric value
	fn read_value(&mut self) -> f64 {
		print!("\n>> ");
		io::stdout().flush().unwrap();	
		let mut strvalue: String = ' '.to_string();     // = String::with_capacity(1000);
		io::stdin().read_line(&mut strvalue).expect("error");
		
		// if strvalue.trim() == "q" {return -1.0};
		
		let value = strvalue.trim().parse::<f64>().expect("Algo no anduvo");
		// println!("leyo {}",value);
		// io::stdout().flush().unwrap();
		return value;

	}
} // end impl

fn main_loop() {
	let _alt_screen = AltScreen::enable();

/* 	clear_screen();
	//alt_screen.disable();  // this is an example. not use
	
	println!("Item\tResult\tExpression");
	print!("\n> ");
	io::stdout().flush().unwrap(); */
	

	let mut state = CalState {
		history: Vec::new(),
		expresion: Vec::new(),
		resultado: Vec::new(),
		variable_x: 1.0,
		variable_i: -10.0,
		variable_f: 10.0,
		variable_n: 30, // actually it is 10 plus 1 - includes extreems values i and f
		min_y: f64::MAX, //99999999.0,
		max_y: f64::MIN, //-99999999.0,
		do_quit: false,
		vars: HashMap::new(),
		next_step: 0.0,
		show_results_expressions: true,
		save_line: " ".to_string(),
		save_file_contents: " ".to_string(),
	}; 

	state.do_quit = false;
	let mut input_line: String = String::with_capacity(1000);
	//state.vars.insert("dsz_index".to_string(), 4.0); //hashmap initialize dsz with 4 times for now
	//state.vars.insert("test".to_string(), 0.0); //hashmap 
	clear_screen();
	print!("\n> ");
	io::stdout().flush().unwrap();
		
	loop {
		//Read
		input_line.clear();
		io::stdin().read_line(&mut input_line).expect("Algo no anduvo");

		state.save_line = input_line.clone();
	
		let commands: Vec<&str> = input_line.trim().split(' ').collect();

		state.process_line(&commands);
		
		if state.show_results_expressions == true {
			state.display_results_expressions()
		};
			
		if state.do_quit == true {
			break;
		};
	}
}


fn main() {
	main_loop();
}