use glucose::group_theory::extended_euclidean_algorithm::extended_euclidean_as_dmatrix;
use glucose::DMatrix;
use iced::{text_input, Align, Column, Element, Row, Text, TextInput, button, Button};
use std::num::ParseIntError;
use glucose::group_theory::primes::GroupType;

#[derive(Debug, Clone)]
pub enum GroupTheoryMessage {
    SwitchState(GroupTheorySubState),
    IcedEEAMessage(IcedEEAMessage),
    IcedGroupMessage(IcedGroupMessage)
}

#[derive(Debug, Clone)]
pub enum GroupTheorySubState {
    None,
    EEA(IcedEEA),
    Groups(IcedGroups),
}

#[derive(Debug, Clone)]
pub enum IcedEEAMessage {
    EditValue(String),
    Calculate,
}

#[derive(Debug, Clone)]
pub enum IcedGroupMessage {
    EditValue(String),
    Calculate,
}

#[derive(Debug, Clone)]
pub struct IcedGroupTheory {
    control: GroupTheoryControls,
    state: GroupTheorySubState,
}

impl IcedGroupTheory {
    pub fn new() -> Self {
        Self {
            control: GroupTheoryControls::default(),
            state: GroupTheorySubState::None,
        }
    }
    pub fn update(&mut self, message: GroupTheoryMessage) {
        match message {
            GroupTheoryMessage::SwitchState(state) => {
                self.state = state;
            }
            GroupTheoryMessage::IcedEEAMessage(message) => {
                match &mut self.state {
                    GroupTheorySubState::EEA(state) => {
                        state.update(message)
                    }
                    _ => { }
                }
            }
            GroupTheoryMessage::IcedGroupMessage(message) => {
                match &mut self.state {
                    GroupTheorySubState::Groups(state) => {
                        state.update(message)
                    }
                    _ => { }
                }
            }
        }
    }

