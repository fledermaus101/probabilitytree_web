use std::{
    array,
    ops::{Div, Mul},
    str::FromStr,
};

use egui::{Align, Direction, Layout, Vec2};
use fraction::{ConstOne, Fraction, One};
use log::debug;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    event_a_name: String,
    event_an_name: String,
    event_b_name: String,
    event_bn_name: String,

    probabilities: [String; 16],
    // p_a: String,
    // p_an: String,
    // p_b: String,
    // p_bn: String,
    //
    // p_a_b: String,
    // p_a_bn: String,
    // p_an_b: String,
    // p_an_bn: String,
    //
    // p_b_a: String,
    // p_b_an: String,
    // p_bn_a: String,
    // p_bn_an: String,
    //
    // p_a_union_b: String,
    // p_a_union_bn: String,
    // p_an_union_b: String,
    // p_an_union_bn: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            event_a_name: String::from("A"),
            event_an_name: String::from("A'"),
            event_b_name: String::from("B"),
            event_bn_name: String::from("B'"),
            probabilities: array::from_fn(|_| String::new()),
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            if ui.button("Calculate").clicked() {
                let mut probabilities_simplified = Vec::with_capacity(10);
                self.probabilities[..12]
                    .array_chunks::<2>()
                    .map(
                        |[p, pn]| match (Fraction::from_str(p), Fraction::from_str(pn)) {
                            (Ok(p), Ok(pn)) => (p + pn == Fraction::one()).then_some(p),
                            (Ok(p), Err(_)) => Some(p),
                            (Err(_), Ok(pn)) => Some(Fraction::one() - pn),
                            (Err(_), Err(_)) => None,
                        },
                    )
                    .collect_into(&mut probabilities_simplified);

                probabilities_simplified.extend(
                    self.probabilities[12..16]
                        .iter()
                        .map(|p| Fraction::from_str(p).ok()),
                );
                debug!("{:?}", &probabilities_simplified);
                let probabilities_new = calculate_missing_probabilities(
                    probabilities_simplified
                        .try_into()
                        .expect("probability array has incorrect size"),
                );
                for (i, p) in probabilities_new[..6].iter().enumerate() {
                    if let Some(p) = p {
                        self.probabilities[i * 2] = p.to_string();
                        self.probabilities[i * 2 + 1] = (Fraction::ONE - *p).to_string();
                    }
                }

                for (i, p) in probabilities_new[6..10].iter().enumerate() {
                    if let Some(p) = p {
                        self.probabilities[12 + i] = p.to_string();
                    }
                }
            }
            if ui.button("Reset").clicked() {
                self.probabilities = Default::default();
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns_const(|[tree_1, tree_2]| {
                tree_1.group(|tree_1| {
                    tree_1.allocate_ui_with_layout(
                        Vec2 { x: 100., y: 0. },
                        Layout::centered_and_justified(Direction::TopDown),
                        |ui| {
                            ui.label("1");
                        },
                    );
                    tree_1.allocate_ui_with_layout(
                        Vec2 { x: 50., y: 0. },
                        Layout::left_to_right(Align::Min),
                        |ui| {
                            ui.columns_const(|[col0, col1]| {
                                col0.text_edit_singleline(&mut self.probabilities[0]);
                                col1.text_edit_singleline(&mut self.probabilities[1]);
                            })
                        },
                    );
                    tree_1.allocate_ui_with_layout(
                        Vec2::new(200., 0.),
                        Layout::left_to_right(Align::Min),
                        |ui| {
                            ui.columns_const(|[col0, col1]| {
                                col0.vertical_centered(|ui| ui.label(&self.event_a_name));
                                col1.vertical_centered(|ui| ui.label(&self.event_an_name));
                            })
                        },
                    );
                    tree_1.allocate_ui_with_layout(
                        Vec2::new(200., 0.),
                        Layout::left_to_right(Align::Min),
                        |ui| {
                            ui.columns_const(|[col0, col1, col2, col3]| {
                                col0.text_edit_singleline(&mut self.probabilities[4]);
                                col1.text_edit_singleline(&mut self.probabilities[5]);
                                col2.text_edit_singleline(&mut self.probabilities[6]);
                                col3.text_edit_singleline(&mut self.probabilities[7]);
                            });
                        },
                    );
                    tree_1.allocate_ui_with_layout(
                        Vec2 { x: 200., y: 0. },
                        Layout::left_to_right(Align::Min),
                        |ui| {
                            ui.columns_const(|[col0, col1, col2, col3]| {
                                col0.vertical_centered(|ui| ui.label(&self.event_b_name));
                                col1.vertical_centered(|ui| ui.label(&self.event_bn_name));
                                col2.vertical_centered(|ui| ui.label(&self.event_b_name));
                                col3.vertical_centered(|ui| ui.label(&self.event_bn_name));
                            });
                        },
                    );
                    tree_1.allocate_ui_with_layout(
                        Vec2 { x: 200., y: 0. },
                        Layout::left_to_right(Align::Min),
                        |ui| {
                            ui.columns_const(|[col0, col1, col2, col3]| {
                                col0.vertical_centered(|ui| {
                                    ui.text_edit_singleline(&mut self.probabilities[12])
                                });
                                col1.vertical_centered(|ui| {
                                    ui.text_edit_singleline(&mut self.probabilities[13])
                                });
                                col2.vertical_centered(|ui| {
                                    ui.text_edit_singleline(&mut self.probabilities[14])
                                });
                                col3.vertical_centered(|ui| {
                                    ui.text_edit_singleline(&mut self.probabilities[15])
                                });
                            });
                        },
                    );
                });
                tree_2.group(|tree_2| {
                    tree_2.allocate_ui_with_layout(
                        Vec2 { x: 100., y: 0. },
                        Layout::centered_and_justified(Direction::TopDown),
                        |ui| {
                            ui.label("1");
                        },
                    );
                    tree_2.allocate_ui_with_layout(
                        Vec2 { x: 200., y: 0. },
                        Layout::left_to_right(Align::Min),
                        |ui| {
                            ui.columns_const(|[col0, col1]| {
                                col0.text_edit_singleline(&mut self.probabilities[2]);
                                col1.text_edit_singleline(&mut self.probabilities[3]);
                            })
                        },
                    );
                    tree_2.allocate_ui_with_layout(
                        Vec2::new(200., 0.),
                        Layout::left_to_right(Align::Min),
                        |ui| {
                            ui.columns_const(|[col0, col1]| {
                                col0.vertical_centered(|ui| ui.label(&self.event_b_name));
                                col1.vertical_centered(|ui| ui.label(&self.event_bn_name));
                            })
                        },
                    );
                    tree_2.allocate_ui_with_layout(
                        Vec2::new(200., 0.),
                        Layout::left_to_right(Align::Min),
                        |ui| {
                            ui.columns_const(|[col0, col1, col2, col3]| {
                                col0.text_edit_singleline(&mut self.probabilities[8]);
                                col1.text_edit_singleline(&mut self.probabilities[9]);
                                col2.text_edit_singleline(&mut self.probabilities[10]);
                                col3.text_edit_singleline(&mut self.probabilities[11]);
                            });
                        },
                    );
                    tree_2.allocate_ui_with_layout(
                        Vec2 { x: 200., y: 0. },
                        Layout::left_to_right(Align::Min),
                        |ui| {
                            ui.columns_const(|[col0, col1, col2, col3]| {
                                col0.vertical_centered(|ui| ui.label(&self.event_a_name));
                                col1.vertical_centered(|ui| ui.label(&self.event_an_name));
                                col2.vertical_centered(|ui| ui.label(&self.event_a_name));
                                col3.vertical_centered(|ui| ui.label(&self.event_an_name));
                            });
                        },
                    );
                    tree_2.allocate_ui_with_layout(
                        Vec2 { x: 200., y: 0. },
                        Layout::left_to_right(Align::Min),
                        |ui| {
                            ui.columns_const(|[col0, col1, col2, col3]| {
                                col0.vertical_centered(|ui| {
                                    ui.text_edit_singleline(&mut self.probabilities[12])
                                });
                                col1.vertical_centered(|ui| {
                                    ui.text_edit_singleline(&mut self.probabilities[14])
                                });
                                col2.vertical_centered(|ui| {
                                    ui.text_edit_singleline(&mut self.probabilities[13])
                                });
                                col3.vertical_centered(|ui| {
                                    ui.text_edit_singleline(&mut self.probabilities[15])
                                });
                            });
                        },
                    );
                });
            });
        });
    }
}

