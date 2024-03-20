use crate::gui::highlighter::syntax::{
    CONTROL_FLOW_COMMANDS, DEFINITION_COMMANDS, OBJ_DEF_COUNT, OTHER_COMMANDS, VAR_DEF_COUNT,
    WORD_LIST_ARGS,
};
use eframe::egui::text::LayoutJob;
use eframe::egui::util::cache::{ComputerMut, FrameCache};
use eframe::egui::*;

#[derive(Default, Debug, Clone)]
pub struct Highlighter {}

impl Highlighter {
    const FONT: FontId = FontId::monospace(16.0);
    const DEFINITION_COLOR: Color32 = Color32::from_rgb(86, 156, 214);
    const CONTROL_FLOW_COLOR: Color32 = Color32::from_rgb(197, 134, 192);
    const COMMAND_COLOR: Color32 = Color32::from_rgb(220, 220, 170);
    const VARIABLE_COLOR: Color32 = Color32::from_rgb(156, 220, 254);
    const OBJECT_COLOR: Color32 = Color32::from_rgb(78, 201, 176);
    const WORD_COLOR: Color32 = Color32::from_rgb(206, 145, 120);
    const NUMBER_COLOR: Color32 = Color32::from_rgb(181, 206, 168);

    pub fn new() -> Self {
        Highlighter {}
    }

    pub fn highlight(&self, ctx: &Context, text: &str, wrap_width: f32) -> LayoutJob {
        ctx.memory_mut(|memory: &mut Memory| {
            memory
                .caches
                .cache::<HighlighterCache>()
                .get((text, wrap_width as u32))
        })
    }

