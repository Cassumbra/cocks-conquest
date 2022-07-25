use bevy::prelude::*;
use unicode_segmentation::UnicodeSegmentation;

//Plugin
#[derive(Default)]
pub struct LogPlugin;

impl Plugin for LogPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<Log>();
    }
}

// Data
#[derive(Default, Clone, Debug)]
pub struct LogFragment {
    pub text: String,
    pub color: Color,
}
impl LogFragment {
    pub fn new(text: String, color: Color) -> Self {
        Self {text, color}
    }
}

// Resources
#[derive(Default)]
pub struct Log{
    pub lines: Vec<Vec<LogFragment>>
}
impl Log {
    pub fn fragment_string ( string: String, color: Color) -> Vec<LogFragment> {
        let strings: Vec<LogFragment> = string
            .split_inclusive(' ')
            .map(|s| LogFragment{text: s.to_string(), color: color} )
            .collect();
        strings
    }

    pub fn string_to_lines_by_width ( string: String, color: Color, width: usize) -> Vec<Vec<LogFragment>> {
        let no_crlf = string.replace("\r\n", "\n");

        let fragments: Vec<LogFragment> = no_crlf
            .split_inclusive([' ', '\n'])
            .map(|s| LogFragment{text: s.to_string(), color: color} )
            .collect();

        println!("fragments length: {}", fragments.len());

        let mut current_width = 0;
        let mut lines: Vec<Vec<LogFragment>> = Vec::new();
        let mut line: Vec<LogFragment> = Vec::new();
        for fragment in fragments {
            let mut fragment_temp = fragment.clone();

            // Does this need to be separated out like this?
            let mut newline = false;
            if fragment_temp.text.contains(['\n']) {
                fragment_temp.text = fragment_temp.text.trim().to_string();
                newline = true;
            }

            if fragment_temp.text.len() > width {
                //let mut temp_fragment = fragment;

                // TODO: I don't wanna do this shit, man.
                todo!();
            }
            else if fragment_temp.text.len() + 1 + current_width > width {
                lines.push(line);
                line = Vec::new();
                line.push(fragment_temp.clone());
                current_width = fragment_temp.text.len();

                println!("current width after wrap: {}", current_width);

                if newline {
                    println!("newline after wrap");

                    lines.push(line);
                    line = Vec::new();
                    current_width = 0;
                }
            }
            else {
                if newline {
                    line.push(fragment_temp.clone());
                    lines.push(line);
                    line = Vec::new();
                    current_width = fragment_temp.text.len();
                }
                else {
                    line.push(fragment_temp.clone());
                    current_width += fragment_temp.text.len();
                }
            }
        }

        lines.push(line);

        println!("line count: {}", lines.len());

        lines
    }

    /// Cuts a string into lines based on line width
    /*
    pub fn string_to_lines_by_width ( string: String, color: Color, width: usize) -> Vec<Vec<LogFragment>> {
        // Create empty string that we can use whenever we want an empty string. This means we shouldn't have to deal with loads of reallocations.
        let empty_string = String::with_capacity(width);
        
        let mut current_width = 0;
        let mut lines: Vec<Vec<LogFragment>> = Vec::new();
        let mut line: Vec<LogFragment> = Vec::new();
        let mut fragment = LogFragment {text: empty_string, color};

        for c in string.graphemes(true) {


            if c == " " {
                fragment.text.push_str(c);
                current_width += 1;

                line.push(fragment);
                fragment.text = empty_string;
            }
            // TODO: Will this work???
            else if c == "\n" {
                line.push(fragment);
                fragment.text = empty_string;

                lines.push(line);
                current_width = 0;
            }
            else {
                fragment.text.push_str(c);
                current_width += 1;
            }

            if current_width > width {


                if fragment.text.len() > width {
                    line.push(fragment);
                    fragment.text = empty_string;

                    lines.push(line);
                    current_width = 0;
                }
                else {
                    lines.push(line);
                    current_width = 0;
                }
            }


        }

        todo!()
    }
     */
    pub fn log_fragments ( &mut self, fragments: Vec<LogFragment> ) {
        self.lines.push(fragments);
    }

    pub fn log_string ( &mut self, string: String ) {
        self.lines.push(Log::fragment_string(string, Color::rgb(0.8, 0.8, 0.8)));
    }

    pub fn log_string_formatted ( &mut self, string: String, color: Color ) {
        self.lines.push(Log::fragment_string(string, color));
    }
}
