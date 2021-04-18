mod process;
mod utils;
mod system;

use std::io::{stdout, Write, Stdout};
use crossterm::QueueableCommand;
use crossterm::terminal::{Clear, ClearType, SetSize};
use crossterm::cursor::{Hide, MoveTo};
use crossterm::style::{PrintStyledContent, Colorize, Print};
use crate::utils::{left_pad, right_pad, center, make_bars};
use std::thread;
use std::time::Duration;
use crate::process::get_process_list::get_process_list;
use winapi::um::winbase::GetComputerNameA;
use winapi::um::memoryapi::MapViewOfFile;
use crate::system::get_system_stats::get_system_stats;
use crate::system::get_cpu_and_mem_usage::{get_cpu_and_mem_usage};
use crate::system::get_cpu_name::get_cpu_name;
use sysinfo::{ProcessExt, System, SystemExt};
use crossterm::event::{poll, read, Event, KeyEvent, KeyCode};
use std::ffi::OsString;
use systemstat::{CPULoad, Memory};
use crate::process::process::Process;

fn draw_blue_header(out: &mut Stdout, host: &OsString, term_width: usize) {
    // write the blue header
    out.queue(MoveTo(0, 0));
    let mut h = format!("RTop on {:?}", host);
    h = format!("{}", center(h, term_width as usize));
    out.queue(Print(h.white().on_blue()));
}

fn draw_cpu_meter(out: &mut Stdout, cpu_mem: &(CPULoad, Memory)) {
    // write CPU meter
    out.queue(MoveTo(3, 2));
    out.queue(Print("CPU".cyan()));
    out.queue(Print("[".white()));
    let cpu_perc = cpu_mem.0.user + cpu_mem.0.system;
    let cpu_bars = make_bars(cpu_perc as f64);
    out.queue(Print(cpu_bars.green()));
    out.queue(MoveTo(26, 2));
    out.queue(Print(left_pad(format!("{:.1}%", cpu_perc * 100.0), 5).dark_grey()));
    out.queue(Print("]".white()));
}

fn draw_mem_meter(out: &mut Stdout, cpu_mem: &(CPULoad, Memory)) {
    // write mem meter
    out.queue(MoveTo(3, 3));
    out.queue(Print("Mem".cyan()));
    out.queue(Print("[".white()));
    let total = cpu_mem.1.total.as_u64();
    let free = cpu_mem.1.free.as_u64();
    let perc = (total - free) as f64 / total as f64;
    let mem_bars = make_bars(perc);
    out.queue(Print(mem_bars.green()));
    out.queue(MoveTo(26, 3));
    out.queue(Print(left_pad(format!("{:.1}%", perc * 100.0), 5).dark_grey()));
    out.queue(Print("]".white()));
}

fn draw_cpu_name(out: &mut Stdout, cpu_name: String) {
    out.queue(MoveTo(35, 2));
    out.queue(Print("Name: ".cyan()));
    out.queue(Print(cpu_name.grey()));
}

fn draw_tasks(out: &mut Stdout, cpu_name_len: usize) {
    out.queue(MoveTo(35 + cpu_name_len as u16 + 8, 2));
    out.queue(Print("Tasks: ".cyan()));
}

fn draw_green_bar(out: &mut Stdout, term_width: usize) {
    out.queue(MoveTo(0, 5));
    let green_bar = center(" a  ".to_string(), term_width - 1 as usize);
    out.queue(Print(green_bar.dark_green().on_dark_green()));
    out.queue(MoveTo(0, 5));
    out.queue(Print(left_pad("ID".to_string(), 10).black().on_dark_green()));
    out.queue(Print(left_pad("USER".to_string(), 10).black().on_dark_green()));
    out.queue(Print(left_pad("PRI".to_string(), 5).black().on_dark_green()));
    out.queue(Print(left_pad("CPU%  ".to_string(), 8).black().on_dark_green()));
}

enum Sort {
    Cpu,
    Mem,
}

