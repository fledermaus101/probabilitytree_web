use std::{
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

    p_a: String,
    p_an: String,
    p_b: String,
    p_bn: String,

    p_a_b: String,
    p_a_bn: String,
    p_an_b: String,
    p_an_bn: String,

    p_b_a: String,
    p_b_an: String,
    p_bn_a: String,
    p_bn_an: String,

    p_a_union_b: String,
    p_a_union_bn: String,
    p_an_union_b: String,
    p_an_union_bn: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            event_a_name: String::from("A"),
            event_an_name: String::from("A'"),
            event_b_name: String::from("B"),
            event_bn_name: String::from("B'"),
            p_a: String::new(),
            p_an: String::new(),
            p_b: String::new(),
            p_bn: String::new(),
            p_a_b: String::new(),
            p_a_bn: String::new(),
            p_an_b: String::new(),
            p_an_bn: String::new(),
            p_b_a: String::new(),
            p_b_an: String::new(),
            p_bn_a: String::new(),
            p_bn_an: String::new(),
            p_a_union_b: String::new(),
            p_a_union_bn: String::new(),
            p_an_union_b: String::new(),
            p_an_union_bn: String::new(),
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
                let (probabilities, probabilities_final) = parse_probabilities(self);
                let (probabilities, probabilities_final) =
                    calculate_missing_probabilities(probabilities, probabilities_final);
                self.p_a = probabilities[0]
                    .map(|x| x.to_string())
                    .unwrap_or(String::new());
                self.p_an = probabilities[0]
                    .map(|x| (Fraction::ONE - x).to_string())
                    .unwrap_or(String::new());

                self.p_b = probabilities[1]
                    .map(|x| x.to_string())
                    .unwrap_or(String::new());
                self.p_bn = probabilities[1]
                    .map(|x| (Fraction::ONE - x).to_string())
                    .unwrap_or(String::new());

                self.p_a_b = probabilities[2]
                    .map(|x| x.to_string())
                    .unwrap_or(String::new());
                self.p_a_bn = probabilities[2]
                    .map(|x| (Fraction::ONE - x).to_string())
                    .unwrap_or(String::new());

                self.p_an_b = probabilities[3]
                    .map(|x| x.to_string())
                    .unwrap_or(String::new());
                self.p_an_bn = probabilities[3]
                    .map(|x| (Fraction::ONE - x).to_string())
                    .unwrap_or(String::new());

                self.p_b_a = probabilities[4]
                    .map(|x| x.to_string())
                    .unwrap_or(String::new());
                self.p_b_an = probabilities[4]
                    .map(|x| (Fraction::ONE - x).to_string())
                    .unwrap_or(String::new());

                self.p_bn_a = probabilities[5]
                    .map(|x| x.to_string())
                    .unwrap_or(String::new());
                self.p_bn_an = probabilities[5]
                    .map(|x| (Fraction::ONE - x).to_string())
                    .unwrap_or(String::new());

                self.p_a_union_b = probabilities_final[0]
                    .map(|x| x.to_string())
                    .unwrap_or(String::new());
                self.p_a_union_bn = probabilities_final[1]
                    .map(|x| x.to_string())
                    .unwrap_or(String::new());
                self.p_an_union_b = probabilities_final[2]
                    .map(|x| x.to_string())
                    .unwrap_or(String::new());
                self.p_an_union_bn = probabilities_final[3]
                    .map(|x| x.to_string())
                    .unwrap_or(String::new());
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns_const(|[tree_1, tree_2]| {
                tree_1.allocate_ui_with_layout(
                    Vec2 { x: 100., y: 0. },
                    Layout::centered_and_justified(Direction::TopDown),
                    |ui| {
                        ui.label("1");
                    },
                );
                tree_1.allocate_ui_with_layout(
                    Vec2 { x: 200., y: 0. },
                    Layout::left_to_right(Align::Min),
                    |ui| {
                        ui.columns(2, |ui| {
                            ui[0].text_edit_singleline(&mut self.p_a);
                            ui[1].text_edit_singleline(&mut self.p_an);
                        })
                    },
                );
                tree_1.allocate_ui_with_layout(
                    Vec2::new(200., 0.),
                    Layout::left_to_right(Align::Min),
                    |ui| {
                        ui.columns(2, |ui| {
                            ui[0].vertical_centered(|ui| ui.label(&self.event_a_name));
                            ui[1].vertical_centered(|ui| ui.label(&self.event_an_name));
                        })
                    },
                );
                tree_1.allocate_ui_with_layout(
                    Vec2::new(200., 0.),
                    Layout::left_to_right(Align::Min),
                    |ui| {
                        ui.columns(4, |ui| {
                            ui[0].text_edit_singleline(&mut self.p_a_b);
                            ui[1].text_edit_singleline(&mut self.p_a_bn);
                            ui[2].text_edit_singleline(&mut self.p_an_b);
                            ui[3].text_edit_singleline(&mut self.p_an_bn);
                        });
                    },
                );
                tree_1.allocate_ui_with_layout(
                    Vec2 { x: 200., y: 0. },
                    Layout::left_to_right(Align::Min),
                    |ui| {
                        ui.columns(4, |ui| {
                            ui[0].vertical_centered(|ui| ui.label(&self.event_b_name));
                            ui[1].vertical_centered(|ui| ui.label(&self.event_bn_name));
                            ui[2].vertical_centered(|ui| ui.label(&self.event_b_name));
                            ui[3].vertical_centered(|ui| ui.label(&self.event_bn_name));
                        });
                    },
                );
                tree_1.allocate_ui_with_layout(
                    Vec2 { x: 200., y: 0. },
                    Layout::left_to_right(Align::Min),
                    |ui| {
                        ui.columns(4, |ui| {
                            ui[0].vertical_centered(|ui| {
                                ui.text_edit_singleline(&mut self.p_a_union_b)
                            });
                            ui[1].vertical_centered(|ui| {
                                ui.text_edit_singleline(&mut self.p_a_union_bn)
                            });
                            ui[2].vertical_centered(|ui| {
                                ui.text_edit_singleline(&mut self.p_an_union_b)
                            });
                            ui[3].vertical_centered(|ui| {
                                ui.text_edit_singleline(&mut self.p_an_union_bn)
                            });
                        });
                    },
                );
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
                        ui.columns(2, |ui| {
                            ui[0].text_edit_singleline(&mut self.p_b);
                            ui[1].text_edit_singleline(&mut self.p_bn);
                        })
                    },
                );
                tree_2.allocate_ui_with_layout(
                    Vec2::new(200., 0.),
                    Layout::left_to_right(Align::Min),
                    |ui| {
                        ui.columns(2, |ui| {
                            ui[0].vertical_centered(|ui| ui.label(&self.event_b_name));
                            ui[1].vertical_centered(|ui| ui.label(&self.event_bn_name));
                        })
                    },
                );
                tree_2.allocate_ui_with_layout(
                    Vec2::new(200., 0.),
                    Layout::left_to_right(Align::Min),
                    |ui| {
                        ui.columns(4, |ui| {
                            ui[0].text_edit_singleline(&mut self.p_b_a);
                            ui[1].text_edit_singleline(&mut self.p_b_an);
                            ui[2].text_edit_singleline(&mut self.p_bn_a);
                            ui[3].text_edit_singleline(&mut self.p_bn_an);
                        });
                    },
                );
                tree_2.allocate_ui_with_layout(
                    Vec2 { x: 200., y: 0. },
                    Layout::left_to_right(Align::Min),
                    |ui| {
                        ui.columns(4, |ui| {
                            ui[0].vertical_centered(|ui| ui.label(&self.event_a_name));
                            ui[1].vertical_centered(|ui| ui.label(&self.event_an_name));
                            ui[2].vertical_centered(|ui| ui.label(&self.event_a_name));
                            ui[3].vertical_centered(|ui| ui.label(&self.event_an_name));
                        });
                    },
                );
                tree_2.allocate_ui_with_layout(
                    Vec2 { x: 200., y: 0. },
                    Layout::left_to_right(Align::Min),
                    |ui| {
                        ui.columns(4, |ui| {
                            ui[0].vertical_centered(|ui| {
                                ui.text_edit_singleline(&mut self.p_a_union_b)
                            });
                            ui[1].vertical_centered(|ui| {
                                ui.text_edit_singleline(&mut self.p_an_union_b)
                            });
                            ui[2].vertical_centered(|ui| {
                                ui.text_edit_singleline(&mut self.p_a_union_bn)
                            });
                            ui[3].vertical_centered(|ui| {
                                ui.text_edit_singleline(&mut self.p_an_union_bn)
                            });
                        });
                    },
                );
            });
        });
    }
}

