use fltk::{app, frame::*, button::*, window::*, menu::*, prelude::*};
use fltk::input::Input;
use fltk::enums::Color;
use fltk::menu::MenuFlag;
use fltk::enums::Shortcut;
use fltk::enums::FrameType;
use std::process::exit;
use rand::prelude::SliceRandom;
use sqlite::{Connection, State};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufReader;
use xmltree::Element;
use fltk::group::Pack;
use fltk::group::Flex;
use fltk::group::PackType;
use fltk::group::FlexType;
use fltk::group::Scroll;
use fltk::dialog;
//use std::sync::Mutex;
//use fltk::valuator::Scrollbar;
//use fltk::valuator::ScrollbarType;
//use fltk::enums::Event;
//use fltk::window::MenuWindow;
use crossclip::{Clipboard, SystemClipboard};
use std::fs::remove_file;
use fltk::image::PngImage;

fn selcopy(word: &str) -> Result<(), Box<dyn std::error::Error>> {
    let clipboard = SystemClipboard::new()?;
    clipboard.set_string_contents(String::from(word))?;
    println!("{}", clipboard.get_string_contents()?);
    Ok(())
}
// method to write values to the table upon importing xml backup file
fn write_table(pwd: String, uname: String, sname: String){
	let conn = Connection::open("passwords.db");
	let _ = conn.expect("REASON").execute("INSERT INTO password VALUES ('".to_owned() + &pwd + "', '" + &uname + "', '" + &sname + "')");
}

// method to read from the passwords.db table
fn read_table() -> (Vec<String>, Vec<String>, Vec<String>) {
	let mut pwds = Vec::new();
	let mut unames = Vec::new();
	let mut snames = Vec::new();
	
	let conn = Connection::open("passwords.db").unwrap();
	let query = "SELECT pw, username, site FROM password";
	
	let mut statement = conn.prepare(query).unwrap();
   
	while let Ok(State::Row) = statement.next() {
		pwds.push(statement.read::<String, _>("pw").unwrap());
		unames.push(statement.read::<String, _>("username").unwrap());
		snames.push(statement.read::<String, _>("site").unwrap());
	}
	return (pwds, unames, snames);
}

/*
fn pw_row() -> Vec<String> {
    	let mut pwds = Vec::new();
    	let conn = Connection::open("passwords.db").unwrap();
	let query = "SELECT pw FROM password";
	
	let mut statement = conn.prepare(query).unwrap();
   
	while let Ok(State::Row) = statement.next() {
		pwds.push(statement.read::<String, _>("pw").unwrap());
	}
	return pwds
}
*/

