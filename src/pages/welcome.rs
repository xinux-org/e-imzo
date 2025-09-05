use crate::app::AppMsg;
use gettextrs::gettext;

use relm4::{
    gtk::{
        self,
        gdk::Texture,
        gdk_pixbuf::Pixbuf,
        gio::{Cancellable, MemoryInputStream},
        glib,
        prelude::*,
    },
    *,
};

use crate::utils::check_service_installed;

fn embedded_logo() -> Texture {
    let bytes = include_bytes!("../../.github/assets/logo.png");
    let g_bytes = glib::Bytes::from(&bytes.to_vec());
    let stream = MemoryInputStream::from_bytes(&g_bytes);
    let pixbuf = Pixbuf::from_stream(&stream, Cancellable::NONE).unwrap();
    Texture::for_pixbuf(&pixbuf)
}

pub struct WelcomeModel;

#[relm4::component(pub)]
impl SimpleComponent for WelcomeModel {
    type Init = ();
    type Input = ();
    type Output = AppMsg;
    type Widgets = AppWidgets;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_hexpand: true,
            set_vexpand: true,
            set_halign: gtk::Align::Center,
            set_valign: gtk::Align::Center,
            set_spacing: 5,
            set_margin_all: 5,

            gtk::Image {
                set_pixel_size: 320,
                set_paintable: Some(&embedded_logo()),
            },

            gtk::Label {
                add_css_class: relm4::css::TITLE_2,
                #[watch]
                set_label: &gettext("Welcome to E-imzo"),
                set_margin_all: 1,
            },

            if check_service_installed("/etc/systemd/user/e-imzo.service") {
              gtk::Label {
                  #[watch]
                  set_label: &gettext("Please click the red button to start e-imzo service"),
                  set_margin_all: 1,
              }
            } else {
              gtk::LinkButton {
                  set_label: &gettext("Please download e-imzo service and relaunch the app again."),
                  set_uri: "https://search.nixos.org/packages?channel=25.05&show=e-imzo&from=0&size=50&sort=relevance&type=packages&query=e-imzo",
              }
            },

            // gtk::Label {
            //     add_css_class: relm4::css::TITLE_4,
            //     #[watch]
            //     set_markup: &gettext("Please download and relaunch the app again."),
            //     set_use_markup: true,
            //     set_margin_all: 5,
            //     set_justify: gtk::Justification::Center,
            // },
        }
    }
    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = WelcomeModel {};
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
