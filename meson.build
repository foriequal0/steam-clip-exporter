project('SteamClipExporter', 'rust')

application_id = 'io.github.foriequal0.SteamClipExporter'
target_name = 'steam-clip-exporter'
output_name = target_name
if host_machine.system() == 'windows'
    output_name = output_name + '.exe'
endif

dependency('gtk4', version : '>= 4.14')
dependency('libadwaita-1', version : '>= 1.8.0')

subdir('resources')

gnome = import('gnome')
cargo = find_program('cargo', required : true)
python = find_program('python3', 'python', required : true)

manifest_path = meson.project_source_root() / 'Cargo.toml'
target_dir = meson.project_build_root() / 'target'
profile = 'release'
cargo_common_args = [
  '--manifest-path', manifest_path,
  '--target-dir', target_dir,
  '--profile', profile,
]

custom_target(
  target_name,
  command : [
    python, files('build-aux/cargo-build-artifact-dir-shim.py'), '--output', target_dir / profile / output_name,
    cargo, 'build', cargo_common_args, '--artifact-dir', '@OUTDIR@',
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
     args : ['test', cargo_common_args],
     workdir : meson.project_source_root(),
)

if host_machine.system() == 'linux'
    gnome.post_install(
        gtk_update_icon_cache: true,
        update_desktop_database : true,
    )
endif

meson.add_dist_script(
  python,
  files('build-aux/dist-vendor.py'),
  '--manifest-path', manifest_path,
)
