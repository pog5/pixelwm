#[macro_use]
extern crate penrose;


use penrose::{
    core::{
        //bindings::KeyEventHandler,
        config::Config,
        helpers::index_selectors,
        //manager::WindowManager,
    },
    logging_error_handler,
    xcb::new_xcb_backed_window_manager,
    Backward, Forward, Less, More, Selector
};

fn main() -> penrose::Result<()> {
    let mut config_builder = Config::default().builder();
    let config = config_builder
        .floating_classes(vec!["xscreensaver", "dmenu", "dunst"])
        .border_px(1)
        .gap_px(0)
        .top_bar(true)
        .show_bar(false)
        .focused_border("#bd93f9")
        .unwrap()
        .build()
        .expect("failed to build config");
    let key_bindings = gen_keybindings! {
        // Program launchers
        "M-p" => run_external!("dmenu_run");
        "M-S-Return" => run_external!("kitty");

        // Exit Penrose (important to remember this one!)
        "M-A-S-Escape" => run_internal!(exit);

        // client management
        "M-j" => run_internal!(cycle_client, Forward);
        "M-k" => run_internal!(cycle_client, Backward);
        "M-S-j" => run_internal!(drag_client, Forward);
        "M-Return" => run_internal!(drag_client, Forward);
        "M-S-k" => run_internal!(drag_client, Backward);
        "M-S-f" => run_internal!(toggle_client_fullscreen, &Selector::Focused);
        "M-S-c" => run_internal!(kill_client);

        // workspace management
        "M-Tab" => run_internal!(toggle_workspace);
        "M-A-period" => run_internal!(cycle_workspace, Forward);
        "M-A-comma" => run_internal!(cycle_workspace, Backward);

        // Layout management
        "M-grave" => run_internal!(cycle_layout, Forward);
        "M-S-grave" => run_internal!(cycle_layout, Backward);
        "M-A-Up" => run_internal!(update_max_main, More);
        "M-A-Down" => run_internal!(update_max_main, Less);
        "M-A-Right" => run_internal!(update_main_ratio, More);
        "M-A-Left" => run_internal!(update_main_ratio, Less);

        //Utils
        "F12" => run_external!("flameshot gui");

        map: { "1", "2", "3", "4", "5", "6", "7", "8", "9" } to index_selectors(9) => {
            "M-{}" => focus_workspace (REF);
            "M-S-{}" => client_to_workspace (REF);
        };
    };

    let mut wm = new_xcb_backed_window_manager(config, vec![], logging_error_handler())?;
    wm.grab_keys_and_run(key_bindings, map!{})
}
