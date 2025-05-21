use std::collections::{BTreeMap, HashMap};

use zellij_tile::prelude::*;

struct State {
    is_enabled: bool,
    mode: InputMode,
    panes: HashMap<usize, Vec<PaneInfo>>,
    tab_keep_prefix: String,
    tabs: Vec<TabInfo>,
    timer_running: bool,
    update_interval: f64,
}

impl Default for State {
    fn default() -> Self {
        Self {
            is_enabled: true,
            mode: InputMode::default(),
            panes: HashMap::default(),
            tab_keep_prefix: "!".to_owned(),
            tabs: Vec::default(),
            timer_running: false,
            update_interval: 0.5,
        }
    }
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);
        subscribe(&[
            EventType::ModeUpdate,
            EventType::PaneUpdate,
            EventType::TabUpdate,
            EventType::Timer,
        ]);
        self.load_configuration(configuration);
        self.start_timer();
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::ModeUpdate(mode_info) => self.mode = mode_info.mode,
            Event::PaneUpdate(pane_manifest) => self.panes = pane_manifest.panes,
            Event::TabUpdate(tabs) => self.tabs = tabs,
            Event::Timer(_t) => 'blk: {
                set_timeout(self.update_interval);

                if self.mode == InputMode::RenameTab {
                    break 'blk;
                }

                for (&tab_index, panes) in &self.panes {
                    match self.tabs.get(tab_index) {
                        Some(tab) if tab.name.starts_with(&self.tab_keep_prefix) => continue,
                        None => continue,
                        _ => (),
                    }

                    for pane in panes.iter().filter(|p| p.is_focused) {
                        let new_name = match pane.title.split_whitespace().next() {
                            Some(t) => t,
                            None => continue,
                        };
                        let new_name = match new_name.split('/').last() {
                            Some(t) => t,
                            None => continue,
                        };
                        rename_tab(tab_index as u32 + 1, new_name);
                    }
                }
            }
            _ => {
                eprintln!("Got unrecognized event: {:?}", event);
            }
        }

        false // We don't have any UI to render
    }
}

impl State {
    fn load_configuration(&mut self, configuration: BTreeMap<String, String>) {
        if let Some(enable) = configuration.get("enable") {
            self.is_enabled = matches!(enable.trim(), "true" | "t" | "yes" | "y" | "1");
        };
        if let Some(tab_keep_prefix) = configuration.get("tab_keep_prefix") {
            self.tab_keep_prefix = tab_keep_prefix.trim().to_owned();
        }
        if let Some(update_interval) = configuration.get("update_interval") {
            self.update_interval = update_interval.trim().parse::<f64>().unwrap();
        };
    }

    fn start_timer(&mut self) {
        if self.is_enabled && !self.timer_running {
            set_timeout(self.update_interval);
            self.timer_running = true;
        }
    }
}
