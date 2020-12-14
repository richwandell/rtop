mod process;
mod utils;

use std::io::{stdout, Write};
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

fn get_cpu_perc() -> f64 {
    return 0.4;
}

fn main() {
    let mut out = stdout();
    let mut term_width: usize = 300;
    let mut term_height: usize = 100;

    let host = gethostname::gethostname();

    loop {
        out.queue(Clear(ClearType::All));

        if let Some((w, h)) = term_size::dimensions() {
            term_width = w;
            term_height = h;
            out.queue(SetSize(term_width as u16, term_height as u16));
        }

        // clear output
        out.queue(Hide);

        // write the blue header
        out.queue(MoveTo(0, 0));
        let mut h = format!("RTop on {:?}", host);
        h = format!("{}", center(h, term_width as usize));
        out.queue(Print(h.white().on_blue()));

        // write CPU meter and Tasks stats
        out.queue(MoveTo(3, 2));
        out.queue(Print("CPU".cyan()));
        out.queue(Print("[".white()));

        let cpu = get_cpu_perc();
        let bars = make_bars(cpu);
        out.queue(Print(bars.green()));
        out.queue(MoveTo(26, 2));
        out.queue(Print(format!("{:.1}%", cpu * 100 as f64).grey()));
        out.queue(Print("]".white()));
        out.queue(MoveTo(35, 2));
        out.queue(Print("Tasks: ".cyan()));


        // write the process list
        let mut processes = get_process_list();
        processes.sort_by(|a, b| {
            a.pid.cmp(&b.pid)
        });
        let mut i = 4;
        for proc in processes {
            out.queue(MoveTo(0, i));
            out.queue(Print(
                format!(
                    "{} {} {}",
                    left_pad(proc.pid.to_string(), 10),
                    left_pad(" ".to_string(), 10),
                    right_pad(proc.name, 20)
                ).white()));
            if i > term_height as u16 {
                break;
            }
            i += 1;
        }

        out.flush();

        thread::sleep(Duration::from_millis(500));
    }
}