    pub fn view(&mut self) -> Element<GroupTheoryMessage> {
        let controls = self.control.view();
        let content = match &mut self.state {
            GroupTheorySubState::None => {
                Column::new()
                    .push(
                        Text::new("Chose what you want to do")
                    ).into()
            }
            GroupTheorySubState::EEA(state) => {
                state
                    .view()
                    .map(move |message| GroupTheoryMessage::IcedEEAMessage(message))

            }
            GroupTheorySubState::Groups(state) => {
                state
                    .view()
                    .map(move |message| GroupTheoryMessage::IcedGroupMessage(message))
                    .into()
            }
        };

        Column::new()
            .push(controls)
            .push(content)
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

#[derive(Debug, Clone)]
pub struct IcedGroups {
    edit_text: String,
    input_text: text_input::State,

    group_type: GroupType,
    prime_factors: Vec<i64>,
    group_size: i64,
    group_size_prime_factors: Vec<i64>,
    group: Vec<i64>,
    possible_orders: Vec<i64>,
    orders: Vec<(i64, i64)>,
    producers: Vec<i64>,

    error_message: Option<String>,
}

impl IcedGroups {
    pub fn new() -> Self {
        Self {
            edit_text: String::default(),
            input_text: text_input::State::new(),
            group_type: GroupType::MultiplicativeStar,
            prime_factors: vec![],
            group_size: 0,
            group_size_prime_factors: vec![],
            group: vec![],
            possible_orders: vec![],
            orders: vec![],
            producers: vec![],
            error_message: None,
        }
    }
    pub fn update(&mut self, message: IcedGroupMessage) {
        match message {
            IcedGroupMessage::EditValue(content) => {
                self.edit_text = content;
            }
            IcedGroupMessage::Calculate => {
                self.error_message = None;
                let split: Vec<String> = self.edit_text.split(", ").into_iter().map(|str| str.to_string()).collect();
                if split.len() < 2 {
                    self.error_message = Some(String::from("input is not correct: num, a; num, m or num, m*"));
                    return;
                }
                let maybe_modulo = split[0].parse::<i64>();
                let maybe_op = split[1].clone();

                if maybe_modulo.is_ok() && maybe_op == "a" || maybe_op == "m" || maybe_op == "mb" || maybe_op == "m*" || maybe_op == "m*b" {
                    use glucose::group_theory::primes::*;
                    let modulo = maybe_modulo.unwrap();
                    let mut should_big = false;
                    self.group_type = match maybe_op.as_ref() {
                        "a" => GroupType::Additive,
                        "m" => GroupType::Multiplicative,
                        "mb" => {
                            should_big = true;
                            GroupType::Multiplicative
                        }
                        "m*" => GroupType::MultiplicativeStar,
                        "m*b" => {
                            should_big = true;
                            GroupType::MultiplicativeStar
                        }
                        _ => { panic!( )}
                    };
                    self.prime_factors = wheel_factorization(modulo);
                    self.group_size = group_size(modulo, self.group_type);
                    self.group_size_prime_factors = wheel_factorization(self.group_size);
                    self.group = group(modulo, self.group_type);
                    self.possible_orders = possible_orders(modulo, self.group_type);
                    self.orders = orders(modulo, &self.group, self.group_type);
                    if !self.group_size > 54 || self.group_type == GroupType::Additive || should_big {
                        self.producers = producers(modulo, &self.group, self.group_type, should_big)
                    } else {
                        self.error_message = Some(String::from("to calculate producers bigger than 54 use 'm*b' or 'mb'"))
                    }
                } else {
                    self.error_message = Some(String::from("input is not correct: num, a or num, m"))
                }
            }
        }
    }

    pub fn view(&mut self) -> Element<IcedGroupMessage> {
        let input = TextInput::new(
            &mut self.input_text,
            "number, a or m",
            &self.edit_text,
            IcedGroupMessage::EditValue,
        ).on_submit(IcedGroupMessage::Calculate);

        let content = Column::new()
            .push(Row::new().max_width(800).push(Text::new(format!("Group Type: {:?}", self.group_type))))
            .push(
                self.prime_factors.iter().fold(
                    Row::new().max_width(800).push(Text::new("Prime Factors: ")), |row, num| {
                        row.push(Text::new(format!("{}, ", num)))
                    }
                )
            )
            .push(Row::new().max_width(800).push(Text::new(format!("Group Size: {}", self.group_size))))
            .push(
                self.group_size_prime_factors.iter().fold(
                    Row::new().max_width(800).push(Text::new("Group Size Prime Factors: ")), |row, num| {
                        row.push(Text::new(format!("{}, ", num)))
                    }
                )
            )
            .push(
                self.group.iter().fold(
                    Row::new().max_width(800).push(Text::new("Coprimes: ")), |row, num| {
                        row.push(Text::new(format!("{}, ", num)))
                    }
                )
            )
            .push(
                self.possible_orders.iter().fold(
                    Row::new().max_width(800).push(Text::new("Possible Orders: ")), |row, num| {
                        row.push(Text::new(format!("{}, ", num)))
                    }
                )
            )
            .push(
                self.orders.iter().fold(
                    Row::new().max_width(800).push(Text::new("Actual Orders: ")), |row, num| {
                        row.push(Text::new(format!("{:?}, ", num)))
                    }
                )
            )
            .push(
                self.producers.iter().fold(
                    Row::new().max_width(800).push(Text::new("Producers: ")), |row, num| {
                        row.push(Text::new(format!("{:?}, ", num)))
                    }
                )
            );

        let maybe_error = match &self.error_message {
            None => Text::new(""),
            Some(message) => Text::new(message).color([0.921, 0.039, 0.039]),
        };

        Column::new()
            .push(input)
            .push(content)
            .push(maybe_error)
            .into()
    }
}

#[derive(Debug, Default, Clone)]
pub struct GroupTheoryControls {
    home_button: button::State,
    eea_button: button::State,
    groups_button: button::State,
}

impl GroupTheoryControls {
    fn view(&mut self) -> Row<GroupTheoryMessage> {
        Row::new()
            .spacing(10)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.home_button, Text::new("Sub-Home"))
                    .on_press(GroupTheoryMessage::SwitchState(GroupTheorySubState::None)),
            )
            .push(
                Button::new(&mut self.eea_button, Text::new("EEA")).on_press(
                    GroupTheoryMessage::SwitchState(GroupTheorySubState::EEA(IcedEEA::new())),
                ),
            )
            .push(
                Button::new(&mut self.groups_button, Text::new("Groups")).on_press(
                    GroupTheoryMessage::SwitchState(GroupTheorySubState::Groups(IcedGroups::new())),
                ),
            )
    }
}