fn calculate_missing_probabilities(
    mut probabilities_start: [Option<Fraction>; 10],
) -> [Option<Fraction>; 10] {
    // p_a
    // p_b
    // p_a_b
    // p_an_b
    // p_b_a
    // p_bn_a
    // path rules
    // a * a_b = a_union_b | 0 2
    // b * b_a = a_union_b | 1 4
    // a * (1-a_b)  = a_union_bn | 0 2
    // (1-b) * bn_a = a_union_bn | 1 5
    // (1-a) * an_b = an_union_b | 0 3
    // b * (1-b_a)  = an_union_b | 1 4
    // (1-a) * (1-an_b) = an_union_bn | 0 3
    // (1-b) * (1-bn_a) = an_union_bn | 1 5
    let (probabilities, probabilities_final) = probabilities_start.split_at_mut(6);
    // special case, where not all probabilities can be found
    // a_union_b + a_union_bn + an_union_b + an_union_bn = 1
    if probabilities_final.iter().flatten().count() == 3 {
        probabilities_final[probabilities_final
            .iter()
            .position(Option::is_none)
            .unwrap()] =
            Some(Fraction::ONE - probabilities_final.iter().flatten().sum::<Fraction>());
    }
    let mut new_information_found = true;
    while new_information_found {
        new_information_found = false;
        debug!("Start of round:");
        // a_union_b + a_union_bn = p_a
        if let (Some(p1), Some(p2)) = (probabilities_final[0], probabilities_final[1]) {
            debug!("a_union_b + an_union_bn = p_a = {}", p1 + p2);
            probabilities[0] = Some(p1 + p2);
        }
        // an_union_b + an_union_bn = 1 - p_a
        if let (Some(p1), Some(p2)) = (probabilities_final[2], probabilities_final[3]) {
            debug!("an_union_b + an_union_bn = p_an = {}", p1 + p2);
            probabilities[0] = Some(Fraction::ONE - (p1 + p2));
        }
        // a_union_b + an_union_b = p_b
        if let (Some(p1), Some(p2)) = (probabilities_final[0], probabilities_final[2]) {
            debug!("a_union_b + an_union_b = p_b = {}", p1 + p2);
            probabilities[1] = Some(p1 + p2);
        }
        // a_union_bn + an_union_bn = 1 - p_a
        if let (Some(p1), Some(p2)) = (probabilities_final[1], probabilities_final[3]) {
            debug!("a_union_bn + an_union_bn= p_bn = {}", p1 + p2);
            probabilities[1] = Some(Fraction::ONE - (p1 + p2));
        }
        for (i, p_final) in probabilities_final.iter_mut().enumerate() {
            let (ab, rest) = probabilities.split_at_mut(2);
            let p1 = &mut ab[0];
            *p1 = p1.map(|p| if i / 2 == 1 { Fraction::ONE - p } else { p });
            let p1_cond = &mut rest[i / 2];
            *p1_cond = p1_cond.map(|p| if i % 2 == 1 { Fraction::ONE - p } else { p });
            new_information_found |= calculate_missing_probabilitiy([p1, p1_cond, p_final]);
            *p1 = p1.map(|p| if i / 2 == 1 { Fraction::ONE - p } else { p });
            *p1_cond = p1_cond.map(|p| if i % 2 == 1 { Fraction::ONE - p } else { p });
            if new_information_found {
                if let (Some(x), Some(y), Some(z)) = (p1, p1_cond, &p_final) {
                    debug!("after calculate p1 - {i}: {x} {y} {z}");
                }
            }

            let p2 = &mut ab[1];
            *p2 = p2.map(|p| if i % 2 == 1 { Fraction::ONE - p } else { p });
            let p2_cond = &mut rest[i % 2 + 2];
            *p2_cond = p2_cond.map(|p| if i / 2 == 1 { Fraction::ONE - p } else { p });
            new_information_found |= calculate_missing_probabilitiy([p2, p2_cond, p_final]);
            *p2 = p2.map(|p| if i % 2 == 1 { Fraction::ONE - p } else { p });
            *p2_cond = p2_cond.map(|p| if i / 2 == 1 { Fraction::ONE - p } else { p });
            if new_information_found {
                if let (Some(x), Some(y), Some(z)) = (p2, p2_cond, &p_final) {
                    debug!("after calculate p2 - {i}: {x} {y} {z}")
                }
            }
        }
    }
    probabilities_start
}

