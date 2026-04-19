project('SteamClipExporter', 'rust')

application_id = 'io.github.foriequal0.SteamClipExporter'
output_name = 'steam-clip-exporter'

dependency('gtk4', version : '>= 4.20')
dependency('libadwaita-1', version : '>= 1.8.0')

subdir('resources')

gnome = import('gnome')
cargo = find_program('cargo', required : true)
python = find_program('python3', 'python', required : true)

shim_output = meson.project_source_root() / 'target/release' / output_name
custom_target(
  output_name,
  command : [
    python, files('build-aux/cargo-build-artifact-dir-shim.py'), '--output', shim_output,
    cargo, 'build', '--release', '--artifact-dir', '@OUTDIR@',
  ],
  output : output_name,
  build_by_default : true,
  build_always_stale : true,
  console : true,
  install : true,
  install_dir : get_option('bindir'),
)

test('cargo_test',
     cargo,
     args : ['test'],
     workdir : meson.project_source_root(),
)

gnome.post_install(
    gtk_update_icon_cache: true,
    update_desktop_database : true,
)
