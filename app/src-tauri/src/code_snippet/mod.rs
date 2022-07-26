use tauri::{
    plugin::{Builder, TauriPlugin},
    AppHandle, Manager, Runtime, State,
};

#[derive(Default)]
struct CodeSnippetState {}

// this will be accessible with `invoke('plugin:awesome|do_something')`.
#[tauri::command]
fn do_something<R: Runtime>(_app: AppHandle<R>, _state: State<'_, CodeSnippetState>) {
    // you can access `MyState` here!
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("code-snipper")
        .invoke_handler(tauri::generate_handler![do_something])
        .setup(|app_handle| {
            // setup plugin specific state here
            app_handle.manage(CodeSnippetState::default());
            Ok(())
        })
        .build()
}
