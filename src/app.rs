use crate::{
    config::{APP_ID, PROFILE},
    modals::{about::AboutDialog, awesome::AwesomeModel},
    pages::{
        select_mode::{SelectModeMsg, SelectModePage},
        welcome::WelcomeModel,
    },
    utils::check_service_active,
};
use gettextrs::gettext;
use relm4::{
    actions::{RelmAction, RelmActionGroup},
    adw::{self, prelude::*},
    gtk::{self, gio, glib},
    main_application, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};
use std::convert::identity;

use std::process::Command;

#[derive(Debug, Clone)]
pub enum Page {
    Welcome,
    SelectMode,
}

pub struct App {
    page: Page,
    welcome_page: Controller<WelcomeModel>,
    select_mode_page: Controller<SelectModePage>,
    service_active: bool,
    service: gtk::Button,
}

#[derive(Debug)]
pub enum AppMsg {
    Quit,
    SelectMode(SelectModeMsg),
    StartService,
    StopService,
    RefreshService(bool),
}

relm4::new_action_group!(pub WindowActionGroup, "win");
relm4::new_stateless_action!(AwesomeAction, WindowActionGroup, "awesome");
relm4::new_stateless_action!(pub ShortcutsAction, WindowActionGroup, "show-help-overlay");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();
    type Widgets = AppWidgets;

    menu! {
        primary_menu: {
            section! {
                &gettext("Awesome E-IMZO") => AwesomeAction,
                &gettext("Keyboard") => ShortcutsAction,
                &gettext("About E-IMZO Manager") => AboutAction,
            }
        }
    }
    view! {
        #[root]
        main_window = adw::ApplicationWindow::new(&main_application()) {
            set_visible: true,
            // width and height below
            set_size_request: (500, 600),
            set_default_size: (500, 600),

            connect_close_request[sender] => move |_| {
                sender.input(AppMsg::Quit);
                glib::Propagation::Stop
            },
            #[wrap(Some)]
            set_help_overlay: shortcuts = &gtk::Builder::from_resource(
                    "/uz/xinux/EIMZOManager/gtk/help-overlay.ui"
                )
                .object::<gtk::ShortcutsWindow>("help_overlay")
                .unwrap() -> gtk::ShortcutsWindow {
                    set_transient_for: Some(&main_window),
                    set_application: Some(&main_application()),
            },

            add_css_class?: if PROFILE == "Devel" {
                    Some("devel")
            } else {
                None
            },
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_vexpand: true,
                set_hexpand: true,

                adw::HeaderBar {
                    pack_start = &gtk::Button {
                        set_icon_name: "list-add-symbolic",
                        add_css_class: "flat",
                        connect_clicked => AppMsg::SelectMode(SelectModeMsg::OpenFile),
                        #[watch]
                        set_visible: matches!(model.page, Page::SelectMode),
                    },
                    #[name(service)]
                    pack_start = &gtk::Button {
                      set_label: "ON",
                      add_css_class: "suggested-action",
                      connect_clicked => if !model.service_active {AppMsg::StartService} else {AppMsg::StopService},
                    },

                    pack_end = &gtk::MenuButton {
                        set_icon_name: "open-menu-symbolic",
                        set_menu_model: Some(&primary_menu),
                    }
                },
                match model.page {
                    Page::Welcome => gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_vexpand: true,
                        set_hexpand: true,
                        append: model.welcome_page.widget()
                    },
                    Page::SelectMode => gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_vexpand: true,
                        set_hexpand: true,
                        append: model.select_mode_page.widget()
                    },
                },
            },
        },
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let welcome_page = WelcomeModel::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let select_mode_page = SelectModePage::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let page: Page = if check_service_active("e-imzo.service") {
            Page::SelectMode
        } else {
            Page::Welcome
        };

        let mut model = Self {
            page: page,
            welcome_page: welcome_page,
            select_mode_page: select_mode_page,
            service_active: check_service_active("e-imzo.service"),
            service: gtk::Button::new(),
        };

        let widgets = view_output!();
        let service = widgets.service.clone();
        model.service = service;
        widgets.load_window_size();

        let awesome_action = {
            RelmAction::<AwesomeAction>::new_stateless(move |_| {
                tracing::info!("AwesomeAction triggered");
                AwesomeModel::builder().launch(()).detach();
            })
        };

        let shortcuts_action = {
            let shortcuts = widgets.shortcuts.clone();
            RelmAction::<ShortcutsAction>::new_stateless(move |_| {
                shortcuts.present();
            })
        };

        let about_action = {
            RelmAction::<AboutAction>::new_stateless(move |_| {
                AboutDialog::builder().launch(()).detach();
            })
        };

        let sender_clone = sender.input_sender().clone();
        glib::timeout_add_seconds_local(1, move || {
            let active = check_service_active("e-imzo.service");
            sender_clone.send(AppMsg::RefreshService(active)).ok();
            glib::ControlFlow::Continue
        });

        let mut actions = RelmActionGroup::<WindowActionGroup>::new();
        actions.add_action(awesome_action);
        actions.add_action(shortcuts_action);
        actions.add_action(about_action);
        actions.register_for_widget(&widgets.main_window);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::Quit => main_application().quit(),
            AppMsg::SelectMode(msg) => {
                self.select_mode_page.emit(msg);
            }
            AppMsg::StartService => {
                let _ = Command::new("systemctl")
                    .arg("start")
                    .arg("--user")
                    .arg("e-imzo.service")
                    .status();
                self.service_active = true;
            }
            AppMsg::StopService => {
                let _ = Command::new("systemctl")
                    .arg("stop")
                    .arg("--user")
                    .arg("e-imzo.service")
                    .status();
                self.service_active = false;
            }
            AppMsg::RefreshService(active) => {
                self.service_active = active;
                if check_service_active("e-imzo.service") {
                    self.service.set_label("OFFrefresh");
                    self.service.add_css_class("destructive-action");
                    self.service_active = true;
                } else {
                    self.service.set_label("ONrefresh");
                    self.service.add_css_class("suggested-action");
                    self.service_active = false;
                }
            }
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        widgets.save_window_size().unwrap();
    }
}

impl AppWidgets {
    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let settings = gio::Settings::new(APP_ID);
        let (width, height) = self.main_window.default_size();

        settings.set_int("window-width", width)?;
        settings.set_int("window-height", height)?;

        settings.set_boolean("is-maximized", self.main_window.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let settings = gio::Settings::new(APP_ID);

        let width = settings.int("window-width");
        let height = settings.int("window-height");
        let is_maximized = settings.boolean("is-maximized");

        self.main_window.set_default_size(width, height);

        if is_maximized {
            self.main_window.maximize();
        }
    }
}
