use crate::errors::LoadError;
use crate::matrix::{IcedMatrix, IcedMatrixOperation, MatrixMessage};
use crate::utils::loading_message;
use iced::{
    button, executor, pick_list, scrollable, Align, Application, Button, Column,
    Command, Element, HorizontalAlignment, Length, PickList, Row, Scrollable,
    Settings, Text,
};

pub mod errors;
pub mod matrix;
pub mod utils;

fn main() -> iced::Result {
    Pyruvate::run(Settings::default())
}

#[derive(Debug)]
pub enum Pyruvate {
    Loading,
    Loaded(State),
}

#[derive(Debug, Default, Clone)]
pub struct State {
    scroll: scrollable::State,
    matrices: Vec<IcedMatrix>,
    selected_mat_op: IcedMatrixOperation,
    calculate_button: button::State,
    matrix_op: pick_list::State<IcedMatrixOperation>,
    result: Option<IcedMatrix>,
}

impl State {
    async fn load() -> Result<Self, LoadError> {
        Ok(State::default())
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<State, LoadError>),
    MatrixMessage(usize, MatrixMessage),
    MatrixOpSelected(IcedMatrixOperation),
    MatrixCalculate,
    None,
}

impl Application for Pyruvate {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self::Loading,
            Command::perform(State::load(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        String::from("Pyruvate")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            Pyruvate::Loading => {
                match message {
                    Message::Loaded(Ok(state)) => {
                        *self = Pyruvate::Loaded(State {
                            scroll: state.scroll,
                            matrices: vec![IcedMatrix::new(), IcedMatrix::new()],
                            selected_mat_op: Default::default(),
                            calculate_button: Default::default(),
                            matrix_op: Default::default(),
                            result: None,
                        });
                    }
                    Message::Loaded(Err(_)) => {
                        *self = Pyruvate::Loaded(State::default());
                    }
                    _ => {}
                }
                Command::none()
            }
            Pyruvate::Loaded(state) => {
                match message {
                    Message::MatrixMessage(id, matrix_message) => {
                        if let Some(matrix) = state.matrices.get_mut(id) {
                            matrix.update(matrix_message)
                        }
                    }
                    Message::MatrixOpSelected(op) => {
                        state.selected_mat_op = op;
                    }
                    Message::MatrixCalculate => {
                        if state.matrices[0].is_initialized() && state.matrices[1].is_initialized()
                        {
                            match state.selected_mat_op {
                                IcedMatrixOperation::Add => {
                                    state.result = Some(IcedMatrix::from_matrix(
                                        state.matrices[0].get_matrix_unchecked()
                                            + state.matrices[1].get_matrix_unchecked(),
                                    ))
                                }
                                IcedMatrixOperation::Sub => {
                                    state.result = Some(IcedMatrix::from_matrix(
                                        state.matrices[0].get_matrix_unchecked()
                                            - state.matrices[1].get_matrix_unchecked(),
                                    ))
                                }
                                IcedMatrixOperation::Mul => {
                                    state.result = Some(IcedMatrix::from_matrix(
                                        state.matrices[0].get_matrix_unchecked()
                                            * state.matrices[1].get_matrix_unchecked(),
                                    ))
                                }
                            }
                        }
                    }
                    _ => {}
                }
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        match self {
            Pyruvate::Loading => loading_message(),
            Pyruvate::Loaded(State {
                scroll,
                matrices,
                selected_mat_op,
                calculate_button,
                matrix_op,
                result,
            }) => {
                let title = Text::new("Matrices")
                    .width(Length::Fill)
                    .size(100)
                    .color([0.0, 0.0, 0.0])
                    .horizontal_alignment(HorizontalAlignment::Center);

                let row = matrices.iter_mut().enumerate().fold(
                    Row::new().spacing(30).align_items(Align::Center),
                    |row, (i, matrix)| {
                        row.push(
                            matrix
                                .view()
                                .map(move |message| Message::MatrixMessage(i, message)),
                        )
                    },
                );

                let mat_op_selector = Row::new().push(PickList::new(
                    matrix_op,
                    &IcedMatrixOperation::ALL[..],
                    Some(*selected_mat_op),
                    Message::MatrixOpSelected,
                )).push(
                    Button::new(calculate_button, Text::new("Calculate"))
                        .on_press(Message::MatrixCalculate));

                let maybe_result =
                    match result {
                        None => Row::new().push(Text::new("")),
                        Some(mat) => {
                            let string_mat = mat.get_matrix_unchecked().to_string_vec();
                            string_mat
                                .data
                                .iter()
                                .fold(
                                    Row::new().spacing(10).align_items(Align::Center),
                                    |row, chunk| {
                                        row.push(chunk.iter().fold(Column::new(), |col, item| {
                                            col.push(Text::new(item))
                                        }))
                                    },
                                )
                                .into()
                        }
                    };

                let content = Column::new()
                    .align_items(Align::Center)
                    .spacing(20)
                    .push(title)
                    .push(mat_op_selector)
                    .push(row)
                    .push(maybe_result);

                Scrollable::new(scroll).padding(40).push(content).into()
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Controls {
    home_button: button::State,
    matrices_button: button::State,
}

// impl Controls {
//     fn view(&mut self, )
// }
