use crate::models::{ConsonantCluster, PhonemizerProject};
use crate::ui::helpers::{help_marker, section_header};
use egui::Ui;

pub fn render_clusters_view(ui: &mut Ui, project: &mut PhonemizerProject) {
    section_header(
        ui,
        "Encontros Consonantais (Onset & Coda Clusters)",
        Some("Configure o desmembramento de encontros consonantais, tempos relativos, limites de BPM e compressao em andamentos rapidos."),
    );

    ui.columns(2, |columns| {

        columns[0].vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Ataque (Onset Clusters: br, pl, str)").heading().size(15.0));
                help_marker(ui, "Clusters de consoantes que iniciam uma silaba antes da vogal.");
            });

            if ui.button("Adicionar Onset Cluster").clicked() {
                project.onset_clusters.push(ConsonantCluster::new("novo", vec!["c1".to_string(), "c2".to_string()], true));
            }
            ui.add_space(5.0);

            let mut remove_onset = None;
            egui::ScrollArea::vertical().id_salt("onset_clusters_scroll").show(ui, |ui| {
                for (idx, cluster) in project.onset_clusters.iter_mut().enumerate() {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Cluster:");
                            ui.text_edit_singleline(&mut cluster.cluster);
                            help_marker(ui, "Texto do encontro consonantal (ex: 'br', 'str').");
                            if ui.button("Excluir").clicked() {
                                remove_onset = Some(idx);
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label("Componentes:");
                            let mut comps = cluster.components.join(" ");
                            if ui.text_edit_singleline(&mut comps).changed() {
                                cluster.components = comps.split_whitespace().map(|s| s.to_string()).collect();
                            }
                            help_marker(ui, "Consoantes individuais separadas por espaco (ex: 'b rh' ou 's t r').");
                        });
                        ui.horizontal(|ui| {
                            ui.label("BPM Maximo:");
                            ui.add(egui::DragValue::new(&mut cluster.bpm_shortening_threshold).range(60..=300));
                            ui.checkbox(&mut cluster.allow_elision_if_fast, "Elisao rapida");
                            help_marker(ui, "Se o andamento da musica for superior a este BPM, comprime o cluster para nao atrasar a nota.");
                        });
                    });
                }
            });
            if let Some(idx) = remove_onset {
                if idx < project.onset_clusters.len() {
                    project.onset_clusters.remove(idx);
                }
            }
        });

        columns[1].vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Coda (Coda Clusters: st, nd, lk)").heading().size(15.0));
                help_marker(ui, "Clusters de consoantes que encerram uma silaba apos a vogal.");
            });

            if ui.button("Adicionar Coda Cluster").clicked() {
                project.coda_clusters.push(ConsonantCluster::new("novo_coda", vec!["c1".to_string(), "c2".to_string()], false));
            }
            ui.add_space(5.0);

            let mut remove_coda = None;
            egui::ScrollArea::vertical().id_salt("coda_clusters_scroll").show(ui, |ui| {
                for (idx, cluster) in project.coda_clusters.iter_mut().enumerate() {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Cluster:");
                            ui.text_edit_singleline(&mut cluster.cluster);
                            help_marker(ui, "Texto do cluster final de silaba (ex: 'st', 'nd').");
                            if ui.button("Excluir").clicked() {
                                remove_coda = Some(idx);
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label("Componentes:");
                            let mut comps = cluster.components.join(" ");
                            if ui.text_edit_singleline(&mut comps).changed() {
                                cluster.components = comps.split_whitespace().map(|s| s.to_string()).collect();
                            }
                            help_marker(ui, "Consoantes individuais que compoem o cluster final.");
                        });
                    });
                }
            });
            if let Some(idx) = remove_coda {
                if idx < project.coda_clusters.len() {
                    project.coda_clusters.remove(idx);
                }
            }
        });
    });
}
