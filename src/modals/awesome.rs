use gettextrs::gettext;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct AwesomeModel;

#[relm4::component(pub)]
impl SimpleComponent for AwesomeModel {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        adw::Dialog {
            set_title: &gettext("List of used e-imzo websites"),
            set_follows_content_size: true,
            set_presentation_mode: adw::DialogPresentationMode::Floating,

            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar,

                #[wrap(Some)]
                set_content = &adw::PreferencesPage {
                    adw::PreferencesGroup {
                        adw::ActionRow {
                            set_title: "ahost.uz",
                            add_suffix = &gtk::LinkButton::builder()
                                .uri("https://clients.ahost.uz/login")
                                .child(&gtk::Image::from_icon_name("document-send-symbolic"))
                                .build(),
                        },

                        adw::ActionRow {
                            set_title: "id.egov.uz",
                            add_suffix = &gtk::LinkButton::builder()
                                .uri("https://id.egov.uz/oz")
                                .child(&gtk::Image::from_icon_name("document-send-symbolic"))
                                .build(),
                        },

                        adw::ActionRow {
                            set_title: "didox.uz",
                            add_suffix = &gtk::LinkButton::builder()
                                .uri("https://didox.uz/login_with_signature")
                                .child(&gtk::Image::from_icon_name("document-send-symbolic"))
                                .build(),
                        },

                        adw::ActionRow {
                            set_title: "birdarcha.uz",
                            add_suffix = &gtk::LinkButton::builder()
                                .uri("https://new.birdarcha.uz/login")
                                .child(&gtk::Image::from_icon_name("document-send-symbolic"))
                                .build(),
                        },

                        adw::ActionRow {
                            set_title: "e-invoice.uz",
                            add_suffix = &gtk::LinkButton::builder()
                                .uri("https://e-invoice.uz/register/")
                                .child(&gtk::Image::from_icon_name("document-send-symbolic"))
                                .build(),
                        },

                        adw::ActionRow {
                            set_title: "my.mehnat.uz",
                            add_suffix = &gtk::LinkButton::builder()
                                .uri("https://my.mehnat.uz/login#")
                                .child(&gtk::Image::from_icon_name("document-send-symbolic"))
                                .build(),
                        },
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self;
        let widgets = view_output!();
        let window = relm4::main_application().active_window();
        root.present(window.as_ref());

        ComponentParts { model, widgets }
    }
}
