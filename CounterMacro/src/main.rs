use gtk::{
    Box, Button, Label, Orientation, Window,
    prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt},
};
use relm4::{ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent, component};

#[derive(Debug)]
enum AppMsg {
    Increment,
    Decrement,
}

struct AppModel {
    counter: u8,
}

#[component]
impl SimpleComponent for AppModel {
    type Init = u8;

    type Input = AppMsg;
    type Output = ();

    view! {
        Window {
            set_title: Some("Simple app"),
            set_default_width: 300,
            set_default_height: 100,

            Box {
                set_orientation: Orientation::Vertical,
                set_spacing: 5,
                set_margin_all:5,

                Button {
                    set_label: "Increment",
                    connect_clicked => AppMsg::Increment
                },

                Button::with_label("Decrement") {
                    connect_clicked => AppMsg::Decrement
                },

                Label {
                    #[watch]
                    set_label: &format!("Counter: {}", model.counter),
                    set_margin_all: 5
                }
            }
        }
    }

    fn init(
        counter: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel { counter };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::Increment => self.counter = self.counter.wrapping_add(1),
            AppMsg::Decrement => self.counter = self.counter.wrapping_sub(1),
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.test.simple_manual");
    app.run::<AppModel>(0);
}
