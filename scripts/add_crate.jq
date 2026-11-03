(.crates | map(.display_name)) as $n
| (.crates[] | select(.display_name == "kernel")) as $k
| .crates += [{
    display_name: "mt76r",
    root_module: $root,
    edition: "2021",
    is_workspace_member: true,
    cfg: $k.cfg,
    env: $k.env,
    deps: [
      {crate: ($n | index("core")),   name: "core"},
      {crate: ($n | index("kernel")), name: "kernel"}
    ]
  }]
