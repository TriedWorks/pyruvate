use crate::errors::LoadError;
use crate::matrix::{IcedMatrix, IcedMatrixOperation, MatrixMessage};
use crate::utils::loading_message;
use glucose::*;
use iced::{
    button, executor, pick_list, scrollable, text_input, Align, Application, Button, Column,
    Command, Container, Element, Font, HorizontalAlignment, Length, PickList, Row, Scrollable,
    Settings, Text,
};
use std::error::Error;

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
    matrix_op: pick_list::State<IcedMatrixOperation>,
    result: IcedMatrix,
}

impl State {
    async fn load() -> Result<Self, LoadError> {
        Ok(State::default())
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<State, LoadError>),
    InputChanged(String, usize),
    MatrixMessage(usize, MatrixMessage),
    MatrixOpSelected(IcedMatrixOperation),
    InitMatrix((usize, usize)),
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
                            matrices: vec![IcedMatrix::new((3, 3))],
                            selected_mat_op: Default::default(),
                            matrix_op: Default::default(),
                            result: IcedMatrix::new((3, 3)),
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
                    Message::InputChanged(value, index) => {}
                    Message::InitMatrix(size) => state.matrices.push(IcedMatrix::new(size)),
                    Message::MatrixMessage(id, MatrixMessage::Delete) => {
                        state.matrices.remove(id);
                    }
                    Message::MatrixMessage(id, matrix_message) => {
                        if let Some(matrix) = state.matrices.get_mut(id) {
                            matrix.update(matrix_message)
                        }
                    }
                    Message::MatrixOpSelected(op) => {
                        state.selected_mat_op = op;
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

                let mat_op_selecter = PickList::new(
                    matrix_op,
                    &IcedMatrixOperation::ALL[..],
                    Some(*selected_mat_op),
                    Message::MatrixOpSelected,
                );

                let content = Column::new()
                    .align_items(Align::Center)
                    .spacing(20)
                    .push(title)
                    .push(mat_op_selecter)
                    .push(row);

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
