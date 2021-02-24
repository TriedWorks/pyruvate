use crate::utils::{delete_icon, edit_icon};
use glucose::DMatrix;
use iced::{button, text_input, Align, Button, Column, Element, Row, Text, TextInput};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IcedMatrixOperation {
    Add,
    Sub,
    Mul,
}

impl std::fmt::Display for IcedMatrixOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IcedMatrixOperation::Add => "Add",
                IcedMatrixOperation::Sub => "Sub",
                IcedMatrixOperation::Mul => "Mul",
            }
        )
    }
}

impl IcedMatrixOperation {
    pub const ALL: [IcedMatrixOperation; 3] = [
        IcedMatrixOperation::Add,
        IcedMatrixOperation::Sub,
        IcedMatrixOperation::Mul,
    ];
}

impl Default for IcedMatrixOperation {
    fn default() -> Self {
        Self::Add
    }
}

#[derive(Debug, Clone)]
pub enum IcedMatrixState {
    Display {
        edit_button: button::State,
    },
    Editing {
        text_input: text_input::State,
        delete_button: button::State,
        finish_button: button::State,
    },
}

impl Default for IcedMatrixState {
    fn default() -> Self {
        Self::Display {
            edit_button: button::State::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct IcedMatrix {
    mat: DMatrix<f64>,
    edit_text: String,
    grid: Vec<String>,
    state: IcedMatrixState,
}

#[derive(Debug, Clone)]
pub enum MatrixMessage {
    EditedValue(String),
    SaveMatrix,
    Edit,
    Finish,
    Delete,
}

impl IcedMatrix {
    pub fn new(size: (usize, usize)) -> Self {
        IcedMatrix {
            mat: DMatrix::default_with_size(size),
            edit_text: String::default(),
            grid: vec![String::from("0.0"); size.0 * size.1],
            state: IcedMatrixState::Display {
                edit_button: button::State::new(),
            },
        }
    }

    pub fn update(&mut self, message: MatrixMessage) {
        match message {
            MatrixMessage::SaveMatrix => {
                let mut counter = 0;
                for m in 0..self.mat.size.0 {
                    for n in 0..self.mat.size.1 {
                        self.mat.data[n][m] = self.grid[counter].parse().unwrap();
                        counter += 1;
                    }
                }
            }
            MatrixMessage::EditedValue(content) => {
                self.edit_text = content;
            }
            MatrixMessage::Edit => {
                self.state = IcedMatrixState::Editing {
                    text_input: text_input::State::focused(),
                    delete_button: button::State::new(),
                    finish_button: button::State::new(),
                }
            }
            MatrixMessage::Finish => {
                self.state = IcedMatrixState::Display {
                    edit_button: button::State::new(),
                }
            }
            MatrixMessage::Delete => {}
        }
    }

    pub fn view(&mut self) -> Element<MatrixMessage> {
        let mut values = self.grid.iter().peekable();
        let mut chunked: Vec<Vec<&String>> = Vec::new();
        while values.peek().is_some() {
            let chunk = values.by_ref().take(self.mat.size.0).collect();
            chunked.push(chunk)
        }

        let row = chunked.iter().fold(
            Row::new().spacing(10).align_items(Align::Center),
            |row, chunk| {
                row.push(
                    chunk
                        .iter()
                        .fold(Column::new(), |col, item| col.push(Text::new(*item))),
                )
            },
        );
        match &mut self.state {
            IcedMatrixState::Display { edit_button } => Column::new()
                .push(Button::new(edit_button, edit_icon()).on_press(MatrixMessage::Edit))
                .push(row)
                .into(),
            IcedMatrixState::Editing {
                delete_button,
                text_input,
                finish_button,
            } => {
                let input =
                    TextInput::new(text_input, "", &self.edit_text, MatrixMessage::EditedValue);

                Column::new()
                    .push(
                        Row::new()
                            .push(
                                Button::new(
                                    delete_button,
                                    Row::new()
                                        .spacing(10)
                                        .push(delete_icon())
                                        .push(Text::new("Delete")),
                                )
                                .on_press(MatrixMessage::Delete),
                            )
                            .push(
                                Button::new(finish_button, edit_icon())
                                    .on_press(MatrixMessage::Finish),
                            ),
                    )
                    .push(row)
                    .push(input)
                    .into()
            }
        }
    }
}
