use crate::{
    config::Config,
    learning::{self, save_learning, Answer, LearnState},
    question,
};
use eframe::{
    egui::{self, FontId, RichText, Vec2},
    epaint::Color32,
};
use std::collections::HashMap;
use std::path::Path;

pub fn run(
    mut config: Config,
    mut learning: HashMap<String, LearnState>,
    questions: Vec<question::Question>,
) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 1000.0]),
        ..Default::default()
    };

    // Application state
    let mut eligible_questions = question::get_eligible_questions(&questions, &config);
    let mut print_question =
        learning::get_next_print_question(&eligible_questions, &mut learning, &config);
    let mut has_answered = false;
    let mut has_answered_first = false;
    let mut given_answer: usize = 0;
    let mut answer_text = String::from("");

    const IMAGE_WIDTH: f32 = 250.0;
    const IMAGE_HEIGHT: f32 = 250.0;

    let index_name_tuples = vec![(0, "A"), (1, "B"), (2, "C"), (3, "D")];

    // GUI routines
    eframe::run_simple_native("Funkfragenhelfer V 0.1", options, move |ctx, _frame| {
        egui_extras::install_image_loaders(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                answer_text = String::from("Bild");

                ui.label(RichText::new("Bereitsteller der Fragen: Bundesnetzagentur, \
                                                   Datensatz: 'Prüfungsfragen zum Erwerb von Amateurfunkprüfungsbescheinigungen, \
                                                   2. Auflage, Dezember 2023', Lizenz der Fragen: dl-de/by-2-0"
                ).font(FontId::proportional(10.0)).color(Color32::DARK_GRAY));

                ui.horizontal(|ui| {
                    ui.label("Frageklassen:");
                    if ui.checkbox(&mut config.include_v, "V").changed() {
                        if config.all_includes_false() {
                            config.include_v = true;
                        }
                        config.save();
                        eligible_questions = question::get_eligible_questions(&questions, &config);
                    }
                    if ui.checkbox(&mut config.include_b, "B").changed() {
                        if config.all_includes_false() {
                            config.include_b = true;
                        }
                        config.save();
                        eligible_questions = question::get_eligible_questions(&questions, &config);
                    }
                    if ui.checkbox(&mut config.include_n, "N").changed() {
                        if config.all_includes_false() {
                            config.include_n = true;
                        }
                        config.save();
                        eligible_questions = question::get_eligible_questions(&questions, &config);
                    }
                    if ui.checkbox(&mut config.include_e, "E").changed() {
                        if config.all_includes_false() {
                            config.include_e = true;
                        }
                        config.save();
                        eligible_questions = question::get_eligible_questions(&questions, &config);
                    }
                    if ui.checkbox(&mut config.include_a, "A").changed() {
                        if config.all_includes_false() {
                            config.include_a = true;
                        }
                        config.save();
                        eligible_questions = question::get_eligible_questions(&questions, &config);
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Nur anzuzeigen (wenn gegeben):");
                    if ui.checkbox(&mut config.prefer_wrong, "Falsch beantwortete Fragen").changed() {
                        config.save();
                    }
                    if ui.checkbox(&mut config.prefer_marked, "Markierte Fragen").changed() {
                        config.save();
                    }
                    if ui.checkbox(&mut config.prefer_new, "Neue Fragen").changed() {
                        config.save();
                    }
                });

                ui.separator();
                if print_question.question.picture_question.len() > 0 {
                    let pathstr = format!(
                        "file://resources/fragenkatalog/svgs/{}.svg",
                        print_question.question.picture_question
                    );
                    let relpath = format!(
                        "./resources/fragenkatalog/svgs/{}.svg",
                        print_question.question.picture_question
                    );
                    let path = Path::new(&relpath);

                    if path.exists() {
                        ui.add(
                            egui::Image::new(pathstr)
                                .fit_to_exact_size(Vec2::new(IMAGE_WIDTH, IMAGE_HEIGHT))
                                .maintain_aspect_ratio(true)
                                .bg_fill(Color32::DARK_GRAY),
                        );
                    }
                }

                ui.heading(&print_question.question.identifier);
                ui.label(&print_question.question.question);

                if print_question.question.picture_a.len() > 0 {
                    ui.separator();

                    ui.horizontal(|ui| {
                        for &index_name_tuple in &index_name_tuples {
                            ui.label(format!("{}:", index_name_tuple.1));
                            ui.add(
                                egui::Image::new(format!(
                                    "file://resources/fragenkatalog/svgs/{}.svg",
                                    print_question.get_shuffled_picture(index_name_tuple.0)
                                ))
                                .fit_to_exact_size(Vec2::new(IMAGE_WIDTH, IMAGE_HEIGHT))
                                .maintain_aspect_ratio(true)
                                .bg_fill(Color32::DARK_GRAY),
                            );
                        }
                    });
                } else {
                    answer_text = String::from("Antwort");
                }

                for &index_name_tuple in &index_name_tuples {
                    ui.separator();
                    if ui.button(format!("{} {}", answer_text, index_name_tuple.1)).clicked() {
                        if !has_answered {
                            has_answered = true;
                            has_answered_first = true;
                            given_answer = index_name_tuple.0;
                        }
                    }
                    ui.label(print_question.get_shuffled_answer(index_name_tuple.0));
                }

                ui.separator();
                if learning.get(&print_question.question.identifier).unwrap().marked {
                    if ui.button("[X] Entmarkieren").clicked() {
                        learning.get_mut(&print_question.question.identifier).unwrap().marked = false;
                    }
                    learning::save_learning(&learning);
                } else {
                    if ui.button("[ ] Markieren").clicked() {
                        learning.get_mut(&print_question.question.identifier).unwrap().marked = true;
                    }
                    learning::save_learning(&learning);
                }

                ui.separator();
                if has_answered {
                    if print_question.answer_shuffle[given_answer] == Answer::A {
                        ui.label("Korrekt!");
                        if has_answered_first {
                            learning::handle_correct_answer(
                                &mut learning,
                                &print_question.question.identifier,
                                &config,
                            );
                            learning::save_learning(&learning);
                            has_answered_first = false;
                        }
                    } else {
                        ui.label(format!(
                            "Falsch! Richtige Antwort ist {:?}",
                            print_question.get_correct_answer()
                        ));
                        if has_answered_first {
                            learning::handle_wrong_answer(
                                &mut learning,
                                &print_question.question.identifier,
                            );
                            learning::save_learning(&learning);
                            has_answered_first = false;
                        }
                    }
                    if ui.button("Nächste Frage").clicked() {
                        print_question =
                            learning::get_next_print_question(&eligible_questions, &mut learning, &config);
                        has_answered = false;
                    }
                } else {
                    if ui.button("Überspringen").clicked() {
                        print_question =
                            learning::get_next_print_question(&eligible_questions, &mut learning, &config);
                    }
                }
            });
        });
    })
}
