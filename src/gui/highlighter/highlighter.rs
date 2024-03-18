use crate::gui::highlighter::syntax::{CONTROL_FLOW_COMMANDS, DEFINITION_COMMANDS, OTHER_COMMANDS};
use eframe::egui::text::LayoutJob;
use eframe::egui::util::cache::{ComputerMut, FrameCache};
use eframe::egui::*;

#[derive(Default)]
pub struct Highlighter {}

impl Highlighter {
    const FONT: FontId = FontId::monospace(16.0);
    const DEFINITION_COLOR: Color32 = Color32::from_rgb(86, 156, 214);
    const CONTROL_FLOW_COLOR: Color32 = Color32::from_rgb(197, 134, 192);
    const COMMAND_COLOR: Color32 = Color32::from_rgb(220, 220, 170);
    const VARIABLE_COLOR: Color32 = Color32::from_rgb(156, 220, 254);
    const WORD_COLOR: Color32 = Color32::from_rgb(206, 145, 120);
    const NUMBER_COLOR: Color32 = Color32::from_rgb(181, 206, 168);

    pub fn new() -> Self {
        Highlighter {}
    }

    pub fn highlight(&self, ctx: &Context, text: &str, wrap_width: u32) -> LayoutJob {
        ctx.memory_mut(|memory: &mut Memory| {
            memory
                .caches
                .cache::<HighlighterCache>()
                .get((text, wrap_width))
        })
    }

    fn compute_highlights(&self, text: &str, wrap_width: u32) -> LayoutJob {
        let mut job =
            LayoutJob::simple(String::new(), Self::FONT, Color32::WHITE, wrap_width as f32);
        let mut block = CodeBlock::new(text);
        let mut element = String::new();
        let mut allow_whitespace = false;
        while block.current_char() != '\0' {
            while block.current_char().is_whitespace() && block.current_char() != '\0' {
                element.push(block.current_char());
                block.next();
            }
            if !element.is_empty() {
                job.append(
                    &element,
                    0.0,
                    TextFormat::simple(Self::FONT, Color32::WHITE),
                );
                element = String::new();
            }
            if block.current_char() == ';' {
                while block.current_char() != '\n' && block.current_char() != '\0' {
                    element.push(block.current_char());
                    block.next();
                }
                job.append(&element, 0.0, TextFormat::simple(Self::FONT, Color32::GRAY));
                element = String::new();
            }
            while block.current_char() == '(' || block.current_char() == ')' {
                job.append(
                    &block.current_char().to_string(),
                    0.0,
                    TextFormat::simple(Self::FONT, Color32::WHITE),
                );
                block.next();
                element = String::new();
            }
            while block.current_char() == '[' || block.current_char() == ']' {
                job.append(
                    &block.current_char().to_string(),
                    0.0,
                    TextFormat::simple(Self::FONT, Color32::WHITE),
                );
                block.next();
                element = String::new();
            }
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
                if DEFINITION_COMMANDS.contains(&element) {
                    job.append(
                        &element,
                        0.0,
                        TextFormat::simple(Self::FONT, Self::DEFINITION_COLOR),
                    );
                } else if CONTROL_FLOW_COMMANDS.contains(&element) {
                    job.append(
                        &element,
                        0.0,
                        TextFormat::simple(Self::FONT, Self::CONTROL_FLOW_COLOR),
                    );
                } else if OTHER_COMMANDS.contains(&element) {
                    job.append(
                        &element,
                        0.0,
                        TextFormat::simple(Self::FONT, Self::COMMAND_COLOR),
                    );
                } else if element.starts_with(':') {
                    job.append(
                        &element,
                        0.0,
                        TextFormat::simple(Self::FONT, Self::VARIABLE_COLOR),
                    );
                } else if element.starts_with('"') {
                    job.append(
                        &element,
                        0.0,
                        TextFormat::simple(Self::FONT, Self::WORD_COLOR),
                    );
                } else if element.chars().nth(0).unwrap().is_numeric() {
                    job.append(
                        &element,
                        0.0,
                        TextFormat::simple(Self::FONT, Self::NUMBER_COLOR),
                    );
                } else {
                    job.append(
                        &element,
                        0.0,
                        TextFormat::simple(Self::FONT, Color32::WHITE),
                    );
                }
                element = String::new();
            }
        }
        job
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
