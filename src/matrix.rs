use crate::utils::{delete_icon, edit_icon, new_icon};
use glucose::DMatrix;
use iced::{button, text_input, Align, Button, Column, Element, Length, Row, Text, TextInput};

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
    Uninitialized {
        new_button: button::State,
    },
    Initializing {
        text_input: text_input::State,
        cancel_button: button::State,
    },
    Display {
        edit_button: button::State,
    },
    Editing {
        text_input: text_input::State,
        delete_button: button::State,
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
    mat: Option<DMatrix<f64>>,
    edit_text: String,
    state: IcedMatrixState,
}

#[derive(Debug, Clone)]
pub enum MatrixMessage {
    Create,
    Finish,
    Cancel,

    Edit,
    EditedValue(String),
    Delete,
}

impl IcedMatrix {
    pub fn new() -> Self {
        IcedMatrix {
            mat: None,
            edit_text: String::default(),
            state: IcedMatrixState::Uninitialized {
                new_button: button::State::new(),
            },
        }
    }

    pub fn from_matrix(matrix: DMatrix<f64>) -> Self {
        IcedMatrix {
            mat: Some(matrix),
            edit_text: "".to_string(),
            state: IcedMatrixState::Display {
                edit_button: Default::default(),
            },
        }
    }

    pub fn update(&mut self, message: MatrixMessage) {
        match message {
            MatrixMessage::Create => {
                self.state = IcedMatrixState::Initializing {
                    text_input: text_input::State::focused(),
                    cancel_button: button::State::new(),
                }
            }
            MatrixMessage::Finish => {
                self.mat = Some(DMatrix::<f64>::from(self.edit_text.as_str()));
                println!("{:?}", self.mat.as_ref().unwrap());
                self.state = IcedMatrixState::Display {
                    edit_button: button::State::new(),
                }
            }
            MatrixMessage::Cancel => {
                self.edit_text.clear();
                self.state = IcedMatrixState::Uninitialized {
                    new_button: button::State::new(),
                }
            }
            MatrixMessage::EditedValue(content) => {
                self.edit_text = content;
            }
            MatrixMessage::Edit => {
                self.state = IcedMatrixState::Editing {
                    text_input: text_input::State::focused(),
                    delete_button: button::State::new(),
                }
            }
            MatrixMessage::Delete => {
                self.edit_text.clear();
                self.state = IcedMatrixState::Uninitialized {
                    new_button: Default::default(),
                }
            }
        }
    }

    pub fn view(&mut self) -> Element<MatrixMessage> {
        match &mut self.state {
            IcedMatrixState::Uninitialized { new_button } => Column::new()
                .align_items(Align::Center)
                .push(
                    Button::new(
                        new_button,
                        Row::new().push(new_icon()).push(Text::new("New")),
                    )
                    .on_press(MatrixMessage::Create),
                )
                .into(),
            IcedMatrixState::Initializing {
                text_input,
                cancel_button,
            } => Column::new()
                .width(Length::Units(200))
                .push(
                    Row::new().push(
                        Button::new(
                            cancel_button,
                            Row::new().push(delete_icon()).push(Text::new("Cancel")),
                        )
                        .on_press(MatrixMessage::Cancel),
                    ),
                )
                .push(
                    TextInput::new(text_input, "", &self.edit_text, MatrixMessage::EditedValue)
                        .on_submit(MatrixMessage::Finish),
                )
                .into(),
            IcedMatrixState::Display { edit_button } => {
                let string_mat = self.mat.as_ref().unwrap().to_string_vec();

                let row = string_mat.data.iter().fold(
                    Row::new().spacing(10).align_items(Align::Center),
                    |row, chunk| {
                        row.push(
                            chunk
                                .iter()
                                .fold(Column::new(), |col, item| col.push(Text::new(item))),
                        )
                    },
                );
                Column::new()
                    .push(Button::new(edit_button, edit_icon()).on_press(MatrixMessage::Edit))
                    .push(row)
                    .into()
            }
            IcedMatrixState::Editing {
                delete_button,
                text_input,
            } => Column::new()
                .width(Length::Units(200))
                .push(
                    Row::new().push(
                        Button::new(
                            delete_button,
                            Row::new()
                                .spacing(10)
                                .push(delete_icon())
                                .push(Text::new("Delete")),
                        )
                        .on_press(MatrixMessage::Delete),
                    ),
                )
                .push(
                    TextInput::new(text_input, "", &self.edit_text, MatrixMessage::EditedValue)
                        .on_submit(MatrixMessage::Finish),
                )
                .into(),
        }
    }
    pub fn is_initialized(&self) -> bool {
        self.mat.is_some()
    }

    pub fn get_matrix_unchecked(&self) -> DMatrix<f64> {
        self.mat.clone().unwrap()
    }
}
