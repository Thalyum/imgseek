#
# imgseek - Insert description here
#
# Copyright (C) 2022 Paul-Erwan RIO <paulerwan.rio@proton.me>
#
#

project('imgseek', 'rust',
        version : '0.1.1',
        license : '',
        default_options : [
          'prefix=/usr/local',
          'sysconfdir=/etc',
        ],
        meson_version : '>= 0.50')

prefixdir = get_option('prefix')
bindir = join_paths(prefixdir, get_option('bindir'))
libdir = join_paths(prefixdir, get_option('libdir'))
sysconfdir = join_paths(prefixdir, get_option('sysconfdir'))
includedir = join_paths(prefixdir, get_option('includedir'))
datadir = join_paths(prefixdir, get_option('datadir'))

enable_man_pages = get_option('man-pages')
if enable_man_pages
  pandoc = find_program('pandoc', required : true)
  if pandoc.found()
    subdir('man')
  endif
endif

if get_option('buildtype').startswith('debug')
  cargo_mode = 'debug'
else
  cargo_mode = 'release'
endif

subdir('shell-completion')

cargo = find_program('cargo')
cargo_vendor = find_program('cargo-vendor', required: false)
cargo_wrapper = find_program('build-aux/cargo')

program = custom_target('imgseek',
                        build_by_default: true,
                        console: true,
                        output: ['imgseek'],
                        install: true,
                        install_dir: bindir,
                        command: [cargo_wrapper,
                                  '@CURRENT_SOURCE_DIR@',
                                  '@OUTPUT@',
                                  cargo_mode,
                                 ])

archive = meson.project_name() + '-' + meson.project_version()
distribute = run_target('distribute',
                        depends : program,
                        command : ['build-aux/distribute', archive])

status = [
  '@0@ @1@'.format(meson.project_name(), meson.project_version()),

  'prefix:         @0@'.format(prefixdir),
  'sysconfdir:     @0@'.format(sysconfdir),
  'libdir:         @0@'.format(libdir),
  'includedir:     @0@'.format(includedir),
  'bindir:         @0@'.format(bindir),
  '',
  'man:            @0@'.format(enable_man_pages),
]

message('\n         '.join(status))
