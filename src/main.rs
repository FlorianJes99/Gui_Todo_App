use eframe::egui::{self};

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My Todo App",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}

#[derive(serde::Serialize, serde::Deserialize)]
struct MyApp {
    todo_input_text: String,
    todos: Vec<Todo>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            todo_input_text: String::new(),
            todos: vec![],
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct Todo {
    todo_text: String,
    completed: bool,
}

impl std::fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.todo_text)
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut index = -1;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Todos:\n");
            for (pos, todo) in self.todos.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    if ui.button("Remove").clicked() {
                        index = pos as i32;
                    }
                    ui.add_space(5f32);
                    _ = ui.checkbox(&mut todo.completed, "");
                    ui.add_space(5f32);
                    let mut label = ui.label(format!("{}:\t{}", pos + 1, todo));
                    label = label.interact(egui::Sense::click());
                    if label.clicked() {
                        todo.completed = !todo.completed;
                    }
                });
            }
            if index != -1 {
                self.todos.remove(index as usize);
            }
            ui.horizontal(|_ui| ());
            ui.horizontal(|ui| {
                let todo_input_box = ui.text_edit_singleline(&mut self.todo_input_text);
                if todo_input_box.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.add_todo();
                }
                let mut add_todo_btn = ui.button("Add Todo!");
                add_todo_btn =
                    add_todo_btn.on_hover_text("You can also add a Todo pressing the Enter key.");
                if add_todo_btn.clicked() {
                    self.add_todo();
                }

                let remove_all_finished = ui.button("Remove all finished Todos");
                if remove_all_finished.clicked() {
                    self.remove_all_finished();
                }
                todo_input_box.request_focus();
            });
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
    fn add_todo(&mut self) {
        let todo = Todo {
            todo_text: self.todo_input_text.clone(),
            completed: false,
        };
        self.todos.push(todo);
        self.todo_input_text = String::new();
    }

    fn remove_all_finished(&mut self) {
        self.todos = self
            .todos
            .iter()
            .cloned()
            .filter(|it| !it.completed)
            .collect();
    }
}