    // TODO: Clean this garbage up.
    fn compute_highlights(&self, text: &str, wrap_width: u32) -> LayoutJob {
        let mut job =
            LayoutJob::simple(String::new(), Self::FONT, Color32::WHITE, wrap_width as f32);
        let mut block = CodeBlock::new(text);
        let mut element = String::new();
        let mut allow_whitespace = false;
        let mut bracket_count: i8 = 0;
        let mut var_def_count: u8 = 0;
        let mut obj_def_count: u8 = 0;
        let mut word_list_count: u8 = 0;
        let mut is_let = false;
        while block.current_char() != '\0' {
            // Whitespace
            while block.current_char().is_whitespace() && block.current_char() != '\0' {
                element.push(block.current_char());
                block.next();
            }
            if !element.is_empty() {
                self.append_job(&mut job, &element, Color32::WHITE);
                element = String::new();
            }
            // Comments
            if block.current_char() == ';' {
                while block.current_char() != '\n' && block.current_char() != '\0' {
                    element.push(block.current_char());
                    block.next();
                }
                self.append_job(&mut job, &element, Color32::GRAY);
                element = String::new();
            }
            // Parenthesis
            while block.current_char() == '(' || block.current_char() == ')' {
                let text = &block.current_char().to_string();
                self.append_job(&mut job, text, Color32::WHITE);
                block.next();
                element = String::new();
            }
            // Lists and Brackets
            while block.current_char() == '[' || block.current_char() == ']' {
                if block.current_char() == '[' {
                    bracket_count += 1;
                } else {
                    bracket_count -= 1;
                }
                if bracket_count == 0 {
                    is_let = false;
                }
                let text = &block.current_char().to_string();
                self.append_job(&mut job, text, Color32::WHITE);
                element = String::new();
                // If we read a '[' and are expecting a word-list, read the whole
                // word-list as one token.
                if block.current_char() == '[' && word_list_count > 0 {
                    block.next();
                    while block.current_char() != ']' && block.current_char() != '\0' {
                        element.push(block.current_char());
                        block.next();
                    }
                    self.append_job(&mut job, &element, Self::WORD_COLOR);
                    let text = &block.current_char().to_string();
                    self.append_job(&mut job, text, Color32::WHITE);
                    element = String::new();
                    // If we read a full list from '[' to ']' we exit the word list.
                    word_list_count = 0;
                }
                block.next();
            }
            // Read a Standard Identifier
            while (!block.current_char().is_whitespace() || allow_whitespace)
                && block.current_char() != ')'
                && block.current_char() != ']'
                && block.current_char() != '\0'
            {
                if block.current_char() == '|' {
                    if element == "\"" {
                        allow_whitespace = true;
                    } else if allow_whitespace {
                        allow_whitespace = false;
                    }
                }
                element.push(block.current_char());
                block.next();
            }
            if !element.is_empty() {
                if DEFINITION_COMMANDS.contains(&element) && !is_let {
                    // Definition Command
                    if let Some(count) = VAR_DEF_COUNT.get(&element) {
                        var_def_count = count + 1;
                    } else if let Some(count) = OBJ_DEF_COUNT.get(&element) {
                        obj_def_count = count + 1;
                    } else if &element == "let" {
                        is_let = true;
                    }
                    self.append_job(&mut job, &element, Self::DEFINITION_COLOR);
                } else if CONTROL_FLOW_COMMANDS.contains(&element) && !is_let {
                    // Control Flow Command
                    self.append_job(&mut job, &element, Self::CONTROL_FLOW_COLOR);
                } else if OTHER_COMMANDS.contains(&element) && !is_let {
                    // Any Other Command
                    if let Some(count) = OBJ_DEF_COUNT.get(&element) {
                        obj_def_count = count + 1;
                    } else if WORD_LIST_ARGS.contains(&element) {
                        word_list_count = 2;
                    }
                    self.append_job(&mut job, &element, Self::COMMAND_COLOR);
                } else if element.starts_with(':') {
                    // Variables
                    self.append_job(&mut job, &element, Self::VARIABLE_COLOR);
                } else if element.starts_with('"') {
                    // Words
                    if var_def_count > 0 {
                        self.append_job(&mut job, &element, Self::VARIABLE_COLOR);
                    } else if obj_def_count > 0 {
                        self.append_job(&mut job, &element, Self::OBJECT_COLOR);
                    } else {
                        self.append_job(&mut job, &element, Self::WORD_COLOR);
                    }
                } else if element
                    .chars()
                    .all(|c| c.is_numeric() || c == '-' || c == '.')
                {
                    // Numbers
                    self.append_job(&mut job, &element, Self::NUMBER_COLOR);
                } else if element.ends_with(',') {
                    // "talkto" Shortcut
                    self.append_job(&mut job, &element, Self::OBJECT_COLOR);
                } else if element.ends_with("'s") {
                    // "ask" Shortcut
                    self.append_job(&mut job, &element, Self::OBJECT_COLOR);
                    // Show variable color for the word following the "ask" shortcut.
                    var_def_count = 2;
                } else if is_let && bracket_count > 0 && bracket_count % 2 != 0 {
                    // In "let" every other item in the list is a variable name.
                    self.append_job(&mut job, &element, Self::VARIABLE_COLOR);
                } else {
                    // All Other Text
                    self.append_job(&mut job, &element, Color32::WHITE);
                }
                // After processing a non-whitespace token, decrement the counts
                // for each kind of context-aware highlighting.
                if var_def_count > 0 {
                    var_def_count -= 1;
                }
                if obj_def_count > 0 {
                    obj_def_count -= 1;
                }
                if word_list_count > 0 {
                    // If a function with word-list args has a normal word as input, this
                    // should automatically skip the word-list context-aware highlighting.
                    word_list_count -= 1;
                }
                element = String::new();
            }
        }
        job
    }

    fn append_job(&self, job: &mut LayoutJob, text: &str, color: Color32) {
        job.append(text, 0.0, TextFormat::simple(Self::FONT, color));
    }
}

type HighlighterCache = FrameCache<LayoutJob, Highlighter>;

impl ComputerMut<(&str, u32), LayoutJob> for Highlighter {
    fn compute(&mut self, (text, wrap_width): (&str, u32)) -> LayoutJob {
        self.compute_highlights(text, wrap_width)
    }
}

struct CodeBlock<'a> {
    text: &'a str,
    position: usize,
}

impl<'a> CodeBlock<'a> {
    fn new(text: &'a str) -> Self {
        CodeBlock { text, position: 0 }
    }

    fn current_char(&self) -> char {
        self.text.chars().nth(self.position).unwrap_or('\0')
    }

    fn next(&mut self) {
        self.position += 1;
    }
}