fn parse_probabilities(app: &App) -> ([Option<Fraction>; 6], [Option<Fraction>; 4]) {
    let probabilities = [
        (&app.p_a, &app.p_an),
        (&app.p_b, &app.p_bn),
        (&app.p_a_b, &app.p_a_bn),
        (&app.p_an_b, &app.p_an_bn),
        (&app.p_b_a, &app.p_b_an),
        (&app.p_bn_a, &app.p_bn_an),
    ]
    .map(
        |(p, pn)| match (Fraction::from_str(p), Fraction::from_str(pn)) {
            (Ok(p), Ok(pn)) => (p + pn == Fraction::one()).then_some(p),
            (Ok(p), Err(_)) => Some(p),
            (Err(_), Ok(pn)) => Some(Fraction::one() - pn),
            (Err(_), Err(_)) => None,
        },
    );

    let probabilities_final = [
        &app.p_a_union_b,
        &app.p_a_union_bn,
        &app.p_an_union_b,
        &app.p_an_union_bn,
    ]
    .map(|p| Fraction::from_str(p).ok());
    (probabilities, probabilities_final)
}

fn calculate_missing_probabilities(
    mut probabilities: [Option<Fraction>; 6],
    mut probabilities_final: [Option<Fraction>; 4],
) -> ([Option<Fraction>; 6], [Option<Fraction>; 4]) {
    let mut new_information_found = true;
    while new_information_found {
        new_information_found = false;
        if probabilities_final.into_iter().flatten().count() == 3 {
            probabilities_final[probabilities_final
                .into_iter()
                .position(|x| x.is_none())
                .unwrap()] =
                Some(Fraction::ONE - probabilities_final.into_iter().flatten().sum::<Fraction>());
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
        println!("New information found! Next round!");
        println!();
    }
    // p_a
    // p_b
    // p_a_b
    // p_an_b
    // p_b_a
    // p_bn_a
    // a_union_b + a_union_bn + an_union_b + an_union_bn = 1
    // <=>
    // a_union_b + a_union_bn = a
    // a_union_b + an_union_b = b
    // path rules
    // a * a_b = a_union_b | 0 2
    // b * b_a = a_union_b | 1 4
    // a * (1-a_b)  = a_union_bn | 0 2
    // (1-b) * bn_a = a_union_bn | 1 5
    // (1-a) * an_b = an_union_b | 0 3
    // b * (1-b_a)  = an_union_b | 1 4
    // (1-a) * (1-an_b) = an_union_bn | 0 3
    // (1-b) * (1-bn_a) = an_union_bn | 1 5
    (probabilities, probabilities_final)
}

fn calculate_missing_probabilitiy(array: [&mut Option<Fraction>; 3]) -> bool {
    if 2 != array.iter().filter_map(|x| **x).count() {
        return false;
    }

    println!("{}", array.iter().position(|x| x.is_none()).unwrap());
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
        ];
        let probabilities_final = [None, None, None, Some(Fraction::new(1u64, 10u64))];

        let (p_new, p_new_final) =
            calculate_missing_probabilities(probabilities, probabilities_final);
        let names = ["p_a", "p_b", "p_a_b", "p_an_b", "p_b_a", "p_bn_a"];
        for (p, name) in p_new.map(Option::unwrap).iter().zip(names) {
            println!("{name}, {p}");
        }
        let p_correct = [(3u32, 5u32), (3, 4), (3, 4), (3, 4), (3, 5), (3, 5)]
            .map(|(n, d)| Fraction::new(n, d))
            .map(Some);
        let p_final_correct = [(9u32, 20u32), (3, 20), (3, 10), (1, 10)]
            .map(|(n, d)| Fraction::new(n, d))
            .map(Some);
        assert_eq!(p_new, p_correct);
        assert_eq!(p_new_final, p_final_correct);
    }

    #[test]
    fn end_probabilities() {
        let probabilities = [None; 6];
        let probabilities_final = [Some((1u32, 4u32)), Some((1u32, 4u32)), Some((1, 4)), None]
            .map(|opt| opt.map(|(n, d)| Fraction::new(n, d)));

        let (_, p_new_final) = calculate_missing_probabilities(probabilities, probabilities_final);
        assert!(p_new_final
            .iter()
            .all(|x| *x == Some(Fraction::new(1u32, 4u32))));
    }
}
