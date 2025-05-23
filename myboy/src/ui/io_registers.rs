use egui::RichText;

use crate::io::io_registers::IORegisters;

pub struct IORegisterView<'a> {
    pub registers: &'a IORegisters,
}

impl egui::Widget for IORegisterView<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.label(RichText::new("Serial Data").underline());
            ui.horizontal(|ui| {
                ui.label("SB (Serial transfer #FF01):");
                ui.label(format!("0x{:02X}", self.registers.read_byte(0xff01)));
            });
            ui.horizontal(|ui| {
                ui.label("SC (Serial control #FF02):");
                ui.label(format!("0x{:02X}", self.registers.read_byte(0xff02)));
            });

            ui.label(RichText::new("Timers").underline());
            ui.horizontal(|ui| {
                ui.label("DIV (Divider #FF04):");
                ui.label(format!("0x{:02X}", self.registers.read_byte(0xff04)))
                    .on_hover_text(format!("System Counter: {:#X}", self.registers.timers.sys));
            });
            ui.horizontal(|ui| {
                ui.label("TIMA (Timer #FF05):");
                ui.label(format!("0x{:02X}", self.registers.read_byte(0xff05)));
            });
            ui.horizontal(|ui| {
                ui.label("TMA (Timer Modulo #FF06):");
                ui.label(format!("0x{:02X}", self.registers.read_byte(0xff06)));
            });
            ui.horizontal(|ui| {
                ui.label("TAC (Timer Control #FF07):");
                ui.label(format!("0x{:02X}", self.registers.read_byte(0xff07)));
            });

            let lcdc_reg = self.registers.get_lcdc_register();
            ui.label(RichText::new("LCD-Control Register (#FF40)").underline());
            ui.horizontal(|ui| {
                ui.label("LCD Enabled:");
                ui.label(format!("{}", lcdc_reg.lcd_enabled()));
            });
            ui.horizontal(|ui| {
                ui.label("Window Tile Map Display Select:");
                ui.label(format!("{}", lcdc_reg.window_tile_map_bank()));
            });
            ui.horizontal(|ui| {
                ui.label("Window Display Enabled:");
                ui.label(format!("{}", lcdc_reg.window_enabled()));
            });
            ui.horizontal(|ui| {
                ui.label("BG & Window Tile Data Select:");
                ui.label(format!("{}", lcdc_reg.bgwin_tile_data_area()));
            });
            ui.horizontal(|ui| {
                ui.label("BG Tile Map Display Select:");
                ui.label(format!("{}", lcdc_reg.bg_tile_map_bank()));
            });
            ui.horizontal(|ui| {
                ui.label("Sprite Size:");
                ui.label(format!("{}", lcdc_reg.obj_size()));
            });
            ui.horizontal(|ui| {
                ui.label("Sprite Display Enabled:");
                ui.label(format!("{}", lcdc_reg.obj_enabled()));
            });
            ui.horizontal(|ui| {
                ui.label("BG Display:");
                ui.label(format!("{}", lcdc_reg.bgwin_enabled()));
            });

            ui.separator();

            ui.label(RichText::new("LCD").underline());
            ui.horizontal(|ui| {
                ui.label("LCDSTAT (Status #FF41):");
                ui.label(format!("0x{:02X}", self.registers.get_lcdstat()));
            });
            ui.horizontal(|ui| {
                ui.label("LY (Line Register #FF44):");
                ui.label(format!("{}", self.registers.get_lcd_ly()));
            });

            ui.label(RichText::new("General IO Registers (#FF00)").underline());
            ui.horizontal(|ui| {
                ui.label("#FF00:");
                ui.label(format!("0x{:02X}", self.registers.read_byte(0xff00)));
            });
        })
        .response
    }
}
