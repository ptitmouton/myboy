use egui::TextStyle;
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use mygbcartridge::cartridge::Cartridge;

pub struct AsmTextTable {
    pub cartridge: Cartridge,
    pub selected_address: Option<u16>,
    pub scroll_to_selected: bool,
}

impl egui::Widget for AsmTextTable {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.heading(&self.cartridge.get_title());
            let body_text_size = TextStyle::Body.resolve(ui.style()).size;
            StripBuilder::new(ui)
                .size(Size::remainder().at_least(100.0)) // for the table
                .size(Size::exact(body_text_size)) // for the source code link
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        egui::ScrollArea::horizontal().show(ui, |ui| {
                            self.asm_text_table(ui);
                        });
                    });
                });

            ui.response()
        })
        .response
    }
}

impl AsmTextTable {
    pub fn asm_text_table(&self, ui: &mut egui::Ui) {
        self.create_table_builder(ui)
            .striped(true)
            .resizable(false)
            .column(Column::auto().at_most(50.0))
            .column(
                Column::remainder()
                    .at_least(40.0)
                    .clip(true)
                    .resizable(true),
            )
            .column(Column::auto())
            .sense(egui::Sense::click())
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Address");
                });
                header.col(|ui| {
                    ui.strong("Text");
                });
                header.col(|ui| {
                    ui.strong("mnemonic");
                });
            })
            .body(|body| {
                body.rows(25.0, self.cartridge.data.len(), |mut row| {
                    let index = row.index();
                    if let Some(selected_address) = self.selected_address {
                        println!(
                            "row.index(): {}, self.selected_address: {}",
                            index, selected_address
                        );
                        row.set_selected(selected_address as usize == index);
                    }
                    row.col(|ui| {
                        ui.label(format!("0x{:04X}", index));
                    });
                    row.col(|ui| {
                        ui.label(format!("0x{:02X}", self.cartridge.data[index]));
                    });
                    row.col(|ui| {
                        ui.label("?");
                    });
                })
            });
    }

    fn create_table_builder<'a>(&'a self, ui: &'a mut egui::Ui) -> TableBuilder<'a> {
        if self.scroll_to_selected {
            if let Some(target_row) = self.selected_address {
                return TableBuilder::new(ui).scroll_to_row(target_row as usize, None);
            }
        }

        TableBuilder::new(ui)
    }
}
