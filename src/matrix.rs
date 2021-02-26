use crate::utils::{delete_icon, edit_icon, new_icon};
use glucose::DMatrix;
use iced::{
    button, text_input, Align, Button, Column, Element, Length, Radio, Row, Text, TextInput,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IcedMatrixOperation {
    Add,
    Sub,
    Mul,
}

impl From<IcedMatrixOperation> for String {
    fn from(op: IcedMatrixOperation) -> Self {
        String::from(match op {
            IcedMatrixOperation::Add => "Addition",
            IcedMatrixOperation::Sub => "Subtraction",
            IcedMatrixOperation::Mul => "Multiplication",
        })
    }
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
pub enum MatrixMessage {
    IcedMatrixMessage(usize, IcedMatrixMessage),
    MatrixOpSelected(IcedMatrixOperation),
    MatrixCalculate,
    ReuseResult,
}

#[derive(Debug, Default, Clone)]
pub struct MatrixCalculationState {
    matrices: Vec<IcedMatrix>,
    selected_mat_op: Option<IcedMatrixOperation>,
    calculate_button: button::State,
    reuse_button: button::State,
    result: Option<IcedMatrix>,
    error_message: Option<String>,
}

impl MatrixCalculationState {
    pub fn new() -> Self {
        MatrixCalculationState {
            matrices: vec![IcedMatrix::new(), IcedMatrix::new()],
            selected_mat_op: Default::default(),
            calculate_button: Default::default(),
            reuse_button: Default::default(),
            result: None,
            error_message: None,
        }
    }

    pub fn update(&mut self, message: MatrixMessage) {
        match message {
            MatrixMessage::IcedMatrixMessage(id, matrix_message) => {
                if let Some(matrix) = self.matrices.get_mut(id) {
                    matrix.update(matrix_message)
                }
            }
            MatrixMessage::MatrixOpSelected(op) => {
                self.selected_mat_op = Some(op);
            }
            MatrixMessage::MatrixCalculate => {
                self.error_message = None;
                if self.matrices[0].is_initialized() && self.matrices[1].is_initialized() {
                    match self.selected_mat_op {
                        Some(IcedMatrixOperation::Add) => {
                            if self.matrices[0].get_size_unchecked()
                                == self.matrices[1].get_size_unchecked()
                            {
                                self.result = Some(IcedMatrix::from_matrix(
                                    self.matrices[0].get_matrix_unchecked()
                                        + self.matrices[1].get_matrix_unchecked(),
                                ))
                            } else {
                                self.error_message = Some(String::from("Matrix sizes not equal"));
                            }
                        }
                        Some(IcedMatrixOperation::Sub) => {
                            if self.matrices[0].get_size_unchecked()
                                == self.matrices[1].get_size_unchecked()
                            {
                                self.result = Some(IcedMatrix::from_matrix(
                                    self.matrices[0].get_matrix_unchecked()
                                        - self.matrices[1].get_matrix_unchecked(),
                                ))
                            } else {
                                self.error_message = Some(String::from("Matrix sizes not equal"));
                            }
                        }
                        Some(IcedMatrixOperation::Mul) => {
                            if self.matrices[0].get_size_unchecked().1
                                == self.matrices[1].get_size_unchecked().0
                            {
                                self.result = Some(IcedMatrix::from_matrix(
                                    self.matrices[0].get_matrix_unchecked()
                                        * self.matrices[1].get_matrix_unchecked(),
                                ))
                            } else {
                                self.error_message =
                                    Some(String::from("column size of mat1 != row size of mat2"));
                            }
                        }
                        None => {
                            self.error_message = Some(String::from("please select an operation"))
                        }
                    }
                } else {
                    self.error_message = Some(String::from("Matrices not initialized"))
                }
            }
            MatrixMessage::ReuseResult => {
                if self.result.is_some() {
                    self.matrices[0] = self.result.clone().unwrap()
                } else {
                    self.error_message = Some(String::from("Please calculate a result first"))
                }
            }
        }
    }

    pub fn view(&mut self) -> Element<MatrixMessage> {
        let title = Text::new("Matrices").size(30).color([0.0, 0.0, 0.0]);
        let matrices = self.matrices.iter_mut().enumerate().fold(
            Row::new().spacing(30).align_items(Align::Center),
            |row, (i, matrix)| {
                row.push(
                    matrix
                        .view()
                        .map(move |message| MatrixMessage::IcedMatrixMessage(i, message)),
                )
            },
        );

        // let mat_op_selector = Row::new()
        //     .spacing(20)
        //     .push(PickList::new(
        //         &mut self.matrix_op,
        //         &IcedMatrixOperation::ALL[..],
        //         Some(self.selected_mat_op),
        //         MatrixMessage::MatrixOpSelected,
        //     ))
        //     .push(
        //         Button::new(&mut self.calculate_button, Text::new("Calculate"))
        //             .on_press(MatrixMessage::MatrixCalculate),
        //     );

        let mat_op_selector = Row::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Self::selection_column(self.selected_mat_op))
            .push(
                Column::new()
                    .align_items(Align::Center)
                    .push(
                        Button::new(&mut self.calculate_button, Text::new("Calculate"))
                            .on_press(MatrixMessage::MatrixCalculate),
                    )
                    .push(
                        Button::new(&mut self.reuse_button, Text::new("Reuse"))
                            .on_press(MatrixMessage::ReuseResult),
                    ),
            );

        let maybe_result = match &self.result {
            None => Row::new().push(Text::new("")),
            Some(mat) => {
                let string_mat = mat.get_matrix_unchecked().to_string_vec();
                string_mat
                    .data
                    .iter()
                    .fold(
                        Row::new().spacing(10).align_items(Align::Center),
                        |row, chunk| {
                            row.push(
                                chunk
                                    .iter()
                                    .fold(Column::new(), |col, item| col.push(Text::new(item))),
                            )
                        },
                    )
                    .into()
            }
        };

        let maybe_error = match &self.error_message {
            None => Text::new(""),
            Some(message) => Text::new(message).color([0.921, 0.039, 0.039]),
        };

        Column::new()
            .align_items(Align::Center)
            .spacing(20)
            .push(title)
            .push(mat_op_selector)
            .push(matrices)
            .push(maybe_result)
            .push(maybe_error)
            .into()
    }

    fn selection_column<'a>(selection: Option<IcedMatrixOperation>) -> Column<'a, MatrixMessage> {
        Column::new()
            .padding(5)
            .spacing(5)
            .push(Text::new("Select an operation"))
            .push(IcedMatrixOperation::ALL.iter().cloned().fold(
                Column::new().padding(5).spacing(5),
                |column, operation| {
                    column.push(Radio::new(
                        operation,
                        operation,
                        selection,
                        MatrixMessage::MatrixOpSelected,
                    ))
                },
            ))
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
pub enum IcedMatrixMessage {
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

    pub fn update(&mut self, message: IcedMatrixMessage) {
        match message {
            IcedMatrixMessage::Create => {
                self.state = IcedMatrixState::Initializing {
                    text_input: text_input::State::focused(),
                    cancel_button: button::State::new(),
                }
            }
            IcedMatrixMessage::Finish => {
                self.mat = Some(DMatrix::<f64>::from(self.edit_text.as_str()));
                self.state = IcedMatrixState::Display {
                    edit_button: button::State::new(),
                }
            }
            IcedMatrixMessage::Cancel => {
                self.edit_text.clear();
                self.state = IcedMatrixState::Uninitialized {
                    new_button: button::State::new(),
                }
            }
            IcedMatrixMessage::EditedValue(content) => {
                self.edit_text = content;
            }
            IcedMatrixMessage::Edit => {
                self.state = IcedMatrixState::Editing {
                    text_input: text_input::State::focused(),
                    delete_button: button::State::new(),
                }
            }
            IcedMatrixMessage::Delete => {
                self.edit_text.clear();
                self.state = IcedMatrixState::Uninitialized {
                    new_button: Default::default(),
                }
            }
        }
    }

    pub fn view(&mut self) -> Element<IcedMatrixMessage> {
        match &mut self.state {
            IcedMatrixState::Uninitialized { new_button } => Column::new()
                .align_items(Align::Center)
                .push(
                    Button::new(
                        new_button,
                        Row::new().push(new_icon()).push(Text::new("New")),
                    )
                    .on_press(IcedMatrixMessage::Create),
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
                        .on_press(IcedMatrixMessage::Cancel),
                    ),
                )
                .push(
                    TextInput::new(
                        text_input,
                        "",
                        &self.edit_text,
                        IcedMatrixMessage::EditedValue,
                    )
                    .on_submit(IcedMatrixMessage::Finish),
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
                    .push(Button::new(edit_button, edit_icon()).on_press(IcedMatrixMessage::Edit))
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
                        .on_press(IcedMatrixMessage::Delete),
                    ),
                )
                .push(
                    TextInput::new(
                        text_input,
                        "",
                        &self.edit_text,
                        IcedMatrixMessage::EditedValue,
                    )
                    .on_submit(IcedMatrixMessage::Finish),
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

    pub fn get_size_unchecked(&self) -> (usize, usize) {
        self.mat.as_ref().unwrap().size
    }
}
