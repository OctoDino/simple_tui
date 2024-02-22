use colored::Colorize;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::{execute, cursor::MoveUp};
use std::io::{self, Write, Read}; // <--- bring flush() into scope

pub fn user_input(context: &str) -> String { //Getting the users Input
    print!("{}", context);
    io::stdout().flush().unwrap();
    let mut val = String::new();

    io::stdin().read_line(&mut val)
        .expect("Error getting input");
    //println!("You entered {}", val);
    return val
}

pub fn list_selection(list: Box<[&str]>) -> i32 {

    //A simple navigation menu, where you navigate with the arrow keys
    let selected_bullet: i32 = (list.len() - 1) as i32;

    return rerender_list(list, selected_bullet, true);


    fn rerender_list(list: Box<[&str]>, mut selected_bullet: i32, first_time: bool) -> i32{
        //Debug purposes println!("{}", selected_bullet);
        let mut move_bullet: i32;
        //Clear previous list

        if first_time == false {
            print!("\x1B[{}F\x1B[0J", list.len() + 3);
            io::stdout().flush().ok();
        }else {
            ()
        }

        //render new List
        for n in 0..=list.len() - 1 {
            if selected_bullet == n as i32{
                println!(" ◉ {}", list[n].bold());
            }
            else {
                println!(" ○ {}", list[n])
            }
        }

        //Wait for keyboard input
        move_bullet = get_arrow_key();
        match move_bullet {
            0 => if selected_bullet == 0 {
                rerender_list(list, selected_bullet, false)
            }
            else { rerender_list(list, selected_bullet - 1, false) }
            1 => if selected_bullet == (list.len() - 1) as i32 {
                rerender_list(list, selected_bullet, false)
            }
            else { rerender_list(list, selected_bullet + 1, false) }
            2 => if selected_bullet == 0 {
                rerender_list(list, selected_bullet, false)
            }
            else { rerender_list(list, selected_bullet - 1, false) }
            3 => if selected_bullet == (list.len() - 1) as i32 {
                rerender_list(list, selected_bullet, false)
            }
            else { rerender_list(list, selected_bullet + 1, false) }
            4 => return selected_bullet,
            62 => return -1,
            _ =>  return -2
        }
    }
}

fn get_arrow_key() -> i32 {
    println!("\n{}", "💡 Hint: Press an arrow key (or 'q' to quit)");
    loop {
        io::stdout().flush().expect("Failed to flush stdout");

        match event::read().expect("Failed to read event") {
            Event::Key(KeyEvent { code, kind, .. }) =>
                if kind == KeyEventKind::Press { //Unix: "KeyboardEnhancementFlags::REPORT_EVENT_TYPES" has been enabled with "PushKeyboardEnhancementFlags" (https://docs.rs/crossterm/latest/crossterm/event/struct.KeyEvent.html)
                    match code {
                        KeyCode::Up => return 0, //Up arrow pressed
                        KeyCode::Down => return 1, //Down arrow pressed
                        KeyCode::Left => return 2, //Left arrow pressed
                        KeyCode::Right => return 3, //Right arrow pressed
                        KeyCode::Enter => return 4, //Enter key is pressed
                        KeyCode::Char('q') => {
                            return 62 //q key is pressed => Quit
                        }
                        _ => (), //other key pressed
                    }
                }
            _ => (), //other event occurred
        }
    }
}