fn calculate_missing_probabilitiy(array: [&mut Option<Fraction>; 3]) -> bool {
    if 2 != array.iter().filter_map(|x| **x).count() {
        return false;
    }

    debug!("{}", array.iter().position(|x| x.is_none()).unwrap());
    match array.iter().position(|x| x.is_none()).unwrap() {
        0 => *array[0] = (*array[2]).zip_with(*array[1], Fraction::div),
        1 => *array[1] = (*array[2]).zip_with(*array[0], Fraction::div),
        2 => *array[2] = (*array[0]).zip_with(*array[1], Fraction::mul),
        _ => unreachable!("Array size cannot be larger than 3"),
    };
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p_a_and_p_an_union_bn_and_p_b_an() {
        let probabilities = [
            Some(Fraction::new(3u64, 5u64)),
            None,
            None,
            None,
            Some(Fraction::new(3u64, 5u64)),
            None,
            None,
            None,
            None,
            Some(Fraction::new(1u64, 10u64)),
        ];

        let p_new = calculate_missing_probabilities(probabilities);
        let names = [
            "p_a",
            "p_b",
            "p_a_b",
            "p_an_b",
            "p_b_a",
            "p_bn_a",
            "p_a_union_b",
            "p_a_union_bn",
            "p_an_union_b",
            "p_an_union_bn",
        ];
        for (p, name) in p_new.map(Option::unwrap).iter().zip(names) {
            debug!("{name}, {p}");
        }
        let p_correct = [
            (3u32, 5u32),
            (3, 4),
            (3, 4),
            (3, 4),
            (3, 5),
            (3, 5),
            (9u32, 20u32),
            (3, 20),
            (3, 10),
            (1, 10),
        ]
        .map(|(n, d)| Fraction::new(n, d))
        .map(Some);
        assert_eq!(p_new, p_correct);
    }

    #[test]
    fn end_probabilities() {
        let probabilities = [
            None,
            None,
            None,
            None,
            None,
            None,
            Some((1u32, 4u32)),
            Some((1u32, 4u32)),
            Some((1, 4)),
            None,
        ]
        .map(|opt| opt.map(|(n, d)| Fraction::new(n, d)));

        let p_new = calculate_missing_probabilities(probabilities);
        assert!(p_new[6..10]
            .iter()
            .all(|x| *x == Some(Fraction::new(1u32, 4u32))));
    }
}
