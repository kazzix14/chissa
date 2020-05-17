const DATA_LENGTH: usize = 128;
const NUM_COMMANDS: usize = 26;

pub struct Interpreter {
    source: String,
    instruction_pointer: usize,
    data_pointer: usize,
    mark: Option<usize>,
    data: [i32; DATA_LENGTH],
    copied_data: i32,
    last: Operations,
    comment_depth: u32,
}

impl Iterator for Interpreter {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        use Operations::*;

        let src: &[u8] = self.source.as_ref();
        let len = src.len();

        if len <= self.instruction_pointer {
            return None;
        }

        let op = src[self.instruction_pointer];

        match op.into() {
            IncrementPointer if self.comment_depth == 0 => self.increment_pointer(),
            DecrementPointer if self.comment_depth == 0 => self.decrement_pointer(),
            PlaceMark if self.comment_depth == 0 => self.place_mark(),
            GotoMark if self.comment_depth == 0 => self.goto_mark(),
            SkipIfZero if self.comment_depth == 0 => self.skip_if_zero(),
            Input if self.comment_depth == 0 => self.input(),
            Output if self.comment_depth == 0 => self.output(),
            IncrementValue if self.comment_depth == 0 => self.increment_value(),
            DecrementValue if self.comment_depth == 0 => self.decrement_value(),
            IncrementValue10 if self.comment_depth == 0 => self.increment_value_10(),
            DecrementValue10 if self.comment_depth == 0 => self.decrement_value_10(),
            Repeat(count) if self.comment_depth == 0 => self.repeat(count),
            Replace if self.comment_depth == 0 => self.replace(),
            SetZero if self.comment_depth == 0 => self.set_zero(),
            CopyValue if self.comment_depth == 0 => self.copy_value(),
            PasteValue if self.comment_depth == 0 => self.paste_value(),
            BeginComment => self.begin_comment(),
            EndComment => self.end_comment(),
            Nop => (),
            _ => (),
        }

        if (Operations::from(op) != Nop) && (self.comment_depth == 0) {
            self.last = op.into();
        }

        self.instruction_pointer += 1;
        Some(op as char)
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            source: "".to_string(),
            instruction_pointer: 0,
            data_pointer: 0,
            mark: None,
            data: [0; DATA_LENGTH],
            copied_data: 0,
            last: Operations::Nop,
            comment_depth: 0,
        }
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn source_mut(&mut self) -> &mut str {
        &mut self.source
    }

    pub fn set_source(&mut self, source: String) {
        self.source = source;
    }

    fn increment_pointer(&mut self) {
        self.data_pointer += 1;
    }

    fn decrement_pointer(&mut self) {
        self.data_pointer -= 1;
    }

    fn place_mark(&mut self) {
        self.mark = Some(self.instruction_pointer);
    }

    fn goto_mark(&mut self) {
        self.instruction_pointer = self.mark.expect("mark not set");
    }

    fn skip_if_zero(&mut self) {
        if self.data[self.data_pointer] == 0 {
            self.instruction_pointer += 1;
        }
    }

    fn input(&mut self) {
        unimplemented!();
    }

    fn output(&self) {
        use std::char;
        let c = self.data[self.data_pointer];
        print!("{}", char::from_u32(c as u32).unwrap());
    }

    fn increment_value(&mut self) {
        self.increment_value_by(1);
    }

    fn decrement_value(&mut self) {
        self.increment_value_by(-1);
    }

    fn increment_value_10(&mut self) {
        self.increment_value_by(10);
    }

    fn decrement_value_10(&mut self) {
        self.increment_value_by(-10);
    }

    fn increment_value_by(&mut self, n: i32) {
        self.data[self.data_pointer] += n;
    }

    fn repeat(&mut self, count: usize) {
        use Operations::*;

        for _ in 0..count {
            match &self.last {
                IncrementPointer => self.increment_pointer(),
                DecrementPointer => self.decrement_pointer(),
                PlaceMark => self.place_mark(),
                GotoMark => self.goto_mark(),
                SkipIfZero => (),
                Input => self.input(),
                Output => self.output(),
                IncrementValue => self.increment_value(),
                DecrementValue => self.decrement_value(),
                IncrementValue10 => self.increment_value_10(),
                DecrementValue10 => self.decrement_value_10(),
                Repeat(_) => panic!("do not place repeat command repeatingly"),
                Replace => self.replace(),
                SetZero => self.set_zero(),
                CopyValue => self.copy_value(),
                PasteValue => self.paste_value(),
                BeginComment => self.begin_comment(),
                EndComment => self.end_comment(),
                Nop => (),
            }
        }
    }

    fn replace(&mut self) {
        let destination = self.data_pointer;
        let key = self.data[self.data_pointer];
        let key = key % NUM_COMMANDS as i32;
        let command = match key {
            0 => '>',
            1 => '<',
            2 => '[',
            3 => ']',
            4 => 'j',
            5 => 'i',
            6 => 'o',
            7 => '+',
            8 => '-',
            9 => '^',
            10 => 'v',
            11 => '1',
            12 => '2',
            13 => '3',
            14 => '4',
            15 => '5',
            16 => '6',
            17 => '7',
            18 => '8',
            19 => '9',
            20 => 'r',
            21 => '0',
            22 => 'c',
            23 => 'p',
            24 => '(',
            25 => ')',
            _ => unreachable!(),
        };

        unsafe {
            if self.source().len() <= destination {
                panic!("out of bounds");
            }
            self.source.as_mut_vec()[destination] = command as u8;
        }
    }
    fn set_zero(&mut self) {
        self.data[self.data_pointer] = 0;
    }

    fn copy_value(&mut self) {
        self.copied_data = self.data[self.data_pointer];
    }

    fn paste_value(&mut self) {
        self.data[self.data_pointer] = self.copied_data;
    }

    fn begin_comment(&mut self) {
        self.comment_depth += 1;
    }

    fn end_comment(&mut self) {
        self.comment_depth -= 1;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operations {
    IncrementPointer,
    DecrementPointer,
    PlaceMark,
    GotoMark,
    SkipIfZero,
    Input,
    Output,
    IncrementValue,
    DecrementValue,
    IncrementValue10,
    DecrementValue10,
    Repeat(usize),
    Replace,
    SetZero,
    CopyValue,
    PasteValue,
    BeginComment,
    EndComment,
    Nop,
}

impl<T> From<T> for Operations
where
    T: Into<char>,
{
    fn from(c: T) -> Operations {
        use Operations::*;
        match c.into() {
            '>' => IncrementPointer,
            '<' => DecrementPointer,
            '[' => PlaceMark,
            ']' => GotoMark,
            'j' => SkipIfZero,
            'i' => Input,
            'o' => Output,
            '+' => IncrementValue,
            '-' => DecrementValue,
            '^' => IncrementValue10,
            'v' => DecrementValue10,
            '1' => Repeat(1),
            '2' => Repeat(2),
            '3' => Repeat(3),
            '4' => Repeat(4),
            '5' => Repeat(5),
            '6' => Repeat(6),
            '7' => Repeat(7),
            '8' => Repeat(8),
            '9' => Repeat(9),
            'r' => Replace,
            '0' => SetZero,
            'c' => CopyValue,
            'p' => PasteValue,
            '(' => BeginComment,
            ')' => EndComment,
            _ => Nop,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
