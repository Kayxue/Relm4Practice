use gtk::{
    ApplicationWindow, Box, Button, Image, Orientation,
    prelude::{BoxExt, ButtonExt, OrientableExt},
};
use rand::rng;
use rand::seq::IteratorRandom;
use relm4::{
    ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent, component,
    set_global_css,
};

const ICON_LIST: &[&str] = &[
    "bookmark-new-symbolic",
    "edit-copy-symbolic",
    "edit-cut-symbolic",
    "edit-find-symbolic",
    "starred-symbolic",
    "system-run-symbolic",
    "emoji-objects-symbolic",
    "emoji-nature-symbolic",
    "display-brightness-symbolic",
];

fn random_icon_name() -> &'static str {
    ICON_LIST
        .iter()
        .choose(&mut rng())
        .expect("Could not choose a random icon")
}

fn get_unique_icon(exclude: &'static str) -> &'static str {
    let mut rnd = random_icon_name();
    while rnd == exclude {
        rnd = random_icon_name();
    }
    rnd
}

#[tracker::track]
struct AppModel {
    first_icon: &'static str,
    second_icon: &'static str,
    identical: bool,
}

#[derive(Debug)]
enum AppInput {
    UpdateFirst,
    UpdateSecond,
}

#[component]
impl SimpleComponent for AppModel {
    type Init = ();

    type Input = AppInput;
    type Output = ();

    view! {
        #[root]
        ApplicationWindow {
            #[track = "model.changed(AppModel::identical())"]
            set_class_active: ("identical", model.identical),
            Box {
                set_orientation: Orientation::Horizontal,
                set_spacing: 10,
                set_margin_all: 10,
                Box {
                    set_orientation: Orientation::Vertical,
                    set_spacing: 10,
                    Image {
                        set_pixel_size: 50,
                        #[track = "model.changed(AppModel::first_icon())"]
                        set_icon_name: Some(model.first_icon)
                    },
                    Button {
                        set_label: "New random image",
                        connect_clicked => AppInput::UpdateFirst
                    }
                },
                Box {
                    set_orientation: Orientation::Vertical,
                    set_spacing: 10,
                    Image {
                        set_pixel_size: 50,
                        #[track = "model.changed(AppModel::second_icon())"]
                        set_icon_name: Some(model.second_icon)
                    },
                    Button {
                        set_label: "New random image",
                        connect_clicked => AppInput::UpdateSecond
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel {
            first_icon: random_icon_name(),
            second_icon: random_icon_name(),
            identical: false,
            tracker: 0,
        };

        let widgets = view_output!();

        ComponentParts {
            model,
            widgets: widgets,
        }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        self.reset();

        match message {
            AppInput::UpdateFirst => self.set_first_icon(get_unique_icon(self.first_icon)),
            AppInput::UpdateSecond => self.set_second_icon(get_unique_icon(self.second_icon)),
        }
        self.set_identical(self.first_icon == self.second_icon);
    }
}

fn main() {
    let app = RelmApp::new("relm4.test.simple");
    set_global_css(".identical { background: #00ad5c; }");
    app.run::<AppModel>(());
}
