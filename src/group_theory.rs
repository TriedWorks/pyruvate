use glucose::group_theory::extended_euclidean_algorithm::extended_euclidean_as_dmatrix;
use glucose::DMatrix;
use iced::{text_input, Align, Column, Element, Row, Text, TextInput};
use std::num::ParseIntError;

#[derive(Debug, Clone)]
pub enum GroupTheoryMessage {
    IcedEEAMessage(IcedEEAMessage),
}

#[derive(Debug, Clone)]
pub enum IcedEEAMessage {
    EditValue(String),
    Calculate,
}

#[derive(Debug, Clone, Default)]
pub struct GroupTheoryState {
    eea: IcedEEA,
}

impl GroupTheoryState {
    pub fn new() -> Self {
        Self {
            eea: IcedEEA::new(),
        }
    }
    pub fn update(&mut self, message: GroupTheoryMessage) {
        match message {
            GroupTheoryMessage::IcedEEAMessage(eea_message) => {
                self.eea.update(eea_message);
            }
        }
    }

    pub fn view(&mut self) -> Element<GroupTheoryMessage> {
        self.eea
            .view()
            .map(move |message| GroupTheoryMessage::IcedEEAMessage(message))
            .into()
    }
}

#[derive(Debug, Clone, Default)]
pub struct IcedEEA {
    edit_text: String,
    input_text: text_input::State,
    result: Option<DMatrix<i64>>,
    error_message: Option<String>,
}

impl IcedEEA {
    pub fn new() -> Self {
        Self {
            edit_text: String::default(),
            input_text: text_input::State::new(),
            result: None,
            error_message: None,
        }
    }

    pub fn update(&mut self, message: IcedEEAMessage) {
        match message {
            IcedEEAMessage::EditValue(content) => {
                self.edit_text = content;
            }
            IcedEEAMessage::Calculate => {
                self.error_message = None;
                let values: Vec<Result<i64, ParseIntError>> = self
                    .edit_text
                    .split(", ")
                    .into_iter()
                    .map(|val| val.parse::<i64>())
                    .collect();

                if values.get(0).is_some() && values.get(1).is_some() {
                    if values[0].is_ok() && values[1].is_ok() {
                        let a = *values[0].as_ref().unwrap();
                        let b = *values[1].as_ref().unwrap();
                        if a < b {
                            let result = extended_euclidean_as_dmatrix(a, b);
                            self.result = Some(result)
                        } else {
                            self.error_message = Some(String::from("a must be smaller than b"))
                        }
                    } else {
                        self.error_message = Some(String::from("a and/or b is not a number"))
                    }
                } else {
                    self.error_message =
                        Some(String::from("a and b are not given or format is wrong"))
                }
            }
        }
    }

    pub fn view(&mut self) -> Element<IcedEEAMessage> {
        let input = TextInput::new(
            &mut self.input_text,
            "",
            &self.edit_text,
            IcedEEAMessage::EditValue,
        )
        .on_submit(IcedEEAMessage::Calculate);

        let maybe_result = match &self.result {
            None => Row::new().push(Text::new("")),
            Some(result) => result
                .data
                .iter()
                .enumerate()
                .fold(
                    Row::new().spacing(20).align_items(Align::Center),
                    |row, (i, chunk)| {
                        row.push(chunk.iter().fold(
                            Column::new().push(Text::new(EEA_INDICES[i])),
                            |col, item| col.push(Text::new(item.to_string())),
                        ))
                    },
                )
                .into(),
        };

        let maybe_error = match &self.error_message {
            None => Text::new(""),
            Some(message) => Text::new(message).color([0.921, 0.039, 0.039]),
        };

        Column::new()
            .max_width(750)
            .align_items(Align::Center)
            .spacing(20)
            .push(input)
            .push(maybe_result)
            .push(maybe_error)
            .into()
    }
}

const EEA_INDICES: [char; 5] = ['a', 'b', 'k', 's', 't'];