fn draw_process_list(out: &mut Stdout, processes: &Vec<Process>, process_list_start_row: u16,
                     term_width: usize, term_height: usize, highlighted_row: u16) {
    let mut i = process_list_start_row;
    for proc in processes {
        let cpu_usage = format!("{:.1}%  ", proc.cpu_usage);
        out.queue(MoveTo(0, i));
        if highlighted_row + process_list_start_row == i {
            out.queue(Print(
                format!(
                    "{}{}{}{}{}",
                    left_pad(proc.pid.to_string(), 10),
                    left_pad(proc.user.to_string(), 10),
                    left_pad(proc.base_priority.to_string(), 5),
                    left_pad(cpu_usage, 8),
                    right_pad(proc.name.clone(), term_width - 33)
                ).black().on_blue()));
        } else {
            out.queue(Print(
                format!(
                    "{}{}{}{}{}",
                    left_pad(proc.pid.to_string(), 10),
                    left_pad(proc.user.to_string(), 10),
                    left_pad(proc.base_priority.to_string(), 5),
                    left_pad(cpu_usage, 8),
                    right_pad(proc.name.clone(), term_width - 33)
                ).grey()));
        }
        if i == (term_height - 2) as u16 {
            break;
        }
        i += 1;
    }
}

fn draw_bottom_bar(out: &mut Stdout, term_height: usize) {
    out.queue(MoveTo(0, term_height as u16));
    out.queue(Print("F1"));
    out.queue(MoveTo(2, term_height as u16));
    out.queue(Print("Help  ".black().on_blue()));

    out.queue(MoveTo(8, term_height as u16));
    out.queue(Print("F2"));
    out.queue(MoveTo(10, term_height as u16));
    out.queue(Print("Setup ".black().on_blue()));

    out.queue(MoveTo(16, term_height as u16));
    out.queue(Print("F3"));
    out.queue(MoveTo(18, term_height as u16));
    out.queue(Print("Search".black().on_blue()));

    out.queue(MoveTo(24, term_height as u16));
    out.queue(Print("F4"));
    out.queue(MoveTo(26, term_height as u16));
    out.queue(Print("Filter".black().on_blue()));
}

fn main() {
    let mut out = stdout();
    let mut term_width: usize = 300;
    let mut term_height: usize = 100;

    let host = gethostname::gethostname();
    const PROCESS_LIST_START_ROW: u16 = 6;
    let mut system = System::new();
    let mut highlighted_row = 0;
    let cpu_name = get_cpu_name();
    let cpu_name_len = cpu_name.len().clone();
    let mut sort_by = Sort::Cpu;
    let mut sort_menu_open = false;

    loop {
        system.refresh_processes();
        let mut sys_proc_info = system.get_processes();

        if let Some((w, h)) = term_size::dimensions() {
            if term_width != w || term_height != h {
                out.queue(Clear(ClearType::All));
            }
            term_width = w;
            term_height = h;
            out.queue(SetSize(term_width as u16, term_height as u16));
        }

        // clear output
        out.queue(Hide);

        draw_blue_header(&mut out, &host, term_width);
        let cpu_mem = get_cpu_and_mem_usage();
        draw_cpu_meter(&mut out, &cpu_mem);
        draw_mem_meter(&mut out, &cpu_mem);
        draw_cpu_name(&mut out, cpu_name.clone());
        draw_tasks(&mut out, cpu_name_len);
        draw_green_bar(&mut out, term_width);


        // write the process list
        let mut processes = get_process_list(sys_proc_info);
        processes.sort_by(|a, b| {
            match sort_by {
                Sort::Cpu => {
                    b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap()
                }
                Sort::Mem => {
                    b.working_set_size.partial_cmp(&a.working_set_size).unwrap()
                }
            }
        });
        draw_process_list(&mut out, &processes, PROCESS_LIST_START_ROW, term_width,
                          term_height, highlighted_row);
        draw_bottom_bar(&mut out, term_height);



        match read() {
            Ok(event) => {
                match event {
                    Event::Key(event) => {
                        match event {
                            KeyEvent { code, modifiers } => {
                                match code {
                                    KeyCode::Up => {
                                        if highlighted_row > 0 {
                                            highlighted_row -= 1;
                                        }
                                    }
                                    KeyCode::Down => {
                                        if highlighted_row < term_height as u16 {
                                            highlighted_row += 1;
                                        }
                                    }
                                    KeyCode::Enter => {}
                                    KeyCode::F(which) => {
                                        if which == 6 {
                                            sort_menu_open = !sort_menu_open;
                                        }
                                    }
                                    KeyCode::Char(char) => {
                                        if char == 'q' {
                                            break;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        out.flush();

        thread::sleep(Duration::from_millis(500));
    }
}