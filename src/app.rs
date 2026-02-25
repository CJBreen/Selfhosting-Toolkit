use egui::Align2;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};
use std::default::Default;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Default)]
pub struct PrototypeUI {
    name: String,
    desc: String,
    #[serde(skip)] // disable auto-serialization
    value: f32,
    show_nextcloud_viewport: bool,
    _temp_value: usize,
    _title: String,
    _text: String,
}


impl PrototypeUI {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }

    }
    //show the nextcloud installation guide
    pub fn show_guide(&mut self, ctx: &egui::Context) {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("immediate_viewport"),
            egui::ViewportBuilder::default()
                .with_title("Immediate Viewport")
                .with_inner_size([500.0, 500.0]),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This backend only supports one viewport."
                );
                //top panel for nextcloud settings
                egui::TopBottomPanel::top("top_panel_nextcloud").show(ctx, |ui| {
                    egui::MenuBar::new().ui(ui, |ui| {
                        // NOTE: no File->Quit on web pages!
                        let is_web = cfg!(target_arch = "wasm32");
                        if !is_web {
                            ui.menu_button("Options", |ui| {
                                if ui.button("Quit").clicked() {
                                    self.show_nextcloud_viewport = false;
                                }
                            });
                            ui.add_space(16.0);
                        }
                    });
                });
                egui::SidePanel::left("installation_method").show(ctx, |_ui| {
                    });
                egui::CentralPanel::default().show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.heading("Installation Guide");
                        ui.heading("Tip: you can click any command to copy it.");
                        ui.add_space(20.0);
                        self._title = String::from("Manual Installation");


                        ui.collapsing(self._title.clone(), |ui|{
                            ui.label("Step 1: Set up Docker");
                            ui.label("If you have Docker installed, we can skip this step!");
                            ui.label("To check if Docker is installed, run this command:");
                            ui.add_space(10.0);
                            ui.separator();


                            if ui.label("which docker").clicked() {
                                ui.ctx().copy_text("which docker".to_owned()); //copy text to clipboard
                                let mut toasts = Toasts::new()
                                    .anchor(Align2::RIGHT_TOP, [10.0, 10.0])
                                    .direction(egui::Direction::TopDown);

                                toasts.add(Toast {
                                    text: "Copied to Clipboard".into(),
                                    kind: ToastKind::Info,
                                    options: ToastOptions::default()
                                        .duration_in_seconds(3.0)
                                        .show_progress(true)
                                        .show_icon(true),
                                    style: Default::default(),
                                });
                                toasts.show(ctx);
                            }
                            ui.separator();


                            ui.collapsing("Explanation of Command", |ui| {
                                ui.label("This command finds what instance of docker is installed.");
                                ui.label("If it returns an output, you have it installed and can go to step 2.");
                                ui.label("If it does not return anything, continue with this step.");
                            });
                            ui.add_space(10.0);

                            ui.label("If you do not have docker installed, we can use a simple script to install it!");
                            ui.label("Open up your terminal and paste this command.");
                            ui.separator();

                            //convenience script to copy
                            if ui.label("'curl -fsSL https://get.docker.com | sudo sh'").clicked() {
                                ui.ctx().copy_text("curl -fsSL https://get.docker.com | sudo sh".to_owned());
                            }


                            ui.separator();
                            ui.collapsing("Explanation of command", |ui| {
                                ui.label("This command will pull the docker convenience script and run it.");
                                ui.label("If this failed, it means you do not have curl installed.");
                                ui.label("Simply install 'curl' with your distribution's package manager.");


                            });
                            ui.add_space(20.0);
                            ui.label("Step 2: Install Compose and Enable Services");
                            ui.label("Next, we need to install Docker Compose. This will let us store all configuration into one file.");
                            ui.label("Simply install it with your package manager.");
                            ui.add_space(10.0);

                            ui.label("Next, we need to enable the services.");
                            ui.label("Use this command to enable it. Keep in mind you need to will need your sudo password.");
                            ui.add_space(10.0);
                            ui.separator();
                            if ui.label("sudo systemctl enable --now docker.service \n sudo systemctl enable --now containerd.service").clicked() {
                                ui.ctx().copy_text("sudo systemctl enable --now docker.service \n sudo systemctl enable --now containerd.service".to_string());
                            }
                            ui.separator();

                            ui.add_space(20.0);
                            ui.label("Step 3: Create Compose.yaml");
                            ui.label("Create a new folder. This folder will hold your configurations in one file.");
                            ui.label("Next, create a file and name it 'compose.yaml'.");
                            ui.add_space(10.0);

                            ui.collapsing("What does that mean?", |ui| {
                                ui.label("A YAML file is something that can be easily read by people and computers.");
                                ui.label("It is used to store data that can be used in other apps.");
                                ui.label("This is what Docker will use to read your configurations");

                            });
                            ui.add_space(10.0);
                            ui.label("Below is a configuration you can use for the application.");
                            ui.label("Simply paste in the data and save the file.");
                            ui.separator();
                            ui.add_space(10.0);
                            let mut yaml_data = "name: nextcloud-aio
                                                services:
                                                    nextcloud-aio-mastercontainer:
                                                        image: ghcr.io/nextcloud-releases/all-in-one:latest
                                                           init: true
                                                           restart: always
                                                           container_name: nextcloud-aio-mastercontainer
                                                           volumes:
                                                              - nextcloud_aio_mastercontainer:/mnt/docker-aio-config
                                                              - /var/run/docker.sock:/var/run/docker.sock:ro
                                                            network_mode: bridge
                                                            ports:
                                                              - 80:80
                                                              - 8080:8080
                                                              - 8443:8443
                                                           environment:
                                                             - SKIP_DOMAIN_VALIDATION=true

                                                volumes:
                                                   nextcloud_aio_mastercontainer:
                                                    name: nextcloud_aio_mastercontainer";
                            ui.collapsing("YAML data", |ui| {
                                ui.text_edit_multiline(&mut yaml_data);
                            });
                            ui.add_space(20.0);
                            ui.label("Now, we can run the service using this command:");
                            ui.separator();
                            if ui.label("docker compose up -d").clicked() {
                                ui.ctx().copy_text("docker compose up".to_owned());
                            }
                            ui.separator();
                            ui.add_space(20.0);
                            ui.label("Now that it is running, you can go to:");
                            ui.separator();
                            if ui.label("https://localhost::8443").clicked() {
                                ui.ctx().copy_text("https://localhost:8443".to_owned());
                            }
                            ui.separator();
                            ui.label("And finish setting up Nextcloud!");




                        });
                        ui.collapsing("Automatic Installation", |ui| {
                            ui.label("If you wish to use an automatic install, you can use this script to automatically configure a local installation of Nextcloud");
                            if ui.button("Automatic Installation Script").clicked() {
                            };
                        })



                    });
                })

                });
                if ctx.input(|i| i.viewport().close_requested()) {
                    self.show_nextcloud_viewport = false;
                }
            }

    }







impl eframe::App for PrototypeUI {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self,
              ctx: &egui::Context,
              _frame: &mut eframe::Frame) {
        //top panel
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("Options", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });
        egui::SidePanel::left("quick_apps_panel").show(ctx, |ui| {
            ui.label("Install services:");
            ui.separator();
            if ui.button("Nextcloud").clicked() {
                // let mut app_name = Application::default().name;
                // app_name = String::from("[App] Settings");
                //opens up new window for user to be guided through nextcloud installation
                self.show_nextcloud_viewport = true;
                //debug to console to check viewport
                println!("Nextcloud is working");
            }

            if ui.button("Add new Application").clicked() {
                ui.label("Button clicked");

            }

        });
        //if true, open up the nextcloud installation guide window
        if self.show_nextcloud_viewport {
            self.show_guide(ctx);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // central UI
            ui.vertical_centered(|ui| {
                ui.heading("Prototype for Demonstration");

            });
            ui.separator();

            //TODO: Pull compose data from yaml file and show here
            egui::Window::new("App Details").show(ctx, |ui| {
                ui.add_space(16.0);
                ui.label("App Name:");
                ui.label("IP Address:");
                ui.label("Port:");
                ui.label("Status:");
            });


            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }


    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}




