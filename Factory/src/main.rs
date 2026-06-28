use gtk::{
    Box, Button, Label, Orientation, Window,
    prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt},
};
use relm4::{
    ComponentParts, ComponentSender, FactorySender, RelmApp, RelmWidgetExt, SimpleComponent,
    factory::{DynamicIndex, FactoryComponent, FactoryVecDeque},
};

#[derive(Debug)]
struct Counter {
    value: u8,
}

#[derive(Debug)]
enum CounterMsg {
    Increment,
    Decrement,
}

#[derive(Debug)]
enum CounterOutput {
    SendFront(DynamicIndex),
    MoveUp(DynamicIndex),
    MoveDown(DynamicIndex),
}

#[relm4::factory]
impl FactoryComponent for Counter {
    type Init = u8;
    type Input = CounterMsg;
    type Output = CounterOutput;
    type CommandOutput = ();
    type ParentWidget = Box;

    view! {
        #[root]
        Box{
            set_orientation: Orientation::Horizontal,
            set_spacing: 10,

            #[name(label)]
            Label{
                #[watch]
                set_label: &self.value.to_string(),
                set_width_chars: 3
            },

            #[name(add_button)]
            Button{
                set_label: "+",
                connect_clicked => CounterMsg::Increment
            },

            #[name(remove_button)]
            Button{
                set_label: "-",
                connect_clicked => CounterMsg::Decrement
            },

            #[name(move_up_button)]
            Button{
                set_label: "Up",
                connect_clicked[sender, index] => move |_|{
                    sender.output(CounterOutput::MoveUp(index.clone())).unwrap();
                }
            },

            #[name(move_down_button)]
            Button{
                set_label: "Down",
                connect_clicked[sender, index] => move |_|{
                    sender.output(CounterOutput::MoveDown(index.clone())).unwrap();
                }
            },

            #[name(to_front_button)]
            Button{
                set_label: "To Start",
                connect_clicked[sender,index] => move |_|{
                    sender.output(CounterOutput::SendFront(index.clone())).unwrap();
                }
            }
        }
    }

    fn init_model(init: Self::Init, _index: &Self::Index, _sender: FactorySender<Self>) -> Self {
        Self { value: init }
    }

    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        match msg {
            CounterMsg::Increment => {
                self.value = self.value.wrapping_add(1);
            }
            CounterMsg::Decrement => {
                self.value = self.value.wrapping_sub(1);
            }
        }
    }
}

struct App {
    created_widgets: u8,
    counters: FactoryVecDeque<Counter>,
}

#[derive(Debug)]
enum AppMsg {
    AddCounter,
    RemoveCounter,
    SendFront(DynamicIndex),
    MoveUp(DynamicIndex),
    MoveDown(DynamicIndex),
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = u8;
    type Input = AppMsg;
    type Output = ();

    view! {
        Window{
            set_title: Some("Factory example"),
            set_default_size: (300, 100),

            Box{
                set_orientation: Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                Button{
                    set_label: "Add counter",
                    connect_clicked => AppMsg::AddCounter,
                },

                Button{
                    set_label: "Remove counter",
                    connect_clicked => AppMsg::RemoveCounter
                },

                #[local_ref]
                counter_box -> Box{
                    set_orientation: Orientation::Vertical,
                    set_spacing: 5,
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let counters = FactoryVecDeque::builder().launch(Box::default()).forward(
            sender.input_sender(),
            |output| match output {
                CounterOutput::SendFront(index) => AppMsg::SendFront(index),
                CounterOutput::MoveUp(index) => AppMsg::MoveUp(index),
                CounterOutput::MoveDown(index) => AppMsg::MoveDown(index),
            },
        );

        let model = App {
            created_widgets: init,
            counters,
        };

        let counter_box = model.counters.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::AddCounter => {
                self.counters.guard().push_back(self.created_widgets);
                self.created_widgets = self.created_widgets.wrapping_add(1);
            }
            AppMsg::RemoveCounter => {
                self.counters.guard().pop_back();
            }
            AppMsg::SendFront(index) => {
                self.counters.guard().move_front(index.current_index());
            }
            AppMsg::MoveDown(index) => {
                let index = index.current_index();
                let new_index = index + 1;
                if new_index < self.counters.len() {
                    self.counters.guard().move_to(index, new_index);
                }
            }
            AppMsg::MoveUp(index) => {
                let index = index.current_index();
                if index != 0 {
                    self.counters.guard().move_to(index, index - 1);
                }
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.example.factory");
    app.run::<App>(0);
}
