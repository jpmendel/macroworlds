use phf::{phf_map, phf_set};

pub const DEFINITION_COMMANDS: phf::Set<&'static str> = phf_set!(
    "clearname",
    "clearnames",
    "end",
    "let",
    "local",
    "make",
    "newtext",
    "newturtle",
    "remove",
    "to",
    "turtlesown",
);

pub const CONTROL_FLOW_COMMANDS: phf::Set<&'static str> = phf_set!(
    "again",
    "and",
    "carefully",
    "dolist",
    "dotimes",
    "forever",
    "if",
    "ifelse",
    "not",
    "op",
    "or",
    "output",
    "repeat",
    "run",
    "wait",
);

pub const OTHER_COMMANDS: phf::Set<&'static str> = phf_set!(
    "abs",
    "again",
    "announce",
    "arctan",
    "ascii",
    "ask",
    "back",
    "bf",
    "bg",
    "bk",
    "bl",
    "butfirst",
    "butlast",
    "cc",
    "cg",
    "char",
    "chdir",
    "clean",
    "cleartext",
    "clicked?",
    "color",
    "colorunder",
    "cos",
    "count",
    "ct",
    "currentdir",
    "difference",
    "directories",
    "distance",
    "empty?",
    "equal?",
    "errormessage",
    "exp",
    "fd",
    "files",
    "first",
    "fontsize",
    "forward",
    "fput",
    "freeze",
    "greater?",
    "heading",
    "home",
    "ht",
    "int",
    "item",
    "key?",
    "keydown?",
    "last",
    "left",
    "less?",
    "list",
    "list?",
    "ln",
    "loadpict",
    "loadshape",
    "loadtext",
    "log",
    "lput",
    "lt",
    "member?",
    "minus",
    "number?",
    "on?",
    "pd",
    "pensize",
    "pi",
    "pick",
    "pictlist",
    "placepict",
    "pos",
    "power",
    "pr",
    "print",
    "procedures",
    "product",
    "projectsize",
    "pu",
    "quotient",
    "random",
    "readchar",
    "readclick",
    "remainder",
    "remove",
    "resett",
    "right",
    "round",
    "rt",
    "run",
    "se",
    "sentence",
    "setbg",
    "setc",
    "setcolor",
    "setfontsize",
    "seth",
    "setheading",
    "setpensize",
    "setpos",
    "setprojectsize",
    "setsh",
    "setshape",
    "setsize",
    "setstyle",
    "setx",
    "sety",
    "shape",
    "show",
    "sin",
    "size",
    "sqrt",
    "st",
    "sum",
    "talkto",
    "tan",
    "text",
    "textlist",
    "timer",
    "touching?",
    "towards",
    "tto",
    "unfreeze",
    "visible?",
    "who",
    "word",
    "word?",
    "xcor",
    "ycor",
);

pub const VAR_DEF_COUNT: phf::Map<&'static str, u8> = phf_map!(
    "clearname" => 1,
    "local" => 1,
    "make" => 1,
    "turtlesown" => 1,
);

pub const OBJ_DEF_COUNT: phf::Map<&'static str, u8> = phf_map!(
    "ask" => 1,
    "distance" => 1,
    "newtext" => 1,
    "newturtle" => 1,
    "remove" => 1,
    "talkto" => 1,
    "touching?" => 2,
    "towards" => 1,
    "tto" => 1,
);

pub const WORD_LIST_ARGS: phf::Set<&'static str> = phf_set!("announce", "print", "show");
