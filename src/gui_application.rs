use std::process::{ExitCode, Termination};
use gtk::{Application, ApplicationWindow, Button, CustomFilter, CustomSorter, DragSource, DropTarget, FilterChange, FilterListModel, gio, Label, ListItem, ListView, Orientation, PolicyType, ScrolledWindow, SignalListItemFactory, SingleSelection, SorterChange, SortListModel, Widget};
use gtk::gdk::{ContentProvider, DragAction};
use gtk::glib;
use gtk::glib::{clone};
use gtk::prelude::*;
use crate::gui_application::integer_object::StringObject;

mod integer_object;

const APP_ID: &str = "com.datdeveloper.DatModManager";

pub fn gui_application() -> ExitCode {
    // Create a new gui_application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    // Run the gui_application
    app.run().report()
}

pub fn build_ui(app: &Application) {
    // Create a `Vec<IntegerObject>` with numbers from 0 to 100_000
    let vector: Vec<StringObject> =
        (0..=100).into_iter().map(|val| {
            let value = format!("test{val}");
            StringObject::new(value.as_str(), val)
        }).collect();

    // Create new model
    let model = gio::ListStore::new(StringObject::static_type());

    // Add the vector to the model
    model.extend_from_slice(&vector);

    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        // Create label
        let label = Label::new(None);

        let drag_source = DragSource::builder()
            .actions(DragAction::COPY)
            .build();

        drag_source.connect_prepare(
            move |_, _, _| {
                let test = "test";
                Some(ContentProvider::for_value(&test.to_value()))
            }
        );
        label.add_controller(drag_source);

        let drop_target = DropTarget::builder()
            .actions(DragAction::COPY)
            .build();

        drop_target.connect_drop(clone!(@weak label => @default-return false,
            move |drop_target, value, _, _| {
                let test =  value.get::<String>().expect("Whoops");

                println!("{test}");
                true
            }
        ));
        label.add_controller(drop_target);

        let list_item = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem");

        list_item.set_child(Some(&label));

        // Bind `list_item->item->number` to `label->label`
        list_item
            .property_expression("item")
            .chain_property::<StringObject>("name")
            .bind(&label, "label", Widget::NONE);
    });

    let sorter = CustomSorter::new(move |obj1, obj2| {
        // Get `IntegerObject` from `glib::Object`
        let integer_object_1 = obj1
            .downcast_ref::<StringObject>()
            .expect("The object needs to be of type `StringObject`.");
        let integer_object_2 = obj2
            .downcast_ref::<StringObject>()
            .expect("The object needs to be of type `StringObject`.");

        // Get property "number" from `IntegerObject`
        let number_1 = integer_object_1.property::<u32>("pos");
        let number_2 = integer_object_2.property::<u32>("pos");

        // Reverse sorting order -> large numbers come first
        number_1.cmp(&number_2).into()
    });
    let sort_model = SortListModel::new(Some(model), Some(sorter.clone()));

    let selection_model = SingleSelection::new(Some(sort_model));
    let list_view = ListView::new(Some(selection_model), Some(factory));
    //
    // let drop_target = DropTarget::new(Type::STRING, DragAction::COPY);
    //
    // // drop_target.connect_accept(
    // //     move |drop_target, _| {
    // //         println!("Test");
    // //         true
    // //     }
    // // );
    //
    // drop_target.connect_drop(|drop_target, value, _, _| {
    //     let out = value.get::<String>().unwrap();
    //     println!("{out}");
    //     true
    // });
    //
    // list_view.add_controller(drop_target);

    // list_view.connect_activate(move |list_view, position| {
    //     // Get `IntegerObject` from model
    //     let model = list_view.model().expect("The model has to exist.");
    //     let integer_object = model
    //         .item(position)
    //         .and_downcast::<StringObject>()
    //         .expect("The item has to be an `IntegerObject`.");
    //
    //     // Notify that the filter and sorter have been changed
    //     filter.changed(FilterChange::Different);
    //     sorter.changed(SorterChange::Different);
    // });

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&list_view)
        .build();

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .default_width(600)
        .default_height(300)
        .child(&scrolled_window)
        .build();

    // Present the window
    window.present();
}

// pub fn build_ui(app: &Application) {
//     // Create two buttons
//     let button_increase = Button::builder()
//         .label("Increase")
//         .margin_top(12)
//         .margin_bottom(12)
//         .margin_start(12)
//         .margin_end(12)
//         .build();
//     let button_decrease = Button::builder()
//         .label("Decrease")
//         .margin_top(12)
//         .margin_bottom(12)
//         .margin_start(12)
//         .margin_end(12)
//         .build();
//
//     // Reference-counted object with inner mutability
//     let number = Rc::new(Cell::new(0));
//
//     // Connect callbacks
//     // When a button is clicked, `number` and label of the other button will be changed
//     button_increase.connect_clicked(clone!(@weak number, @weak button_decrease =>
//         move |_| {
//             number.set(number.get() + 1);
//             button_decrease.set_label(&number.get().to_string());
//     }));
//
//     let drag_source = DragSource::new();
//
//     drag_source.connect_prepare(|drag_source, _, _| {
//         Some(ContentProvider::for_value(&5.to_value()))
//     });
//
//     button_increase.add_controller(drag_source);
//
//     button_decrease.connect_clicked(clone!(@weak button_increase =>
//         move |_| {
//             number.set(number.get() - 1);
//             button_increase.set_label(&number.get().to_string());
//     }));
//
//     let drop_target = DropTarget::new(Type::I32, DragAction::COPY);
//     drop_target.connect_drop(
//          move |drop_target, value, _, _| {
//             let out = value.get::<i32>().unwrap();
//             println!("{out}");
//             true
//     });
//
//     button_decrease.add_controller(drop_target);
//
//     // Add buttons to `gtk_box`
//     let gtk_box = gtk::Box::builder()
//         .orientation(Orientation::Vertical)
//         .build();
//     gtk_box.append(&button_increase);
//     gtk_box.append(&button_decrease);
//
//     // Create a window
//     let window = ApplicationWindow::builder()
//         .gui_application(app)
//         .title("My GTK App")
//         .child(&gtk_box)
//         .build();
//
//     // Present the window
//     window.present();
// }