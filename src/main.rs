extern crate rand;
// 2023-03-31 EP: Latest version, including end of stack failure prevention.
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
use std::process::Command;

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
	TotalSum,
	Sum, // to communication area
	ListPrograms, // list program files 
}

#[derive(Clone)]
enum Input {
	Number(f64),
	String(String),
	Add,
	Subtract,
	Multiply,
	Divide,
	Remainder,
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
	Run,
	Wrt,
	Ltt,
	Lte,
	Gtt,
	Gte,
	Eqq,
	Neq,
	Gto,
	Read,
	UndoN,
	StrEqq,
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
			Input::Remainder=> write!(f, "%"),
			Input::Sqrt => write!(f, "sqrt"),
			Input::Pow => write!(f, "pwr"),
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
const SWITCH: &str = "switch";

struct AltScreen;

impl Drop for AltScreen {
	fn drop(&mut self) {
		print!("{csi}?1049l", csi = CSI)
	}
}

/* impl AltScreen {
	fn enable() -> Self {
		print!("{csi}?1049h", csi = CSI);
		Self{}
	}
	
	#[allow(dead_code)]
	fn disable(self) {
		drop(self);
	}
} */

//Clear screen and position cursor
fn clear_screen() {
	print!("{csi}H{csi}J", csi = CSI);
}

//Clear_prev go to previous line and clear it
/* fn clear_prev() {
	print!("{csi}F{csi}2K", csi = CSI);
} */

fn parse(input_line: &str) -> Action {
	match input_line.trim() {
		"q" => Action::Quit,
		"u" => Action::Undo,
		"c" => Action::ClearAll, // clears x, and all history
		"cpy" => Action::Copy,
		"h" => Action::Help,
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
		"tsum" => Action::TotalSum,  // Shows the total sum of all values -- to remove or replace 2022-05-05 
		"sum" => Action::Sum, // to communication area
		"lst" => Action::ListPrograms, // list program files

		// Inputs -- operand, operator, function. They stay in the history
		"x" => Action::Input(Input::X), // enter parameter x
		"+" | "pls" => Action::Input(Input::Add),
		"-" | "mns" => Action::Input(Input::Subtract),
		"*" | "mlt" => Action::Input(Input::Multiply),
		"/" | "div" => Action::Input(Input::Divide),
		"%" | "rem" => Action::Input(Input::Remainder),
		"sqrt" => Action::Input(Input::Sqrt),
		"pwr" => Action::Input(Input::Pow),
		"rnd" => Action::Input(Input::Rnd),
		"sin" => Action::Input(Input::Sin),
		/* i => Action::Input(
				Input::Number(i.parse::<f64>().expect("Something didn't work"))),  */
		i => 
			{
			let value = i.parse::<f64>();
			match value 
				{
				Ok(i) => Action::Input(Input::Number(i)),
					//Action::Input(Input::Number(i.parse::<f64>().expect("Something didn't work"))),
					
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
		"run" | "" => Instruction::Run,
		"wrt" => Instruction::Wrt,
		"ltt" => Instruction::Ltt,
		"lte" => Instruction::Lte,
		"gtt" => Instruction::Gtt,
		"gte" => Instruction::Gte,
		"eqq" => Instruction::Eqq,
		"neq" => Instruction::Neq,
		"gto" => Instruction::Gto,
		"read" => Instruction::Read,
		"u" => Instruction::UndoN,
		"streqq" => Instruction::StrEqq,
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
	str_vars: HashMap<String, String>,
	next_step: f64,
	show_results_expressions: bool,
	save_line: String,
	save_file_contents: String,
	// points: Vec<(i32, i32)>,
	communication_area: String,
}

impl CalState {
	
	fn display_results_expressions(&self) {
		

		//	clear_screen();  // commented 2021-05-08

		let mut printing_x: String;	
		
		let iter = self.resultado
			.iter()
			.zip(self.expresion.iter())
			.enumerate();

		let precision = *(self.vars.get("precision").unwrap()) as usize;
		
		
		for (i, (r, e)) in iter {
			if e.find('x') != None {
				
				//let mut var_x = format!("{0:.5}",self.variable_x);
				let mut var_x = format!("{:.*}", precision, self.variable_x);
				
				if var_x.find('.') != None {
					var_x = var_x.trim_end_matches('0').to_string();
					var_x = var_x.trim_end_matches('.').to_string();
				}
				printing_x = format!("x={}",var_x);
			}
			else {
				printing_x = String::from(" ");
			}
			
			
			//let mut result_str = format!("{0:.5}",r);
			let mut result_str = format!("{:.*}", precision, r);
			if result_str.find('.') != None {
				result_str = result_str.trim_end_matches('0').to_string();
				result_str = result_str.trim_end_matches('.').to_string();
			}
			
			
			let mut num: usize = 2;
			if result_str.len() < 6 + precision {
				num = 6 + precision - result_str.len();
			}
			let spaces_after_result = " ".repeat(num);
			

			println!("#{:0>2}  {}{}{}  {}", i , result_str, spaces_after_result, e, printing_x);
			
		} // End for loop
		
		/* print!("\n> ");
		io::stdout().flush().unwrap(); */
		
	} //end display_results_expressions

	// shows tabulation
	fn display_tab(&self) {

		let iter = self.resultado
			.iter()
			.zip(self.expresion.iter())
			.enumerate();
			
		let precision = *(self.vars.get("precision").unwrap()) as usize;
		let mut var_x = format!("{:.*}", precision, self.variable_x);
		
		if var_x.find('.') != None {
			var_x = var_x.trim_end_matches('0').to_string();
			var_x = var_x.trim_end_matches('.').to_string();
		}	
			
		let mut num = 2;
		if var_x.len() < 6 + precision {
			num = 6 + precision - var_x.len();
		}
		let spaces_after_x = " ".repeat(num);
			
			
		for (_i, (r, e)) in iter {
			let mut result_str = format!("{:.*}", precision, r);
			if result_str.find('.') != None {
				result_str = result_str.trim_end_matches('0').to_string();
				result_str = result_str.trim_end_matches('.').to_string();
			}

			println!("y={}  x={}{}y={}", e, var_x, spaces_after_x, result_str);
			//println!("y={}    x={}    y={}", e, self.variable_x.to_string(), r);
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
		
		let min_x_print = format!("{:.*}", 2, self.variable_i);
		let max_x_print = format!("{:.*}", 2, self.variable_i + (delta * (self.variable_n+1) as f64));
		
		let min_y_print = format!("{:.*}", 2, &self.min_y);
		let max_y_print = format!("{:.*}", 2, &self.max_y);
		
		println!("min x {}, max x {}", min_x_print, max_x_print);
		println!("min y {}, max y {}", min_y_print, max_y_print);
		
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
			//let zz = ((((0.0 - &self.min_y)/(&self.max_y - &self.min_y)) * &YVERTICAL)/1.214285).floor();

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

	// action undo, undon
	fn undo(&mut self, value: i32) {
		for _i in 0..value {
			self.history.pop().expect("Something didn't work en undo");
		}
		self.recreate();
	}

	// action toi -- assign to variable x
	fn to_x(&mut self){
		self.history.pop().expect("Something didn't work"); // remove last value from history		
		self.variable_x = self.resultado.pop().expect("Something didn't work"); // latest value will be x
		
		//self.vars.insert("x".to_string(), self.variable_x); //hashmap
		self.recreate();
	}

	fn to_i(&mut self) {
		self.history.pop().expect("Something didn't work"); // remove last value from history		
		self.variable_i = self.resultado.pop().expect("Something didn't work"); // latest value will be x
		self.recreate();
	}	

	fn to_f(&mut self) {
		self.history.pop().expect("Something didn't work"); // remove last value from history		
		self.variable_f = self.resultado.pop().expect("Something didn't work"); // latest value will be x
		self.recreate();
	}

	fn to_n(&mut self) {
		self.history.pop().expect("Something didn't work"); // remove last value from history		
		self.variable_n = self.resultado.pop().expect("Something didn't work") as i32; // latest value will be x
		self.recreate();
	}

//match act {
//			&Input::Add => {

	fn recreate(&mut self) {
		self.resultado.clear();
		self.expresion.clear();
		// let mut prev_action: Input = Input::Add; // Input::String("$".to_string());     //use Option --commented out
		// let mut with_brackets: bool;
		for action in self.history.clone().iter() {
			self.do_step(&action);
		} // end-for
	}
	
	fn retrieve_last(&mut self) -> f64 {
		let last_result = self.resultado.pop().expect("Something didn't work");
		let last_result = last_result as usize;  //  replace with f64 usize function
		return *self.resultado.get(last_result).unwrap();
	}

	//action copy_value
	fn copy_value(&mut self){
		let y = self.retrieve_last();
		//let y = self.resultado[last_result].clone(); // option Some(_) or None
		
		self.resultado.push(y);
		
		self.expresion.pop().expect("Something didn't work");
		
		let precision = *(self.vars.get("precision").unwrap()) as usize;
		let z = format!("{:.*}", precision, y);
		
		//let z = y.to_string();
		self.expresion.push(z);
		
		
		self.history.pop().expect("Something didn't work en copy_value");
		self.history.push(Input::Number(y));
	}
	
	// action help
	fn help(&mut self) {
		println!(
"
	Inputs
		x            variable x
		pls          (a)+(b), consumes (a) and (b)
		mns          (a)-(b), consumes (a) and (b)
		mlt          (a)*(b), consumes (a) and (b)
		div          (a)/(b), consumes (a) and (b)
		sqrt         square-root(a), consumes (a)
		pwr          (a)^(b), consumes (a) and (b)
		sin          sin(a), consumes (a)
		<123.45>     enters numeric-value to (a)
		rnd          random value

	Actions
		q            quit
		u            undo, removes last (a)
		c            clears variable x and history
		h            help
		shw          show status
		hst          show history items with numbers
		hsth         show history in one line
		<enter>      refresh screen
		.            reprocess program file,
		
		cpy          copy value in item (a) from stack to new (a)
		rpl          replace expresion in item (a) in history (hst) with value (b)
		             
		tox          assigns (a) to variable x, consumes (a)
		toi          assigns (a) to variable i, consumes (a)
		tof          assigns (a) to variable f, consumes (a)
		ton          assigns (a) to variable n, consumes (a)
		tab          tabulate based on i, f, n
		grph         Graph based on i, f, n
		grphv        Graph with vertical y axe  
\npress enter");
// second help page
		let _string = self.read_string();
		println!(
"	
	Instructions 
		dsz,label              decrements variable dsz_index, if variable > 1 goto label 
		dsz,label,variable     decrements variable, if variable > 1 goto label 
		sto,variable,value     stores value in variable
		sto,variable           stores (a) in variable
							   
		rcl,variable           retrieve variable to (a)
		prt,variable           print variable
		prl,literal            print a text
		run,file               process program file
		,file                  process program file
		wrt,file               write current input line to program file
							   
		ltt,label              if (a)<(b) goto label
		lte,label              if (a)<=(b) goto label
		gtt,label              if (a)>(b) goto label
		gte,label              if (a)>=(b) goto label
		eqq,label              if (a)=(b) goto label
		neq,label              if (a)<>(b) goto label
		gto,label              goto label
		lbl,label              declare label
		read,variable          reads variable from stdin, if non-numeric defaults to zero
		read,variable,label    reads variable, if non-numeric goes to label
		u,number               undo n times as number
		streqq,label,variable,value    if string variable contents is equal to value goes to label
"
		);

		print!("\n> ");
		io::stdout().flush().unwrap();
	}

//	Action show variables
	fn show_variables(&mut self) {
		println!("i = {}",self.variable_i);
		println!("f = {}",self.variable_f);
		println!("n = {}",self.variable_n);
		println!("x = {}",self.variable_x);
		let dsz_index = *(self.vars.get(&"dsz_index".to_string()).unwrap()); //hashmap get
		println!("dsz_index = {}", dsz_index);
		let precision = *(self.vars.get(&"precision".to_string()).unwrap()); //
		println!("precision = {}", precision);
		println!("command = [{}]", self.save_line.trim());
		println!("program = [{}]", self.save_file_contents);
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
		let value = self.history.pop().expect("Something didn't work");

 		if let Some(Input::Number(item)) = Some(self.history.pop().expect("Something didn't work")) {
			let i = item as usize;
			if let Some(element) = self.history.get_mut(i) {
				*element = value;
				self.recreate()	 
			}
		}
		else {
		}; 
	}

	// action TotalSum -- to remove or replace
	fn total_sum(&self) {
		let mut total: f64 = 0.0;
		for s in &self.resultado {
			total+=s;
		}
		println!("Total Sum = {}", total);
		print!("\n> ");
		io::stdout().flush().unwrap();
	}

	// Action List program files - Open an explorer session in windows
	fn list_programs(&self) {
		println!("List program functionality not enabled");
		//println!( "Opening" );
		//Command::new( "explorer" )
        //.arg( ".\\programs" ) // <- Specify the directory you'd like to open.
        //.spawn( )
        //.unwrap( );
	}

	// action Sum -- sends the total sum to the opposite calculator
	fn to_communication_area(&mut self) {
		let mut total: f64 = 0.0;
		for s in &self.resultado {
			total+=s;
		}
		self.communication_area = total.to_string();
		// println!("Sent: Total Sum = {}", total);
		// print!("\n> ");
		// io::stdout().flush().unwrap();
	}

	// action clear
	fn clear_all(&mut self){
		self.resultado.clear();
		self.expresion.clear();
		self.history.clear();
		self.variable_x = 1.0;
	}

	fn do_step(&mut self, action: &Input){
		
		self.step_resultado(action); 
		
		self.step_expresion(action);
		
	}

	fn step_resultado(&mut self, act: &Input) {
		match act {
			&Input::Add => {
				let x = self.resultado.pop().expect("Something didn't work");
				//let y = self.resultado.pop().expect("Something didn't work");
				let y = match self.resultado.pop() {
					Some(top) => top, 
					None => {
						0.0 as f64
					}
				};
				self.resultado.push(y+x);
			},
			&Input::Subtract => {
				let x = self.resultado.pop().expect("Something didn't work");
				//let y = self.resultado.pop().expect("Something didn't work");
				let y = match self.resultado.pop() {
					Some(top) => top, 
					None => {
						//0.0 as f64
						x+x
					}
				};
				self.resultado.push(y-x);
			},
			&Input::Multiply => {
				let x = self.resultado.pop().expect("Something didn't work");
				//let y = self.resultado.pop().expect("Something didn't work");
				let y = match self.resultado.pop() {
					Some(top) => top, 
					None => {
						1.0 as f64
					}
				};
				self.resultado.push(y*x);
			},
			&Input::Divide => {
				let x = self.resultado.pop().expect("Something didn't work");
				//let y = self.resultado.pop().expect("Something didn't work");
				let y = match self.resultado.pop() {
					Some(top) => top, 
					None => {
						//1.0 as f64
						(x*x) as f64
					}
				};
				self.resultado.push(y/x);
			}
			&Input::Remainder => {
				let x = self.resultado.pop().expect("Something didn't work");
				//let y = self.resultado.pop().expect("Something didn't work");
				let y = match self.resultado.pop() {
					Some(top) => top, 
					None => {
						1.0 as f64
					}
				};
				self.resultado.push(y%x);
			}
			&Input::Sqrt => {
				let x = self.resultado.pop().expect("Something didn't work");
				self.resultado.push(x.sqrt());
			}
			&Input::Pow => {
				let x = self.resultado.pop().expect("Something didn't work");
				//let y = self.resultado.pop().expect("Something didn't work");
				let y = match self.resultado.pop() {
					Some(top) => top, 
					None => {
						1.0 as f64
					}
				};
				self.resultado.push(y.powf(x)); // y elvated to x 
			}
			&Input::Sin => {
				let x = self.resultado.pop().expect("Something didn't work");
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
				let last = self.expresion.pop().expect("Something didn't work");
				//let prev = self.expresion.pop().expect("Something didn't work");
				let prev = match self.expresion.pop() {
					Some(top) => top, 
					None => {
						"0".to_string()
					}
				};
					self.expresion.push(format!("({}+{})", &prev, &last))
			},
			&Input::Subtract => {
				let last = self.expresion.pop().expect("Something didn't work");
				//let prev = self.expresion.pop().expect("Something didn't work");
				let prev = match self.expresion.pop() {
					//Some(top) => top, 
					Some(top) => format!("({}-{})", top, &last),
					None => {
						//"0".to_string()
						last
					}
				};
				//self.expresion.push(format!("({}-{})", &prev, &last));
				self.expresion.push(prev);
			},
			&Input::Multiply => {
				let last = self.expresion.pop().expect("Something didn't work");
				//let prev = self.expresion.pop().expect("Something didn't work");
				let prev = match self.expresion.pop() {
					Some(top) => top, 
					None => {
						"1".to_string()
					}
				};
				self.expresion.push(format!("({}*{})", &prev, &last));
			},
			&Input::Divide => {
				let last = self.expresion.pop().expect("Something didn't work");
				//let prev = self.expresion.pop().expect("Something didn't work");
				let prev = match self.expresion.pop() {
					Some(top) => top, 
					None => {
						"1".to_string()
					}
				};
				self.expresion.push(format!("({}/{})", &prev, &last));
			},
			&Input::Remainder => {
				let last = self.expresion.pop().expect("Something didn't work");
				//let prev = self.expresion.pop().expect("Something didn't work");
				let prev = match self.expresion.pop() {
					Some(top) => top, 
					None => {
						"1".to_string()
					}
				};
				self.expresion.push(format!("({}%{})", &prev, &last));
			},
			&Input::Sqrt => {
				let last = self.expresion.pop().expect("Something didn't work");
				self.expresion.push(format!("sqrt({})", &last));
			},
			&Input::Pow => {
				let last = self.expresion.pop().expect("Something didn't work");
				//let prev = self.expresion.pop().expect("Something didn't work");
				let prev = match self.expresion.pop() {
					Some(top) => top, 
					None => {
						"1".to_string()
					}
				};
				self.expresion.push(format!("pwr({},{})", &prev, &last));
			},
						&Input::Sin => {
				let last = self.expresion.pop().expect("Something didn't work");
				self.expresion.push(format!("sin({})", &last));
			},
			&Input::X => {
				self.expresion.push(format!("x"));
			},
			&Input::Rnd => {
				self.expresion.push(format!("rnd"));
			},
			&Input::Number(i) => {
				let precision = *(self.vars.get("precision").unwrap()) as usize;
				let mut ii = format!("{:.*}", precision, i);
				//let mut ii = format!("{0:.5}",i);
				
				
				if ii.find('.') != None {
					ii = ii.trim_end_matches('0').to_string();
					ii = ii.trim_end_matches('.').to_string();
				}
				self.expresion.push(ii);
				//self.expresion.push(i.to_string());
			},
			Input::String(_st) => {
				//self.expresion.push(st);
			},
		}
	}
	
		// improvements
		//
		// "dsz"
		//			dsz,labelname
		//			dsz,label,variable
		// for		future: issue, too long instruction
		// "for"	for,label,ii,step,max		// increment ii by step, if less or eq than max goto label
		//     		for,label,ii,,max  // step one
		// arrays	future
		// imporve code, use regex to replace new lines
		//			use tuples returning from functions, 
		//			use Return type for validation and error msgs
		//			save status for cancelations. For now it's not bad idea
		// 			remove many recreates
	
	fn write_file(&self, filename: &str) {		

		let mut file = File::create(filename).expect("Something didn't work");

		writeln!(&mut file, "{}", &self.save_line).expect("Something didn't work");
		
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
									let value2 = self.resultado.pop().expect("Something didn't work");
									let value1 = self.resultado.pop().expect("Something didn't work");
									self.history.pop().expect("Something didn't work"); // remove last value from history
									self.history.pop().expect("Something didn't work"); // remove last value from history
									if value1 < value2 {
										self.find_label(&instruction, &commands, &mut change_control);
										self.recreate();
									}
								},
								Instruction::Lte => {
									let value2 = self.resultado.pop().expect("Something didn't work");
									let value1 = self.resultado.pop().expect("Something didn't work");
									self.history.pop().expect("Something didn't work"); // remove last value from history
									self.history.pop().expect("Something didn't work"); // remove last value from history
									if value1 <= value2 {
										self.find_label(&instruction, &commands, &mut change_control);
										self.recreate();
									}
								},
								Instruction::Gtt => {
									let value2 = self.resultado.pop().expect("Something didn't work");
									let value1 = self.resultado.pop().expect("Something didn't work");
									self.history.pop().expect("Something didn't work"); // remove last value from history
									self.history.pop().expect("Something didn't work"); // remove last value from history
									if value1 > value2 {
										self.find_label(&instruction, &commands, &mut change_control);
										self.recreate();
									}
								},
								Instruction::Gte => {
									let value2 = self.resultado.pop().expect("Something didn't work");
									let value1 = self.resultado.pop().expect("Something didn't work");
									self.history.pop().expect("Something didn't work"); // remove last value from history
									self.history.pop().expect("Something didn't work"); // remove last value from history
									if value1 >= value2 {
										self.find_label(&instruction, &commands, &mut change_control);
										self.recreate();
									}
								},
								// eqq,labelname 
								Instruction::Eqq => {
									let value2 = self.resultado.pop().expect("Something didn't work");
									let value1 = self.resultado.pop().expect("Something didn't work");
									self.history.pop().expect("Something didn't work"); // remove last value from history
									self.history.pop().expect("Something didn't work"); // remove last value from history
									if value1 == value2 {
										self.find_label(&instruction, &commands, &mut change_control);
										self.recreate();
									}
								},
								Instruction::Neq => {
									let value2 = self.resultado.pop().expect("Something didn't work");
									let value1 = self.resultado.pop().expect("Something didn't work");
									self.history.pop().expect("Something didn't work"); // remove last value from history
									self.history.pop().expect("Something didn't work"); // remove last value from history
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
											let value = strvalue.parse::<f64>().expect("Something didn't work");
											self.vars.insert(varname.to_string(), value); //.expect("algo no anda por aca"); // insert hashmap
											
										}
										else {
											self.history.pop().expect("Something didn't work"); // remove last value from history
											let value = self.resultado.pop().expect("Something didn't work");
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
								// streqq,label,varname,strvalue
								// if content of str variable is equal to value then go to label
								Instruction::StrEqq => {
									if let Some(varname) = &instruction.get(2) {
										let varname_string = varname.to_string();
										let var_content = &*(self.str_vars.get(&varname_string).unwrap()); //hashmap get	
										if let Some(strvalue) = &instruction.get(3) {
											if strvalue.to_string() == *var_content.trim() {
												self.find_label(&instruction, &commands, &mut change_control);
											}
										}
									}
								}
								
								
								//prt,varname - print variable
								Instruction::Prt => {
									if let Some(varname) = &instruction.get(1) {
										let varname_string = varname.to_string();
										let var_content = *(self.vars.get(&varname_string).unwrap()); //hashmap get	
										println!("{} = {}", varname_string, var_content);
										/* print!("\n> ");
										io::stdout().flush().unwrap(); */
										self.show_results_expressions = false;

									}
								}, 
								//prl,literal - print literal
								Instruction::Prl => {
									if let Some(literal) = &instruction.get(1) {
										//let varname_string = varname.to_string();
										let lit = literal.replace(".", " ");
										println!("{}", lit);
										self.show_results_expressions = false;

									}
								},
								//reads a value from stdin and stores it into variable
								// read,variable			if not numeric defauls to zero
								// read,variable,label		if not numeric, goes to label
								Instruction::Read => {
									if let Some(varname) = &instruction.get(1) {
										let strvalue = self.read_string();
										let value = &strvalue.trim().parse::<f64>();
										match value {
											Ok(i) => {
												self.vars.insert(varname.to_string(), *i);
											},
											Err(_er) => {
												self.str_vars.insert(varname.to_string(), strvalue);
												if let Some(label) = &instruction.get(2) {
													self.find_label2(&label, &commands, &mut change_control);
												} else {
													self.vars.insert(varname.to_string(), 0.0); // default is zero
												}
											},
										}										
										//let value = strvalue.trim().parse::<f64>().expect("Something didn't work");
										//self.vars.insert(varname.to_string(), value);
									}
								},
								// prcess program file run,filename    
								// extension is always txt but doesn't have to be entered
								Instruction::Run => {
									if let Some(filename) = &instruction.get(1) {
										let filename_str = format!("programs/{}.txt",filename.to_string());
										self.process_file(&filename_str);
										self.show_results_expressions = false; // cancels all possible trues
									}
								},
								// writes a file 
								Instruction::Wrt => {
									if let Some(filename) = &instruction.get(1) {
										let filename_str = format!("programs/{}.txt",filename.to_string());
										self.write_file(&filename_str);
										self.show_results_expressions = false;
									}
								},
								// undo,n
								Instruction::UndoN => {
									if let Some(strvalue) = &instruction.get(1) {
										let value = strvalue.parse::<i32>().expect("Something didn't work");
										self.undo(value);
									}
								}
								
								Instruction::Other => {},  // future
							} // end match known instructions using comma
						} // has read instruction(0)
					} // command with a comma was processed
				} // command processed
				if change_control == true {break;};  // break for loop

			} // for
			
		} // while
	} // end function

	fn find_label2(&mut self, label: &str, commands: &Vec<&str>, change_control: &mut bool) {
		//if let Some(label) = &instruction.get(1) { 
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
		//}
	}

	fn find_label(&mut self, instruction: &Vec<&str>, commands: &Vec<&str>, mut change_control: &mut bool) {
		if let Some(label) = &instruction.get(1) {
			self.find_label2(&label, &commands, &mut change_control);
		}
	}
	
	fn process_command(&mut self, command: &str){
			match parse(&command.to_string()) {
				Action::Quit => {
					self.do_quit = true;
					return;
				},
				Action::Undo => self.undo(1),
				Action::ClearAll => {
					self.clear_all();
					self.show_results_expressions = true;
				},
				Action::Copy => self.copy_value(),
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
				},
				Action::ShowHistoryH => {
					self.show_history_h();
					self.show_results_expressions = false;
				},
				Action::Replace => self.replace(),
				
				Action::TotalSum => {
					self.total_sum();
					self.show_results_expressions = false;
				},
				
				Action::Sum => {
					self.to_communication_area();
					self.show_results_expressions = true;
				},
				
				Action::ListPrograms => {
					self.list_programs();
					self.show_results_expressions = true;
				},
				
				Action::Refresh => {
					self.show_results_expressions = true;
				},
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
			
	} // end fn process_command
	
	fn read_string(&mut self) -> String {
		print!("\n>> ");
		io::stdout().flush().unwrap();	
		let mut strvalue: String = ' '.to_string();
		io::stdin().read_line(&mut strvalue).expect("error");
		return strvalue;
	}	
	

	
	
	
} // end impl CalState

fn state_init() -> CalState {
	let state = CalState {
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
		str_vars: HashMap::new(),
		next_step: 0.0,
		show_results_expressions: true,
		save_line: " ".to_string(),
		save_file_contents: " ".to_string(),
		communication_area: " ".to_string(),
	}; 
	return state;
}

fn main_loop() {
//	let _alt_screen = AltScreen::enable();

/* 	clear_screen();
	//alt_screen.disable();  // this is an example. not use
	
	println!("Item\tResult\tExpression");
	print!("\n> ");
	io::stdout().flush().unwrap(); */

// cal 1
	let mut state = state_init();

	state.vars.insert("dsz_index".to_string(), 4.0); //hashmap insert default value
	state.vars.insert("precision".to_string(), 2.0); // default number of decimals to display

	state.do_quit = false;
	
// cal 2
	let mut state2 = state_init();
	
	state2.vars.insert("dsz_index".to_string(), 4.0); //hashmap insert default value
	state2.vars.insert("precision".to_string(), 2.0); // default number of decimals to display

	state2.do_quit = false;

	
	let mut input_line: String = String::with_capacity(1000);
	//state.vars.insert("dsz_index".to_string(), 4.0); //hashmap initialize dsz with 4 times for now
	//state.vars.insert("test".to_string(), 0.0); //hashmap 
	clear_screen();
	
	// print!("\n> ");    // commented 2021-05-08
	
	//print!("\n~1~ > ");
	print!("\n(1)> ");
	io::stdout().flush().unwrap();
	
	// clone <- Clone , implicit clone <- Copy, in general Copy implemented on all primitive types.
	let mut switch_cal: i32 = 1;

	
	loop {
//Read
		input_line.clear();
		io::stdin().read_line(&mut input_line).expect("Something didn't work");

// switch only word in line. Manual entry.
		if input_line.trim() == SWITCH {
			if switch_cal == 2 {
				switch_cal = 1;
			} else {
				switch_cal = 2;
			}
		}
		
		let commands: Vec<&str> = input_line.trim().split(' ').collect();
		
// switch between cal 1 and cal 2
		if switch_cal == 2 {
			state2.communication_area = " ".to_string();
			state2.save_line = input_line.clone();
			state2.process_line(&commands);
			if state2.communication_area != " ".to_string() {
				let string_aux = state2.communication_area.clone();
				let commands: Vec<&str> = string_aux.trim().split(' ').collect();
				//opposite calculator
				state.process_line(&commands); 
			}
		}
		else {
			state.communication_area = " ".to_string();
			state.save_line = input_line.clone();
			state.process_line(&commands);
			if state.communication_area != " ".to_string() {
				let string_aux = state.communication_area.clone();
				let commands: Vec<&str> = string_aux.trim().split(' ').collect();
				//opposite calculator
				state2.process_line(&commands); 
			}
		}
		
		if state.do_quit == true || state2.do_quit {
			break;
		}; 
		

		
		clear_screen();
		
		if state.show_results_expressions == true && state2.show_results_expressions == true {
			if state.resultado.len() != 0 {
				println!("Calculator (1)");
				state.display_results_expressions();
			}
			if state2.resultado.len() != 0 {
				if state.resultado.len() != 0 {
					println!(" ");
				};
				println!("Calculator (2)");
				state2.display_results_expressions();
			}
			print!("\n({})> ", switch_cal);
			io::stdout().flush().unwrap();
		};		
		

	}  // end loop
}


fn main() {
	main_loop();
}
