#
# flash-img-seeker - Insert description here
#
# Copyright (C) 2022 Paul-Erwan RIO <paulerwan.rio@proton.me>
#
#

custom_target('flash-img-seeker.1',
              output : 'flash-img-seeker.1',
              input : 'flash-img-seeker.1.md',
              command : [pandoc,
                         '--output', '@OUTPUT@',
                         '--standalone',
                         '--from', 'markdown',
                         '--to', 'man',
                         '@INPUT@'],
              install : true,
              install_dir : join_paths(get_option('mandir'), 'man1'))