fn main() {
    let app = app::App::default();
        let (s, _r) = app::channel();
        
        // if this is the first run and the passwords.db file does not exist, create it
	let conn = Connection::open("passwords.db");
	let _ = conn.expect("REASON").execute(
		"create table if not exists password (
			pw TEXT PRIMARY KEY,
			username TEXT NOT NULL,
			site TEXT NOT NULL
		 )"
	);

    let mut wind = Window::new(200, 90, 750, 600, "Knapsack"); // 720, 600
    wind.set_color(Color::from_rgb(54,69,79));
    let wind3 = wind.clone();
    
    let mut pwlab: Input = Input::new(5, 40, 200, 30, "Password");
    pwlab.set_value("password");
    let entry = pwlab.clone();
    let entry1 = pwlab.clone();
    let entry5 = pwlab.clone();
    let entry6 = pwlab.clone();
    let entry7 = pwlab.clone();
    //let entry8 = pwlab.clone();
    
    #[derive(Clone)]
    enum Message {
    Choice1,
    Choice2,
    Choice3,
    Choice4,
    Choice5,
    }

    let mut menu = SysMenuBar::default().with_size(800, 35);
        menu.set_frame(FrameType::FlatBox);
        menu.add_emit(
            "&File/Delete Database\t",
            Shortcut::Ctrl | 'd',
            MenuFlag::Normal,
            s.clone(),
            Message::Choice1,
        );
        menu.add_emit(
            "&File/Export Backup\t",
            Shortcut::Ctrl | 'e',
            MenuFlag::Normal,
            s.clone(),
            Message::Choice2,
        );
        menu.add_emit(
            "&File/Import Backup\t",
            Shortcut::Ctrl | 'i',
            MenuFlag::Normal,
            s.clone(),
            Message::Choice3,
        );
        menu.add_emit(
            "&File/Quit\t",
            Shortcut::Ctrl | 'q',
            MenuFlag::Normal,
            s.clone(),
            Message::Choice4,
        );
        menu.add_emit(
            "&Help/About\t",
            Shortcut::None,
            MenuFlag::Normal,
            s.clone(),
            Message::Choice5,
        );

        if let Some(mut item) = menu.find_item("&File/Quit\t") {
            item.set_label_color(Color::Red);
            item.set_callback(move |_| quiteme());
        }
        /*
        if let Some(mut item) = menu.find_item("&File/Import Backup\t") {
            let wind3 = wind.clone();
            item.set_callback(move |_| importbu(entry6.clone(), wind3.clone(), allpack.clone()));
        }
        */
        if let Some(mut item) = menu.find_item("&File/Export Backup\t") {
            item.set_callback(move |_| exportbu());
        }
        /*
        if let Some(mut item) = menu.find_item("&Help/About\t") {
            item.set_callback(move |_| aboutme());
        }
        */

    fn quiteme() {
    	exit(0);
    }
    
    fn importbu(ent: Input, wind: Window, allpack: Scroll) {
        let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseFile);
        dialog.set_filter("*.xml");
        dialog.show();
        
        let file = File::open(&Path::new(&dialog.filename())).unwrap();
	let reader = BufReader::new(file);
	let names_element = Element::parse_all(reader).unwrap();
	
	for p in names_element {
		
		let pp = p.as_element();
		
		let pwd = pp.expect("REASON").get_child("pw").expect("Can't find pw element");
		let mypassword = pwd.get_text().unwrap().to_string().replace("expl", "!").replace("aatt", "@").replace("&amp;", "&").replace("\"", "");
		let unme = pp.expect("REASON").get_child("username").expect("Can't find username element");
		let myusername = unme.get_text().unwrap().to_string().replace("\"", "");
		
		let snme = pp.expect("REASON").get_child("site").expect("Can't find pw element");
		let mysite = snme.get_text().unwrap().to_string().replace("\"", "");
		
		write_table(mypassword, myusername, mysite);
	}
	makerows(ent.clone(), wind.clone(), allpack.clone(), "import");
    }
    
    // method to remove the database
    fn removedb(allpack: Scroll) {
        let path = "passwords.db";
	match remove_file(path) {
        Ok(_) =>println!("file removed"),
	Err(e) => println!("{}", e)
	}
	let conn = Connection::open("passwords.db");
	let _ = conn.expect("REASON").execute(
		"create table if not exists password (
			pw TEXT PRIMARY KEY,
			username TEXT NOT NULL,
			site TEXT NOT NULL
		 )"
	);
	//removedatabase(allpack.clone());
	allpack.clone().clear();
        app::redraw();
    }
    
    // method to export all data in database to an xml file
    fn exportbu() {
        let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseSaveFile);
        dialog.set_filter("*.xml");
        dialog.show();

        // variables to store table values
	let mut pwds = Vec::new();
	let mut unames = Vec::new();
	let mut snames = Vec::new();
	
	// read info from the passwords table and store its values in variables
	(pwds, unames, snames) = read_table();

        // Write the variables to an xml file
        let mut i = 0;
	let mut x: String = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n".to_string();
	while i < pwds.len() {
		x = x.to_owned() + "<backuppasswords>\n";
		x = x.to_owned() + "    <pw>" + &pwds[i].to_string().replace("!", "expl").replace("@", "aatt").replace("&", "&amp;") + "</pw>\n";
		x = x.to_owned() + "    <username>" + &unames[i].to_string() + "</username>\n";
		x = x.to_owned() + "    <site>" + &snames[i].to_string() + "</site>\n";
		x = x.to_owned() + "</backuppasswords>\n";
		i += 1;
	}
	
	// create the xml file save path
	let binding = dialog.filename();
	let fullpath = binding.display().to_string() + ".xml";
	let path = Path::new(&fullpath);
	
	// create the file path or throw exception if it cant be done
	let display = path.display();
	let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
	};

        // write the file to disk
	match file.write_all(x.as_bytes()) {
		Err(why) => panic!("couldn't write to {}: {}", display, why),
		Ok(_) => println!("successfully wrote to {}", display),
	}
    }
    
    // Method to show "About" information
    fn aboutme(popwind: Window) {
        let popwind2 = popwind.clone();
        let popwind3 = popwind.clone(); 
            let gnu: &str = "Knapsack Password Manager \n Version 1.0\nWritten by Kevin Hansen \n Updated: 10/1/2024\n\nKnapsack password manager was designed to create, store, and organize unique \nand random passwords. Dislike coming up with strong passwords? The generate \nbutton makes it easy for you. Never lose or forget your passwords again.\nConvenient one click copy feature on passwords and usernames. Need to change \na password, simply click on generate to create a new password then \nclick the update button in line with the corresponding password.\n
            GNU Public Licence\n\nThis program is free software; you can redistribute it and/or modify it under \nthe terms of the GNU General Public License as published by the Free Software \nFoundation; either version 3 of the License, or at your option any later version. This \nprogram is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; \nwithout even the implied warranty of MERCHANTABILITY or FITNESS FOR A\n PARTICULAR PURPOSE. See the GNU General Public License for more details. You \nshould have received a copy of the GNU General Public License along with this \nprogram. If not, see \nhttps://www.gnu.org/licenses";
            //dialog::message(400, 350, gnu);
            let mut abt = Flex::new(0, 0, 700, 550, ""); //600
	    abt.set_type(FlexType::Column);
	    abt.set_margins(0, 5, 0, 5);
	    abt.set_frame(FrameType::FlatBox);
	    abt.set_pad(10);
	    abt.set_color(Color::Black);
	    
            let mut fr = Frame::default().with_size(50, 0).with_label(gnu);
	    fr.set_label_color(Color::White);
	    fr.set_label_size(16);
	    //popwind.add(&fr);
	    abt.add(&fr);
	    //abt.fixed(&fr, 600);
	    
	    let mut cl = Button::default().with_size(20, 0).with_label("Close");
	    cl.set_label_color(Color::White);
	    cl.set_label_size(16);
	    cl.set_color(Color::Blue);
	    cl.set_frame(FrameType::RShadowBox);
	    cl.set_callback(move |_| popwind.clone().hide());
	    //popwind2.clone().add(&cl);
	    abt.fixed(&cl, 40);
	    popwind2.clone().add(&abt);
            popwind3.clone().show();
    }
   
    // Method to create a random 12 character string made up of 3 numbers, 3 lower case and 3 upper case letters, and 3 special characters
    fn generate() -> String {
        let symbols: Vec<&str> = vec!["!", "@", "#", "$", "%", "&", "*", "?"];
	let numbers: Vec<&str> = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"];
	let big_alph: Vec<&str> = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
	let small_al: Vec<&str> = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
	let mut sample1: Vec<_> = symbols.choose_multiple(&mut rand::thread_rng(), 3).collect();
	let mut sample2: Vec<_> = numbers.choose_multiple(&mut rand::thread_rng(), 3).collect();
	let mut sample3: Vec<_> = big_alph.choose_multiple(&mut rand::thread_rng(), 3).collect();
	let mut sample4: Vec<_> = small_al.choose_multiple(&mut rand::thread_rng(), 3).collect();
	sample1.append(&mut sample2);
	sample1.append(&mut sample3);
	sample1.append(&mut sample4);
	let sample5: Vec<_> = sample1.choose_multiple(&mut rand::thread_rng(), 12).collect();
	let mut mystring: String = "".to_string();
	let mut i = 0;
	while i < 12 {
		mystring = mystring + sample5.get(i).expect("REASON");
		i += 1;
	}

	return mystring;
    }
    
    // Clear all strings from the input filds
    fn clearentries(e: &mut Input, u: &mut Input, s: &mut Input) {
        e.set_value("");
        u.set_value("");
        s.set_value("");
    }
    
    // save the new password to the database and show it in the input field if conditions are correct 
    fn saveme(pw: &mut Input, uname: &mut Input, stname: &mut Input, ent: Input, wind: Window, allpack: Scroll) {
        if pw.value().to_string() == "password".to_string() {
		dialog::message_default("password can't be password");
	}
	
        else if pw.value() != "" && uname.value() != "" && stname.value() != "" {
		let newstring = "INSERT INTO password VALUES ('".to_owned() + pw.value().as_str() + "', '" + uname.value().as_str() + "', '" + stname.value().as_str() + "');";
		let conn = Connection::open("passwords.db");
		let _ = conn.expect("REASON").execute(newstring);
	}
	else if pw.value() == "" && uname.value() == "" && stname.value() != "" {
		dialog::message_default("password and user name cant be empty");
    	}
    	
	else if pw.value() == "" && uname.value() != "" && stname.value() == "" {
		dialog::message_default("passowrd and site name can't be empty");
	}
        else if pw.value() != "" && uname.value() == "" && stname.value() == "" {
		dialog::message_default("user name and site name can't be empty");
	}
        else if pw.value() != "" && uname.value() != "" && stname.value() == "" {
		dialog::message_default("site name can't be empty");
	}
	else if pw.value() != "" && uname.value() == "" && stname.value() != "" {
		dialog::message_default("user name cant be empty");
	}
	else if pw.value() == "" && uname.value() != "" && stname.value() != "" {
		dialog::message_default("password can't be empty");
	}
	
	else {
	        dialog::message_default("All fields must be filled in");
	}	
	makerows(ent.clone(), wind.clone(), allpack.clone(), "save");
    }
    /*
    // method to remove the current database
    fn removedatabase(mut allpack: Scroll) {
        allpack.clear();
        app::redraw();
    }*/
    
    // method to create the scrollable rows of passwords, usernames, and sites
    fn makerows(e5: Input, wind: Window, mut allpack: Scroll, which: &str) {
	let mut mainflex = Pack::new(0, 110, 730, 400, ""); //684
	mainflex.set_type(PackType::Vertical);
	
	// variables to store table values
	let mut pwds = Vec::new();
	let mut unames = Vec::new();
	let mut snames = Vec::new();
	
	// read info from the passwords table and store its values in variables
	(pwds, unames, snames) = read_table();

	let mut i = 0;
	let mut x = 115;
	while i < unames.len() {
	    let p: &str = &pwds.get(i).unwrap().replace("@", "@@").replace("&amp", "@@").replace("&amp;", "&&").replace("&", "&&");
	    
	    let mut hpack = Flex::new(0, x, 700, 40, ""); //600
	    hpack.set_type(FlexType::Row);
	    hpack.set_margins(0, 5, 0, 5);
	    hpack.set_frame(FrameType::FlatBox);
	    hpack.set_pad(10);

	    let hh = hpack.clone();
	    
	    let mut ff = Button::default().with_size(50, 0).with_label(&p);
	    ff.set_frame(FrameType::FlatBox);
	    ff.set_label_color(Color::White);
	    if i%2 == 0 {
	      ff.set_color(Color::from_rgb(128,126,120));
	    }
	    else {
	      ff.set_color(Color::from_rgb(54,69,79));
	    }
	  
	    ff.set_label_size(18);
	    let f1 = ff.clone();
	    let f2 = ff.clone();
	    ff.clone().set_callback(move |_| selcopy(ff.label().as_str()).expect("Reason"));

	    hpack.add(&f1);

	    if i%2 == 0 {
	      hpack.set_color(Color::from_rgb(128,126,120));
	    }
	    else {
	      hpack.set_color(Color::from_rgb(54,69,79)); //54,69,79
	    }

	    let mut uu = Frame::default().with_size(50, 0).with_label(unames.get(i).unwrap());
	    uu.set_label_color(Color::White);
	    uu.set_label_size(16);
	    hpack.add(&uu);
	    
	    let mut ss = Frame::default().with_size(50, 0).with_label(snames.get(i).unwrap());
	    ss.set_label_color(Color::White);
	    ss.set_label_size(16);
	    hpack.add(&ss);
	    
	    let mut upd = Button::default().with_size(60, 40).with_label("Update");
	    upd.set_color(Color::from_rgb(136,139,141));
	    upd.set_label_color(Color::White);
	    let val = e5.clone();
	    upd.set_callback(move |_| updateme(val.clone(), f2.clone()));
	    upd.set_frame(FrameType::RShadowBox);
	    hpack.fixed(&upd, 80);
	    
	    let mut delet = Button::default().with_size(60, 40).with_label("Delete");
	    delet.set_color(Color::from_rgb(136,139,141));
	    delet.set_label_color(Color::White);
	    let delstring = "DELETE FROM password WHERE pw = '".to_owned() + &p + "';";
	    delet.set_callback(move |_| deleteme(&delstring, hh.clone()));
	    delet.set_frame(FrameType::RShadowBox);
	    hpack.fixed(&delet, 80);
	    
	    hpack.end();
	    mainflex.add(&hpack);

	    x+=40; //25
	    i+=1;
	}
	
	mainflex.end();
	mainflex.resizable(&wind);
	
	allpack.add(&mainflex);
	
	allpack.resizable(&wind);
        allpack.show();
        
        // if content has changed then refresh the window
        if which != "firstrun" {
            app::redraw();
        }	
    }
   
    // method to remove a row that had the info deleted
    fn deleteme(pwd: &str, mut hp: Flex) {
        hp.hide();
        let mypwd = &pwd.replace("&&", "&").replace("@@", "@");
        let conn = Connection::open("passwords.db");
	let _ = conn.expect("REASON").execute(&mypwd);
    }
    
    // Change the password
    fn updateme(pwd: Input, ff: Button) { 
        if pwd.value() == "password" {
            dialog::message_default("password can't be password");
        }
        else if pwd.value() == "" {
            dialog::message_default("password can't be empty");
        }
        else {
            let hh = ff.clone();
	    let newstring = "UPDATE password SET pw = '".to_owned() + pwd.value().as_str() + "' WHERE pw = '" + hh.label().as_str() + "';";
	    let conn = Connection::open("passwords.db");
	    let _ = conn.expect("REASON").execute(newstring);
	    let gg = ff.clone();
            gg.with_label(pwd.value().as_str());
       }
    }
   
    // Generate a new random password button
    let mut gene = Button::new(220, 40, 150, 30, "Generate");
    gene.set_color(Color::from_rgb(136,139,141));
    gene.set_label_color(Color::White);
    gene.set_frame(FrameType::RShadowBox);
    
    // Button to clear all input fields
    let mut cl = Button::new(380, 40, 150, 30, "Clear Entries");
    cl.set_color(Color::from_rgb(136,139,141));
    cl.set_label_color(Color::White);
    cl.set_frame(FrameType::RShadowBox);
    
    // Input for username
    let mut uname = Input::new(5, 75, 200, 30, "");
    uname.set_value("username");
    let entry2 = uname.clone();
    
    // Input for the site
    let mut sname = Input::new(220, 75, 200, 30, "");
    sname.set_value("site");
    let entry3 = sname.clone();
    
    // button to save all the new input data to the database
    let mut savebut = Button::new(440, 75, 90, 30, "Save");
    savebut.set_color(Color::from_rgb(136,139,141));
    savebut.set_label_color(Color::White);
    savebut.set_frame(FrameType::RShadowBox);
    
    // place knapsack image
    let mut png = PngImage::load("knapsack.png").unwrap();
    png.scale(85, 45, false, true); //55
    let mut ss = Frame::new(550, 44, 90, 60, "");
    ss.set_image(Some(png));
    
    // Label for "Password Manager"
    let mut pm = Frame::new(650, 45, 30, 30, "");
    pm.set_label("Password \nManager");
    pm.set_label_color(Color::White);
    pm.set_label_size(10);
    
    let wind2 = wind.clone();
    
    // Scrollable area
    let mut allpack = Scroll::new(0, 110, 750, 475, ""); //700
    allpack.set_color(Color::from_rgb(128,126,120));

    // Clones for the scrollable area    
    let myallpack = allpack.clone();
    let myallpack2 = allpack.clone();
    //let myallpack3 = allpack.clone();
    let myallpack4 = allpack.clone();
    
    // Trigger the "makerows" method to show all of the database data in rows
    makerows(entry5, wind2.clone(), myallpack.clone(), "firstrun");
    
    allpack.end();
    allpack.resizable(&wind);
    allpack.show();
    
    // menu item callbacks -- Import database and Delete database
    if let Some(mut item) = menu.find_item("&File/Import Backup\t") {
	let wind3 = wind.clone();
	item.set_callback(move |_| importbu(entry6.clone(), wind3.clone(), allpack.clone()));
    }
    if let Some(mut item) = menu.find_item("&File/Delete Database\t") {
        item.set_callback(move |_| removedb(myallpack4.clone()));
    }
    let mut popwind = Window::new(27, 10, 700, 550, "About"); // 720, 600
    popwind.set_color(Color::Black);
    popwind.end();
    popwind.hide();
    let popwind2 = popwind.clone();
    
    if let Some(mut item) = menu.find_item("&Help/About\t") {
            item.set_callback(move |_| aboutme(popwind2.clone()));
        }
    
    wind.end();
    wind.show();
    
    // callbacks for generate, clear inputs, and save button
    gene.set_callback(move |_| pwlab.clone().set_value(&generate().to_string()));
    cl.set_callback(move |_| clearentries(&mut entry.clone(), &mut uname.clone(), &mut sname.clone()));
    savebut.set_callback(move |_| saveme(&mut entry1.clone(), &mut entry2.clone(), &mut entry3.clone(), entry7.clone(), wind3.clone(), myallpack2.clone()));
    
    app.run().unwrap();
    
}
