//! This module defines the full Funkfragenhelfer GUI using egui and eframe.
//! Note that egui uses an "immediate mode" GUI concept, which means
//! that the GUI is written in each frame, so that the behavior of GUI
//! elements is written together with their action behavior (i.e.,
//! GUI elements are declared and act right in the run loop).
//!
//! If you wish to change Frunkfragenhelfer's GUI framework, you just
//! have to edit this file as the other modules are GUI-framework-agnostic.

// IMPORTS SECTION //
use crate::{
    config::Config,
    learning::{self, save_learning, Answer, LearnStates, Statistics},
    question,
};
use eframe::{
    egui::{self, FontId, RichText, Vec2},
    epaint::Color32,
};
use std::path::Path;

// CONSTANTS SECTION //
/// Maximal image width for the GUI display
const MAX_IMAGE_WIDTH: f32 = 250.0;
/// Maximal image height for the GUI display
const MAX_IMAGE_HEIGHT: f32 = 250.0;

/// Runs the Funkfragenhelfer GUI
///
/// Here, the full GUI and all its actions are defined.
///
/// ### Arguments
/// * config: The current Funkfragenhelfer configuration
/// * learn_states: The current question LearnStates
/// * questions: The set of all questions that can be asked
pub fn run(
    mut config: Config,
    mut learn_states: LearnStates,
    questions: Vec<question::Question>,
) -> Result<(), eframe::Error> {
    // Set the egui options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 1000.0]),
        ..Default::default()
    };

    // Application state
    let mut has_answered = false;
    let mut has_answered_first = false;
    let mut eligible_questions = question::get_eligible_questions(&questions, &config);
    let mut given_answer: usize = 0;
    let mut print_question =
        learning::get_next_print_question(&eligible_questions, &mut learn_states, &config);
    let mut correct_answers_since_start = 0;
    let mut answers_since_start = 0;
    let mut statistics = Statistics::new(&eligible_questions, &learn_states);

    let index_name_tuples = vec![(0, "A"), (1, "B"), (2, "C"), (3, "D")];

    // GUI main run loop; We use "run_simple_native" as the simplest
    // egui wrapper available.
    eframe::run_simple_native("Funkfragenhelfer V 0.2.1", options, move |ctx, _frame| {
        // Image loader do teh question images; Can also load SVGs
        egui_extras::install_image_loaders(ctx);

        // Central widget which includes all other widhets
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                let mut answer_text = String::from("Bild");

                // Show question data source
                ui.label(RichText::new("Bereitsteller der Fragen: Bundesnetzagentur, \
                                                   Datensatz: 'Prüfungsfragen zum \
                                                   Erwerb von Amateurfunkprüfungsbescheinigungen, \
                                                   3. Auflage, März 2024', Lizenz der Fragen: dl-de/by-2-0"
                ).font(FontId::proportional(10.0)).color(Color32::DARK_GRAY));

                // Show and handle question categories
                ui.horizontal(|ui| {
                    ui.label("Fragekategorien:");
                    let mut update_config = |config: &mut Config| {
                        config.save();
                        eligible_questions = question::get_eligible_questions(&questions, &config);
                        statistics = Statistics::new(&eligible_questions, &learn_states);
                    };
                    if ui.checkbox(&mut config.include_v, "V").changed() {
                        if config.all_includes_false() {
                            config.include_v = true;
                        }
                        update_config(&mut config);
                    }
                    if ui.checkbox(&mut config.include_b, "B").changed() {
                        if config.all_includes_false() {
                            config.include_b = true;
                        }
                        update_config(&mut config);
                    }
                    if ui.checkbox(&mut config.include_n, "N").changed() {
                        if config.all_includes_false() {
                            config.include_n = true;
                        }
                        update_config(&mut config);
                    }
                    if ui.checkbox(&mut config.include_e, "E").changed() {
                        if config.all_includes_false() {
                            config.include_e = true;
                        }
                        update_config(&mut config);
                    }
                    if ui.checkbox(&mut config.include_a, "A").changed() {
                        if config.all_includes_false() {
                            config.include_a = true;
                        }
                        update_config(&mut config);
                    }
                });

                // Show and handle question filters
                ui.horizontal(|ui| {
                    ui.label("Filter (falls anwendbar):");
                    if ui.checkbox(&mut config.prefer_marked, "Markierte Fragen").changed() {
                        config.save();
                    }
                    if ui.checkbox(&mut config.prefer_wrong, "Falsch beantwortete Fragen").changed() {
                        config.save();
                    }
                    if ui.checkbox(&mut config.prefer_new, "Noch nicht beantwortete Fragen").changed() {
                        config.save();
                    }
                });

                // Show picture question if one exists. Here, it is also made sure that such
                // a picture really exists as there seem to be some wrong picture associations
                // in the Bundesnetzagentur dataset.
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
                                .fit_to_exact_size(Vec2::new(MAX_IMAGE_WIDTH, MAX_IMAGE_HEIGHT))
                                .maintain_aspect_ratio(true)
                                .bg_fill(Color32::DARK_GRAY),
                        );
                    }
                }

                // Print the current question's identifier and the question itself
                // (without answers yet).
                ui.heading(&print_question.question.identifier);
                ui.label(&print_question.question.question);

                // Show picture answers (if they exist)
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
                                .fit_to_exact_size(Vec2::new(MAX_IMAGE_WIDTH, MAX_IMAGE_HEIGHT))
                                .maintain_aspect_ratio(true)
                                .bg_fill(Color32::DARK_GRAY),
                            );
                        }
                    });
                } else {
                    answer_text = String::from("Antwort");
                }

                // Handle answer printing (if an answer has no text, "" is displayed)
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

                // Handling of question marking
                ui.separator();
                if learn_states.get(&print_question.question.identifier).unwrap().marked {
                    if ui.button("[X] Entmarkieren").clicked() {
                        learn_states.get_mut(&print_question.question.identifier).unwrap().marked = false;
                    }
                    save_learning(&learn_states);
                } else {
                    if ui.button("[ ] Markieren").clicked() {
                        learn_states.get_mut(&print_question.question.identifier).unwrap().marked = true;
                    }
                    save_learning(&learn_states);
                }

                // Handling of the case that the user answered the question
                ui.separator();
                if has_answered {
                    if print_question.answer_shuffle[given_answer] == Answer::A {
                        ui.label("Korrekt!");
                        if has_answered_first {
                            correct_answers_since_start += 1;
                            answers_since_start += 1;
                            learning::handle_correct_answer(
                                &mut learn_states,
                                &print_question.question.identifier,
                                &config,
                            );
                            learning::save_learning(&learn_states);
                            statistics = Statistics::new(&eligible_questions, &learn_states);
                            has_answered_first = false;
                        }
                    } else {
                        ui.label(format!(
                            "Falsch! Richtige Antwort ist {:?}",
                            print_question.get_correct_answer()
                        ));
                        if has_answered_first {
                            answers_since_start += 1;
                            learning::handle_wrong_answer(
                                &mut learn_states,
                                &print_question.question.identifier,
                            );
                            learning::save_learning(&learn_states);
                            statistics = Statistics::new(&eligible_questions, &learn_states);
                            has_answered_first = false;
                        }
                    }
                    if ui.button("Nächste Frage").clicked() {
                        print_question =
                            learning::get_next_print_question(&eligible_questions, &mut learn_states, &config);
                        has_answered = false;
                    }
                } else {
                    if ui.button("Überspringen").clicked() {
                        print_question =
                            learning::get_next_print_question(&eligible_questions, &mut learn_states, &config);
                    }
                }
                ui.separator();
                ui.label(RichText::new(format!("Aktuelle Session:")).strong());
                ui.label(format!("Korrekt beantwortete Fragen: {}, {} %", correct_answers_since_start, if answers_since_start > 0 {correct_answers_since_start * 100 / answers_since_start} else {0}));
                ui.label(format!("Insgesamt beantwortete Fragen: {}", answers_since_start));
                ui.separator();
                ui.label(RichText::new(format!("Lernfortschritt:")).strong());
                ui.label(format!("Korrekt beantwortete Fragen: {}, {} %", statistics.correct_answers, if statistics.questions > 0 {statistics.correct_answers * 100 / statistics.questions} else {0}));
                ui.label(format!("Noch nicht korrekt beantwortete Fragen: {}, {} %", statistics.no_correct_answers, if statistics.questions > 0 {statistics.no_correct_answers * 100 / statistics.questions} else {0}));
                ui.label(format!("Fragen insgesamt: {}", statistics.questions));
                ui.separator();
                let mut keys = statistics.count_per_bin.keys().collect::<Vec<_>>();
                keys.sort();
                ui.label(RichText::new(format!("Fragen pro Lerntopf:")).strong());
                for i in keys.iter() {
                    let count = &statistics.count_per_bin.get(i).unwrap();
                    ui.label(format!("Lerntopf '{}': {}", i, count));
                }
            });
        });
    })
}
