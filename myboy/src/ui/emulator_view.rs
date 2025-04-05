use std::thread;

use egui::{CentralPanel, CollapsingHeader, Response, RichText, SidePanel, Widget};

use crate::{device::device::Device, emulator::EmulatorInstance};

use super::{asm_text::AsmTextTable, cpu_registers::CPURegisterView, io_registers::IORegisterView};

enum MainView {
    Program,
    Memory,
    Serial,
}

pub struct EmulatorView<'a> {
    emulator: &'a mut EmulatorInstance,
    active_view: MainView,
}

impl EmulatorView<'_> {
    pub fn new(emulator: &mut EmulatorInstance) -> EmulatorView {
        EmulatorView {
            emulator,
            active_view: MainView::Program,
        }
    }
}

impl Widget for EmulatorView<'_> {
    fn ui(mut self, ui: &mut egui::Ui) -> Response {
        ui.group(|ui| {
            SidePanel::left("side_panel").show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    if self.emulator.device.running {
                        if ui.button("Pause").clicked() {
                            self.emulator.device.running = false;
                        }
                    } else {
                        if ui.button("Run").clicked() {
                            unsafe {
                                let raw_device_pointer =
                                    &mut self.emulator.device as *mut Device as usize;
                                thread::spawn(move || {
                                    let raw_device = raw_device_pointer as *mut Device;
                                    let _ = <*mut Device>::as_mut(raw_device).unwrap().run();
                                });
                            }
                        }
                        if ui.button("> Step").clicked() {
                            self.emulator.device.running = false;
                            self.emulator.device.step();
                        }
                    }

                    ui.menu_button(
                        format!("{:.2}x", self.emulator.device.speed_multiplier),
                        |ui| {
                            ui.label("speed multiplier");
                            ui.horizontal(|ui| {
                                ui.label("Speed: ");
                                ui.add_enabled_ui(
                                    self.emulator.device.speed_multiplier > 0.059,
                                    |ui| {
                                        if ui.button("-").clicked() {
                                            self.emulator.device.speed_multiplier -= 0.05;
                                        }
                                    },
                                );
                                ui.label(format!("{:.2}x", self.emulator.device.speed_multiplier));
                                if ui.button("+").clicked() {
                                    self.emulator.device.speed_multiplier += 0.05;
                                }
                            });
                        },
                    );
                });

                ui.separator();

                CollapsingHeader::new("CPU Registers")
                    .default_open(true)
                    .show(ui, |ui| {
                        CPURegisterView {
                            cpu: &self.emulator.device.cpu,
                        }
                        .ui(ui)
                    });

                CollapsingHeader::new("IO Registers")
                    .default_open(true)
                    .show(ui, |ui| {
                        IORegisterView {
                            registers: &self.emulator.device.mem_map.io_registers,
                        }
                        .ui(ui)
                    });

                CollapsingHeader::new("Interrupts")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.columns(2, |columns| {
                                columns[0].label("");
                                columns[1].label(RichText::new("enabled?").size(10.0));

                                columns[0].label("V-Blank: ");
                                columns[1].label(format!(
                                    "{}",
                                    self.emulator
                                        .device
                                        .mem_map
                                        .ie_register
                                        .is_vblank_handler_enabled()
                                ));
                                columns[0].label("LCD Stat: ");
                                columns[1].label(format!(
                                    "{}",
                                    self.emulator
                                        .device
                                        .mem_map
                                        .ie_register
                                        .is_lcd_handler_enabled()
                                ));
                                columns[0].label("Timer: ");
                                columns[1].label(format!(
                                    "{}",
                                    self.emulator
                                        .device
                                        .mem_map
                                        .ie_register
                                        .is_timer_handler_enabled()
                                ));
                                columns[0].label("Serial: ");
                                columns[1].label(format!(
                                    "{}",
                                    self.emulator
                                        .device
                                        .mem_map
                                        .ie_register
                                        .is_serial_handler_enabled()
                                ));
                                columns[0].label("Joypad: ");
                                columns[1].label(format!(
                                    "{}",
                                    self.emulator
                                        .device
                                        .mem_map
                                        .ie_register
                                        .is_joypad_handler_enabled()
                                ));
                            });
                        });
                    });
            });

            CentralPanel::default().show_inside(ui, |ui| {
                let cartridge = &self.emulator.cartridge_data.cartridge;
                ui.heading(cartridge.get_title());
                ui.horizontal(|ui| {
                    ui.label("Cartridge Type:");
                    ui.label(format!("{}", cartridge.get_cartridge_type().unwrap()));
                    ui.label("Licensee:");
                    ui.label(format!("{}", cartridge.get_licensee().unwrap()));
                    ui.label("ROM Size:");
                    ui.label(format!("{}", cartridge.get_rom_size()));
                    // ui.label("Manufacturer Code:");
                    // ui.label(format!("{}", cartridge.manufacturer_code()));
                    ui.label("ROM Bank Count:");
                    ui.label(format!("{}", cartridge.get_rom_bank_count()));
                    ui.label("GBC support:");
                    ui.label(format!("{}", cartridge.get_gbc_support()));
                });

                ui.horizontal_top(|ui| {
                    let program_button = ui.button("program");
                    let memory_button = ui.button("memory");
                    let serial_button = ui.button("serial");
                    match self.active_view {
                        MainView::Program => {
                            program_button.enabled();
                        }
                        MainView::Memory => {
                            memory_button.enabled();
                        }
                        MainView::Serial => {
                            serial_button.enabled();
                        }
                    };
                    if program_button.clicked() {
                        self.active_view = MainView::Program;
                    }
                    if memory_button.clicked() {
                        self.active_view = MainView::Memory;
                    }
                    if serial_button.clicked() {
                        self.active_view = MainView::Serial;
                    }
                });

                match self.active_view {
                    MainView::Program => {
                        ui.add(AsmTextTable::new(self.emulator));
                    }
                    MainView::Memory => {
                        ui.label("Memory");
                    }
                    MainView::Serial => {
                        ui.label("Serial");

                        ui.code(
                            RichText::new(String::from_utf8_lossy(
                                &self.emulator.device.serial_buffer,
                            ))
                            .monospace(),
                        );
                    }
                };
            });
        })
        .response
    }
}
