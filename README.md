# tab-rename
*tab-rename* is a Zellij plugin to automatically rename your tab to the program
running (or the current working directory) on the focused pane. To manually set
a tab name, add `!` (can be configured) to the start of the name, e.g. `!editor`. 

# Configuration
|Key            |Type   |Description     |
|---------------|-------|----------------|
|enable         |boolean|Default: `true` |
|tab_keep_prefix|string |Default: `"!"`  |
|update_interval|float  |Default: `0.5`|
