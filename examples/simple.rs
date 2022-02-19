use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{gtk, send, ComponentParts, RelmApp, Sender, SimpleComponent, WidgetPlus};

struct AppModel {
    counter: u8,
}

#[derive(Debug)]
enum AppMsg {
    Increment,
    Decrement,
}

#[relm4_macros::component]
impl SimpleComponent for AppModel {
    // AppWidgets is generated by the macro
    type Widgets = AppWidgets;

    type InitParams = u8;

    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Simple app"),
            set_default_width: 300,
            set_default_height: 100,

            &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                &gtk::Button {
                    set_label: "Increment",
                    connect_clicked(input) => move |_| {
                        send!(input, AppMsg::Increment);
                    }
                },

                &gtk::Button {
                    set_label: "Decrement",
                    connect_clicked(input) => move |_| {
                        send!(input, AppMsg::Decrement);
                    }
                },

                append = &gtk::Label {
                    set_label: watch!(&format!("Counter: {}", model.counter)),
                    set_margin_all: 5,
                }
            }
        }
    }

    // Initialize the UI.
    fn init_parts(
        counter: Self::InitParams,
        root: &Self::Root,
        input: &mut Sender<Self::Input>,
        _output: &mut Sender<Self::Output>,
    ) -> ComponentParts<Self, Self::Widgets> {
        let model = AppModel { counter };

        // Insert the macro codegen here
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        msg: Self::Input,
        _input: &mut Sender<Self::Input>,
        _ouput: &mut Sender<Self::Output>,
    ) {
        match msg {
            AppMsg::Increment => {
                self.counter = self.counter.wrapping_add(1);
            }
            AppMsg::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
            }
        }
    }
}

fn main() {
    let app: RelmApp<AppModel> = RelmApp::new("relm4.test.simple");
    app.run(0);
}
