use gtk::{
    Box, Button, Label, Orientation, Window,
    glib::clone,
    prelude::{BoxExt, ButtonExt, GtkWindowExt},
};
use relm4::{ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

#[derive(Debug)]
enum AppMsg {
    Increment,
    Decrement,
}

struct AppModel {
    counter: u8,
}

struct AppWidgets {
    label: Label,
}

impl SimpleComponent for AppModel {
    type Input = AppMsg;
    type Output = ();
    type Init = u8;
    type Root = Window;
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        Window::builder()
            .title("Simple App")
            .default_width(300)
            .default_height(100)
            .build()
    }

    fn init(
        counter: Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel { counter };

        let vbox = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(5)
            .build();

        let inc_button = Button::with_label("Increment");
        let dec_button = Button::with_label("Decrement");

        let label = Label::new(Some(&format!("Counter: {}", model.counter)));
        label.set_margin_all(5);

        window.set_child(Some(&vbox));
        vbox.set_margin_all(5);
        vbox.append(&inc_button);
        vbox.append(&dec_button);
        vbox.append(&label);

        inc_button.connect_clicked(clone!(
            #[strong]
            sender,
            move |_| {
                sender.input(AppMsg::Increment);
            }
        ));

        dec_button.connect_clicked(clone!(
            #[strong]
            sender,
            move |_| {
                sender.input(AppMsg::Decrement);
            }
        ));

        let widgets = AppWidgets { label };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::Increment => self.counter = self.counter.wrapping_add(1),
            AppMsg::Decrement => self.counter = self.counter.wrapping_sub(1),
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        widgets
            .label
            .set_label(&format!("Counter: {}", self.counter));
    }
}

fn main() {
    let app = RelmApp::new("relm4.test.simple_manual");
    app.run::<AppModel>(0);
}
