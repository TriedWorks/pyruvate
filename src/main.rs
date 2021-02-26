#![windows_subsystem = "windows"]

use crate::errors::LoadError;
use crate::group_theory::{GroupTheoryMessage, GroupTheoryState};
use crate::matrix::{MatrixCalculationState, MatrixMessage};
use crate::utils::loading_message;
use iced::{
    button, executor, scrollable, Align, Application, Button, Column, Command, Container, Element,
    HorizontalAlignment, Length, Row, Scrollable, Settings, Text,
};

pub mod errors;
pub mod group_theory;
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

#[derive(Debug, Clone)]
pub struct State {
    scroll: scrollable::State,
    controls: Controls,
    current: SubState,
}

impl Default for State {
    fn default() -> Self {
        Self {
            scroll: scrollable::State::new(),
            controls: Controls::default(),
            current: SubState::None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SubState {
    None,
    Matrix(MatrixCalculationState),
    GroupTheory(GroupTheoryState),
}

impl State {
    async fn load() -> Result<Self, LoadError> {
        Ok(State::default())
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<State, LoadError>),
    MatrixMessage(MatrixMessage),
    GroupTheoryMessage(GroupTheoryMessage),
    SwitchState(SubState),
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
                            controls: state.controls,
                            current: state.current,
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
                    Message::SwitchState(new) => {
                        state.current = new;
                    }
                    Message::MatrixMessage(sub_message) => match &mut state.current {
                        SubState::Matrix(sub_state) => sub_state.update(sub_message),
                        _ => {}
                    },
                    Message::GroupTheoryMessage(sub_message) => match &mut state.current {
                        SubState::GroupTheory(sub_state) => sub_state.update(sub_message),
                        _ => {}
                    },
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
                controls,
                current,
            }) => {
                let title = Text::new("Pyruvate")
                    .width(Length::Fill)
                    .size(40)
                    .color([0.0, 0.0, 0.0])
                    .horizontal_alignment(HorizontalAlignment::Center);

                let sub_state_content = match current {
                    SubState::None => Column::new()
                        .push(Text::new("Please Select Something"))
                        .into(),
                    SubState::Matrix(sub_state) => sub_state
                        .view()
                        .map(move |message| Message::MatrixMessage(message)),
                    SubState::GroupTheory(sub_state) => sub_state
                        .view()
                        .map(move |message| Message::GroupTheoryMessage(message)),
                };
                let controls = controls.view();

                let content = Column::new()
                    .spacing(20)
                    .max_width(800)
                    .align_items(Align::Center)
                    .push(title)
                    .push(controls)
                    .push(sub_state_content);

                Scrollable::new(scroll)
                    .padding(40)
                    .push(Container::new(content).width(Length::Fill).center_x())
                    .into()
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Controls {
    home_button: button::State,
    matrices_button: button::State,
    group_theory_button: button::State,
}

impl Controls {
    fn view(&mut self) -> Row<Message> {
        Row::new()
            .spacing(10)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.home_button, Text::new("Home"))
                    .on_press(Message::SwitchState(SubState::None)),
            )
            .push(
                Button::new(&mut self.matrices_button, Text::new("Matrices")).on_press(
                    Message::SwitchState(SubState::Matrix(MatrixCalculationState::new())),
                ),
            )
            .push(
                Button::new(&mut self.group_theory_button, Text::new("Group Theory")).on_press(
                    Message::SwitchState(SubState::GroupTheory(GroupTheoryState::new())),
                ),
            )
    }
}
